// Author: Joncantplay
fn main() {
    println!("cargo:rerun-if-changed=resources/codecs/ffmpeg.exe");
    let target = std::env::var("TARGET").unwrap_or_default();
    if target.contains("windows") {
        assert!(
            std::fs::metadata("resources/codecs/ffmpeg.exe").is_ok_and(|file| file.len() > 1024),
            "The bundled modern-stream decoder is missing. Run tools/build-codecs.sh in an x64 MSVC/MSYS2 environment first."
        );
    }
}
