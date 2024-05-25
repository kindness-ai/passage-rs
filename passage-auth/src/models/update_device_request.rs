use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateDeviceRequest {
    #[serde(rename = "friendly_name")]
    pub friendly_name: String,
}

impl UpdateDeviceRequest {
    pub fn new(friendly_name: String) -> UpdateDeviceRequest {
        UpdateDeviceRequest { friendly_name }
    }
}
