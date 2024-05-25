use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SocialConnections {
    #[serde(rename = "apple", skip_serializing_if = "Option::is_none")]
    pub apple: Option<serde_json::Value>,
    #[serde(rename = "google", skip_serializing_if = "Option::is_none")]
    pub google: Option<serde_json::Value>,
    #[serde(rename = "github", skip_serializing_if = "Option::is_none")]
    pub github: Option<serde_json::Value>,
}

impl SocialConnections {
    pub fn new() -> SocialConnections {
        SocialConnections {
            apple: None,
            google: None,
            github: None,
        }
    }
}
