#![allow(clippy::all)]

use crate::{
    models::{self, id_token_request::IdTokenRequest},
    Config, Passage, PassageError,
};

pub struct OAuth2<'c> {
    client: &'c Passage,
}

impl<'c> OAuth2<'c> {
    pub fn new(client: &'c Passage) -> Self {
        Self { client }
    }

    /// Authenticate a one-time passcode for a user. This endpoint checks that
    /// the one-time passcode is valid, then returns an authentication token
    /// for the user.
    pub async fn apple_oauth2_callback(
        &self,
        state: &str,
        code: Option<&str>,
        id_token: Option<&str>,
        user: Option<&str>,
        error: Option<&str>,
    ) -> Result<(), PassageError> {
        todo!()
    }

    pub async fn apple_oauth2_callback_default_dev(
        &self,
        state: &str,
        code: Option<&str>,
        id_token: Option<&str>,
        user: Option<&str>,
        error: Option<&str>,
    ) -> Result<(), PassageError> {
        todo!()
    }

    pub async fn current_user_social_link_account(
        &self,
        code: &str,
        verifier: &str,
    ) -> Result<(), PassageError> {
        todo!()
    }

    pub async fn exchange_social_id_token(
        &self,
        id_token_request: IdTokenRequest,
    ) -> Result<models::AuthResponse, PassageError> {
        todo!()
    }

    pub async fn exchange_social_token(
        &self,
        code: &str,
        verifier: &str,
    ) -> Result<models::AuthResponse, PassageError> {
        todo!()
    }

    /// Kick off OAuth2 flow with connection provider request params described in https://openid.net/specs/openid-connect-core-1_0.html#AuthRequest
    pub async fn get_authorize(
        &self,
        redirect_uri: &str,
        code_challenge: &str,
        code_challenge_method: &str,
        connection_type: &str,
        state: Option<&str>,
    ) -> Result<(), PassageError> {
        todo!()
    }

    pub fn oauth2_callback(
        &self,
        code: &str,
        state: Option<&str>,
        error: Option<&str>,
        error_description: Option<&str>,
    ) -> Result<(), PassageError> {
        todo!()
    }

    pub async fn oauth2_callback_default_dev(
        &self,
        code: &str,
        state: Option<&str>,
        error: Option<&str>,
        error_description: Option<&str>,
    ) -> Result<(), PassageError> {
        todo!()
    }
}
