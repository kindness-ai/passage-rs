use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JwkResponse {
    #[serde(rename = "keys")]
    pub keys: Vec<models::JwkResponseKeysInner>,
}

impl JwkResponse {
    pub fn new(keys: Vec<models::JwkResponseKeysInner>) -> JwkResponse {
        JwkResponse { keys }
    }
}
