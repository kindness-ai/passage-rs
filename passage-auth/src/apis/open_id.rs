use crate::{models::open_id_configuration::OpenIdConfiguration, Config, Passage, PassageError};

pub struct OpenId<'c> {
    client: &'c Passage,
}

impl<'c> OpenId<'c> {
    pub fn new(client: &'c Passage) -> Self {
        Self { client }
    }

    /// Get OpenID Configuration for an app.
    pub async fn get_open_id_configuration(&self) -> Result<OpenIdConfiguration, PassageError> {
        self.client
            .get("/apps/{app_id}/.well-known/openid-configuration")
            .await
    }
}
