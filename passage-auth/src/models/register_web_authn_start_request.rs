use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegisterWebAuthnStartRequest {
    /// valid email or E164 phone number
    #[serde(rename = "identifier")]
    pub identifier: String,
    #[serde(
        rename = "authenticator_attachment",
        skip_serializing_if = "Option::is_none"
    )]
    pub authenticator_attachment: Option<models::AuthenticatorAttachment>,
}

impl RegisterWebAuthnStartRequest {
    pub fn new(identifier: String) -> RegisterWebAuthnStartRequest {
        RegisterWebAuthnStartRequest {
            identifier,
            authenticator_attachment: None,
        }
    }
}
