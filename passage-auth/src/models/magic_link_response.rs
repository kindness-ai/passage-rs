use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MagicLinkResponse {
    #[serde(rename = "magic_link")]
    pub magic_link: Box<models::MagicLink>,
}

impl MagicLinkResponse {
    pub fn new(magic_link: models::MagicLink) -> MagicLinkResponse {
        MagicLinkResponse {
            magic_link: Box::new(magic_link),
        }
    }
}
