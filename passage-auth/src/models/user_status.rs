use serde::{Deserialize, Serialize};

/// UserStatus : User status: active, inactive, pending
/// User status: active, inactive, pending
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UserStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "pending")]
    Pending,
}

impl std::fmt::Display for UserStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Inactive => write!(f, "inactive"),
            Self::Pending => write!(f, "pending"),
        }
    }
}

impl Default for UserStatus {
    fn default() -> UserStatus {
        Self::Active
    }
}
