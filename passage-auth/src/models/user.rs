use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub email: String,
    /// Whether or not the user's email has been verified
    pub email_verified: bool,
    pub id: String,
    pub phone: String,
    /// Whether or not the user's phone has been verified
    pub phone_verified: bool,
    pub status: models::UserStatus,
    pub user_metadata: Option<serde_json::Value>,
    pub webauthn: bool,
    pub webauthn_types: Vec<models::WebAuthnType>,
}
