use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegisterOneTimePasscodeRequest {
    /// valid email or E164 phone number
    #[serde(rename = "identifier")]
    pub identifier: String,
    /// language of the email to send (optional)
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

impl RegisterOneTimePasscodeRequest {
    pub fn new(identifier: String) -> RegisterOneTimePasscodeRequest {
        RegisterOneTimePasscodeRequest {
            identifier,
            language: None,
        }
    }
}
