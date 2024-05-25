use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Model403Error {
    #[serde(rename = "code")]
    pub code: models::Model403Code,
    #[serde(rename = "error")]
    pub error: String,
}

impl Model403Error {
    pub fn new(code: models::Model403Code, error: String) -> Model403Error {
        Model403Error { code, error }
    }
}
