use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CredentialCreationPublicKeyUser {
    /// A human-palatable name for the user account, intended only for display.
    /// For example, \"Alex P. Müller\" or \"田中 倫\". The Relying Party SHOULD
    /// let the user choose this, and SHOULD NOT restrict the choice more than
    /// necessary.
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// A serialized URL which resolves to an image associated with the entity.
    /// For example, this could be a user’s avatar or a Relying Party's logo.
    /// This URL MUST be an a priori authenticated URL. Authenticators MUST
    /// accept and store a 128-byte minimum length for an icon member’s value.
    /// Authenticators MAY ignore an icon member’s value if its length is
    /// greater than 128 bytes. The URL’s scheme MAY be \"data\" to avoid
    /// fetches of the URL, at the cost of needing more storage.  Deprecated:
    /// this has been removed from the specification recommendations.
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// ID is the user handle of the user account entity. To ensure secure operation, authentication and authorization decisions MUST be made on the basis of this id member, not the displayName nor name members. See Section 6.1 of [RFC8266](https://www.w3.org/TR/webauthn/#biblio-rfc8266).
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// A human-palatable name for the entity. Its function depends on what the
    /// PublicKeyCredentialEntity represents:  When inherited by
    /// PublicKeyCredentialRpEntity it is a human-palatable identifier for the
    /// Relying Party, intended only for display. For example, \"ACME
    /// Corporation\", \"Wonderful Widgets, Inc.\" or \"ОАО Примертех\".  When
    /// inherited by PublicKeyCredentialUserEntity, it is a human-palatable
    /// identifier for a user account. It is intended only for display, i.e.,
    /// aiding the user in determining the difference between user accounts with
    /// similar displayNames. For example, \"alexm\",
    /// \"alex.p.mueller@example.com\" or \"+14255551234\".
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CredentialCreationPublicKeyUser {
    pub fn new() -> CredentialCreationPublicKeyUser {
        CredentialCreationPublicKeyUser {
            display_name: None,
            icon: None,
            id: None,
            name: None,
        }
    }
}
