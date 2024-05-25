use serde::{Deserialize, Serialize};

/// TtlDisplayUnit : Deprecated Property. The preferred unit for displaying the
/// TTL. This value is for display only. * `s` - seconds * `m` - minutes * `h` -
/// hours * `d` - days Deprecated Property. The preferred unit for displaying
/// the TTL. This value is for display only. * `s` - seconds * `m` - minutes *
/// `h` - hours * `d` - days
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TtlDisplayUnit {
    #[serde(rename = "s")]
    S,
    #[serde(rename = "m")]
    M,
    #[serde(rename = "h")]
    H,
    #[serde(rename = "d")]
    D,
}

impl std::fmt::Display for TtlDisplayUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::S => write!(f, "s"),
            Self::M => write!(f, "m"),
            Self::H => write!(f, "h"),
            Self::D => write!(f, "d"),
        }
    }
}

impl Default for TtlDisplayUnit {
    fn default() -> TtlDisplayUnit {
        Self::S
    }
}
