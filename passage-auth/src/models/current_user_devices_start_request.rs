use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurrentUserDevicesStartRequest {
    #[serde(
        rename = "authenticator_attachment",
        skip_serializing_if = "Option::is_none"
    )]
    pub authenticator_attachment: Option<models::AuthenticatorAttachment>,
}

impl CurrentUserDevicesStartRequest {
    pub fn new() -> CurrentUserDevicesStartRequest {
        CurrentUserDevicesStartRequest {
            authenticator_attachment: None,
        }
    }
}
