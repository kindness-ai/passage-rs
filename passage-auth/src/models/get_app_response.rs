use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetAppResponse {
    #[serde(rename = "app")]
    pub app: Box<models::App>,
}

impl GetAppResponse {
    pub fn new(app: models::App) -> GetAppResponse {
        GetAppResponse { app: Box::new(app) }
    }
}
