use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OneTimePasscodeResponse {
    /// The ID of the one-time passcode. Provide it when activating.
    #[serde(rename = "otp_id")]
    pub otp_id: String,
}

impl OneTimePasscodeResponse {
    pub fn new(otp_id: String) -> OneTimePasscodeResponse {
        OneTimePasscodeResponse { otp_id }
    }
}
