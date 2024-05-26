use crate::{
    models::{AuthResponse, RefreshAuthTokenRequest},
    Config, Passage, PassageError,
};

pub struct Tokens<'c> {
    client: &'c Passage,
}

impl<'c> Tokens<'c> {
    pub fn new(client: &'c Passage) -> Self {
        Self { client }
    }

    /// Creates and returns a new auth token and a new refresh token
    pub async fn refresh_auth_token(
        &self,
        request: RefreshAuthTokenRequest,
    ) -> Result<AuthResponse, PassageError> {
        self.client.post("/apps/{app_id}/tokens", request).await
    }

    /// Revokes the refresh token
    pub async fn revoke_refresh_token(&self, refresh_token: &str) -> Result<(), PassageError> {
        self.client
            .delete_with_query("/apps/{app_id}/tokens", &[("refresh_token", refresh_token)])
            .await
    }
}
