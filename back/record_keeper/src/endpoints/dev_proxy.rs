use rocket::http::uri::Absolute;
use rocket::response::Redirect;
use rocket::State;
use std::env;

use crate::models::app_config::AppConfig;

#[get("/<_..>", rank = 2)]
pub fn vite_proxy(config: &State<AppConfig>) -> Redirect {
    let absolute_uri = Absolute::parse(&config.vite_dev_server).unwrap();
    Redirect::to(absolute_uri)
}
