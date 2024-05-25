use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CredentialAssertionResponseResponse {
    #[serde(rename = "authenticatorData", skip_serializing_if = "Option::is_none")]
    pub authenticator_data: Option<String>,
    /// From the spec https://www.w3.org/TR/webauthn/#dom-authenticatorresponse-clientdatajson This attribute contains a JSON serialization of the client data passed to the authenticator by the client in its call to either create() or get().
    #[serde(rename = "clientDataJSON", skip_serializing_if = "Option::is_none")]
    pub client_data_json: Option<String>,
    #[serde(rename = "signature", skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(rename = "userHandle", skip_serializing_if = "Option::is_none")]
    pub user_handle: Option<String>,
}

impl CredentialAssertionResponseResponse {
    pub fn new() -> CredentialAssertionResponseResponse {
        CredentialAssertionResponseResponse {
            authenticator_data: None,
            client_data_json: None,
            signature: None,
            user_handle: None,
        }
    }
}
