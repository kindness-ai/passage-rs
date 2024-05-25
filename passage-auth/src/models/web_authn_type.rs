use serde::{Deserialize, Serialize};

/// WebAuthnType : The type of this credential
/// The type of this credential
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WebAuthnType {
    #[serde(rename = "passkey")]
    Passkey,
    #[serde(rename = "security_key")]
    SecurityKey,
    #[serde(rename = "platform")]
    Platform,
}

impl std::fmt::Display for WebAuthnType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Passkey => write!(f, "passkey"),
            Self::SecurityKey => write!(f, "security_key"),
            Self::Platform => write!(f, "platform"),
        }
    }
}

impl Default for WebAuthnType {
    fn default() -> WebAuthnType {
        Self::Passkey
    }
}
