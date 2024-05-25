use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticateWebAuthnFinishWithTransactionRequest {
    #[serde(rename = "handshake_id")]
    pub handshake_id: String,
    #[serde(rename = "handshake_response")]
    pub handshake_response: Box<models::CredentialAssertionResponse>,
    /// the transaction ID used when starting this login attempt (optional)
    #[serde(
        rename = "transaction_id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_id: Option<Option<String>>,
}

impl AuthenticateWebAuthnFinishWithTransactionRequest {
    pub fn new(
        handshake_id: String,
        handshake_response: models::CredentialAssertionResponse,
    ) -> AuthenticateWebAuthnFinishWithTransactionRequest {
        AuthenticateWebAuthnFinishWithTransactionRequest {
            handshake_id,
            handshake_response: Box::new(handshake_response),
            transaction_id: None,
        }
    }
}
