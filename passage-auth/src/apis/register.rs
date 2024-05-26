use crate::{
    models::{
        self, AuthResponse, Nonce, OneTimePasscodeResponse, RegisterMagicLinkRequest,
        RegisterMagicLinkResponse, RegisterOneTimePasscodeRequest, RegisterWebAuthnFinishRequest,
        RegisterWebAuthnFinishWithTransactionRequest, RegisterWebAuthnStartRequest,
        RegisterWebAuthnStartResponse, RegisterWebAuthnStartWithTransactionRequest,
    },
    Config, Passage, PassageError,
};

pub struct Register<'c> {
    client: &'c Passage,
}

impl<'c> Register<'c> {
    pub fn new(client: &'c Passage) -> Self {
        Self { client }
    }

    /// Create a user and send an registration email or SMS to the user. The
    /// user will receive an email or text with a link to complete their
    /// registration.
    pub async fn register_magic_link(
        &self,
        request: RegisterMagicLinkRequest,
    ) -> Result<RegisterMagicLinkResponse, PassageError> {
        self.client
            .post("/apps/{app_id}/register/magic-link", request)
            .await
    }

    /// Create a user and send a registration email or SMS to the user. The user
    /// will receive an email or text with a one-time passcode to complete their
    /// registration.
    pub async fn register_one_time_passcode(
        &self,
        request: RegisterOneTimePasscodeRequest,
    ) -> Result<OneTimePasscodeResponse, PassageError> {
        self.client
            .post("/apps/{app_id}/register/otp", request)
            .await
    }

    /// Complete a WebAuthn registration and authenticate the user. This
    /// endpoint accepts and verifies the response from
    /// `navigator.credential.create()` and returns an authentication token
    /// for the user.
    pub async fn register_webauthn_finish(
        &self,
        request: RegisterWebAuthnFinishRequest,
    ) -> Result<AuthResponse, PassageError> {
        self.client
            .post("/apps/{app_id}/register/webauthn/finish", request)
            .await
    }

    /// Complete a WebAuthn registration and authenticate the user via a
    /// transaction. This endpoint accepts and verifies the response from
    /// `navigator.credential.create()` and returns a nonce meant to be
    /// exchanged for an authentication token for the user.
    pub async fn register_webauthn_finish_with_transaction(
        &self,
        request: RegisterWebAuthnFinishWithTransactionRequest,
    ) -> Result<Nonce, PassageError> {
        self.client
            .post(
                "/apps/{app_id}/register/transactions/webauthn/finish",
                request,
            )
            .await
    }

    /// Initiate a WebAuthn registration and create the user. This endpoint
    /// creates a WebAuthn credential creation challenge that is used to
    /// perform the registration ceremony from the browser.
    pub async fn register_webauthn_start(
        &self,
        request: RegisterWebAuthnStartRequest,
    ) -> Result<RegisterWebAuthnStartResponse, PassageError> {
        self.client
            .post("/apps/{app_id}/register/webauthn/start", request)
            .await
    }

    /// Initiate a WebAuthn registration and create the user via a transaction.
    /// This endpoint creates a WebAuthn credential creation challenge that
    /// is used to perform the registration ceremony from the browser.
    pub async fn register_webauthn_start_with_transaction(
        &self,
        request: RegisterWebAuthnStartWithTransactionRequest,
    ) -> Result<models::RegisterWebAuthnStartWithTransactionResponse, PassageError> {
        self.client
            .post(
                "/apps/{app_id}/register/transactions/webauthn/start",
                request,
            )
            .await
    }
}
