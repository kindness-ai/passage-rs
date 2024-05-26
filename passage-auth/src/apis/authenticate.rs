use std::fmt::format;

use serde::Deserialize;

use crate::{
    models::{
        authenticate_web_authn_finish_with_transaction_request::AuthenticateWebAuthnFinishWithTransactionRequest,
        authenticate_web_authn_start_with_transaction_request::AuthenticateWebAuthnStartWithTransactionRequest,
        authenticate_web_authn_start_with_transaction_response::AuthenticateWebAuthnStartWithTransactionResponse,
        Nonce,
    },
    AuthError, Config, Passage, PassageError,
};

#[derive(Debug, Deserialize)]
struct Claims {
    sub: String,
}

pub struct Authenticate<'c> {
    client: &'c Passage,
}

impl<'c> Authenticate<'c> {
    pub fn new(client: &'c Passage) -> Self {
        Self { client }
    }

    /// Complete a WebAuthn authentication and authenticate the user via a
    /// transaction. This endpoint accepts and verifies the response from
    /// `navigator.credential.get()` and returns a nonce meant to be exchanged
    /// for an authentication token for the user.
    pub async fn authenticate_webauthn_finish_with_transaction(
        &self,
        auth_finish_request: AuthenticateWebAuthnFinishWithTransactionRequest,
    ) -> Result<Nonce, PassageError> {
        self.client
            .post(
                "/apps/{app_id}/authenticate/transactions/webauthn/finish",
                auth_finish_request,
            )
            .await
    }

    /// Initiate a WebAuthn authentication for a user via a transaction. This
    /// endpoint creates a WebAuthn credential assertion challenge that is used
    /// to perform the authentication ceremony from the browser.
    pub async fn authenticate_webauthn_start_with_transaction(
        &self,
        auth_start_request: Option<AuthenticateWebAuthnStartWithTransactionRequest>,
    ) -> Result<AuthenticateWebAuthnStartWithTransactionResponse, PassageError> {
        self.client
            .post(
                "/apps/{app_id}/authenticate/transactions/webauthn/start",
                auth_start_request,
            )
            .await
    }

    /// Verifies the Passage authentication token.
    /// When successful, the resulting `String` is the authenticated Passage
    /// user ID. See [Validation Passage JWTs](https://docs.passage.id/backend/overview/other#validation-passage-jwts) for details.
    pub fn authenticate_token(&self, token: &str) -> Result<String, AuthError> {
        use jsonwebtoken::{decode, decode_header, jwk::Jwk, Algorithm, DecodingKey, Validation};

        let jwk = self.client.pub_jwk().ok_or(AuthError::PubKeyMissing)?;
        let jwk: Jwk =
            serde_json::from_str(jwk).map_err(|e| AuthError::PubKeyParsing(e.to_string()))?;

        let header = decode_header(token).map_err(AuthError::TokenHeaderDecoding)?;
        if header.kid != jwk.common.key_id {
            return Err(AuthError::KidMismatch(header.kid, jwk.common.key_id));
        }
        let expected_iss = format!("https://auth.passage.id/v1/apps/{}", self.client.app_id());
        let mut validation = Validation::new(Algorithm::RS256);
        validation
            .required_spec_claims
            .extend(["exp", "iss", "nbf", "sub"].into_iter().map(String::from));
        validation.validate_exp = true;
        validation.validate_nbf = true;
        validation.leeway = 0;
        validation.set_issuer(&[expected_iss]);

        let decoding_key = DecodingKey::from_jwk(&jwk).map_err(AuthError::TokenDecoding)?;
        let token = decode::<Claims>(token, &decoding_key, &validation)
            .map_err(AuthError::TokenDecoding)?;

        Ok(token.claims.sub)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_pub_jwk() {
        let passage = Passage::with_config(Config::default().with_app_id("fake".to_string()))
            .set_pub_jwk(PUB_JWK.to_string());
        assert!(passage.pub_jwk().is_some());
    }

    // Generated using https://mkjwk.org/
    const PUB_JWK: &str = r#"{
        "kty": "RSA",
        "e": "AQAB",
        "use": "sig",
        "kid": "r50vKukJl4oVaT78O0ELIGS4w8ynMY_4lRSBq-uvTX4",
        "alg": "RS256",
        "n": "rJGYlYJPZZmeZUyxtEdbbzyMZrBbJPMbhkaioazk6_43d9SIYcVWouei6R5WXQrO6chx3HaSUOqRcYv4oF9x6FVrBWSGyxbzjltcnwKOWn3K8qmJWQvv2nLvLJvf_wdUR2IlH2SfGEE9Om6mJG6tw4Hvn0FauCvnS_a5E5oi0-Mp8rDK3KaHKTr7YHPNzKZzYryF8Ids2mb7PULxFNErIUmB6yTuxUjmbLXwRK2nHe2gHnaepYqcTZIQcTgfS8NeAqKUHWwRkvqmi_pIr9g8azwCqQ8cHpaOoxyUtTlSva1ggkiinJdeIP1-RF-ElflqGtqLXF9OJc8Kcd1ivIaEaQ"
    }"#;

