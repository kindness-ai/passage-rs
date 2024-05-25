use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddDeviceStartResponse {
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

impl AddDeviceStartResponse {
    pub fn new(handshake: models::CredentialCreationChallenge) -> AddDeviceStartResponse {
        AddDeviceStartResponse {
            handshake: Box::new(handshake),
            user: None,
        }
    }
}
