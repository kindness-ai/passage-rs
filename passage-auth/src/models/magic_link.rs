use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MagicLink {
    pub id: String,
}

impl MagicLink {
    pub fn new(id: String) -> MagicLink {
        MagicLink { id }
    }
}
