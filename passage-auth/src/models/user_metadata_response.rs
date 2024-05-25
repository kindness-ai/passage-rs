use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserMetadataResponse {
    #[serde(rename = "user_metadata")]
    pub user_metadata: serde_json::Value,
}

impl UserMetadataResponse {
    pub fn new(user_metadata: serde_json::Value) -> UserMetadataResponse {
        UserMetadataResponse { user_metadata }
    }
}
