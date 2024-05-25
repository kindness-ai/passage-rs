use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateUserEmailRequest {
    /// language of the email to send (optional)
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(rename = "magic_link_path", skip_serializing_if = "Option::is_none")]
    pub magic_link_path: Option<String>,
    #[serde(rename = "new_email")]
    pub new_email: String,
    #[serde(rename = "redirect_url", skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
}

impl UpdateUserEmailRequest {
    pub fn new(new_email: String) -> UpdateUserEmailRequest {
        UpdateUserEmailRequest {
            language: None,
            magic_link_path: None,
            new_email,
            redirect_url: None,
        }
    }
}
