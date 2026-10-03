// Original album artwork lookup by BrettMayson; error handling updated by Joncantplay.
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct ResultWrapper {
    results: Vec<AlbumSearchResult>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlbumSearchResult {
    #[serde(rename = "artworkUrl100")]
    artwork_url_100: String,
    #[serde(rename = "trackId")]
    track_id: u64,
}

pub fn search_album(track: &str) -> Option<String> {
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build().ok()?;
    let track = urlencoding::encode(track);
    let url = format!("https://itunes.apple.com/search?term={track}&media=music&limit=1");
    let text = client.get(url).send().ok()?.error_for_status().ok()?.text().ok()?;
    let result = serde_json::from_str::<ResultWrapper>(&text).ok()?.results.into_iter().next()?;
    let url = result.artwork_url_100.replace("100x100bb", "400x400bb");
    let tmp_dir = dirs::cache_dir()?.join("live_radio");
    std::fs::create_dir_all(&tmp_dir).ok()?;
    let tmp_file_path = tmp_dir.join(format!("{}.jpg", result.track_id));
    if !tmp_file_path.exists() {
        let bytes = client.get(url).send().ok()?.error_for_status().ok()?.bytes().ok()?;
        std::fs::write(&tmp_file_path, bytes).ok()?;
    }
    Some(tmp_file_path.to_string_lossy().to_string())
}
