use std::fmt;

use jsonwebtoken as jwt;
use serde::Deserialize;

use crate::models;

#[derive(Debug, thiserror::Error)]
pub enum PassageError {
    /// Underlying error from reqwest library after an API call was made
    #[error("http error: {0}")]
    Reqwest(#[from] reqwest::Error),
    /// Passage returns error object with details of API call failure
    #[error("api error: {:?}", .0)]
    ApiError(ApiError),
    /// Error there's an issue serializing or deserializing data
    #[error("failed to serializing or deserializing api response: {0}")]
    Serde(#[from] serde_json::Error),
    /// Error from client side validation
    /// or when builder fails to build request before making API call
    #[error("invalid args: {0}")]
    InvalidArgument(String),
    // Authentication failures when validating a JWT.
    #[error("failed to authenticate: {0}")]
    AuthError(AuthError),
}

/// The Passage API returns an error enum that can contain various
/// unique codes for each status code, along with an associated error message.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ApiError {
    Status400(models::Model400Error),
    Status401(models::Model401Error),
    Status403(models::Model403Error),
    Status404(models::Model404Error),
    Status409(models::Model409Error),
    Status500(models::Model500Error),
    UnknownValue(serde_json::Value),
}

impl From<ApiError> for PassageError {
    fn from(e: ApiError) -> Self {
        PassageError::ApiError(e)
    }
}

/// The error type for possible authentication failures when validating a JWT.
#[derive(Debug, PartialEq)]
pub enum AuthError {
    /// Failed to decode the header of the Passage JWT
    /// (e.g. the `psg_auth_token` cookie value).
    /// See associated `jwt::errors::Error` for details.
    TokenHeaderDecoding(jwt::errors::Error),

    /// Key IDs of public JWK and Passage JWT do not match
    KidMismatch(Option<String>, Option<String>),

    /// Public JWK was not provided
    PubKeyMissing,

    /// Failed to parse the provided public JWK
    PubKeyParsing(String),

    /// Failed to decode and validate the Passage JWT
    /// (e.g. the `psg_auth_token` cookie value).
    /// See associated `jwt::errors::Error` for details.
    TokenDecoding(jwt::errors::Error),
}

impl From<jwt::errors::Error> for PassageError {
    fn from(e: jwt::errors::Error) -> Self {
        AuthError::TokenDecoding(e).into()
    }
}

impl From<AuthError> for PassageError {
    fn from(e: AuthError) -> Self {
        PassageError::AuthError(e)
    }
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AuthError::TokenHeaderDecoding(e) => {
                write!(f, "Failed to decode the header of the Passage JWT: {}", e)
            }
            AuthError::KidMismatch(kid1, kid2) => write!(
                f,
                "Key IDs of public JWK and Passage JWT do not match: {} vs {}",
                kid1.as_deref().unwrap_or("None"),
                kid2.as_deref().unwrap_or("None")
            ),
            AuthError::PubKeyMissing => write!(f, "Public JWK was not provided"),
            AuthError::PubKeyParsing(e) => {
                write!(f, "Failed to parse the provided public JWK: {}", e)
            }
            AuthError::TokenDecoding(e) => {
                write!(f, "Failed to decode and validate the Passage JWT: {}", e)
            }
        }
    }
}
