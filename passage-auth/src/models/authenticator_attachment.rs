use serde::{Deserialize, Serialize};

/// AuthenticatorAttachment : selects the type of authentication that will be
/// used in this WebAuthN flow request selects the type of authentication that
/// will be used in this WebAuthN flow request
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AuthenticatorAttachment {
    #[serde(rename = "platform")]
    Platform,
    #[serde(rename = "cross-platform")]
    CrossPlatform,
    #[serde(rename = "any")]
    Any,
}

impl std::fmt::Display for AuthenticatorAttachment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Platform => write!(f, "platform"),
            Self::CrossPlatform => write!(f, "cross-platform"),
            Self::Any => write!(f, "any"),
        }
    }
}

impl Default for AuthenticatorAttachment {
    fn default() -> AuthenticatorAttachment {
        Self::Platform
    }
}
