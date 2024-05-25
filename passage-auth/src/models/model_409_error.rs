use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Model409Error {
    #[serde(rename = "code")]
    pub code: models::Model409Code,
    #[serde(rename = "error")]
    pub error: String,
}

impl Model409Error {
    pub fn new(code: models::Model409Code, error: String) -> Model409Error {
        Model409Error { code, error }
    }
}
