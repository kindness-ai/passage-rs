use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OtpAuthMethod {
    /// Maximum time (IN SECONDS) for the auth to expire.
    #[serde(rename = "ttl", skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i32>,
    #[serde(rename = "ttl_display_unit", skip_serializing_if = "Option::is_none")]
    pub ttl_display_unit: Option<models::TtlDisplayUnit>,
}

impl OtpAuthMethod {
    pub fn new() -> OtpAuthMethod {
        OtpAuthMethod {
            ttl: None,
            ttl_display_unit: None,
        }
    }
}
