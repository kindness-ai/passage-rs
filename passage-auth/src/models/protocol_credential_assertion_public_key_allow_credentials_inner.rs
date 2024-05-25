use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProtocolCredentialAssertionPublicKeyAllowCredentialsInner {
    /// CredentialID The ID of a credential to allow/disallow.
    #[serde(rename = "id")]
    pub id: String,
    /// The authenticator transports that can be used.
    #[serde(rename = "transports", skip_serializing_if = "Option::is_none")]
    pub transports: Option<Vec<String>>,
    /// The valid credential types.
    #[serde(rename = "type")]
    pub r#type: String,
}

impl ProtocolCredentialAssertionPublicKeyAllowCredentialsInner {
    pub fn new(
        id: String,
        r#type: String,
    ) -> ProtocolCredentialAssertionPublicKeyAllowCredentialsInner {
        ProtocolCredentialAssertionPublicKeyAllowCredentialsInner {
            id,
            transports: None,
            r#type,
        }
    }
}
