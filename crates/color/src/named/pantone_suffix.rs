use crate::ParseColorError;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;

/// The Pantone fan-book variant - the suffix letter of a Pantone color
/// name (`Pantone 485 C`) naming the paper stock the Formula Guide
/// swatch is printed on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum PantoneSuffix {
    /// `C` - for glossy, coated surfaces and papers.
    #[cfg_attr(feature = "schemars", schemars(rename = "C"))]
    Coated,
    /// `U` - for uncoated, matte papers and substrates.
    #[cfg_attr(feature = "schemars", schemars(rename = "U"))]
    Uncoated,
    /// `M` - for specialized matte print results.
    #[cfg_attr(feature = "schemars", schemars(rename = "M"))]
    Matte,
}

impl PantoneSuffix {
    /// The suffix letter: `"C"`, `"U"` or `"M"`.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Coated => "C",
            Self::Uncoated => "U",
            Self::Matte => "M",
        }
    }
}

impl FromStr for PantoneSuffix {
    type Err = ParseColorError;

    /// Parses the suffix letter or the spelled-out variant, case-insensitively.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "c" | "coated" => Ok(Self::Coated),
            "u" | "uncoated" => Ok(Self::Uncoated),
            "m" | "matte" => Ok(Self::Matte),
            _ => Err(ParseColorError::UnknownPantoneSuffix { input: s.into() }),
        }
    }
}

impl Display for PantoneSuffix {
    /// Formats the suffix letter (`C`, `U` or `M`).
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The wire format is the suffix letter.
impl Serialize for PantoneSuffix {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

/// Deserialization accepts the letter or the spelled-out variant.
impl<'de> Deserialize<'de> for PantoneSuffix {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer)?.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "zbus")]
impl zvariant::Type for PantoneSuffix {
    const SIGNATURE: &'static zvariant::Signature = <str as zvariant::Type>::SIGNATURE;
}

#[cfg(feature = "async-graphql")]
async_graphql::scalar!(PantoneSuffix);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_str_and_display_produce_the_letter() {
        assert_eq!(PantoneSuffix::Coated.as_str(), "C");
        assert_eq!(PantoneSuffix::Uncoated.to_string(), "U");
        assert_eq!(PantoneSuffix::Matte.to_string(), "M");
    }

    #[test]
    fn from_str_accepts_letter_and_word_case_insensitively() {
        assert_eq!("C".parse::<PantoneSuffix>().unwrap(), PantoneSuffix::Coated);
        assert_eq!("coated".parse::<PantoneSuffix>().unwrap(), PantoneSuffix::Coated);
        assert_eq!("U".parse::<PantoneSuffix>().unwrap(), PantoneSuffix::Uncoated);
        assert_eq!("MATTE".parse::<PantoneSuffix>().unwrap(), PantoneSuffix::Matte);
        assert_eq!("x".parse::<PantoneSuffix>().unwrap_err(), ParseColorError::UnknownPantoneSuffix { input: "x".into() });
    }

    #[test]
    fn serde_round_trip() {
        let json = serde_json::to_string(&PantoneSuffix::Coated).unwrap();
        assert_eq!(json, "\"C\"");
        assert_eq!(serde_json::from_str::<PantoneSuffix>("\"uncoated\"").unwrap(), PantoneSuffix::Uncoated);
    }
}