    #[test]
    fn reject_missing_pub_key() {
        let passage: Passage<Config> = Passage::default();

        let jwt: &str = "eyJraWQiOiJyNTB2S3VrSmw0b1ZhVDc4TzBFTElHUzR3OHluTVlfNGxSU0JxLXV2VFg0IiwiYWxnIjoiUlMyNTYiLCJ0eXAiOiJKV1QifQ.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmJmIjoxNjc1NDYxNjg4LCJpc3MiOiJodHRwczovL2F1dGgucGFzc2FnZS5pZC92MS9hcHBzL2Zha2UiLCJleHAiOjIwMDAwMDAwMDB9.hPDcPU5Y84MTiQZ9uZ0aJqxzLEBQiD9F2xWeZINGIKbwehHudExV0MoqoLxHnpUcGIKPIaW0FjCDCZcJA2dGoLC6n-X8l7qUgMJBbbCIEtNhQNMe4AIlEpsmk3t83WNXSQVeh2fKBAJ1X_oad1RRNuQUgCam6MMJx8m3AozPBAXcGjS6D_pJ7N0oPEm5uNq_nSx0GqF0aEUMRiTqG1mY7f8mJtch7vJqxwWPlBZ32lrPmW0xswYLEx2sVZTnYFZqroZH31KePIpHoawrFTNuHQAsSCd1hI8Fj2gZ0ZfT8MFKftbx7_1Pum4KwK4eMv-W2urPsFH3-uU2G0wOaAi-yQ";
        let res = passage.authenticate().authenticate_token(jwt);

        match res {
            Err(AuthError::PubKeyMissing) => {}
            _ => unreachable!("missing pub key was not properly rejected: {:?}", res),
        }
    }

    #[test]
    fn reject_bad_signature() {
        let passage: Passage<Config> = Passage::default().set_pub_jwk(PUB_JWK.to_string());

        let jwt: &str = "eyJraWQiOiJyNTB2S3VrSmw0b1ZhVDc4TzBFTElHUzR3OHluTVlfNGxSU0JxLXV2VFg0IiwiYWxnIjoiUlMyNTYiLCJ0eXAiOiJKV1QifQ.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmJmIjoxNjc1NDYxNjg4LCJpc3MiOiJodHRwczovL2F1dGgucGFzc2FnZS5pZC92MS9hcHBzL2Zha2UiLCJleHAiOjIwMDAwMDAwMDB9.Pxj_GZChf9Cx70QAIpUpAPkJVFErhkxYrJCF3XHLyBdStWy17BrVVhnR2GBG5DCHOmI9jleUre-PUokETTu_nqAGhPB1fulouZUZwZPgJqS6kxQf4VSjumgTDUdmKyptAL2Yo1HOd-bqJrrSrLEST1iQgnWWuHmRcztQn89AxAGJkycAG6Pj8ot7qp3LC6xgOzlqL4mEqgLPNw-R_U_9Zr7Pqy8IbVWBxPz1rF9mPKPib1CLCQ_Jk_Ncmq_LyP70otyssmIEDvAovJn8tSsdIho9W4qGvSpHKeqZTxN0xJq-2KUXnORgrGOVu3cudc7SXmw31g3ZcRY09NUO0Q2uTg";
        let res = passage.authenticate().authenticate_token(jwt);

        match res {
            Err(AuthError::TokenDecoding(_)) => {}
            _ => unreachable!("bad signature was not properly rejected: {:?}", res),
        }
    }

