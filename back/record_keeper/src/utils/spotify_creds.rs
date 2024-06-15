use std::env;

pub fn get_spotify_client_id() -> String {
    env::var("SPOTIFY_CLIENT_ID").unwrap()
}

pub fn get_spotify_client_secret() -> String {
    env::var("SPOTIFY_CLIENT_SECRET").unwrap()
}
