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
    let track = urlencoding::encode(track);
    let url = format!("https://itunes.apple.com/search?term={track}&media=music&limit=1&_=1790527953928");
    let response = reqwest::blocking::get(&url).expect("Failed to send request").text().expect("Failed to read response");
    debug!("Response: {}", response);
    let result = serde_json::from_str::<ResultWrapper>(&response).expect("Failed to parse response");
    let result = result.results.into_iter().next()?;
    let url = result.artwork_url_100.replace("100x100bb", "400x400bb");
    let track_id = result.track_id;
    let tmp_dir = dirs::cache_dir().expect("Failed to get cache dir").join("live_radio");
    let tmp_file_path = tmp_dir.join(format!("{track_id}.jpg"));
    if !tmp_file_path.exists() {
        let mut tmp_file = std::fs::File::create(&tmp_file_path).expect("Failed to create temp file");
        let mut response = reqwest::blocking::get(&url).expect("Failed to download image");
        std::io::copy(&mut response, &mut tmp_file).expect("Failed to write to temp file");
    }

    Some(tmp_file_path.to_string_lossy().to_string())
}