    #[test]
    fn reject_bad_kid() {
        let passage = Passage::with_config(Config::default().with_app_id("fake".to_string()))
            .set_pub_jwk(PUB_JWK.to_string());

        let jwt = "eyJraWQiOiJyNTB2S3VrSmw0b1ZhVDc4TzBFTElHUzR3OHluTVlfNGxSU0JxLXV2VFgzIiwiYWxnIjoiUlMyNTYiLCJ0eXAiOiJKV1QifQ.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmJmIjoxNjc1NDYxNjg4LCJpc3MiOiJodHRwczovL2F1dGgucGFzc2FnZS5pZC92MS9hcHBzL2Zha2UiLCJleHAiOjIwMDAwMDAwMDB9.UzLMx80WPn8UG2RcWKxR9OSimouOI8Ag4bS5IHOzI0ueVG4qu55JvQGPEsKbevmEzVUchj1F-r2BgKK87TThQ4L112WgntNomV19kGUaGPPhkqrmMS5-bk3wAjhTCXgg84QeuMKlqN7PpF6MP1u98psWLfHHFXLl2Sy6aDsjtT8Hag8NmWn83sz2oNLqJfXmApZ3lFpwIT4o8B6ZTVF7USTNxHlt9vtA7OdYDF4V1ZPMRAf4xOStfUayOLoHwnv0YX3IR5NvVhuMo1Ej4p2S6_q8pjx-8-CM5gCFRNt0xSGG6LXdH971wTbvTDVfVeBEABmBul5KXVNOZ54YUkZcpQ";
        let res = passage.authenticate().authenticate_token(jwt);

        match res {
            Err(AuthError::KidMismatch(_, _)) => {}
            _ => unreachable!("incorrect kid was not properly rejected: {:?}", res),
        }
    }

    #[test]
    fn reject_missing_sub() {
        let passage = Passage::with_config(Config::default().with_app_id("fake".to_string()))
            .set_pub_jwk(PUB_JWK.to_string());

        let jwt = "eyJraWQiOiJyNTB2S3VrSmw0b1ZhVDc4TzBFTElHUzR3OHluTVlfNGxSU0JxLXV2VFg0IiwiYWxnIjoiUlMyNTYiLCJ0eXAiOiJKV1QifQ.eyJuYmYiOjE2NzU0NjE2ODgsImlzcyI6Imh0dHBzOi8vYXV0aC5wYXNzYWdlLmlkL3YxL2FwcHMvZmFrZSIsImV4cCI6MjAwMDAwMDAwMH0.orBGLQfdSKV1NLyJqXZapREZIT7BAb33vY1ovvM3lbHS9S7fNT_qZz-bQZZ_NkrL9nMB8mmX2A4PyHWfin1pHZOvhNKhcsVeIfZHBP9SYzUzXsWdqmSiPqd6VBAhQZs1OSwJz4K6JV4_igR40QImxRvg2AXcu3AiUdGU0nuuJ9Vtd7RwdXUx41cVpIyCiOsN4kPFpVaSYQ1-Qn9aowBea5j4h7EIhZaLAkTDJT3KuQxyxhJnO2-XubrQREwd8CilOIV1evrdaQkR4Xqw3FBcvjOiRW6zW0sIdANxk_jIqC2Vdp0feQKYvUFxea3xHAujz5TIi9q7sJzgJPBsjI1MzA";
        let res = passage.authenticate().authenticate_token(jwt);

        match res {
            Err(AuthError::TokenDecoding(_)) => {}
            _ => unreachable!("missing sub was not properly rejected: {:?}", res),
        }
    }

