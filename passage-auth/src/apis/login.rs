use crate::{
    models::{
        login_magic_link_request::LoginMagicLinkRequest,
        login_magic_link_response::LoginMagicLinkResponse,
        login_one_time_passcode_request::LoginOneTimePasscodeRequest,
        login_web_authn_finish_request::LoginWebAuthnFinishRequest,
        login_web_authn_start_request::LoginWebAuthnStartRequest,
        login_web_authn_start_response::LoginWebAuthnStartResponse, AuthResponse,
        OneTimePasscodeResponse,
    },
    Config, Passage, PassageError,
};

pub struct Login<'c, Config> {
    client: &'c Passage<Config>,
}

impl<'c> Login<'c, Config> {
    pub fn new(client: &'c Passage<Config>) -> Self {
        Self { client }
    }

    /// Send a login email or SMS to the user. The user will receive an email or
    /// text with a link to complete their login.
    ///
    /// Returns a 403 if Login code is enabled which is the default and
    /// recommended by Passage.
    pub async fn login_magic_link(
        &self,
        request: LoginMagicLinkRequest,
    ) -> Result<LoginMagicLinkResponse, PassageError> {
        self.client
            .post("/apps/{app_id}/login/magic-link", request)
            .await
    }

    /// Send a login email or SMS to the user. The user will receive an email or
    /// text with a one-time passcode to complete their login.
    pub async fn login_one_time_passcode(
        &self,
        request: LoginOneTimePasscodeRequest,
    ) -> Result<OneTimePasscodeResponse, PassageError> {
        self.client.post("/apps/{app_id}/login/otp", request).await
    }

    /// Complete a WebAuthn login and authenticate the user. This endpoint
    /// accepts and verifies the response from `navigator.credential.get()`
    /// and returns an authentication token for the user.
    pub async fn login_webauthn_finish(
        &self,
        request: LoginWebAuthnFinishRequest,
    ) -> Result<AuthResponse, PassageError> {
        self.client
            .post("/apps/{app_id}/login/webauthn/finish", request)
            .await
    }

    /// Initiate a WebAuthn login for a user. This endpoint creates a WebAuthn
    /// credential assertion challenge that is used to perform the login
    /// ceremony from the browser.
    pub async fn login_webauthn_start(
        &self,
        request: Option<LoginWebAuthnStartRequest>,
    ) -> Result<LoginWebAuthnStartResponse, PassageError> {
        self.client
            .post("/apps/{app_id}/login/webauthn/start", request)
            .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::models::{
        login_magic_link_request::LoginMagicLinkRequest,
        login_one_time_passcode_request::LoginOneTimePasscodeRequest,
        login_web_authn_finish_request::LoginWebAuthnFinishRequest,
        login_web_authn_start_request::LoginWebAuthnStartRequest,
    };

    #[tokio::test]
    async fn test_login_magic_link() {
        let config = Config::default().with_app_id("PaItOH7Ul7n2Xt3uxY671sFN".into());
        let client = Passage::with_config(config);

        let request = LoginMagicLinkRequest {
            identifier: "ted@tedlasso.org".to_string(),
            ..Default::default()
        };

        match client.login().login_magic_link(request).await {
            Ok(response) => {
                println!("Response: {:?}", response);
                assert!(!response.magic_link.id.is_empty());
            }
            Err(err) => {
                panic!("Error: {:?}", err);
            }
        }
    }
}
