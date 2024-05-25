use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[allow(clippy::enum_variant_names)]
pub enum Model404Code {
    #[serde(rename = "app_not_found")]
    AppNotFound,
    #[serde(rename = "user_not_found")]
    UserNotFound,
    #[serde(rename = "magic_link_not_found")]
    MagicLinkNotFound,
    #[serde(rename = "social_connection_not_found")]
    SocialConnectionNotFound,
    #[serde(rename = "transaction_not_found")]
    TransactionNotFound,
}

impl std::fmt::Display for Model404Code {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AppNotFound => write!(f, "app_not_found"),
            Self::UserNotFound => write!(f, "user_not_found"),
            Self::MagicLinkNotFound => write!(f, "magic_link_not_found"),
            Self::SocialConnectionNotFound => write!(f, "social_connection_not_found"),
            Self::TransactionNotFound => write!(f, "transaction_not_found"),
        }
    }
}

impl Default for Model404Code {
    fn default() -> Model404Code {
        Self::AppNotFound
    }
}
