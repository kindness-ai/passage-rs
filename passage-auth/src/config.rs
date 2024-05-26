use reqwest::header::HeaderMap;

/// Default v1 Passage Auth API base
pub const PASSAGE_AUTH_API_BASE: &str = "https://auth.passage.id/v1";

#[derive(Clone, Debug)]
pub struct Config {
    api_base: String,
    api_key: Option<String>,
    app_id: String,
    pub_jwk: Option<String>,
    user_bearer_token: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_base: PASSAGE_AUTH_API_BASE.into(),
            api_key: std::env::var("PASSAGE_API_KEY").ok(),
            app_id: std::env::var("PASSAGE_APP_ID").unwrap_or_else(|_| "".to_string()),
            pub_jwk: std::env::var("PASSAGE_PUB_JWK").ok(),
            user_bearer_token: None,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_api_base(mut self, api_base: String) -> Self {
        self.api_base = api_base;
        self
    }

    pub fn with_api_key(mut self, api_key: String) -> Self {
        if api_key.is_empty() {
            return self;
        }
        self.api_key = Some(api_key);
        self
    }

    pub fn with_app_id(mut self, app_id: String) -> Self {
        self.app_id = app_id;
        self
    }

    pub fn with_pub_jwk(mut self, pub_jwk: String) -> Self {
        if pub_jwk.is_empty() {
            return self;
        }
        self.pub_jwk = Some(pub_jwk);
        self
    }

    pub fn with_user_bearer_token(mut self, user_bearer_token: String) -> Self {
        self.user_bearer_token = Some(user_bearer_token);
        self
    }

    pub fn pub_jwk(&self) -> Option<&String> {
        self.pub_jwk.as_ref()
    }

    pub fn url(&self, path: &str) -> String {
        format!(
            "{}{}",
            self.api_base,
            path.replace("{app_id}", &self.app_id)
        )
    }

    pub fn query(&self) -> Vec<(&str, &str)> {
        vec![]
    }

    /// TODO: Can probably just return a token and the http client can handle
    /// the header
    pub fn bearer_auth(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        if let Some(token) = &self.user_bearer_token {
            headers.insert(
                "Authorization",
                format!("Bearer {}", token).parse().unwrap(),
            );
        }
        headers
    }

    /// TODO: Remove once we have `passage-manage` crate
    pub fn api_key_auth(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        if let Some(api_key) = &self.api_key {
            headers.insert(
                "Authorization",
                format!("Bearer {}", api_key).parse().unwrap(),
            );
        }
        headers
    }

    pub fn app_id(&self) -> &str {
        &self.app_id
    }
}
