use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoginWebAuthnFinishRequest {
    #[serde(rename = "handshake_id")]
    pub handshake_id: String,
    #[serde(rename = "handshake_response")]
    pub handshake_response: Box<models::CredentialAssertionResponse>,
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl LoginWebAuthnFinishRequest {
    pub fn new(
        handshake_id: String,
        handshake_response: models::CredentialAssertionResponse,
    ) -> LoginWebAuthnFinishRequest {
        LoginWebAuthnFinishRequest {
            handshake_id,
            handshake_response: Box::new(handshake_response),
            user_id: None,
        }
    }
}
