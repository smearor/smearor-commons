use crate::Color;
use crate::CssNamedColor;
use crate::ParseColorError;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;

/// A color in hex string representation: `#RGB`, `#RGBA`, `#RRGGBB` or
/// `#RRGGBBAA`.
///
/// `HexColor` is the validated textual form of a [`Color<u8>`]:
/// [`FromStr`] accepts only the canonical hex notations **with a
/// leading `#`**; [`Display`] always formats `#RRGGBB` (opaque) or
/// `#RRGGBBAA`.
///
/// Deserialization is more lenient than [`FromStr`]: on the wire a
/// [`CssNamedColor`] keyword (`"red"`, `"transparent"`, ...) is accepted
/// and normalized to its hex value; serialization always emits the hex
/// form. Use [`Color`]'s own `FromStr` for parsing user input that may
/// be either.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HexColor(Color<u8>);

/// Parses a single hex digit, reporting the offending character.
fn hex_digit(input: &str, character: char) -> Result<u8, ParseColorError> {
    character.to_digit(16).map(|value| value as u8).ok_or_else(|| ParseColorError::InvalidHexDigit {
        input: input.into(),
        character,
    })
}

/// Parses the digits after the `#` prefix into `[r, g, b, a]` byte
/// components. Shorthand digits are doubled (`F0A` = `FF00AA`).
fn hex_components(input: &str, digits: &str) -> Result<[u8; 4], ParseColorError> {
    let digits: Vec<char> = digits.chars().collect();
    let digit = |index: usize| hex_digit(input, digits[index]);
    let pair = |index: usize| Ok(digit(index)? << 4 | digit(index + 1)?);
    match digits.len() {
        3 => Ok([digit(0)? * 0x11, digit(1)? * 0x11, digit(2)? * 0x11, 255]),
        4 => Ok([digit(0)? * 0x11, digit(1)? * 0x11, digit(2)? * 0x11, digit(3)? * 0x11]),
        6 => Ok([pair(0)?, pair(2)?, pair(4)?, 255]),
        8 => Ok([pair(0)?, pair(2)?, pair(4)?, pair(6)?]),
        count => Err(ParseColorError::InvalidDigitCount { input: input.into(), count }),
    }
}

impl HexColor {
    /// The wrapped byte color.
    pub const fn as_color(&self) -> Color<u8> {
        self.0
    }

    /// Unwraps into the byte color.
    pub const fn into_color(self) -> Color<u8> {
        self.0
    }
}

impl FromStr for HexColor {
    type Err = ParseColorError;

    /// Parses a `#`-prefixed hex color. The `#` is mandatory; CSS named
    /// colors are rejected - they are not hex.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digits = s.strip_prefix('#').ok_or_else(|| ParseColorError::MissingHashPrefix { input: s.into() })?;
        Ok(Self(Color::from(hex_components(s, digits)?)))
    }
}

impl Display for HexColor {
    /// Formats `#RRGGBB` when opaque, `#RRGGBBAA` otherwise.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0.a == 255 {
            write!(f, "#{:02x}{:02x}{:02x}", self.0.r, self.0.g, self.0.b)
        } else {
            write!(f, "#{:02x}{:02x}{:02x}{:02X}", self.0.r, self.0.g, self.0.b, self.0.a)
        }
    }
}

impl From<Color<u8>> for HexColor {
    fn from(color: Color<u8>) -> Self {
        Self(color)
    }
}

impl From<Color<f64>> for HexColor {
    fn from(color: Color<f64>) -> Self {
        Self(Color::<u8>::from(color))
    }
}

impl From<Color<f32>> for HexColor {
    fn from(color: Color<f32>) -> Self {
        Self(Color::<u8>::from(color))
    }
}

impl From<HexColor> for Color<u8> {
    fn from(hex: HexColor) -> Self {
        hex.into_color()
    }
}

impl From<HexColor> for Color<f64> {
    fn from(hex: HexColor) -> Self {
        hex.into_color().into()
    }
}

impl From<HexColor> for Color<f32> {
    fn from(hex: HexColor) -> Self {
        hex.into_color().into()
    }
}

