use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticateWebAuthnStartWithTransactionRequest {
    /// the transaction ID to associate with this authentication attempt
    /// (optional)
    #[serde(
        rename = "transaction_id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub transaction_id: Option<Option<String>>,
}

impl AuthenticateWebAuthnStartWithTransactionRequest {
    pub fn new() -> AuthenticateWebAuthnStartWithTransactionRequest {
        AuthenticateWebAuthnStartWithTransactionRequest {
            transaction_id: None,
        }
    }
}
