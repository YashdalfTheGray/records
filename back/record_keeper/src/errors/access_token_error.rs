use std::{error::Error, fmt::Display};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessTokenError {
    pub status_code: u16,
}

impl Error for AccessTokenError {}

impl AccessTokenError {
    pub fn new(status_code: u16) -> AccessTokenError {
        AccessTokenError { status_code }
    }
}

impl Display for AccessTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed to get access token from spotify. Status code: {}",
            self.status_code
        )
    }
}
