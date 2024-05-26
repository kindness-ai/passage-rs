use crate::{
    models::{
        activate_one_time_passcode_request::ActivateOneTimePasscodeRequest,
        create_user_params::CreateUserParams, user_response::UserResponse, AuthResponse,
    },
    Config, Passage, PassageError,
};

pub struct Users<'c, Config> {
    client: &'c Passage<Config>,
}

impl<'c> Users<'c, Config> {
    pub fn new(client: &'c Passage<Config>) -> Self {
        Self { client }
    }
}

impl<'c> Users<'c, Config> {
    /// Get information about a user, given the user's identifier (email or
    /// phone number)
    pub async fn get_user(&self, identifier: &str) -> Result<UserResponse, PassageError> {
        let app_id = self.client.app_id();
        self.client
            .get_with_query(
                &format!("/apps/{app_id}/users"),
                &[("identifier", identifier)],
            )
            .await
    }

    /// Create a new user if the Passage app is configured for Public Signups
    /// 
    /// Without Public Signups, you'll need to use the `passage-manage` and an API key
    pub async fn create_user(
        &self,
        request: CreateUserParams,
    ) -> Result<UserResponse, PassageError> {
        let app_id = self.client.app_id();
        self.client
            .post(&format!("/apps/{app_id}/users"), request)
            .await
    }
}

#[cfg(test)]
mod tests {
    use rand::{distributions::Alphanumeric, Rng};

    use super::*;

    #[tokio::test]
    async fn test_get_user_found() {
        let client =
            Passage::with_config(Config::default().with_app_id("PaItOH7Ul7n2Xt3uxY671sFN".to_string()));
        let response = client
            .user()
            .get_user("ted@tedlasso.org".into())
            .await
            .unwrap();

        let user = response.user.expect("We should be able to find Ted, unless the server is down.");

        assert_eq!(user.id, "AabRBkquedeVBxv9kFyfeXHI");
        assert_eq!(user.email, "ted@tedlasso.org");
        assert_eq!(user.email_verified, true);
    }

    #[tokio::test]
    async fn test_get_user_missing() {
        let client =
            Passage::with_config(Config::default().with_app_id("PaItOH7Ul7n2Xt3uxY671sFN".to_string()));
        let response = client
            .user()
            .get_user("rupert.mannion@tedlasso.org".into())
            .await
            .unwrap();

        let user = response.user;

        assert!(user.is_none());
    }

    #[tokio::test]
    async fn test_create_user() {
        let client = Passage::with_config(
            Config::default().with_app_id("PaItOH7Ul7n2Xt3uxY671sFN".to_string()),
        );

        let local_part = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(20)
            .map(char::from)
            .collect::<String>()
            .to_lowercase();

        let response = client
            .user()
            .create_user(CreateUserParams::new(format!("{local_part}@tedlasso.org")))
            .await
            .unwrap();

        let user = response.user.expect("We should be able to create an account, unless the email is already taken or the server is down.");

        assert_eq!(user.email, format!("{local_part}@tedlasso.org"));
        assert_eq!(user.email_verified, false);
    }
}