    #[test]
    fn reject_missing_nbf() {
        let passage = Passage::with_config(Config::default().with_app_id("fake".to_string()))
            .set_pub_jwk(PUB_JWK.to_string());

        let jwt = "eyJraWQiOiJyNTB2S3VrSmw0b1ZhVDc4TzBFTElHUzR3OHluTVlfNGxSU0JxLXV2VFg0IiwiYWxnIjoiUlMyNTYiLCJ0eXAiOiJKV1QifQ.eyJzdWIiOiIxMjM0NTY3ODkwIiwiaXNzIjoiaHR0cHM6Ly9hdXRoLnBhc3NhZ2UuaWQvdjEvYXBwcy9mYWtlIiwiZXhwIjoyMDAwMDAwMDAwfQ.lZ2zTZmJsIcQE2XDV-N8sVFvK2AxN4GWW_fId6yc2uSJFtQc26HcB0ywGn7BjhB8OD4rX3WkA9XqyUl51fKCnVlE8hlk4VlfDyewKahJkPmoqNX7QwDzA9ORd-5FlZJ1_8nsMzH0jn8ydkKJORgxGKfj_xZD73mW9gz31bVbYddPmPcAmhuJCvI_4dlNVmIfEk-UUNmtIJEc89iwbcg_baEUJDXXztUfYhw4M3WC58ptI5GZk9JLIcq5PU59Sn495d14Xeek19PD93ypSwsLRAwXXU6OQgRbFZtYdrshcDpZ3339RfuO6xBlTBqet5BbVMm-f28Mlqw2x2UvuQ_h-g";
        let res = passage.authenticate().authenticate_token(jwt);

        match res {
            Err(AuthError::TokenDecoding(_)) => {}
            _ => unreachable!("missing nbf was not properly rejected: {:?}", res),
        }
    }

    #[test]
    fn reject_future_nbf() {
        let passage = Passage::with_config(Config::default().with_app_id("fake".to_string()))
            .set_pub_jwk(PUB_JWK.to_string());

        let jwt = "eyJraWQiOiJyNTB2S3VrSmw0b1ZhVDc4TzBFTElHUzR3OHluTVlfNGxSU0JxLXV2VFg0IiwiYWxnIjoiUlMyNTYiLCJ0eXAiOiJKV1QifQ.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmJmIjoyMDAwMDAwMDAwLCJpc3MiOiJodHRwczovL2F1dGgucGFzc2FnZS5pZC92MS9hcHBzL2Zha2UiLCJleHAiOjIwMDAwMDAwNjB9.HoPwswPk4euGoVyXZMlo2lwzUhHtXrzhyc5ZGy8QI0pStvkYB_fDyyPsL8u-TuHKdm5ezakQr1mvYdnJABpMi1X3qsUMYbU2Rs0Wk906YYnzMAmMANRkKAXw5uLTBjdWu_NG-KMYWom_N0rYGBGGAq5np8k1OHJWrZDJamCdqqcIY7n7hD4mwXMwzLoKRQQtojvtRijnzKGMThUfe7-0YrMPIi941P2Z86MSDXennU2cuoJXAMYndxfdNFXyt74DocTKXEfWR1gtdZqcUCG12TAhWxm_6qRjMcDTiO1gpXGjoommCMxgRU3Mm-XM734MHLMFWFma9Ldci8rbrmeypg";
        let res = passage.authenticate().authenticate_token(jwt);

        match res {
            Err(AuthError::TokenDecoding(_)) => {}
            _ => unreachable!("future nbf was not properly rejected: {:?}", res),
        }
    }

