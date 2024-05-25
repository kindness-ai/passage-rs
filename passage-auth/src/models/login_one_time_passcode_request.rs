use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoginOneTimePasscodeRequest {
    /// valid email or E164 phone number
    #[serde(rename = "identifier")]
    pub identifier: String,
    /// language of the email to send (optional)
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

impl LoginOneTimePasscodeRequest {
    pub fn new(identifier: String) -> LoginOneTimePasscodeRequest {
        LoginOneTimePasscodeRequest {
            identifier,
            language: None,
        }
    }
}
