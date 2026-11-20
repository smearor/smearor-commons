use crate::ParseColorError;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;

/// The Pantone Fashion, Home + Interiors (FHI) material suffix - the
/// code of a Pantone FHI color name (`Pantone 19-4057 TCX True Blue`)
/// naming the material the physical swatch is produced on.
///
/// FHI colors are material-independent: the same code and name exist
/// on every substrate, so the suffix qualifies a swatch rather than
/// the color itself - unlike [`crate::PantoneSuffix`], which is part
/// of the PMS color name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum PantoneFhiSuffix {
    /// `TCX` - Textile Cotton eXtended, the cotton fabric guide.
    #[cfg_attr(feature = "schemars", schemars(rename = "TCX"))]
    Cotton,
    /// `TPX` - Textile Paper eXtended, the discontinued paper guide
    /// (superseded by `TPG`).
    #[cfg_attr(feature = "schemars", schemars(rename = "TPX"))]
    Paper,
    /// `TPG` - Textile Paper Green, the current paper guide.
    #[cfg_attr(feature = "schemars", schemars(rename = "TPG"))]
    PaperGreen,
}

impl PantoneFhiSuffix {
    /// The suffix code: `"TCX"`, `"TPX"` or `"TPG"`.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Cotton => "TCX",
            Self::Paper => "TPX",
            Self::PaperGreen => "TPG",
        }
    }
}

impl FromStr for PantoneFhiSuffix {
    type Err = ParseColorError;

    /// Parses the suffix code or the spelled-out variant, case-insensitively.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "tcx" | "cotton" => Ok(Self::Cotton),
            "tpx" | "paper" => Ok(Self::Paper),
            "tpg" | "paper green" => Ok(Self::PaperGreen),
            _ => Err(ParseColorError::UnknownPantoneFhiSuffix { input: s.into() }),
        }
    }
}

impl Display for PantoneFhiSuffix {
    /// Formats the suffix code (`TCX`, `TPX` or `TPG`).
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The wire format is the suffix code.
impl Serialize for PantoneFhiSuffix {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

/// Deserialization accepts the code or the spelled-out variant.
impl<'de> Deserialize<'de> for PantoneFhiSuffix {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer)?.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "zbus")]
impl zvariant::Type for PantoneFhiSuffix {
    const SIGNATURE: &'static zvariant::Signature = <str as zvariant::Type>::SIGNATURE;
}

#[cfg(feature = "async-graphql")]
async_graphql::scalar!(PantoneFhiSuffix);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_str_and_display_produce_the_code() {
        assert_eq!(PantoneFhiSuffix::Cotton.as_str(), "TCX");
        assert_eq!(PantoneFhiSuffix::Paper.to_string(), "TPX");
        assert_eq!(PantoneFhiSuffix::PaperGreen.to_string(), "TPG");
    }

    #[test]
    fn from_str_accepts_code_and_word_case_insensitively() {
        assert_eq!("TCX".parse::<PantoneFhiSuffix>().unwrap(), PantoneFhiSuffix::Cotton);
        assert_eq!("cotton".parse::<PantoneFhiSuffix>().unwrap(), PantoneFhiSuffix::Cotton);
        assert_eq!("TPX".parse::<PantoneFhiSuffix>().unwrap(), PantoneFhiSuffix::Paper);
        assert_eq!("tpg".parse::<PantoneFhiSuffix>().unwrap(), PantoneFhiSuffix::PaperGreen);
        assert_eq!("Paper Green".parse::<PantoneFhiSuffix>().unwrap(), PantoneFhiSuffix::PaperGreen);
        assert_eq!("x".parse::<PantoneFhiSuffix>().unwrap_err(), ParseColorError::UnknownPantoneFhiSuffix { input: "x".into() });
    }

    #[test]
    fn serde_round_trip() {
        let json = serde_json::to_string(&PantoneFhiSuffix::Cotton).unwrap();
        assert_eq!(json, "\"TCX\"");
        assert_eq!(serde_json::from_str::<PantoneFhiSuffix>("\"paper\"").unwrap(), PantoneFhiSuffix::Paper);
    }
}
