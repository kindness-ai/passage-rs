use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivateOneTimePasscodeRequest {
    #[serde(rename = "otp")]
    pub otp: String,
    /// The ID of the one-time passcode.
    #[serde(rename = "otp_id")]
    pub otp_id: String,
}

impl ActivateOneTimePasscodeRequest {
    pub fn new(otp: String, otp_id: String) -> ActivateOneTimePasscodeRequest {
        ActivateOneTimePasscodeRequest { otp, otp_id }
    }
}
