use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LyricsResponse {
    synced_lyrics: Option<String>,
}

pub fn fetch_lyrics(
    title: &str,
    artist: &str,
) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("Versync/0.1.0")
        .timeout(Duration::from_secs(10))
        .build()?;

    let response = client
        .get("https://lrclib.net/api/get")
        .query(&[("track_name", title), ("artist_name", artist)])
        .send()?;

    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(None);
    }

    let response = response.error_for_status()?;
    let data: LyricsResponse = response.json()?;

    Ok(data.synced_lyrics)
}
