use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetMagicLinkStatusRequest {
    pub id: String,
}

impl GetMagicLinkStatusRequest {
    pub fn new(id: String) -> GetMagicLinkStatusRequest {
        GetMagicLinkStatusRequest { id }
    }
}
