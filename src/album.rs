// https://itunes.apple.com/search?callback=jQuery110205701247333515289_1790527953922&term=Rivo+%26+CLOVES+Forever+Till+The+End+(Radio+Edit)&media=music&limit=1&_=1790527953928

// {
//  "resultCount":1,
//  "results": [
// {"wrapperType":"track", "kind":"song", "artistId":1750904468, "collectionId":1862838502, "trackId":1862838503, "artistName":"Rivo & CLOVES", "collectionName":"Forever Till The End - Single", "trackName":"Forever Till The End", "collectionCensoredName":"Forever Till The End - Single", "trackCensoredName":"Forever Till The End", "artistViewUrl":"https://music.apple.com/us/artist/rivo/1750904468?uo=4", "collectionViewUrl":"https://music.apple.com/us/album/forever-till-the-end/1862838502?i=1862838503&uo=4", "trackViewUrl":"https://music.apple.com/us/album/forever-till-the-end/1862838502?i=1862838503&uo=4", 
// "previewUrl":"https://audio-ssl.itunes.apple.com/itunes-assets/AudioPreview211/v4/e6/05/4d/e6054d79-4619-ee4c-80d1-e647779794ed/mzaf_1947679214174151306.plus.aac.p.m4a", "artworkUrl30":"https://is1-ssl.mzstatic.com/image/thumb/Music221/v4/40/d2/a6/40d2a6d3-80e0-bbf5-2667-c2342f9d1aa2/198704581595_Cover.jpg/30x30bb.jpg", "artworkUrl60":"https://is1-ssl.mzstatic.com/image/thumb/Music221/v4/40/d2/a6/40d2a6d3-80e0-bbf5-2667-c2342f9d1aa2/198704581595_Cover.jpg/60x60bb.jpg", "artworkUrl100":"https://is1-ssl.mzstatic.com/image/thumb/Music221/v4/40/d2/a6/40d2a6d3-80e0-bbf5-2667-c2342f9d1aa2/198704581595_Cover.jpg/100x100bb.jpg", "collectionPrice":1.29, "trackPrice":1.29, "releaseDate":"2026-01-23T12:00:00Z", "collectionExplicitness":"notExplicit", "trackExplicitness":"notExplicit", "discCount":1, "discNumber":1, "trackCount":1, "trackNumber":1, "trackTimeMillis":223615, "country":"USA", "currency":"USD", "primaryGenreName":"Dance", "isStreamable":true}]
// }
// );

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
    let result = serde_json::from_str::<ResultWrapper>(&response).expect("Failed to parse response");
    let result = result.results.into_iter().next()?;
    let url = result.artwork_url_100.replace("100x100bb", "400x400bb");
    let track_id = result.track_id;
    // download the url to tmp with the track_id as the filename
    let tmp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let tmp_file_path = tmp_dir.path().join(format!("{track_id}.jpg"));
    let mut tmp_file = std::fs::File::create(&tmp_file_path).expect("Failed to create temp file");
    let mut response = reqwest::blocking::get(&url).expect("Failed to download image");
    std::io::copy(&mut response, &mut tmp_file).expect("Failed to write to temp file");

    Some(tmp_file_path.to_string_lossy().to_string())
}
