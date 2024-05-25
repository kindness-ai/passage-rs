use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivateMagicLinkRequest {
    pub magic_link: String,
}

impl ActivateMagicLinkRequest {
    pub fn new(magic_link: String) -> ActivateMagicLinkRequest {
        ActivateMagicLinkRequest { magic_link }
    }
}
