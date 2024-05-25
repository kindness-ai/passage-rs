use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdTokenRequest {
    /// The code given from the native mobile OS to create a token
    #[serde(rename = "code")]
    pub code: String,
    /// The identity JWT token with security claims for validation
    #[serde(rename = "id_token")]
    pub id_token: String,
    /// Connection type; google or apple
    #[serde(rename = "connection_type")]
    pub connection_type: ConnectionType,
}

impl IdTokenRequest {
    pub fn new(code: String, id_token: String, connection_type: ConnectionType) -> IdTokenRequest {
        IdTokenRequest {
            code,
            id_token,
            connection_type,
        }
    }
}
/// Connection type; google or apple
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ConnectionType {
    #[serde(rename = "apple")]
    Apple,
    #[serde(rename = "google")]
    Google,
}

impl Default for ConnectionType {
    fn default() -> ConnectionType {
        Self::Apple
    }
}
