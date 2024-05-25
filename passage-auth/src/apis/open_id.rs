use crate::{models::open_id_configuration::OpenIdConfiguration, Config, Passage, PassageError};

pub struct OpenId<'c, Config> {
    client: &'c Passage<Config>,
}

impl<'c> OpenId<'c, Config> {
    pub fn new(client: &'c Passage<Config>) -> Self {
        Self { client }
    }

    /// Get OpenID Configuration for an app.
    pub async fn get_open_id_configuration(&self) -> Result<OpenIdConfiguration, PassageError> {
        self.client
            .get("/apps/{app_id}/.well-known/openid-configuration")
            .await
    }
}
