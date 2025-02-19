use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AccessTokenResponse {
    pub access_token: String,
    token_type: String,
    expires_in: u64,
    scope: String,
}

