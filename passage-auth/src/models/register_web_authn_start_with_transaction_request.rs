use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegisterWebAuthnStartWithTransactionRequest {
    /// the transaction ID to associate with this webauthn registration
    #[serde(rename = "transaction_id")]
    pub transaction_id: String,
    #[serde(
        rename = "authenticator_attachment",
        skip_serializing_if = "Option::is_none"
    )]
    pub authenticator_attachment: Option<models::AuthenticatorAttachment>,
}

impl RegisterWebAuthnStartWithTransactionRequest {
    pub fn new(transaction_id: String) -> RegisterWebAuthnStartWithTransactionRequest {
        RegisterWebAuthnStartWithTransactionRequest {
            transaction_id,
            authenticator_attachment: None,
        }
    }
}
