use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserResponse {
    #[serde(rename = "user", deserialize_with = "Option::deserialize")]
    pub user: Option<Box<models::User>>,
}

impl UserResponse {
    pub fn new(user: Option<models::User>) -> UserResponse {
        UserResponse {
            user: user.map(Box::new),
        }
    }
}
