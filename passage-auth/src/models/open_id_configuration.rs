use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OpenIdConfiguration {
    #[serde(rename = "authorization_endpoint")]
    pub authorization_endpoint: String,
    #[serde(rename = "issuer")]
    pub issuer: String,
    #[serde(rename = "jwks_uri")]
    pub jwks_uri: String,
}

impl OpenIdConfiguration {
    pub fn new(
        authorization_endpoint: String,
        issuer: String,
        jwks_uri: String,
    ) -> OpenIdConfiguration {
        OpenIdConfiguration {
            authorization_endpoint,
            issuer,
            jwks_uri,
        }
    }
}
