use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProtocolPeriodCredentialAssertion {
    #[serde(rename = "publicKey")]
    pub public_key: Box<models::ProtocolCredentialAssertionPublicKey>,
}

impl ProtocolPeriodCredentialAssertion {
    pub fn new(
        public_key: models::ProtocolCredentialAssertionPublicKey,
    ) -> ProtocolPeriodCredentialAssertion {
        ProtocolPeriodCredentialAssertion {
            public_key: Box::new(public_key),
        }
    }
}
