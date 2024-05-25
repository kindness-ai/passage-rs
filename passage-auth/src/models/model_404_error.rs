use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Model404Error {
    #[serde(rename = "code")]
    pub code: models::Model404Code,
    #[serde(rename = "error")]
    pub error: String,
}

impl Model404Error {
    pub fn new(code: models::Model404Code, error: String) -> Model404Error {
        Model404Error { code, error }
    }
}
