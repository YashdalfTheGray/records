#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket_cors::{AllowedOrigins, CorsOptions};
use std::env;

mod endpoints;
mod models;

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
    // let vite_dev_server: &'static str =
    // env::var("vite_dev_server").unwrap_or_else(|_| "http://localhost:3000".into());
    let vite_dev_server: &'static str = "http://localhost:3000";
    let app_config = AppConfig {
        vite_dev_server: &vite_dev_server,
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
