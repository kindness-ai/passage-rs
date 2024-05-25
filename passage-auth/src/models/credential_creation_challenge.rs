use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CredentialCreationChallenge {
    #[serde(rename = "challenge")]
    pub challenge: Box<models::CredentialCreation>,
    #[serde(rename = "id")]
    pub id: String,
}

impl CredentialCreationChallenge {
    pub fn new(challenge: models::CredentialCreation, id: String) -> CredentialCreationChallenge {
        CredentialCreationChallenge {
            challenge: Box::new(challenge),
            id,
        }
    }
}
