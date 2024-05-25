use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct App {
    #[serde(rename = "allowed_identifier")]
    pub allowed_identifier: String,
    /// Deprecated Property. Please refer to `auth_methods` to view settings for
    /// individual authentication methods.
    #[serde(rename = "auth_fallback_method")]
    pub auth_fallback_method: AuthFallbackMethod,
    /// Deprecated Property. Please refer to `auth_methods` to view settings for
    /// individual authentication methods.
    #[serde(rename = "auth_fallback_method_ttl")]
    pub auth_fallback_method_ttl: i32,
    #[serde(rename = "auth_methods")]
    pub auth_methods: Box<models::AuthMethods>,
    #[serde(rename = "auth_origin")]
    pub auth_origin: String,
    #[serde(rename = "default_language")]
    pub default_language: String,
    #[serde(rename = "element_customization")]
    pub element_customization: Box<models::ElementCustomization>,
    #[serde(rename = "element_customization_dark")]
    pub element_customization_dark: Box<models::ElementCustomization>,
    #[serde(rename = "ephemeral")]
    pub ephemeral: bool,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "layouts")]
    pub layouts: Box<models::Layouts>,
    #[serde(rename = "login_url")]
    pub login_url: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "passage_branding")]
    pub passage_branding: bool,
    #[serde(rename = "public_signup")]
    pub public_signup: bool,
    #[serde(rename = "profile_management")]
    pub profile_management: bool,
    #[serde(rename = "redirect_url")]
    pub redirect_url: String,
    #[serde(rename = "require_email_verification")]
    pub require_email_verification: bool,
    #[serde(rename = "require_identifier_verification")]
    pub require_identifier_verification: bool,
    #[serde(rename = "required_identifier")]
    pub required_identifier: String,
    #[serde(rename = "rsa_public_key")]
    pub rsa_public_key: String,
    #[serde(rename = "session_timeout_length")]
    pub session_timeout_length: i32,
    #[serde(rename = "social_connections")]
    pub social_connections: Box<models::SocialConnections>,
    #[serde(rename = "user_metadata_schema")]
    pub user_metadata_schema: Vec<models::UserMetadataField>,
}

/// Deprecated Property. Please refer to `auth_methods` to view settings for
/// individual authentication methods.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AuthFallbackMethod {
    #[serde(rename = "magic_link")]
    MagicLink,
    #[serde(rename = "otp")]
    Otp,
    #[serde(rename = "none")]
    None,
}

impl Default for AuthFallbackMethod {
    fn default() -> AuthFallbackMethod {
        Self::MagicLink
    }
}
