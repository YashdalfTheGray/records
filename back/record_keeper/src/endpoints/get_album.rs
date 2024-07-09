use std::{error::Error, fs::File, io::BufReader, path::Path};

use rocket::{http::Status, response::status, serde::json::Json, State};

use crate::{
    models::{album_details, app_config},
    utils,
};

#[get("/album?<album>&<artist>")]
pub fn get_album_details(
    config: &State<app_config::AppConfig>,
    album: &str,
    artist: &str,
) -> Result<Json<album_details::Album>, status::Custom<String>> {
    match utils::spotify_creds::access_token(config.spotify_auth_endpoint.to_string()) {
        Ok(access_token) => {
            let file_path = "resources/album.json";
            match read_album_from_file(file_path) {
                Ok(album_details) => Ok(Json(album_details)),
                Err(e) => Err(status::Custom(Status::InternalServerError, "".to_string())),
            }
        }
        Err(e) => Err(status::Custom(Status::InternalServerError, "".to_string())),
    }
}

fn read_album_from_file<P: AsRef<Path>>(path: P) -> Result<album_details::Album, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let album = serde_json::from_reader(reader)?;
    Ok(album)
}
