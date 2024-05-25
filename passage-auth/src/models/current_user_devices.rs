use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurrentUserDevices {
    #[serde(rename = "devices")]
    pub devices: Vec<models::Credential>,
}

impl CurrentUserDevices {
    pub fn new(devices: Vec<models::Credential>) -> CurrentUserDevices {
        CurrentUserDevices { devices }
    }
}
