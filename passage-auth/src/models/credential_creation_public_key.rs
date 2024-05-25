use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CredentialCreationPublicKey {
    #[serde(rename = "attestation", skip_serializing_if = "Option::is_none")]
    pub attestation: Option<String>,
    #[serde(
        rename = "authenticatorSelection",
        skip_serializing_if = "Option::is_none"
    )]
    pub authenticator_selection:
        Option<Box<models::CredentialCreationPublicKeyAuthenticatorSelection>>,
    #[serde(rename = "challenge", skip_serializing_if = "Option::is_none")]
    pub challenge: Option<String>,
    #[serde(rename = "excludeCredentials", skip_serializing_if = "Option::is_none")]
    pub exclude_credentials:
        Option<Vec<models::CredentialCreationPublicKeyExcludeCredentialsInner>>,
    #[serde(rename = "extensions", skip_serializing_if = "Option::is_none")]
    pub extensions: Option<serde_json::Value>,
    #[serde(rename = "pubKeyCredParams", skip_serializing_if = "Option::is_none")]
    pub pub_key_cred_params: Option<Vec<models::CredentialCreationPublicKeyPubKeyCredParamsInner>>,
    #[serde(rename = "rp", skip_serializing_if = "Option::is_none")]
    pub rp: Option<Box<models::CredentialCreationPublicKeyRp>>,
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<models::CredentialCreationPublicKeyUser>>,
}

impl CredentialCreationPublicKey {
    pub fn new() -> CredentialCreationPublicKey {
        CredentialCreationPublicKey {
            attestation: None,
            authenticator_selection: None,
            challenge: None,
            exclude_credentials: None,
            extensions: None,
            pub_key_cred_params: None,
            rp: None,
            timeout: None,
            user: None,
        }
    }
}
