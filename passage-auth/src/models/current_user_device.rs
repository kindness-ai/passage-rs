use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurrentUserDevice {
    #[serde(rename = "device")]
    pub device: Box<models::Credential>,
}

impl CurrentUserDevice {
    pub fn new(device: models::Credential) -> CurrentUserDevice {
        CurrentUserDevice {
            device: Box::new(device),
        }
    }
}
