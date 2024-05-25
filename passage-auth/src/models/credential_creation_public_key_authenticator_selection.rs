use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CredentialCreationPublicKeyAuthenticatorSelection {
    /// AuthenticatorAttachment If this member is present, eligible
    /// authenticators are filtered to only authenticators attached with the
    /// specified AuthenticatorAttachment enum.
    #[serde(
        rename = "authenticatorAttachment",
        skip_serializing_if = "Option::is_none"
    )]
    pub authenticator_attachment: Option<String>,
    /// RequireResidentKey this member describes the Relying Party's
    /// requirements regarding resident credentials. If the parameter is set to
    /// true, the authenticator MUST create a client-side-resident public key
    /// credential source when creating a public key credential.
    #[serde(rename = "requireResidentKey", skip_serializing_if = "Option::is_none")]
    pub require_resident_key: Option<bool>,
    /// ResidentKey this member describes the Relying Party's requirements
    /// regarding resident credentials per Webauthn Level 2.
    #[serde(rename = "residentKey", skip_serializing_if = "Option::is_none")]
    pub resident_key: Option<String>,
    /// UserVerification This member describes the Relying Party's requirements
    /// regarding user verification for the create() operation. Eligible
    /// authenticators are filtered to only those capable of satisfying this
    /// requirement.
    #[serde(rename = "userVerification", skip_serializing_if = "Option::is_none")]
    pub user_verification: Option<String>,
}

impl CredentialCreationPublicKeyAuthenticatorSelection {
    pub fn new() -> CredentialCreationPublicKeyAuthenticatorSelection {
        CredentialCreationPublicKeyAuthenticatorSelection {
            authenticator_attachment: None,
            require_resident_key: None,
            resident_key: None,
            user_verification: None,
        }
    }
}
