// Author: Joncantplay
// Decode modern stream formats through the minimal bundled FFmpeg build.
use std::{
    io::{BufRead, BufReader, Read},
    path::PathBuf,
    process::{Command, Stdio},
    sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, Arc, Mutex, OnceLock},
    time::Duration,
};

use super::{Senders, StreamPacket};

#[cfg(windows)]
#[derive(rust_embed::RustEmbed)]
#[folder = "resources/codecs"]
struct CodecAssets;

fn executable() -> Result<PathBuf, String> {
    static EXECUTABLE: OnceLock<Result<PathBuf, String>> = OnceLock::new();
    EXECUTABLE.get_or_init(|| {
        #[cfg(windows)]
        {
            let embedded = CodecAssets::get("ffmpeg.exe")
                .ok_or_else(|| "unsupported: Bundled modern-stream decoder is missing".to_string())?;
            let folder = std::env::temp_dir().join(format!("live_radio_codecs_{}", std::process::id()));
            std::fs::create_dir_all(&folder).map_err(|e| format!("decode: Could not create decoder folder: {e}"))?;
            let path = folder.join("ffmpeg.exe");
            let existing = std::fs::read(&path).ok();
            if existing.as_deref() != Some(embedded.data.as_ref()) {
                std::fs::write(&path, embedded.data.as_ref())
                    .map_err(|e| format!("decode: Could not extract modern-stream decoder: {e}"))?;
            }
            Ok(path)
        }
        #[cfg(not(windows))]
        { Ok(PathBuf::from("ffmpeg")) }
    }).clone()
}

fn classify(message: &str) -> String {
    let lower = message.to_ascii_lowercase();
    let category = if lower.contains("redirect") {
        "redirect"
    } else if lower.contains("timed out") || lower.contains("timeout") {
        "timeout"
    } else if lower.contains("http error") || lower.contains("server returned") {
        "http"
    } else if lower.contains("connection refused") || lower.contains("resolve") || lower.contains("network is unreachable") {
        "connect"
    } else if lower.contains("not found") || lower.contains("unsupported") || lower.contains("invalid data") || lower.contains("does not contain any stream") {
        "unsupported"
    } else { "decode" };
    format!("{category}: {}", message.chars().take(400).collect::<String>())
}

