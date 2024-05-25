use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SocialConnectionsResponse {
    #[serde(rename = "social_connections")]
    pub social_connections: Box<models::UserSocialConnections>,
}

impl SocialConnectionsResponse {
    pub fn new(social_connections: models::UserSocialConnections) -> SocialConnectionsResponse {
        SocialConnectionsResponse {
            social_connections: Box::new(social_connections),
        }
    }
}
