use serde::{Deserialize, Serialize};

/// FontFamily : Body font family
/// Body font family
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FontFamily {
    #[serde(rename = "Helvetica")]
    Helvetica,
    #[serde(rename = "Arial")]
    Arial,
    #[serde(rename = "Arial Black")]
    ArialBlack,
    #[serde(rename = "Verdana")]
    Verdana,
    #[serde(rename = "Tahoma")]
    Tahoma,
    #[serde(rename = "Trebuchet MS")]
    TrebuchetMs,
    #[serde(rename = "Impact")]
    Impact,
    #[serde(rename = "Gill Sans")]
    GillSans,
    #[serde(rename = "Times New Roman")]
    TimesNewRoman,
    #[serde(rename = "Georgia")]
    Georgia,
    #[serde(rename = "Palatino")]
    Palatino,
    #[serde(rename = "Baskerville")]
    Baskerville,
    #[serde(rename = "Andalé Mono")]
    AndalMono,
    #[serde(rename = "Courier")]
    Courier,
    #[serde(rename = "Lucida")]
    Lucida,
    #[serde(rename = "Monaco")]
    Monaco,
    #[serde(rename = "Bradley Hand")]
    BradleyHand,
    #[serde(rename = "Brush Script MT")]
    BrushScriptMt,
    #[serde(rename = "Luminari")]
    Luminari,
    #[serde(rename = "Comic Sans MS")]
    ComicSansMs,
}

impl std::fmt::Display for FontFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Helvetica => write!(f, "Helvetica"),
            Self::Arial => write!(f, "Arial"),
            Self::ArialBlack => write!(f, "Arial Black"),
            Self::Verdana => write!(f, "Verdana"),
            Self::Tahoma => write!(f, "Tahoma"),
            Self::TrebuchetMs => write!(f, "Trebuchet MS"),
            Self::Impact => write!(f, "Impact"),
            Self::GillSans => write!(f, "Gill Sans"),
            Self::TimesNewRoman => write!(f, "Times New Roman"),
            Self::Georgia => write!(f, "Georgia"),
            Self::Palatino => write!(f, "Palatino"),
            Self::Baskerville => write!(f, "Baskerville"),
            Self::AndalMono => write!(f, "Andalé Mono"),
            Self::Courier => write!(f, "Courier"),
            Self::Lucida => write!(f, "Lucida"),
            Self::Monaco => write!(f, "Monaco"),
            Self::BradleyHand => write!(f, "Bradley Hand"),
            Self::BrushScriptMt => write!(f, "Brush Script MT"),
            Self::Luminari => write!(f, "Luminari"),
            Self::ComicSansMs => write!(f, "Comic Sans MS"),
        }
    }
}

impl Default for FontFamily {
    fn default() -> FontFamily {
        Self::Helvetica
    }
}
