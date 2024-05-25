use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateMetadataRequest {
    #[serde(rename = "user_metadata", skip_serializing_if = "Option::is_none")]
    pub user_metadata: Option<serde_json::Value>,
}

impl UpdateMetadataRequest {
    pub fn new() -> UpdateMetadataRequest {
        UpdateMetadataRequest {
            user_metadata: None,
        }
    }
}
