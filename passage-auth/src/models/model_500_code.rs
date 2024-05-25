use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Model500Code {
    #[serde(rename = "internal_server_error")]
    InternalServerError,
}

impl std::fmt::Display for Model500Code {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InternalServerError => write!(f, "internal_server_error"),
        }
    }
}

impl Default for Model500Code {
    fn default() -> Model500Code {
        Self::InternalServerError
    }
}
