use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegisterWebAuthnStartWithTransactionResponse {
    #[serde(rename = "handshake")]
    pub handshake: Box<models::CredentialCreationChallenge>,
    /// the transaction ID used to start this webauthn registration
    #[serde(rename = "transaction_id", skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

impl RegisterWebAuthnStartWithTransactionResponse {
    pub fn new(
        handshake: models::CredentialCreationChallenge,
    ) -> RegisterWebAuthnStartWithTransactionResponse {
        RegisterWebAuthnStartWithTransactionResponse {
            handshake: Box::new(handshake),
            transaction_id: None,
        }
    }
}
