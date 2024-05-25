use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurrentUser {
    /// When this user was created
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// The user's email
    #[serde(rename = "email")]
    pub email: String,
    /// Whether or not the user's email has been verified
    #[serde(rename = "email_verified")]
    pub email_verified: bool,
    /// The user ID
    #[serde(rename = "id")]
    pub id: String,
    /// The last time this user logged in
    #[serde(rename = "last_login_at")]
    pub last_login_at: String,
    /// How many times the user has successfully logged in
    #[serde(rename = "login_count")]
    pub login_count: i32,
    /// The user's phone
    #[serde(rename = "phone")]
    pub phone: String,
    /// Whether or not the user's phone has been verified
    #[serde(rename = "phone_verified")]
    pub phone_verified: bool,
    #[serde(rename = "social_connections")]
    pub social_connections: Box<models::UserSocialConnections>,
    #[serde(rename = "status")]
    pub status: models::UserStatus,
    /// When this user was last updated
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "user_metadata", deserialize_with = "Option::deserialize")]
    pub user_metadata: Option<serde_json::Value>,
    /// Whether or not the user has authenticated via webAuthn before (if
    /// len(WebAuthnDevices) > 0)
    #[serde(rename = "webauthn")]
    pub webauthn: bool,
    /// The list of devices this user has authenticated with via webAuthn
    #[serde(rename = "webauthn_devices")]
    pub webauthn_devices: Vec<models::Credential>,
    /// List of credential types that user has created
    #[serde(rename = "webauthn_types")]
    pub webauthn_types: Vec<models::WebAuthnType>,
}
