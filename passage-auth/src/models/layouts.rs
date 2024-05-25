use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Layouts {
    #[serde(rename = "profile")]
    pub profile: Vec<models::LayoutConfig>,
    #[serde(rename = "registration")]
    pub registration: Vec<models::LayoutConfig>,
}

impl Layouts {
    pub fn new(
        profile: Vec<models::LayoutConfig>,
        registration: Vec<models::LayoutConfig>,
    ) -> Layouts {
        Layouts {
            profile,
            registration,
        }
    }
}
