use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserMetadataField {
    #[serde(rename = "field_name")]
    pub field_name: String,
    #[serde(rename = "friendly_name")]
    pub friendly_name: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "profile")]
    pub profile: bool,
    #[serde(rename = "registration")]
    pub registration: bool,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl UserMetadataField {
    pub fn new(
        field_name: String,
        friendly_name: String,
        id: String,
        profile: bool,
        registration: bool,
        r#type: String,
    ) -> UserMetadataField {
        UserMetadataField {
            field_name,
            friendly_name,
            id,
            profile,
            registration,
            r#type,
        }
    }
}
