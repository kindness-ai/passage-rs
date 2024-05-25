use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthResponse {
    #[serde(rename = "auth_result")]
    pub auth_result: Box<models::AuthResult>,
}

impl AuthResponse {
    pub fn new(auth_result: models::AuthResult) -> AuthResponse {
        AuthResponse {
            auth_result: Box::new(auth_result),
        }
    }
}
