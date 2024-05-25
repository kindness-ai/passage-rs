use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CredentialCreation {
    #[serde(rename = "publicKey", skip_serializing_if = "Option::is_none")]
    pub public_key: Option<Box<models::CredentialCreationPublicKey>>,
}

impl CredentialCreation {
    pub fn new() -> CredentialCreation {
        CredentialCreation { public_key: None }
    }
}
