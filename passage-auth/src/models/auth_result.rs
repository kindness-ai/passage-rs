use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthResult {
    #[serde(rename = "auth_token")]
    pub auth_token: String,
    #[serde(rename = "redirect_url")]
    pub redirect_url: String,
    #[serde(rename = "refresh_token", skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(
        rename = "refresh_token_expiration",
        skip_serializing_if = "Option::is_none"
    )]
    pub refresh_token_expiration: Option<i32>,
}

impl AuthResult {
    pub fn new(auth_token: String, redirect_url: String) -> AuthResult {
        AuthResult {
            auth_token,
            redirect_url,
            refresh_token: None,
            refresh_token_expiration: None,
        }
    }
}
