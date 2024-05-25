use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CredentialCreationResponseResponse {
    /// AttestationObject is the byte slice version of attestationObject. This
    /// attribute contains an attestation object, which is opaque to, and
    /// cryptographically protected against tampering by, the client. The
    /// attestation object contains both authenticator data and an attestation
    /// statement. The former contains the AAGUID, a unique credential ID, and
    /// the credential public key. The contents of the attestation statement are
    /// determined by the attestation statement format used by the
    /// authenticator. It also contains any additional information that the
    /// Relying Party's server requires to validate the attestation statement,
    /// as well as to decode and validate the authenticator data along with the
    /// JSON-serialized client data.
    #[serde(rename = "attestationObject", skip_serializing_if = "Option::is_none")]
    pub attestation_object: Option<String>,
    /// From the spec https://www.w3.org/TR/webauthn/#dom-authenticatorresponse-clientdatajson This attribute contains a JSON serialization of the client data passed to the authenticator by the client in its call to either create() or get().
    #[serde(rename = "clientDataJSON", skip_serializing_if = "Option::is_none")]
    pub client_data_json: Option<String>,
    #[serde(rename = "transports", skip_serializing_if = "Option::is_none")]
    pub transports: Option<Vec<String>>,
}

impl CredentialCreationResponseResponse {
    pub fn new() -> CredentialCreationResponseResponse {
        CredentialCreationResponseResponse {
            attestation_object: None,
            client_data_json: None,
            transports: None,
        }
    }
}
