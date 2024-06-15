use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ClientTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: String,
}
