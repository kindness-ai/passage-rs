use crate::{
    models::{activate_one_time_passcode_request::ActivateOneTimePasscodeRequest, AuthResponse},
    Config, Passage, PassageError,
};

pub struct Otp<'c> {
    client: &'c Passage,
}

impl<'c> Otp<'c> {
    pub fn new(client: &'c Passage) -> Self {
        Self { client }
    }

    /// Authenticate a one-time passcode for a user. This endpoint checks that
    /// the one-time passcode is valid, then returns an authentication token
    /// for the user.
    pub async fn activate_one_time_passcode(
        &self,
        request: ActivateOneTimePasscodeRequest,
    ) -> Result<AuthResponse, PassageError> {
        self.client
            .post("/apps/{app_id}/otp/activate", request)
            .await
    }
}
