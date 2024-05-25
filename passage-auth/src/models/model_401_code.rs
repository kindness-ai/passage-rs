use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Model401Code {
    #[serde(rename = "discoverable_login_failed")]
    DiscoverableLoginFailed,
    #[serde(rename = "webauthn_login_failed")]
    WebauthnLoginFailed,
    #[serde(rename = "invalid_magic_link")]
    InvalidMagicLink,
    #[serde(rename = "invalid_access_token")]
    InvalidAccessToken,
    #[serde(rename = "exceeded_attempts")]
    ExceededAttempts,
}

impl std::fmt::Display for Model401Code {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::DiscoverableLoginFailed => write!(f, "discoverable_login_failed"),
            Self::WebauthnLoginFailed => write!(f, "webauthn_login_failed"),
            Self::InvalidMagicLink => write!(f, "invalid_magic_link"),
            Self::InvalidAccessToken => write!(f, "invalid_access_token"),
            Self::ExceededAttempts => write!(f, "exceeded_attempts"),
        }
    }
}

impl Default for Model401Code {
    fn default() -> Model401Code {
        Self::DiscoverableLoginFailed
    }
}
