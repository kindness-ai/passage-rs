use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Model400Error {
    #[serde(rename = "code")]
    pub code: models::Model400Code,
    #[serde(rename = "error")]
    pub error: String,
}

impl Model400Error {
    pub fn new(code: models::Model400Code, error: String) -> Model400Error {
        Model400Error { code, error }
    }
}
