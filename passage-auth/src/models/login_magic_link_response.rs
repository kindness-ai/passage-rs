use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoginMagicLinkResponse {
    #[serde(rename = "magic_link")]
    pub magic_link: Box<models::MagicLink>,
}

impl LoginMagicLinkResponse {
    pub fn new(magic_link: models::MagicLink) -> LoginMagicLinkResponse {
        LoginMagicLinkResponse {
            magic_link: Box::new(magic_link),
        }
    }
}
