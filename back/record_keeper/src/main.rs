#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use rocket::{fs::FileServer, shield::Shield};
use rocket_cors::{AllowedOrigins, CorsOptions};
use std::env;
use utils::spotify_creds::{get_spotify_client_id, get_spotify_client_secret};

mod endpoints;
mod errors;
mod models;
mod utils;

use endpoints::{dev_proxy, get_album, health};
use models::app_config::AppConfig;

fn is_dev() -> bool {
    let value = env::var("ROCKET_ENV").unwrap_or_else(|_| "debug".into());

    rocket::debug!("{}", value);

    env::var("ROCKET_ENV")
        .unwrap_or_else(|_| "fuckyourustrocket".into())
        .to_lowercase()
        != "release".to_lowercase()
}

#[launch]
fn rocket() -> _ {
    // load the env from the .env file
    dotenv().ok();

    // check the spotify secrets as a sanity test
    get_spotify_client_id();
    get_spotify_client_secret();

    let app_config = AppConfig {
        vite_dev_server_port: &5173,
        spotify_auth_endpoint: "https://accounts.spotify.com/api/token",
        spotify_api_endpoint: "https://api.spotify.com/v1",
    };

    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    let mut rocket = rocket::build()
        .attach(cors)
        .attach(Shield::new())
        .manage(app_config);

    rocket = rocket.mount(
        "/api",
        routes![health::health_check, get_album::get_album_details],
    );

    if is_dev() {
        rocket = rocket.mount("/", routes![dev_proxy::vite_proxy]);
    } else {
        rocket = rocket.mount(
            "/",
            FileServer::from(concat!(env!("CARGO_MANIFEST_DIR"), "/dist")),
        );
    }

    rocket
}
