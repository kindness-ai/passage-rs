use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProtocolCredentialAssertionPublicKey {
    #[serde(rename = "allowCredentials", skip_serializing_if = "Option::is_none")]
    pub allow_credentials:
        Option<Vec<models::ProtocolCredentialAssertionPublicKeyAllowCredentialsInner>>,
    #[serde(rename = "challenge")]
    pub challenge: String,
    #[serde(rename = "extensions", skip_serializing_if = "Option::is_none")]
    pub extensions: Option<serde_json::Value>,
    #[serde(rename = "rpId", skip_serializing_if = "Option::is_none")]
    pub rp_id: Option<String>,
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    /// UserVerification This member describes the Relying Party's requirements
    /// regarding user verification for the create() operation. Eligible
    /// authenticators are filtered to only those capable of satisfying this
    /// requirement.
    #[serde(rename = "userVerification", skip_serializing_if = "Option::is_none")]
    pub user_verification: Option<String>,
}

impl ProtocolCredentialAssertionPublicKey {
    pub fn new(challenge: String) -> ProtocolCredentialAssertionPublicKey {
        ProtocolCredentialAssertionPublicKey {
            allow_credentials: None,
            challenge,
            extensions: None,
            rp_id: None,
            timeout: None,
            user_verification: None,
        }
    }
}
