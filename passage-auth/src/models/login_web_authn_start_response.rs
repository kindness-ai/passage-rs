use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoginWebAuthnStartResponse {
    #[serde(rename = "handshake")]
    pub handshake: Box<models::CredentialAssertionChallenge>,
    #[serde(
        rename = "user",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub user: Option<Option<Box<models::User>>>,
}

impl LoginWebAuthnStartResponse {
    pub fn new(handshake: models::CredentialAssertionChallenge) -> LoginWebAuthnStartResponse {
        LoginWebAuthnStartResponse {
            handshake: Box::new(handshake),
            user: None,
        }
    }
}
