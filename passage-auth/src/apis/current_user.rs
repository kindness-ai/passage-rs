use crate::{
    models::{
        add_device_finish_request::AddDeviceFinishRequest,
        add_device_start_response::AddDeviceStartResponse, current_user_device::CurrentUserDevice,
        current_user_devices::CurrentUserDevices,
        current_user_devices_start_request::CurrentUserDevicesStartRequest,
        current_user_response::CurrentUserResponse, magic_link_response::MagicLinkResponse,
        social_connections_response::SocialConnectionsResponse,
        update_device_request::UpdateDeviceRequest, update_metadata_request::UpdateMetadataRequest,
        update_user_email_request::UpdateUserEmailRequest,
        user_metadata_response::UserMetadataResponse,
    },
    Config, Passage, PassageError,
};

pub struct CurrentUser<'c, Config> {
    client: &'c Passage<Config>,
}

impl<'c> CurrentUser<'c, Config> {
    pub fn new(client: &'c Passage<Config>) -> Self {
        Self { client }
    }

    /// Revoke a device by ID for the current user. User must be authenticated
    /// via a bearer token.
    pub async fn delete_current_user_device(&self, device_id: &str) -> Result<(), PassageError> {
        let app_id = self.client.app_id();
        self.client
            .delete(&format!("/apps/{app_id}/currentuser/devices/{device_id}",))
            .await
    }

    /// Deletes a social connection for the current user. User must be
    /// authenticated via a bearer token.
    pub async fn delete_current_user_social_connection(
        &self,
        social_connection_type: &str,
    ) -> Result<(), PassageError> {
        let app_id = self.client.app_id();
        self.client
            .delete(&format!(
                "/apps/{app_id}/currentuser/social-connections/{social_connection_type}",
            ))
            .await
    }

    /// Get information about a user that is currently authenticated via bearer
    /// token.
    pub async fn get_current_user(&self) -> Result<CurrentUserResponse, PassageError> {
        self.client.get("/apps/{app_id}/currentuser").await
    }

    /// List all WebAuthn devices for the authenticated user. User must be
    /// authenticated via bearer token.
    pub async fn get_current_user_devices(&self) -> Result<CurrentUserDevices, PassageError> {
        self.client.get("/apps/{app_id}/currentuser/devices").await
    }

    /// Get the user-metadata for the current user.
    pub async fn get_current_user_metadata(&self) -> Result<UserMetadataResponse, PassageError> {
        self.client
            .get("/apps/{app_id}/currentuser/user-metadata")
            .await
    }

    /// Gets social connections for the current user. User must be authenticated
    /// via a bearer token.
    pub async fn get_currentuser_social_connections(
        &self,
    ) -> Result<SocialConnectionsResponse, PassageError> {
        self.client
            .get("/apps/{app_id}/currentuser/social-connections")
            .await
    }

    /// Complete a WebAuthn add device operation for the current user. This
    /// endpoint accepts and verifies the response from
    /// `navigator.credential.create()` and returns the created device for
    /// the user. User must be authenticated via a bearer token.
    pub async fn post_current_user_add_device_finish(
        &self,
        add_device_finish_request: AddDeviceFinishRequest,
    ) -> Result<CurrentUserDevice, PassageError> {
        self.client
            .post(
                "/apps/{app_id}/currentuser/devices/finish",
                add_device_finish_request,
            )
            .await
    }

    /// Initiate a WebAuthn add device operation for the current user. This
    /// endpoint creates a WebAuthn credential creation challenge that is
    /// used to perform the registration ceremony from the browser. User
    /// must be authenticated via a bearer token.
    pub async fn post_current_user_add_device_start(
        &self,
        current_user_devices_start_request: Option<CurrentUserDevicesStartRequest>,
    ) -> Result<AddDeviceStartResponse, PassageError> {
        self.client
            .post(
                "/apps/{app_id}/currentuser/devices/start",
                current_user_devices_start_request,
            )
            .await
    }

    /// Update a device by ID for the current user. Currently the only field
    /// that can be updated is the friendly name. User must be authenticated
    /// via a bearer token.
    pub async fn update_current_user_device(
        &self,
        device_id: &str,
        update_device_request: UpdateDeviceRequest,
    ) -> Result<CurrentUserDevice, PassageError> {
        let app_id = self.client.app_id();
        self.client
            .patch(
                &format!("/apps/{app_id}/currentuser/devices/{device_id}",),
                update_device_request,
            )
            .await
    }

    /// Update the metadata for the current user. Only valid metadata fields are
    /// accepted. Invalid metadata fields that are present will abort the
    /// update. User must be authenticated via a bearer token.
    pub async fn update_current_user_metadata(
        &self,
        update_metadata_request: UpdateMetadataRequest,
    ) -> Result<CurrentUserResponse, PassageError> {
        self.client
            .patch(
                "/apps/{app_id}/currentuser/user-metadata",
                update_metadata_request,
            )
            .await
    }

    /// Initiate an email change for the authenticated user. An email change
    /// requires verification, so an email will be sent to the user which they
    /// must verify before the email change takes effect.
    pub async fn update_email_currentuser(
        &self,
        update_user_email_request: UpdateUserEmailRequest,
    ) -> Result<MagicLinkResponse, PassageError> {
        self.client
            .patch(
                "/apps/{app_id}/currentuser/email",
                update_user_email_request,
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::{models::JwkResponseKeysInner, Config};

    const APP_ID: &str = "PaItOH7Ul7n2Xt3uxY671sFN";
    const JWT: &str = "eyJhbGciOiJSUzI1NiIsImtpZCI6IlBtUkJVeVFkUGZ0eHVJS2E2ZGxtR01aQSIsInR5cCI6IkpXVCJ9.eyJhdWQiOiJodHRwczovL3RlZGxhc3NvLm9yZyIsImV4cCI6MTcxNjY1MDg3MywiaWF0IjoxNzE2NjQ3MjczLCJpc3MiOiJodHRwczovL2F1dGgucGFzc2FnZS5pZC92MS9hcHBzL1BhSXRPSDdVbDduMlh0M3V4WTY3MXNGTiIsIm5iZiI6MTcxNjY0NzI2OCwic3ViIjoiQWFiUkJrcXVlZGVWQnh2OWtGeWZlWEhJIn0.L587jkfRBL48_ec6GSjehDIjIQE_76cnq3QBx2s2OcpQisDNwVcXEWr36UzCdpTdWRIR7Mri0p45QigNUMaCmpbadZvp8jZLVJPMfBrI94kKtX_JR4vx2k9NpB68thwfM3QAk_5w3yEz3oxpetk1G0_amcj8iGk-nupHKVyXhpU8OkK4vOTkrDuO1winviseGiC5E_LDHBPBngdmyD2Pp2X-pGl2JpIohuG9cizqgyU2pjYNqqzXrnZ7FWD3g4EY482O_FPIe2PxMyqbW31dQq-1RCTVAZ_lwbiwxVXFC4Apv-FDOJONpGJT7IyI0tWfWI1fZOO2qX8uUfSqfIGV-g";

    #[tokio::test]
    async fn get_get_current_user() {
        let passage = Passage::with_config(
            Config::default()
                .with_app_id(APP_ID.to_string())
                .with_user_bearer_token(JWT.to_string()),
        );

        let response: Result<CurrentUserResponse, PassageError> =
            passage.current_user().get_current_user().await;
        match response {
            Ok(_) => panic!("Expected JWTnot to be valid, but got Ok"),
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}
