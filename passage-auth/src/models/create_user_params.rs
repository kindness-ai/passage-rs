use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateUserParams {
    #[serde(rename = "identifier")]
    pub identifier: String,
    #[serde(rename = "user_metadata", skip_serializing_if = "Option::is_none")]
    pub user_metadata: Option<serde_json::Value>,
}

impl CreateUserParams {
    pub fn new(identifier: String) -> CreateUserParams {
        CreateUserParams {
            identifier,
            user_metadata: None,
        }
    }
}
