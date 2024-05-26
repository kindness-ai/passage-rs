use crate::{
    models::{jwk_response::JwkResponse, RefreshAuthTokenRequest},
    Config, Passage, PassageError,
};

pub struct Jwks<'c> {
    client: &'c Passage,
}

impl<'c> Jwks<'c> {
    pub fn new(client: &'c Passage) -> Self {
        Self { client }
    }

    //// Get JWKS for an app. KIDs in the JWT can be used to match the appropriate
    /// JWK, and use the JWK's public key to verify the JWT.
    pub async fn get_jwks(&self) -> Result<JwkResponse, PassageError> {
        self.client
            .get("/apps/{app_id}/.well-known/jwks.json")
            .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::{models::JwkResponseKeysInner, Config};

    const APP_ID: &str = "PaItOH7Ul7n2Xt3uxY671sFN";
    const JWK: &str = r#"{"alg":"RS256","e":"AQAB","kid":"PmRBUyQdPftxuIKa6dlmGMZA","kty":"RSA","n":"1ocLCGc8dBtTM8T49vykwEsVt901faUCg3z4__TKU0Ldzk9PVGW7cv_l9fMLel7roM0DTBMRnWB4-xZGtLPuGK7JdwKMhgmgV6ymzrILBFB5O8ozhe78O2Ke9ZojF5y0D70g3XyDe0AfI9DsgjwCjA_1NOzteo6wzTm0j6FZbI1YVwmdxc8nKUJBtHHpTFs9Z5RcGiQff-cPRSxZojUKnQII-6lSLSxxluM1IIUyHqQtDOv708bpUDyspeciPdA7TjQ1h4L6g-oUcFBmqWiDvbhtDR7y1R_cmt5L1ND7AI3PnI3VqVpsn-kTHflQSohd2rj9IyfksZmFhfzbKJC0CQ","use":"sig"}"#;

    #[tokio::test]
    async fn test_get_jwks() -> Result<(), PassageError> {
        let test_jwk: JwkResponseKeysInner = serde_json::from_str(JWK)?;

        let passage = Passage::with_config(Config::default().with_app_id(APP_ID.to_string()));
        let response: JwkResponse = passage.jwks().get_jwks().await?;

        assert!(response.keys.iter().any(|k| dbg!(k) == &test_jwk));
        Ok(())
    }
}
