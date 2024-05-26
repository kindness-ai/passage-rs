use serde::{de::DeserializeOwned, Serialize};

use crate::{
    apis::{
        Authenticate, CurrentUser, Jwks, Login, MagicLink, OpenId, Otp, Register, Tokens, Users,
    },
    config::Config,
    error::ApiError,
    PassageError,
};

#[derive(Debug, Clone)]
pub struct Passage<Config> {
    http_client: reqwest::Client,
    config: Config,
}

impl Default for Passage<Config> {
    fn default() -> Self {
        Self::new()
    }
}

impl Passage<Config> {
    /// Creates a new [Passage] for interacting with the Passage API.
    pub fn new() -> Self {
        Self {
            http_client: reqwest::Client::new(),
            config: Config::default(),
        }
    }
}

impl Passage<Config> {
    /// Create client with a custom HTTP client if needed.
    pub fn build(http_client: reqwest::Client, config: Config) -> Self {
        Self {
            http_client,
            config,
        }
    }

    pub fn with_config(config: Config) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            config,
        }
    }

    pub fn set_pub_jwk(mut self, pub_jwk: String) -> Self {
        self.config = self.config.with_pub_jwk(pub_jwk);
        self
    }

    /// Provide a custom [client] to make all HTTP requests with.
    ///
    /// [client]: reqwest::Client
    pub fn with_http_client(mut self, http_client: reqwest::Client) -> Self {
        self.http_client = http_client;
        self
    }

    // API groups

    /// To call the [Tokens] group related APIs using this client.
    pub fn tokens(&self) -> Tokens<Config> {
        Tokens::new(self)
    }

    /// To call [Register] group related APIs using this client.
    pub fn register(&self) -> Register<Config> {
        Register::new(self)
    }

    /// To call [Otp] group related APIs using this client.
    pub fn otp(&self) -> Otp<Config> {
        Otp::new(self)
    }

    /// To call [OpenId] group related APIs using this client.
    pub fn open_id(&self) -> OpenId<Config> {
        OpenId::new(self)
    }

    /// To call [MagicLink] group related APIs using this client.
    pub fn magic_link(&self) -> MagicLink<Config> {
        MagicLink::new(self)
    }

    /// To call [Login] group related APIs using this client.
    pub fn login(&self) -> Login<Config> {
        Login::new(self)
    }

    /// To call [Jwks] group related APIs using this client.
    pub fn jwks(&self) -> Jwks<Config> {
        Jwks::new(self)
    }

    /// To call [Authenticate] group related APIs using this client.
    pub fn authenticate(&self) -> Authenticate<Config> {
        Authenticate::new(self)
    }

    /// To call [CurrentUser] group related APIs using this client.
    pub fn current_user(&self) -> CurrentUser<Config> {
        CurrentUser::new(self)
    }

    /// To call [Users] group related APIs using this client.
    pub fn users(&self) -> Users<Config> {
        Users::new(self)
    }

    pub fn app_id(&self) -> &str {
        self.config.app_id()
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn pub_jwk(&self) -> Option<&String> {
        self.config.pub_jwk()
    }

    /// Make a GET request to {path} and deserialize the response body
    pub(crate) async fn get<O>(&self, path: &str) -> Result<O, PassageError>
    where
        O: DeserializeOwned,
    {
        let response = self
            .http_client
            .get(self.config.url(path))
            .query(&self.config.query())
            .headers(self.config.bearer_auth())
            .send()
            .await?;

        self.deserialize_response(response).await
    }

    /// Make a GET request to {path} with given Query and deserialize the
    /// response body
    pub(crate) async fn get_with_query<Q, O>(
        &self,
        path: &str,
        query: &Q,
    ) -> Result<O, PassageError>
    where
        O: DeserializeOwned,
        Q: Serialize + ?Sized,
    {
        let response = self
            .http_client
            .get(self.config.url(path))
            .query(&self.config.query())
            .query(query)
            .headers(self.config.bearer_auth())
            .send()
            .await?;

        self.deserialize_response(response).await
    }

    /// Make a POST request to {path} and deserialize the response body
    pub(crate) async fn post<I, O>(&self, path: &str, request: I) -> Result<O, PassageError>
    where
        I: Serialize,
        O: DeserializeOwned,
    {
        let response = self
            .http_client
            .post(self.config.url(path))
            .query(&self.config.query())
            .headers(self.config.bearer_auth())
            .json(&request)
            .send()
            .await?;

        self.deserialize_response(response).await
    }

    /// Make a POST request to {patch} and deserialize the response body
    pub(crate) async fn patch<I, O>(&self, path: &str, request: I) -> Result<O, PassageError>
    where
        I: Serialize,
        O: DeserializeOwned,
    {
        let response = self
            .http_client
            .patch(self.config.url(path))
            .query(&self.config.query())
            .headers(self.config.bearer_auth())
            .json(&request)
            .send()
            .await?;

        self.deserialize_response(response).await
    }

    /// Make a DELETE request to {path}
    pub(crate) async fn delete<O>(&self, path: &str) -> Result<O, PassageError>
    where
        O: DeserializeOwned,
    {
        let response = self
            .http_client
            .delete(self.config.url(path))
            .query(&self.config.query())
            .headers(self.config.bearer_auth())
            .send()
            .await?;

        self.deserialize_response(response).await
    }

    /// Make a DELETE request to {path} with given Query and deserialize the
    /// response body
    pub(crate) async fn delete_with_query<Q, O>(
        &self,
        path: &str,
        query: &Q,
    ) -> Result<O, PassageError>
    where
        O: DeserializeOwned,
        Q: Serialize + ?Sized,
    {
        let response = self
            .http_client
            .delete(self.config.url(path))
            .query(&self.config.query())
            .query(query)
            .headers(self.config.bearer_auth())
            .send()
            .await?;

        self.deserialize_response(response).await
    }

    /// TODO: This needs some love
    async fn deserialize_response<O>(&self, response: reqwest::Response) -> Result<O, PassageError>
    where
        O: DeserializeOwned,
    {
        if response.status().is_success() {
            response.json::<O>().await.map_err(PassageError::from)
        } else {
            Err(response
                .json::<ApiError>()
                .await
                .map_err(PassageError::from)?
                .into())
        }
    }
}
