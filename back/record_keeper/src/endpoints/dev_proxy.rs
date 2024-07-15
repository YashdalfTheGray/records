use rocket::response::Redirect;
use rocket::State;

use crate::models::app_config::AppConfig;

#[get("/<_..>", rank = 2)]
pub fn vite_proxy(config: &State<AppConfig>) -> Redirect {
    let host_and_port = format!("http://localhost:{}/", config.vite_dev_server_port);

    Redirect::to(host_and_port)
}
