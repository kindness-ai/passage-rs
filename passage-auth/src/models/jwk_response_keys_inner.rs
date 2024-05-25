use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JwkResponseKeysInner {
    /// the algorithm for the key
    #[serde(rename = "alg")]
    pub alg: String,
    /// the exponent for the standard pem
    #[serde(rename = "e")]
    pub e: String,
    /// the unique identifier for the key
    #[serde(rename = "kid")]
    pub kid: String,
    /// the key type (https://datatracker.ietf.org/doc/html/rfc7518)
    #[serde(rename = "kty")]
    pub kty: String,
    /// the modulus for a standard pem
    #[serde(rename = "n")]
    pub n: String,
    /// how the key is meant to be used (i.e. 'sig' represents signature)
    #[serde(rename = "use")]
    pub r#use: String,
}

impl JwkResponseKeysInner {
    pub fn new(
        alg: String,
        e: String,
        kid: String,
        kty: String,
        n: String,
        r#use: String,
    ) -> JwkResponseKeysInner {
        JwkResponseKeysInner {
            alg,
            e,
            kid,
            kty,
            n,
            r#use,
        }
    }
}