/// The wire format is the hex string `"#rrggbb(aa)"`.
impl Serialize for HexColor {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

/// Deserialization accepts `#`-prefixed hex and [`CssNamedColor`]
/// keywords; both normalize to the wrapped [`Color<u8>`].
impl<'de> Deserialize<'de> for HexColor {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let input = String::deserialize(deserializer)?;
        if input.starts_with('#') {
            return input.parse().map_err(serde::de::Error::custom);
        }
        input
            .parse::<CssNamedColor>()
            .map(Self::from)
            .map_err(|_| serde::de::Error::custom(ParseColorError::InvalidColor { input }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_parses_all_hex_forms() {
        assert_eq!("#FF6B4A".parse::<HexColor>().unwrap().into_color(), Color::<u8>::new(0xFF, 0x6B, 0x4A));
        assert_eq!("#FF6B4A80".parse::<HexColor>().unwrap().into_color(), Color::<u8>::new_rgba(0xFF, 0x6B, 0x4A, 0x80));
        assert_eq!("#F0A".parse::<HexColor>().unwrap().into_color(), Color::<u8>::new(0xFF, 0x00, 0xAA));
        assert_eq!("#F0AF".parse::<HexColor>().unwrap().into_color(), Color::<u8>::new_rgba(0xFF, 0x00, 0xAA, 0xFF));
    }

    #[test]
    fn from_str_requires_the_hash() {
        assert!("FF6B4A".parse::<HexColor>().is_err());
        let error = "FF6B4A".parse::<HexColor>().unwrap_err();
        assert!(matches!(error, ParseColorError::MissingHashPrefix { .. }));
        assert_eq!(error.input(), "FF6B4A");
    }

    #[test]
    fn from_str_validates_digit_count_and_characters() {
        assert!("#FF".parse::<HexColor>().is_err());
        assert!("#FF00AA0011".parse::<HexColor>().is_err());
        assert!("#".parse::<HexColor>().is_err());
        assert_eq!(
            "#FF00A".parse::<HexColor>().unwrap_err(),
            ParseColorError::InvalidDigitCount {
                input: "#FF00A".into(),
                count: 5,
            }
        );
        assert_eq!(
            "#GGHHII".parse::<HexColor>().unwrap_err(),
            ParseColorError::InvalidHexDigit {
                input: "#GGHHII".into(),
                character: 'G',
            }
        );
    }

    #[test]
    fn error_display_names_the_input_and_reason() {
        assert_eq!("#GGHHII".parse::<HexColor>().unwrap_err().to_string(), "invalid color '#GGHHII': 'G' is not a hex digit");
        assert_eq!("FF".parse::<HexColor>().unwrap_err().to_string(), "invalid color 'FF': expected a leading '#'");
    }

    #[test]
    fn from_str_rejects_named_colors() {
        assert!("red".parse::<HexColor>().is_err());
        assert!("transparent".parse::<HexColor>().is_err());
    }

    #[test]
    fn display_formats_opaque_and_transparent() {
        assert_eq!(HexColor::from(Color::<u8>::new(255, 0, 255)).to_string(), "#ff00ff");
        assert_eq!(HexColor::from(Color::<u8>::new_rgba(255, 0, 255, 128)).to_string(), "#ff00ff80");
    }

    #[test]
    fn display_from_str_round_trip() {
        let hex: HexColor = "#ff6b4a80".parse().unwrap();
        assert_eq!(hex.to_string().parse::<HexColor>().unwrap(), hex);
    }

    #[test]
    fn conversions_cover_all_color_instantiations() {
        let hex: HexColor = "#FF6B4A".parse().unwrap();
        assert_eq!(Color::<u8>::from(hex), Color::<u8>::new(0xFF, 0x6B, 0x4A));
        assert_eq!(Color::<f64>::from(hex).r, 1.0);
        assert_eq!(Color::<f32>::from(hex).r, 1.0);
        assert_eq!(HexColor::from(Color::<f64>::WHITE).into_color(), Color::<u8>::WHITE);
    }

    #[test]
    fn serde_emits_the_hex_form() {
        let json = serde_json::to_string(&HexColor::from(Color::<u8>::new(0xFF, 0x6B, 0x4A))).unwrap();
        assert_eq!(json, "\"#ff6b4a\"");
    }

    #[test]
    fn deserialize_accepts_named_colors() {
        let hex: HexColor = serde_json::from_str("\"rebeccapurple\"").unwrap();
        assert_eq!(hex.into_color(), Color::<u8>::new(0x66, 0x33, 0x99));
        let transparent: HexColor = serde_json::from_str("\"TRANSPARENT\"").unwrap();
        assert_eq!(transparent.into_color(), Color::<u8>::new_rgba(0, 0, 0, 0));
    }

    #[test]
    fn deserialize_rejects_bare_hex_and_unknown_words() {
        assert!(serde_json::from_str::<HexColor>("\"ff6b4a\"").is_err());
        assert!(serde_json::from_str::<HexColor>("\"notacolor\"").is_err());
    }
}