pub fn decode(url: &str, count: &Arc<AtomicUsize>, senders: &Senders) -> Result<(), String> {
    if !(url.starts_with("http://") || url.starts_with("https://")) {
        return Err("unsupported: A direct HTTP(S) stream URL is required".to_string());
    }
    let mut command = Command::new(executable()?);
    command.args([
        "-hide_banner", "-nostdin", "-loglevel", "info",
        "-rw_timeout", "10000000", "-protocol_whitelist", "http,https,tcp,tls,crypto",
        "-re", "-i", url, "-map", "0:a:0", "-vn", "-sn", "-dn",
        "-ac", "1", "-ar", "48000", "-acodec", "pcm_f32le", "-f", "f32le", "pipe:1",
    ]).stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
    }
    let mut process = command.spawn().map_err(|e| format!("decode: Could not start modern-stream decoder: {e}"))?;
    let Some(mut audio) = process.stdout.take() else {
        let _ = process.kill(); let _ = process.wait();
        return Err("decode: Decoder audio pipe is unavailable".to_string());
    };
    let Some(stderr) = process.stderr.take() else {
        let _ = process.kill(); let _ = process.wait();
        return Err("decode: Decoder status pipe is unavailable".to_string());
    };
    let process = Arc::new(Mutex::new(process));
    let errors = Arc::new(Mutex::new(String::new()));
    let stopped = Arc::new(AtomicBool::new(false));
    let monitor_process = process.clone();
    let monitor_count = count.clone();
    let monitor_stopped = stopped.clone();
    let monitor = std::thread::spawn(move || {
        while !monitor_stopped.load(Ordering::Relaxed) {
            if monitor_count.load(Ordering::Relaxed) == 0 {
                if let Ok(mut process) = monitor_process.lock() { let _ = process.kill(); }
                break;
            }
            std::thread::sleep(Duration::from_millis(200));
        }
    });
    let decoder_errors = errors.clone();
    let metadata_senders = senders.clone();
    let messages = std::thread::spawn(move || {
        let mut previous_title = String::new();
        for line in BufReader::new(stderr).lines().map_while(Result::ok) {
            if let Some((_, title)) = line.split_once("StreamTitle") {
                let title = title.trim_start_matches([' ', ':', '=']).trim().trim_matches('\'');
                if !title.is_empty() && title != previous_title {
                    previous_title = title.to_string();
                    if let Ok(senders) = metadata_senders.0.read() {
                        for sender in senders.iter() { let _ = sender.send(StreamPacket::Title(previous_title.clone())); }
                    }
                }
            }
            // Keep a bounded tail for failures; do not forward progress lines to RPT.
            if !line.trim().is_empty() && !line.starts_with("size=") {
                if let Ok(mut errors) = decoder_errors.lock() {
                    errors.push_str(&line); errors.push('\n');
                    if errors.len() > 4096 {
                        let mut cut = errors.len() - 4096;
                        while !errors.is_char_boundary(cut) { cut += 1; }
                        errors.drain(..cut);
                    }
                }
            }
        }
    });
    let mut buffer = [0_u8; 19200];
    let mut remainder = Vec::with_capacity(buffer.len() + 3);
    let mut samples_sent = 0_usize;
    let mut read_error = None;
    let mut cancelled = false;
    loop {
        if count.load(Ordering::Relaxed) == 0 { break; }
        match audio.read(&mut buffer) {
            Ok(0) => break,
            Ok(length) => {
                remainder.extend_from_slice(&buffer[..length]);
                let complete = remainder.len() / 4 * 4;
                let samples: Vec<alto::Mono<f32>> = remainder[..complete].chunks_exact(4)
                    .map(|b| alto::Mono { center: f32::from_le_bytes([b[0], b[1], b[2], b[3]]) }).collect();
                remainder.drain(..complete);
                if !samples.is_empty() {
                    samples_sent += samples.len();
                    if !senders.send_samples(samples, 48000) { cancelled = true; break; }
                }
            }
            Err(error) => { read_error = Some(format!("read: Decoder audio pipe failed: {error}")); break; }
        }
    }
    stopped.store(true, Ordering::Relaxed);
    let status = if let Ok(mut process) = process.lock() {
        // Kill before waiting on early cancellation or a failed pipe.
        if count.load(Ordering::Relaxed) == 0 || read_error.is_some() || cancelled { let _ = process.kill(); }
        process.wait().ok()
    } else { None };
    let _ = monitor.join();
    let _ = messages.join();
    if count.load(Ordering::Relaxed) == 0 { return Ok(()); }
    if let Some(error) = read_error { return Err(error); }
    if !status.is_some_and(|status| status.success()) || samples_sent == 0 {
        let detail = errors.lock().map(|s| s.lines().rev().take(3).collect::<Vec<_>>().into_iter().rev().collect::<Vec<_>>().join(" "))
            .unwrap_or_else(|_| "Decoder status unavailable".to_string());
        return Err(classify(&detail));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{io::{Read, Write}, net::TcpListener, sync::{atomic::AtomicUsize, Arc, RwLock}};
    use crossbeam_channel::unbounded;
    use super::{decode, Senders, StreamPacket};

    #[test]
    fn decodes_http_audio() {
        let server = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}/audio.aac", server.local_addr().unwrap());
        let fixture = include_bytes!("../../tests/fixtures/aac_lc_stereo.adts");
        let request = std::thread::spawn(move || {
            let (mut client, _) = server.accept().unwrap();
            client.set_read_timeout(Some(std::time::Duration::from_secs(10))).unwrap();
            let mut header = [0_u8; 4096];
            let _ = client.read(&mut header);
            write!(client, "HTTP/1.1 200 OK\r\nContent-Type: audio/aac\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", fixture.len()).unwrap();
            client.write_all(fixture).unwrap();
        });
        let (tx, rx) = unbounded();
        let senders = Senders(Arc::new(RwLock::new(vec![tx])));
        let count = Arc::new(AtomicUsize::new(1));
        let result = decode(&url, &count, &senders);
        // A decoder failure must fail the test before waiting for the server.
        result.unwrap();
        request.join().unwrap();
        let mut samples = 0;
        for packet in rx.try_iter() {
            if let StreamPacket::Data(audio, rate) = packet {
                assert_eq!(rate, 48000);
                assert!(audio.iter().all(|sample| sample.center.is_finite()));
                samples += audio.len();
            }
        }
        assert!(samples > 12000);
    }
}
