use crate::utils;

#[get("/album?<album>&<artist>")]
pub fn get_album_details(
    config: &State<AppConfig>,
    album: &str,
    artist: &str,
) -> Result<Json<AlbumDetails>, status::Custom<String>> {
    match utils::spotify_creds::access_token(config.spotify_auth_endpoint) {
        Ok(access_token) => {
            let album_Details = AlbumDetails {};
            Ok(Json(album_Details))
        }
        Err(e) => return Err(status::Custom(Status::ServerError, "")),
    }
}
