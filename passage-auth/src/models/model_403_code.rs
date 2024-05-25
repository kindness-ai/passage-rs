use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Model403Code {
    #[serde(rename = "identifier_not_verified")]
    IdentifierNotVerified,
    #[serde(rename = "operation_not_allowed")]
    OperationNotAllowed,
    #[serde(rename = "user_not_active")]
    UserNotActive,
}

impl std::fmt::Display for Model403Code {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::IdentifierNotVerified => write!(f, "identifier_not_verified"),
            Self::OperationNotAllowed => write!(f, "operation_not_allowed"),
            Self::UserNotActive => write!(f, "user_not_active"),
        }
    }
}

impl Default for Model403Code {
    fn default() -> Model403Code {
        Self::IdentifierNotVerified
    }
}
