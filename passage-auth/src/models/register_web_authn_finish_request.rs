use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegisterWebAuthnFinishRequest {
    #[serde(rename = "handshake_id")]
    pub handshake_id: String,
    #[serde(rename = "handshake_response")]
    pub handshake_response: Box<models::CredentialCreationResponse>,
    #[serde(rename = "user_id")]
    pub user_id: String,
}

impl RegisterWebAuthnFinishRequest {
    pub fn new(
        handshake_id: String,
        handshake_response: models::CredentialCreationResponse,
        user_id: String,
    ) -> RegisterWebAuthnFinishRequest {
        RegisterWebAuthnFinishRequest {
            handshake_id,
            handshake_response: Box::new(handshake_response),
            user_id,
        }
    }
}