    #[test]
    fn reject_missing_iss() {
        let passage = Passage::with_config(Config::default().with_app_id("fake".to_string()))
            .set_pub_jwk(PUB_JWK.to_string());

        let jwt = "eyJraWQiOiJyNTB2S3VrSmw0b1ZhVDc4TzBFTElHUzR3OHluTVlfNGxSU0JxLXV2VFg0IiwiYWxnIjoiUlMyNTYiLCJ0eXAiOiJKV1QifQ.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmJmIjoxNjc1NDYxNjg4LCJleHAiOjIwMDAwMDAwMDB9.T61Xje9mZYOxQSIjvbV30gWqjz8kfZhqVG_KnCxmb3iXuERoTkjZFVZYeuHSKrTHMkxfrhAc7CjgREiHF1fJM9UCDWkl0CMpzfxfg5MVTF-ZoZ3cVmPjd4oslq5Ggjx7coo1kl7OhCY7w9XdGWGu7zCfMYmCNE-LwQ3h1Kj9NkxHv3HtcgKk6fvSdpMJ8IcIuGR-SLgr7yuQs9IBnwXb7tCSjY_5Lg3vpTpgB7_M2485Yyfx6ZUgUgY6u-8E3a2mMGbRtk3G6C_SnH4HTvkn2QGNd9b5F6Llcs4aQpKuSe--GIJg4FNVTKJ0M_27ycSZYu-UMolVmUm4QUqZCmLZzQ";
        let res = passage.authenticate().authenticate_token(jwt);

        match res {
            Err(AuthError::TokenDecoding(_)) => {}
            _ => unreachable!("missing iss was not properly rejected: {:?}", res),
        }
    }

    #[test]
    fn reject_wrong_iss() {
        let passage = Passage::with_config(Config::default().with_app_id("fake".to_string()))
            .set_pub_jwk(PUB_JWK.to_string());

        let jwt = "eyJraWQiOiJyNTB2S3VrSmw0b1ZhVDc4TzBFTElHUzR3OHluTVlfNGxSU0JxLXV2VFg0IiwiYWxnIjoiUlMyNTYiLCJ0eXAiOiJKV1QifQ.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmJmIjoxNjc1NDYxNjg4LCJpc3MiOiJodHRwczovL2F1dGgucGFzc2FnZS5pZC92MS9hcHBzL3dyb25nIiwiZXhwIjoyMDAwMDAwMDAwfQ.DTLsHvjK7ewJ1aajQdddMtHH2rx5ripQzjubOzZExNmtGvaVHUAlaa9vO_gu4NgpGg8m11IotqfeZUqLxVSSJ_GBLFVXcvBp2hRILs8JyU2uRdgur_n6Re1GoQpsfPqNxAdjDnRLE9QaXDDk-ErG3xdM4tDW9x_UGnrnlPAhePhGEXDSYzSDe0RmXFKcS0AzkQMztwiEW3HWunxVmZhMniPVWfzAuFqO28VVzLIpMFDsBsseHUzFhBDyzshNGHmk1t4pgEUXafrqi_DR_ammxP5Wp8U-4syzgNZ1WvVs7hJeXgDHAV3xwMH083p8p1HqqLsz5Zfqw8A6yu8TkcEctw";
        let res = passage.authenticate().authenticate_token(jwt);

        match res {
            Err(AuthError::TokenDecoding(_)) => {}
            _ => unreachable!("wrong iss was not properly rejected: {:?}", res),
        }
    }

    #[test]
    fn reject_missing_exp() {
        let passage = Passage::with_config(Config::default().with_app_id("fake".to_string()))
            .set_pub_jwk(PUB_JWK.to_string());

        let jwt = "eyJraWQiOiJyNTB2S3VrSmw0b1ZhVDc4TzBFTElHUzR3OHluTVlfNGxSU0JxLXV2VFg0IiwiYWxnIjoiUlMyNTYiLCJ0eXAiOiJKV1QifQ.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmJmIjoxNjc1NDYxNjg4LCJpc3MiOiJodHRwczovL2F1dGgucGFzc2FnZS5pZC92MS9hcHBzL2Zha2UifQ.BXvWfR2zFI-Tm72BAZgqQuykfzs4cOswlPP_H-8usiBAwpg6LExhWis9R8YJch5fcHAUfgbIMxZnwhylfESXrqs9QxAarn0M3NIGF8bI32nTNPrQpBUJCzdYh6OCaJ8G7lftY2LTDcGHq0v18ikILykoloN69wjys-eStrW2yr3_XIGSkHpbOjVSSru30XTRndT30rImytR8EBsWN0vsgyucy2X0-NCfsfa3Wl4vQUV5nxtO1ejpTmr0LvfHENEXyEoA2Q5Rr5PuLHF03kbLjlD81OPPETUZdPPKclyjlPozKraX6TnvUGVQq4XM00YlL5qoUZ4HBLIVusKi_d9kPw";
        let res = passage.authenticate().authenticate_token(jwt);

        match res {
            Err(AuthError::TokenDecoding(_)) => {}
            _ => unreachable!("missing exp was not properly rejected: {:?}", res),
        }
    }

