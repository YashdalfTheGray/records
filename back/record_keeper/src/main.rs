#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use rocket::fs::FileServer;
use rocket_cors::{AllowedOrigins, CorsOptions};
use std::env;
use utils::spotify_creds::{get_spotify_client_id, get_spotify_client_secret};

mod endpoints;
mod models;
mod utils;

use endpoints::{dev_proxy, health};
use models::app_config::AppConfig;

fn is_dev() -> bool {
    env::var("ROCKET_ENV")
        .unwrap_or_else(|_| "development".into())
        .to_lowercase()
        == "development".to_lowercase()
}

#[launch]
fn rocket() -> _ {
    // load the env from the .env file
    dotenv().ok();

    // check the spotify secrets as a sanity test
    get_spotify_client_id();
    get_spotify_client_secret();

    // TODO still need to resolve this but we're moving on
    // let vite_dev_server: &'static str =
    // env::var("vite_dev_server").unwrap_or_else(|_| "http://localhost:5173".into());
    let vite_dev_server: &'static str = "http://localhost:5173";
    let app_config = AppConfig {
        vite_dev_server: &vite_dev_server,
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

    let mut rocket = rocket::build().attach(cors).manage(app_config);

    rocket = rocket.mount("/api", routes![health::health_check]);

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
