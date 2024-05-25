use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LayoutConfig {
    #[serde(rename = "h")]
    pub h: i32,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "w")]
    pub w: i32,
    #[serde(rename = "x")]
    pub x: i32,
    #[serde(rename = "y")]
    pub y: i32,
}

impl LayoutConfig {
    pub fn new(h: i32, id: String, w: i32, x: i32, y: i32) -> LayoutConfig {
        LayoutConfig { h, id, w, x, y }
    }
}
