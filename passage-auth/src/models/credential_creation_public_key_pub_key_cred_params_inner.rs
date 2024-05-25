use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CredentialCreationPublicKeyPubKeyCredParamsInner {
    #[serde(rename = "alg", skip_serializing_if = "Option::is_none")]
    pub alg: Option<i32>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl CredentialCreationPublicKeyPubKeyCredParamsInner {
    pub fn new() -> CredentialCreationPublicKeyPubKeyCredParamsInner {
        CredentialCreationPublicKeyPubKeyCredParamsInner {
            alg: None,
            r#type: None,
        }
    }
}
