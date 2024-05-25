use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoginMagicLinkRequest {
    /// valid email or E164 phone number
    #[serde(rename = "identifier")]
    pub identifier: String,
    /// language of the email or SMS to send (optional)
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// path relative to the app's auth_origin (optional)
    #[serde(rename = "magic_link_path", skip_serializing_if = "Option::is_none")]
    pub magic_link_path: Option<String>,
}

impl LoginMagicLinkRequest {
    pub fn new(identifier: String) -> LoginMagicLinkRequest {
        LoginMagicLinkRequest {
            identifier,
            language: None,
            magic_link_path: None,
        }
    }
}
