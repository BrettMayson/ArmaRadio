use std::io::{self, Read, Seek, SeekFrom};
use std::time::Duration;

use m3u8_rs::Playlist;
use reqwest::Url;
use reqwest::blocking::Client;
use symphonia::core::io::MediaSource;
use symphonia::core::probe::Hint;

/// The container format carried by HLS media segments, detected once from the first segment.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Container {
    MpegTs,
    Fmp4,
    /// A bare elementary stream (e.g. ADTS AAC or MP3 frames), not wrapped in a container.
    Raw,
}

fn detect_container(data: &[u8]) -> Container {
    if data.first() == Some(&0x47) {
        Container::MpegTs
    } else if data.len() >= 8 && &data[4..8] == b"ftyp" {
        Container::Fmp4
    } else {
        Container::Raw
    }
}

/// Extracts the raw audio elementary stream from MPEG-TS segments, tracking PAT/PMT state
/// across segments since HLS doesn't guarantee every segment repeats them.
#[derive(Default)]
struct TsDemuxer {
    pmt_pid: Option<u16>,
    audio_pid: Option<u16>,
}

impl TsDemuxer {
    fn process(&mut self, data: &[u8], out: &mut Vec<u8>) {
        let mut offset = 0;
        while offset + 188 <= data.len() {
            let packet = &data[offset..offset + 188];
            offset += 188;
            if packet[0] != 0x47 {
                continue;
            }
            let pid = (u16::from(packet[1] & 0x1F) << 8) | u16::from(packet[2]);
            let payload_unit_start = packet[1] & 0x40 != 0;
            let adaptation_field_control = (packet[3] >> 4) & 0x3;
            let payload_start = match adaptation_field_control {
                0b01 => 4,
                0b11 => {
                    let afl = packet[4] as usize;
                    5 + afl
                }
                _ => continue, // adaptation-only or reserved: no payload
            };
            if payload_start > packet.len() {
                continue;
            }
            let payload = &packet[payload_start..];

            if pid == 0x0000 {
                self.parse_pat(payload, payload_unit_start);
            } else if Some(pid) == self.pmt_pid {
                self.parse_pmt(payload, payload_unit_start);
            } else if Some(pid) == self.audio_pid {
                if payload_unit_start {
                    if let Some(es_start) = parse_pes_header(payload) {
                        out.extend_from_slice(&payload[es_start..]);
                    }
                } else {
                    out.extend_from_slice(payload);
                }
            }
        }
    }

    fn parse_pat(&mut self, payload: &[u8], payload_unit_start: bool) {
        if !payload_unit_start || payload.is_empty() {
            return;
        }
        let pointer = payload[0] as usize;
        let Some(section) = payload.get(1 + pointer..) else {
            return;
        };
        if section.len() < 12 {
            return;
        }
        let section_length = ((usize::from(section[1] & 0x0F)) << 8) | usize::from(section[2]);
        let end = (3 + section_length).min(section.len()).saturating_sub(4);
        let mut i = 8;
        while i + 4 <= end {
            let program_number = (u16::from(section[i]) << 8) | u16::from(section[i + 1]);
            let pid = (u16::from(section[i + 2] & 0x1F) << 8) | u16::from(section[i + 3]);
            if program_number != 0 {
                self.pmt_pid = Some(pid);
                break;
            }
            i += 4;
        }
    }

    fn parse_pmt(&mut self, payload: &[u8], payload_unit_start: bool) {
        if !payload_unit_start || payload.is_empty() {
            return;
        }
        let pointer = payload[0] as usize;
        let Some(section) = payload.get(1 + pointer..) else {
            return;
        };
        if section.len() < 12 {
            return;
        }
        let section_length = ((usize::from(section[1] & 0x0F)) << 8) | usize::from(section[2]);
        let end = (3 + section_length).min(section.len()).saturating_sub(4);
        let program_info_length =
            ((usize::from(section[10] & 0x0F)) << 8) | usize::from(section[11]);
        let mut i = 12 + program_info_length;
        while i + 5 <= end {
            let stream_type = section[i];
            let pid = (u16::from(section[i + 1] & 0x1F) << 8) | u16::from(section[i + 2]);
            let es_info_length =
                ((usize::from(section[i + 3] & 0x0F)) << 8) | usize::from(section[i + 4]);
            // 0x0F = AAC ADTS, 0x11 = AAC LATM/LOAS, 0x03/0x04 = MPEG audio
            if self.audio_pid.is_none() && matches!(stream_type, 0x0F | 0x11 | 0x03 | 0x04) {
                self.audio_pid = Some(pid);
            }
            i += 5 + es_info_length;
        }
    }
}

/// Parses a PES header and returns the offset at which the elementary stream payload starts.
fn parse_pes_header(payload: &[u8]) -> Option<usize> {
    if payload.len() < 9 || payload[0..3] != [0x00, 0x00, 0x01] {
        return None;
    }
    let header_data_length = payload[8] as usize;
    let es_start = 9 + header_data_length;
    (es_start <= payload.len()).then_some(es_start)
}

/// Reads a live or VOD HLS stream, resolving master playlists, following the media playlist's
/// rolling window, and feeding segment bytes to the decoder as a single continuous stream.
///
/// Encrypted segments (`EXT-X-KEY` other than `NONE`) and in-band ID3/title metadata are not
/// supported.
pub struct HlsSource {
    client: Client,
    media_playlist_url: Url,
    next_media_sequence: Option<u64>,
    current_map_uri: Option<String>,
    container: Option<Container>,
    ts_demuxer: TsDemuxer,
    target_duration_secs: u64,
    done: bool,
    pending: Vec<u8>,
    pending_pos: usize,
}

