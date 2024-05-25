use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddDeviceFinishRequest {
    #[serde(rename = "handshake_id")]
    pub handshake_id: String,
    #[serde(rename = "handshake_response")]
    pub handshake_response: Box<models::CredentialCreationResponse>,
    #[serde(rename = "user_id")]
    pub user_id: String,
}

impl AddDeviceFinishRequest {
    pub fn new(
        handshake_id: String,
        handshake_response: models::CredentialCreationResponse,
        user_id: String,
    ) -> AddDeviceFinishRequest {
        AddDeviceFinishRequest {
            handshake_id,
            handshake_response: Box::new(handshake_response),
            user_id,
        }
    }
}
