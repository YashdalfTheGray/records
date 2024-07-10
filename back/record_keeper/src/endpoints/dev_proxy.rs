use rocket::response::Redirect;
use rocket::State;

use crate::models::{app_config::AppConfig, host_details::HostIpAddress};

#[get("/<_..>", rank = 2)]
pub fn vite_proxy(config: &State<AppConfig>, host: HostIpAddress) -> Redirect {
    print!("{}", host);
    let host_and_port = format!("http://{}:{}/", host, config.vite_dev_server_port);

    Redirect::to(host_and_port)
}