impl HlsSource {
    pub fn new(url: &str) -> Result<Self, String> {
        let client = Client::new();
        let mut media_playlist_url = Url::parse(url).map_err(|e| e.to_string())?;

        // Resolve master playlists down to a single media playlist.
        loop {
            let bytes = client
                .get(media_playlist_url.clone())
                .send()
                .and_then(reqwest::blocking::Response::bytes)
                .map_err(|e| e.to_string())?;
            match m3u8_rs::parse_playlist_res(&bytes) {
                Ok(Playlist::MasterPlaylist(master)) => {
                    let variant = master
                        .variants
                        .iter()
                        .find(|v| !v.is_i_frame)
                        .or_else(|| master.variants.first())
                        .ok_or("master playlist has no variants")?;
                    media_playlist_url = media_playlist_url
                        .join(&variant.uri)
                        .map_err(|e| e.to_string())?;
                }
                Ok(Playlist::MediaPlaylist(_)) => break,
                Err(e) => return Err(format!("{e:?}")),
            }
        }

        let mut source = Self {
            client,
            media_playlist_url,
            next_media_sequence: None,
            current_map_uri: None,
            container: None,
            ts_demuxer: TsDemuxer::default(),
            target_duration_secs: 6,
            done: false,
            pending: Vec::new(),
            pending_pos: 0,
        };
        source.refresh_and_queue().map_err(|e| e.to_string())?;
        Ok(source)
    }

    /// A probe hint based on the sniffed container of the first downloaded segment.
    pub fn hint(&self) -> Hint {
        let mut hint = Hint::new();
        if self.container == Some(Container::Fmp4) {
            hint.with_extension("mp4");
        }
        hint
    }

    /// Fetches the current playlist, downloads any new segments, and appends decodable bytes
    /// to the pending buffer. Returns `Ok(true)` if new data was queued.
    fn refresh_and_queue(&mut self) -> io::Result<bool> {
        let bytes = self
            .client
            .get(self.media_playlist_url.clone())
            .send()
            .map_err(io::Error::other)?
            .bytes()
            .map_err(io::Error::other)?;
        let playlist = match m3u8_rs::parse_playlist_res(&bytes) {
            Ok(Playlist::MediaPlaylist(playlist)) => playlist,
            Ok(Playlist::MasterPlaylist(_)) => {
                return Err(io::Error::other("playlist unexpectedly became a master playlist"));
            }
            Err(e) => return Err(io::Error::other(format!("{e:?}"))),
        };
        self.target_duration_secs = playlist.target_duration.max(1);

        let start_seq = playlist.media_sequence;
        let next_expected = self.next_media_sequence.unwrap_or(start_seq);
        let mut buf = Vec::new();

        for (i, segment) in playlist.segments.iter().enumerate() {
            let seq = start_seq + i as u64;
            if seq < next_expected {
                continue;
            }
            if let Some(key) = &segment.key
                && key.method != m3u8_rs::KeyMethod::None
            {
                return Err(io::Error::other("encrypted HLS streams are not supported"));
            }
            if let Some(map) = &segment.map
                && self.current_map_uri.as_deref() != Some(map.uri.as_str())
            {
                let map_url = self
                    .media_playlist_url
                    .join(&map.uri)
                    .map_err(io::Error::other)?;
                let map_bytes = self
                    .client
                    .get(map_url)
                    .send()
                    .map_err(io::Error::other)?
                    .bytes()
                    .map_err(io::Error::other)?;
                buf.extend_from_slice(&map_bytes);
                self.current_map_uri = Some(map.uri.clone());
                self.container.get_or_insert(Container::Fmp4);
            }
            let segment_url = self
                .media_playlist_url
                .join(&segment.uri)
                .map_err(io::Error::other)?;
            let segment_bytes = self
                .client
                .get(segment_url)
                .send()
                .map_err(io::Error::other)?
                .bytes()
                .map_err(io::Error::other)?;
            let container = *self
                .container
                .get_or_insert_with(|| detect_container(&segment_bytes));
            match container {
                Container::MpegTs => self.ts_demuxer.process(&segment_bytes, &mut buf),
                Container::Fmp4 | Container::Raw => buf.extend_from_slice(&segment_bytes),
            }
            self.next_media_sequence = Some(seq + 1);
        }

        let consumed_all = self
            .next_media_sequence
            .is_some_and(|next| next >= start_seq + playlist.segments.len() as u64);
        if playlist.end_list && consumed_all {
            self.done = true;
        }

        let queued = !buf.is_empty();
        if queued {
            self.pending = buf;
            self.pending_pos = 0;
        }
        Ok(queued)
    }
}

impl Read for HlsSource {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        loop {
            if self.pending_pos < self.pending.len() {
                let remaining = &self.pending[self.pending_pos..];
                let n = remaining.len().min(buf.len());
                buf[..n].copy_from_slice(&remaining[..n]);
                self.pending_pos += n;
                return Ok(n);
            }
            if self.done {
                return Ok(0);
            }
            if !self.refresh_and_queue()? {
                if self.done {
                    return Ok(0);
                }
                // Live playlist with no new segments yet; wait roughly half a segment duration.
                std::thread::sleep(Duration::from_secs(self.target_duration_secs.max(2) / 2));
            }
        }
    }
}

impl Seek for HlsSource {
    fn seek(&mut self, _pos: SeekFrom) -> io::Result<u64> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "seeking is not supported on an HLS stream",
        ))
    }
}

impl MediaSource for HlsSource {
    fn is_seekable(&self) -> bool {
        false
    }

    fn byte_len(&self) -> Option<u64> {
        None
    }
}
