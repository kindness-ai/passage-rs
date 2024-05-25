use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CredentialCreationResponse {
    #[serde(
        rename = "authenticatorAttachment",
        skip_serializing_if = "Option::is_none"
    )]
    pub authenticator_attachment: Option<String>,
    #[serde(
        rename = "clientExtensionResults",
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extension_results: Option<serde_json::Value>,
    /// ID is The credential's identifier. The requirements for the identifier
    /// are distinct for each type of credential. It might represent a username
    /// for username/password tuples, for example.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "rawId", skip_serializing_if = "Option::is_none")]
    pub raw_id: Option<String>,
    #[serde(rename = "response", skip_serializing_if = "Option::is_none")]
    pub response: Option<Box<models::CredentialCreationResponseResponse>>,
    #[serde(rename = "transports", skip_serializing_if = "Option::is_none")]
    pub transports: Option<Vec<String>>,
    /// Type is the value of the object's interface object's [[type]] slot,
    /// which specifies the credential type represented by this object. This
    /// should be type \"public-key\" for Webauthn credentials.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl CredentialCreationResponse {
    pub fn new() -> CredentialCreationResponse {
        CredentialCreationResponse {
            authenticator_attachment: None,
            client_extension_results: None,
            id: None,
            raw_id: None,
            response: None,
            transports: None,
            r#type: None,
        }
    }
}
