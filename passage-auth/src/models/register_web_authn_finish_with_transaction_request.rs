use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegisterWebAuthnFinishWithTransactionRequest {
    #[serde(rename = "handshake_id")]
    pub handshake_id: String,
    #[serde(rename = "handshake_response")]
    pub handshake_response: Box<models::CredentialCreationResponse>,
    /// the transaction ID used to finish this webauthn registration
    #[serde(rename = "transaction_id")]
    pub transaction_id: String,
}

impl RegisterWebAuthnFinishWithTransactionRequest {
    pub fn new(
        handshake_id: String,
        handshake_response: models::CredentialCreationResponse,
        transaction_id: String,
    ) -> RegisterWebAuthnFinishWithTransactionRequest {
        RegisterWebAuthnFinishWithTransactionRequest {
            handshake_id,
            handshake_response: Box::new(handshake_response),
            transaction_id,
        }
    }
}
