use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreatePasskeyReadinessRequest {
    #[serde(rename = "device_id")]
    pub device_id: String,
    #[serde(rename = "security_key")]
    pub security_key: bool,
    #[serde(rename = "platform")]
    pub platform: bool,
    #[serde(rename = "synced_credential")]
    pub synced_credential: bool,
    #[serde(rename = "cross_device_credential")]
    pub cross_device_credential: bool,
    #[serde(rename = "conditional_ui")]
    pub conditional_ui: bool,
}

impl CreatePasskeyReadinessRequest {
    pub fn new(
        device_id: String,
        security_key: bool,
        platform: bool,
        synced_credential: bool,
        cross_device_credential: bool,
        conditional_ui: bool,
    ) -> CreatePasskeyReadinessRequest {
        CreatePasskeyReadinessRequest {
            device_id,
            security_key,
            platform,
            synced_credential,
            cross_device_credential,
            conditional_ui,
        }
    }
}
