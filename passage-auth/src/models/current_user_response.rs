use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurrentUserResponse {
    #[serde(rename = "user")]
    pub user: Box<models::CurrentUser>,
}

impl CurrentUserResponse {
    pub fn new(user: models::CurrentUser) -> CurrentUserResponse {
        CurrentUserResponse {
            user: Box::new(user),
        }
    }
}
