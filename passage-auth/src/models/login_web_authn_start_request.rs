use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoginWebAuthnStartRequest {
    /// valid email or E164 phone number
    #[serde(rename = "identifier", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}

impl LoginWebAuthnStartRequest {
    pub fn new() -> LoginWebAuthnStartRequest {
        LoginWebAuthnStartRequest { identifier: None }
    }
}
