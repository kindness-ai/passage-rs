use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CredentialAssertionChallenge {
    #[serde(rename = "challenge")]
    pub challenge: Box<models::ProtocolPeriodCredentialAssertion>,
    #[serde(rename = "id")]
    pub id: String,
}

impl CredentialAssertionChallenge {
    pub fn new(
        challenge: models::ProtocolPeriodCredentialAssertion,
        id: String,
    ) -> CredentialAssertionChallenge {
        CredentialAssertionChallenge {
            challenge: Box::new(challenge),
            id,
        }
    }
}
