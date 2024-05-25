use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Model401Error {
    #[serde(rename = "code")]
    pub code: models::Model401Code,
    #[serde(rename = "error")]
    pub error: String,
}

impl Model401Error {
    pub fn new(code: models::Model401Code, error: String) -> Model401Error {
        Model401Error { code, error }
    }
}
