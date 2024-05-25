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
