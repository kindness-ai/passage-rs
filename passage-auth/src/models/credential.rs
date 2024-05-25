use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Credential {
    /// The first time this webAuthn device was used to authenticate the user
    #[serde(rename = "created_at")]
    pub created_at: String,
    /// The CredID for this webAuthn device (encoded to match what is stored in
    /// psg_cred_obj)
    #[serde(rename = "cred_id")]
    pub cred_id: String,
    /// The friendly name for the webAuthn device used to authenticate
    #[serde(rename = "friendly_name")]
    pub friendly_name: String,
    /// The ID of the webAuthn device used for authentication
    #[serde(rename = "id")]
    pub id: String,
    /// The last time this webAuthn device was used to authenticate the user
    #[serde(rename = "last_login_at")]
    pub last_login_at: String,
    #[serde(rename = "type")]
    pub r#type: models::WebAuthnType,
    /// The last time this webAuthn device was updated
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    /// How many times this webAuthn device has been used to authenticate the
    /// user
    #[serde(rename = "usage_count")]
    pub usage_count: i32,
    /// The UserID for this webAuthn device
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "icons")]
    pub icons: Box<models::WebAuthnIcons>,
}
