use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticateWebAuthnStartWithTransactionResponse {
    #[serde(rename = "handshake")]
    pub handshake: Box<models::CredentialAssertionChallenge>,
    /// the transaction ID provided for this authentication attempt, null if not
    /// provided
    #[serde(
        rename = "transaction_id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_id: Option<Option<String>>,
}

impl AuthenticateWebAuthnStartWithTransactionResponse {
    pub fn new(
        handshake: models::CredentialAssertionChallenge,
    ) -> AuthenticateWebAuthnStartWithTransactionResponse {
        AuthenticateWebAuthnStartWithTransactionResponse {
            handshake: Box::new(handshake),
            transaction_id: None,
        }
    }
}
