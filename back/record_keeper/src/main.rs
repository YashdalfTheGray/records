#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket_cors::{AllowedOrigins, CorsOptions};

mod endpoints;
mod models;

use endpoints::health;

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    rocket::build()
        .attach(cors)
        .mount("/", FileServer::from("front"))
        .mount("/api", routes![health::health_check])
}
