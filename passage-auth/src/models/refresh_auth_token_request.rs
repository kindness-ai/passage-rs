use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RefreshAuthTokenRequest {
    #[serde(rename = "refresh_token")]
    pub refresh_token: String,
}

impl RefreshAuthTokenRequest {
    pub fn new(refresh_token: String) -> RefreshAuthTokenRequest {
        RefreshAuthTokenRequest { refresh_token }
    }
}
