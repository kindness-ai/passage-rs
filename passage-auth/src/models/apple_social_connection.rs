use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppleSocialConnection {
    /// The external ID of the Social Connection.
    #[serde(rename = "provider_id")]
    pub provider_id: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "last_login_at")]
    pub last_login_at: String,
    /// The email of connected social user.
    #[serde(rename = "provider_identifier")]
    pub provider_identifier: String,
}

impl AppleSocialConnection {
    pub fn new(
        provider_id: String,
        created_at: String,
        last_login_at: String,
        provider_identifier: String,
    ) -> AppleSocialConnection {
        AppleSocialConnection {
            provider_id,
            created_at,
            last_login_at,
            provider_identifier,
        }
    }
}
