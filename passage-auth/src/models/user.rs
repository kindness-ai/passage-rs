use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "email")]
    pub email: String,
    /// Whether or not the user's email has been verified
    #[serde(rename = "email_verified")]
    pub email_verified: bool,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "phone")]
    pub phone: String,
    /// Whether or not the user's phone has been verified
    #[serde(rename = "phone_verified")]
    pub phone_verified: bool,
    #[serde(rename = "status")]
    pub status: models::UserStatus,
    #[serde(rename = "user_metadata", deserialize_with = "Option::deserialize")]
    pub user_metadata: Option<serde_json::Value>,
    #[serde(rename = "webauthn")]
    pub webauthn: bool,
    #[serde(rename = "webauthn_types")]
    pub webauthn_types: Vec<models::WebAuthnType>,
}
