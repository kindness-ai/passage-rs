use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Model400Code {
    #[serde(rename = "invalid_request")]
    Request,
    #[serde(rename = "invalid_client_version")]
    ClientVersion,
}

impl std::fmt::Display for Model400Code {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Request => write!(f, "invalid_request"),
            Self::ClientVersion => write!(f, "invalid_client_version"),
        }
    }
}

impl Default for Model400Code {
    fn default() -> Model400Code {
        Self::Request
    }
}
