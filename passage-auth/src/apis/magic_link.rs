use crate::{
    models::{
        activate_magic_link_request::ActivateMagicLinkRequest,
        get_magic_link_status_request::GetMagicLinkStatusRequest, AuthResponse,
    },
    Config, Passage, PassageError,
};

pub struct MagicLink<'c, Config> {
    client: &'c Passage<Config>,
}

impl<'c> MagicLink<'c, Config> {
    pub fn new(client: &'c Passage<Config>) -> Self {
        Self { client }
    }

    /// Authenticate a magic link for a user. This endpoint checks that the
    /// magic link is valid, then returns an authentication token for the
    /// user.
    pub async fn activate_magic_link(
        &self,
        request: ActivateMagicLinkRequest,
    ) -> Result<AuthResponse, PassageError> {
        self.client
            .patch("/apps/{app_id}/magic-link/activate", request)
            .await
    }

    /// Check if a magic link has been activated yet or not. Once the magic link
    /// has been activated, this endpoint will return an authentication
    /// token for the user. This endpoint can be used to initiate a login on
    /// one device and then poll and wait for the login to complete on
    /// another device.
    pub async fn magic_link_status(
        &self,
        request: GetMagicLinkStatusRequest,
    ) -> Result<AuthResponse, PassageError> {
        self.client
            .post("/apps/{app_id}/magic-link/status", request)
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        models::{
            activate_magic_link_request::ActivateMagicLinkRequest,
            get_magic_link_status_request::GetMagicLinkStatusRequest, Model404Code, Model404Error,
        },
        Config, Passage, PassageError,
    };

    #[tokio::test]
    async fn test_activate_magic_link() {
        let config = Config::default().with_app_id("PaItOH7Ul7n2Xt3uxY671sFN".into());
        let client = Passage::with_config(config);

        let request = ActivateMagicLinkRequest {
            magic_link:
                "ml.Q7IJfDLKxfyvTwj6UPtjhMpN.AabRBkquedeVBxv9kFyfeXHI.u4FdRyRFekk8iBMhiXZqHdqy"
                    .to_string(),
        };

        match client.magic_link().activate_magic_link(request).await {
            Ok(response) => {
                panic!("Unexpected Response: {:?}", response);
            }
            Err(_) => {
                // TODO: Work on tests
            }
        }
    }

    #[tokio::test]
    async fn test_magic_link_status() {
        let config = Config::default().with_app_id("PaItOH7Ul7n2Xt3uxY671sFN".into());
        let client = Passage::with_config(config);

        let request = GetMagicLinkStatusRequest {
            id: "KgCQKUz1mY8qz8RhdoVjRSvU".to_string(),
        };

        match client.magic_link().magic_link_status(request).await {
            Ok(response) => {
                println!("Response: {:?}", response);
                assert!(response.auth_result.auth_token.is_empty())
            }
            Err(PassageError::ApiError(crate::error::ApiError::Status404(e404))) => {
                assert_eq!(e404.code, Model404Code::MagicLinkNotFound);
            }
            Err(err) => panic!("Unexpected Error: {:?}", err),
        }
    }
}
