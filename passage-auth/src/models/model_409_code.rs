use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Model409Code {
    #[serde(rename = "user_has_no_passkeys")]
    UserHasNoPasskeys,
}

impl std::fmt::Display for Model409Code {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UserHasNoPasskeys => write!(f, "user_has_no_passkeys"),
        }
    }
}

impl Default for Model409Code {
    fn default() -> Model409Code {
        Self::UserHasNoPasskeys
    }
}
