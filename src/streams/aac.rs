// Author: Joncantplay
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use symphonia::core::{
    audio::SampleBuffer,
    codecs::DecoderOptions,
    errors::Error,
    formats::FormatOptions,
    io::{MediaSource, MediaSourceStream},
    meta::MetadataOptions,
    probe::Hint,
};

use super::Senders;

fn decode_media(
    media: Box<dyn MediaSource>,
    count: &Arc<AtomicUsize>,
    senders: &Senders,
) -> Result<(), String> {
    let source = MediaSourceStream::new(media, Default::default());
    let mut hint = Hint::new();
    hint.with_extension("aac");

    let probed = symphonia::default::get_probe()
        .format(
            &hint,
            source,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .map_err(|error| format!("could not read ADTS stream: {error}"))?;
    let mut format = probed.format;
    let track = format.default_track().ok_or("no AAC track found")?;
    let track_id = track.id;
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &DecoderOptions::default())
        .map_err(|error| format!("could not initialize AAC decoder: {error}"))?;

    while count.load(Ordering::Relaxed) > 0 {
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(Error::IoError(error)) if error.kind() == std::io::ErrorKind::UnexpectedEof => {
                break
            }
            Err(Error::DecodeError(error)) => {
                debug!("Skipping invalid AAC packet: {error}");
                continue;
            }
            Err(error) => return Err(format!("could not read AAC packet: {error}")),
        };
        if packet.track_id() != track_id {
            continue;
        }

        let decoded = match decoder.decode(&packet) {
            Ok(decoded) => decoded,
            Err(Error::DecodeError(error)) => {
                debug!("Skipping invalid AAC audio: {error}");
                continue;
            }
            Err(error) => return Err(format!("could not decode AAC audio: {error}")),
        };

        let channels = decoded.spec().channels.count();
        if channels == 0 || decoded.frames() == 0 {
            continue;
        }
        let sample_rate = decoded.spec().rate as i32;
        let mut pcm = SampleBuffer::<f32>::new(decoded.capacity() as u64, *decoded.spec());
        pcm.copy_interleaved_ref(decoded);
        let samples = pcm
            .samples()
            .chunks_exact(channels)
            .map(|frame| alto::Mono {
                center: frame.iter().copied().sum::<f32>() / channels as f32,
            })
            .collect();
        if !senders.send_samples(samples, sample_rate) {
            return Err("could not deliver AAC audio to listeners".to_string());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{
        io::{Cursor, Read, Seek, SeekFrom},
        sync::{atomic::AtomicUsize, Arc, RwLock},
    };

    use crossbeam_channel::unbounded;
    use symphonia::core::io::MediaSource;

    use super::{decode_media, Senders};
    use crate::streams::StreamPacket;

    struct LiveBytes(Cursor<Vec<u8>>);

    impl Read for LiveBytes {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            self.0.read(buf)
        }
    }

    impl Seek for LiveBytes {
        fn seek(&mut self, _pos: SeekFrom) -> std::io::Result<u64> {
            Err(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "not seekable",
            ))
        }
    }

    impl MediaSource for LiveBytes {
        fn is_seekable(&self) -> bool {
            false
        }
        fn byte_len(&self) -> Option<u64> {
            None
        }
    }

    #[test]
    fn decodes_adts_without_seeking() {
        let bytes = include_bytes!("../../tests/fixtures/aac_lc_stereo.adts").to_vec();
        let (tx, rx) = unbounded();
        let senders = Senders(Arc::new(RwLock::new(vec![tx])));
        let count = Arc::new(AtomicUsize::new(1));

        decode_media(Box::new(LiveBytes(Cursor::new(bytes))), &count, &senders).unwrap();

        let mut frames = 0;
        while let Ok(packet) = rx.try_recv() {
            if let StreamPacket::Data(samples, rate) = packet {
                assert_eq!(rate, 44100);
                assert!(!samples.is_empty());
                frames += 1;
            }
        }
        assert!(frames > 10);
    }
}
