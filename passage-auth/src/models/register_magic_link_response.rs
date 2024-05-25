use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegisterMagicLinkResponse {
    #[serde(rename = "magic_link")]
    pub magic_link: Box<models::MagicLink>,
}

impl RegisterMagicLinkResponse {
    pub fn new(magic_link: models::MagicLink) -> RegisterMagicLinkResponse {
        RegisterMagicLinkResponse {
            magic_link: Box::new(magic_link),
        }
    }
}
