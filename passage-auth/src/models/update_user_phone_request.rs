use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateUserPhoneRequest {
    /// language of the email to send (optional)
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(rename = "magic_link_path", skip_serializing_if = "Option::is_none")]
    pub magic_link_path: Option<String>,
    #[serde(rename = "new_phone", skip_serializing_if = "Option::is_none")]
    pub new_phone: Option<String>,
    #[serde(rename = "redirect_url", skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
}

impl UpdateUserPhoneRequest {
    pub fn new() -> UpdateUserPhoneRequest {
        UpdateUserPhoneRequest {
            language: None,
            magic_link_path: None,
            new_phone: None,
            redirect_url: None,
        }
    }
}
