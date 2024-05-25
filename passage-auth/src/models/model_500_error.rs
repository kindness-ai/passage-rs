use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Model500Error {
    #[serde(rename = "code")]
    pub code: models::Model500Code,
    #[serde(rename = "error")]
    pub error: String,
}

impl Model500Error {
    pub fn new(code: models::Model500Code, error: String) -> Model500Error {
        Model500Error { code, error }
    }
}
