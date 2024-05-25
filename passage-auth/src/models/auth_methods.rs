use serde::{Deserialize, Serialize};

use crate::models;

/// AuthMethods : Denotes what methods this app is allowed to use for
/// authentication with configurations
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthMethods {
    #[serde(rename = "passkeys", skip_serializing_if = "Option::is_none")]
    pub passkeys: Option<serde_json::Value>,
    #[serde(rename = "otp", skip_serializing_if = "Option::is_none")]
    pub otp: Option<Box<models::OtpAuthMethod>>,
    #[serde(rename = "magic_link", skip_serializing_if = "Option::is_none")]
    pub magic_link: Option<Box<models::MagicLinkAuthMethod>>,
}

impl AuthMethods {
    /// Denotes what methods this app is allowed to use for authentication with
    /// configurations
    pub fn new() -> AuthMethods {
        AuthMethods {
            passkeys: None,
            otp: None,
            magic_link: None,
        }
    }
}
