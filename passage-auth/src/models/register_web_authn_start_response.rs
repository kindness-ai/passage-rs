use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegisterWebAuthnStartResponse {
    #[serde(rename = "handshake")]
    pub handshake: Box<models::CredentialCreationChallenge>,
    #[serde(
        rename = "user",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub user: Option<Option<Box<models::User>>>,
}

impl RegisterWebAuthnStartResponse {
    pub fn new(handshake: models::CredentialCreationChallenge) -> RegisterWebAuthnStartResponse {
        RegisterWebAuthnStartResponse {
            handshake: Box::new(handshake),
            user: None,
        }
    }
}
