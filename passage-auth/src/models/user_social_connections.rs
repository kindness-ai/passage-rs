use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserSocialConnections {
    #[serde(rename = "apple", skip_serializing_if = "Option::is_none")]
    pub apple: Option<Box<models::AppleSocialConnection>>,
    #[serde(rename = "github", skip_serializing_if = "Option::is_none")]
    pub github: Option<Box<models::GithubSocialConnection>>,
    #[serde(rename = "google", skip_serializing_if = "Option::is_none")]
    pub google: Option<Box<models::GoogleSocialConnection>>,
}

impl UserSocialConnections {
    pub fn new() -> UserSocialConnections {
        UserSocialConnections {
            apple: None,
            github: None,
            google: None,
        }
    }
}
