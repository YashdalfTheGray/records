use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use reqwest::header::AUTHORIZATION;
use std::env;
use std::error::Error;

use crate::errors::access_token_error::AccessTokenError;
use crate::models::client_token_response::ClientTokenResponse;

pub fn get_spotify_client_id() -> String {
    env::var("SPOTIFY_CLIENT_ID").unwrap()
}

pub fn get_spotify_client_secret() -> String {
    env::var("SPOTIFY_CLIENT_SECRET").unwrap()
}

pub fn client_credential_token() -> String {
    let id = get_spotify_client_id().to_owned();
    let secret = get_spotify_client_secret().to_owned();

    URL_SAFE.encode(format!("{id}:{secret}"))
}

pub fn access_token(spotify_auth_url: String) -> Result<String, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(spotify_auth_url)
        .header(AUTHORIZATION, client_credential_token())
        .send()?;

    if response.status().is_success() {
        Ok(response.json::<ClientTokenResponse>()?.access_token)
    } else {
        Err(Box::new(AccessTokenError::new(response.status().as_u16())))
    }
}
