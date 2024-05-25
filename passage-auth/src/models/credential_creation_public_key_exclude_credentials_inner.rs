use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CredentialCreationPublicKeyExcludeCredentialsInner {
    /// CredentialID The ID of a credential to allow/disallow.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The authenticator transports that can be used.
    #[serde(rename = "transports", skip_serializing_if = "Option::is_none")]
    pub transports: Option<Vec<String>>,
    /// The valid credential types.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl CredentialCreationPublicKeyExcludeCredentialsInner {
    pub fn new() -> CredentialCreationPublicKeyExcludeCredentialsInner {
        CredentialCreationPublicKeyExcludeCredentialsInner {
            id: None,
            transports: None,
            r#type: None,
        }
    }
}