    #[test]
    fn reject_past_exp() {
        let passage = Passage::with_config(Config::default().with_app_id("fake".to_string()))
            .set_pub_jwk(PUB_JWK.to_string());

        let jwt = "eyJraWQiOiJyNTB2S3VrSmw0b1ZhVDc4TzBFTElHUzR3OHluTVlfNGxSU0JxLXV2VFg0IiwiYWxnIjoiUlMyNTYiLCJ0eXAiOiJKV1QifQ.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmJmIjoxNjc1NDYxNjg4LCJpc3MiOiJodHRwczovL2F1dGgucGFzc2FnZS5pZC92MS9hcHBzL2Zha2UiLCJleHAiOjE2NzU0NjE4ODh9.YJlpsocdeIAaMATZDBz5EsdbOhtNDTDGr4_j7mEtWthU0JJdhWIoNxaX5Dep1C9yNf8hFFg0r3re6ImoQz4-vMkOGmObUxAP-7hlMEhfx8ww7Slj1vn_3ZEAHrp3JUS4jirQNak9-qvOr4Ndh0XsvzVAWca216hAo2PMXZwmaS8vBG8bpm5sVFevB6f-rW3OVEgafcmagRlFpgXLum6vcw18nsRV9qcvcQZDlW9x7Z7cJEW13e35qWz_urdOgB9EdD_feVuG1zlE_MbBgE6EtSNTumlqnB_Iae1KeM-nHJkeKkCbfvbd1WCc5lI3N8mv0M7m7nRBxQM6TFSbXqrI2g";
        let res = passage.authenticate().authenticate_token(jwt);

        match res {
            Err(AuthError::TokenDecoding(_)) => {}
            _ => unreachable!("past exp was not properly rejected: {:?}", res),
        }
    }

    #[test]
    fn authenticate_good_token() {
        let passage = Passage::with_config(Config::default().with_app_id("fake".to_string()))
            .set_pub_jwk(PUB_JWK.to_string());

        let jwt = "eyJraWQiOiJyNTB2S3VrSmw0b1ZhVDc4TzBFTElHUzR3OHluTVlfNGxSU0JxLXV2VFg0IiwiYWxnIjoiUlMyNTYiLCJ0eXAiOiJKV1QifQ.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmJmIjoxNjc1NDYxNjg4LCJpc3MiOiJodHRwczovL2F1dGgucGFzc2FnZS5pZC92MS9hcHBzL2Zha2UiLCJleHAiOjIwMDAwMDAwMDB9.hPDcPU5Y84MTiQZ9uZ0aJqxzLEBQiD9F2xWeZINGIKbwehHudExV0MoqoLxHnpUcGIKPIaW0FjCDCZcJA2dGoLC6n-X8l7qUgMJBbbCIEtNhQNMe4AIlEpsmk3t83WNXSQVeh2fKBAJ1X_oad1RRNuQUgCam6MMJx8m3AozPBAXcGjS6D_pJ7N0oPEm5uNq_nSx0GqF0aEUMRiTqG1mY7f8mJtch7vJqxwWPlBZ32lrPmW0xswYLEx2sVZTnYFZqroZH31KePIpHoawrFTNuHQAsSCd1hI8Fj2gZ0ZfT8MFKftbx7_1Pum4KwK4eMv-W2urPsFH3-uU2G0wOaAi-yQ";
        let res = passage.authenticate().authenticate_token(jwt);

        dbg!(&res);
        assert_eq!(res, Ok("1234567890".to_owned()));
    }
}

