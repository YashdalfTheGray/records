use std::fmt::Display;

use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};
use serde::{Deserialize, Serialize};

pub struct HostIpAddress<'r>(&'r str);

#[derive(Debug, Deserialize, Serialize)]
pub enum HostIpError {
    Missing,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for HostIpAddress<'r> {
    type Error = HostIpError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("Host") {
            None => Outcome::Error((Status::BadRequest, HostIpError::Missing)),
            Some(ip) => Outcome::Success(HostIpAddress(ip)),
        }
    }
}

impl<'r> Display for HostIpAddress<'r> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
