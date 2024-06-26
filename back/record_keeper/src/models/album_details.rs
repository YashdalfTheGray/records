use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Album {
    pub album_type: String,
    pub artists: Vec<Artist>,
    pub available_markets: Vec<String>,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: String,
    pub total_tracks: u32,
    pub tracks: Tracks,
    pub uri: String,
}

#[derive(Debug, Deserialize)]
pub struct Artist {
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub name: String,
    pub r#type: String,
    pub uri: String,
}

#[derive(Debug, Deserialize)]
pub struct ExternalUrls {
    pub spotify: String,
}

#[derive(Debug, Deserialize)]
pub struct Image {
    pub height: u32,
    pub url: String,
    pub width: u32,
}

#[derive(Debug, Deserialize)]
pub struct Tracks {
    pub items: Vec<Track>,
    pub total: u32,
}

#[derive(Debug, Deserialize)]
pub struct Track {
    pub artists: Vec<Artist>,
    pub disc_number: u32,
    pub duration_ms: u32,
    pub explicit: bool,
    pub href: String,
    pub id: String,
    pub name: String,
    pub preview_url: Option<String>,
    pub track_number: u32,
    pub uri: String,
}