/* Private key for test:
{
    "p": "9WRlEysjzbea25MPFvMMioGvShW4vZD0Qhhc4yVRZz0PpRXpW5wVQKMJqd1N7vfiXA_OMtGY3pTMegUhF_Mw7W2S1b0_2V_xAXYt8g4G0IY0aT9GBETB63ga4FLccJCSkjIagtt5TOhO5IOIDboghEKkQvguNTSJPi3J5Dvp_PM",
    "kty": "RSA",
    "q": "tAdPC8Yo08Cb951vkfWmjZyJuosjRHcWugvrVivnuWVyHouuX9ktbE-JRREhQ7o-58EXJZJ_el07_IE1xKoKlaJ3saEOfWDOApDiJxbbwwnMGCTqdsi8Q07DN4PgYFcSr5MXd9ZFemqVBXW84yFKVXPNKXfR_VoI9GlURQU6YDM",
    "d": "J_qnHeQNnt0jDBbjiH-LmE6vvE6ZHwtPUiFlJg2XD3FaymEro3MDakQ9wsIrgeyyGQk-D7RMm4BsZ6Dk3cqe6hN38sziSYSssktKPvBpqF9COEu8rSuNys8bx_rovv2ksdD0BrzZ-tWKaNIfnYsiqIuexwduDALn1_p10CvCa9HvY9Z_wcuW4hazdMDXZhQIDexldd6hpdB4XgIftqmvrMV7uTCENcLrZ_daJO_dKugybin828asAjXzua2sNCD3QYKmWVR65p-4PBDBKPFWyEuV3C2zpPE_rBex-B1iOwKwlF_-UPMSpPbaGzgyB2Nl4k1UQ7CZBMWswFnS6FnJ_Q",
    "e": "AQAB",
    "use": "sig",
    "kid": "r50vKukJl4oVaT78O0ELIGS4w8ynMY_4lRSBq-uvTX4",
    "qi": "7-uVCCf2T6nQcp4_jHt1YA6hb9anJY5NjA-kPtm4OcbUKQD5i3XoM7Gu3vMgw0fYdLigDa4Nt6qmpOK-On0S74fJdL4iR_8dsq6ytO1Q3Sl7xkvZmNRiQV2lr-DNLR5Wl8UCeoDmKzF8u3Y1riUkr9sk-mrLTWqMSK_r2Th5NKc",
    "dp": "62Ix0gE_hsTntleJ0emxzeo3ykirvKqeogfckcXqH61ipGgwP7-oYygAzP-LEf6VEtnWYMjMajUxLppc9CxCcnz4rC2sYUa2V0CVMepifwM8ovgeoVmS6dt7bFIPQapr7fBBneQIpszvYCMLDp_LMRL7nYGSUVbjjtE9J8CQ4iE",
    "alg": "RS256",
    "dq": "PnMimoT8-Keh8v1sDIfYZNtec5V8gG2HNraXxmaolYl5UttFe_5MYXwdtBXDIkljNOWob-In0nyxKGByFGygC1Q2jSm_awK_s-gqa0Dkrv2hDOcRZm8vz3FtCr72gLTzyHAP_gQYSeTbGO_EvE16CbaH_tCPyYEIBjDbiK3NmD0",
    "n": "rJGYlYJPZZmeZUyxtEdbbzyMZrBbJPMbhkaioazk6_43d9SIYcVWouei6R5WXQrO6chx3HaSUOqRcYv4oF9x6FVrBWSGyxbzjltcnwKOWn3K8qmJWQvv2nLvLJvf_wdUR2IlH2SfGEE9Om6mJG6tw4Hvn0FauCvnS_a5E5oi0-Mp8rDK3KaHKTr7YHPNzKZzYryF8Ids2mb7PULxFNErIUmB6yTuxUjmbLXwRK2nHe2gHnaepYqcTZIQcTgfS8NeAqKUHWwRkvqmi_pIr9g8azwCqQ8cHpaOoxyUtTlSva1ggkiinJdeIP1-RF-ElflqGtqLXF9OJc8Kcd1ivIaEaQ"
}
*/
