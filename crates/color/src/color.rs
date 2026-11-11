use crate::Luminance;
use crate::LuminanceWeights;
use crate::ParseColorError;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;

/// Generic RGBA color with four components of type `T`.
///
/// Use `Color<f64>` or `Color<f32>` for normalized channel values in
/// `[0.0, 1.0]` (CSS/GTK/cairo operations) and `Color<u8>` for
/// pixel-native channel values in `[0, 255]` (software rendering).
///
/// Conversions between the instantiations are provided via `From`/`Into`.
/// Parsing and formatting use `FromStr`/`Display` and accept all common
/// hex forms: `#RGB`, `#RGBA`, `#RRGGBB` and `#RRGGBBAA` (with or
/// without the leading `#`).
///
/// Semantic color types (`FigureColor`, `LineColor`, `PawnColor`, ...)
/// stay in consumer crates as thin newtype wrappers around `Color<T>`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color<T> {
    /// Red component.
    pub r: T,
    /// Green component.
    pub g: T,
    /// Blue component.
    pub b: T,
    /// Alpha component.
    pub a: T,
}

// ---------------------------------------------------------------------------
// Hex parsing / formatting (shared helpers)
// ---------------------------------------------------------------------------

/// Parses a hex color string into `[r, g, b, a]` byte components.
///
/// Accepts `#RGB`, `#RGBA`, `#RRGGBB` and `#RRGGBBAA` (with or without
/// the leading `#`); shorthand digits are doubled (`#F0A` = `#FF00AA`).
fn parse_hex_components(hex: &str) -> Result<[u8; 4], ParseColorError> {
    let hex = hex.strip_prefix('#').unwrap_or(hex);
    let invalid = |input: &str| ParseColorError::new(input, "contains non-hex characters");
    match hex.len() {
        3 => Ok([
            u8::from_str_radix(&hex[0..1].repeat(2), 16).map_err(|_| invalid(hex))?,
            u8::from_str_radix(&hex[1..2].repeat(2), 16).map_err(|_| invalid(hex))?,
            u8::from_str_radix(&hex[2..3].repeat(2), 16).map_err(|_| invalid(hex))?,
            255,
        ]),
        4 => Ok([
            u8::from_str_radix(&hex[0..1].repeat(2), 16).map_err(|_| invalid(hex))?,
            u8::from_str_radix(&hex[1..2].repeat(2), 16).map_err(|_| invalid(hex))?,
            u8::from_str_radix(&hex[2..3].repeat(2), 16).map_err(|_| invalid(hex))?,
            u8::from_str_radix(&hex[3..4].repeat(2), 16).map_err(|_| invalid(hex))?,
        ]),
        6 => Ok([
            u8::from_str_radix(&hex[0..2], 16).map_err(|_| invalid(hex))?,
            u8::from_str_radix(&hex[2..4], 16).map_err(|_| invalid(hex))?,
            u8::from_str_radix(&hex[4..6], 16).map_err(|_| invalid(hex))?,
            255,
        ]),
        8 => Ok([
            u8::from_str_radix(&hex[0..2], 16).map_err(|_| invalid(hex))?,
            u8::from_str_radix(&hex[2..4], 16).map_err(|_| invalid(hex))?,
            u8::from_str_radix(&hex[4..6], 16).map_err(|_| invalid(hex))?,
            u8::from_str_radix(&hex[6..8], 16).map_err(|_| invalid(hex))?,
        ]),
        _ => Err(ParseColorError::new(hex, "expected 3, 4, 6, or 8 hex digits")),
    }
}

/// Formats byte components as `#RRGGBB` when opaque, `#RRGGBBAA` otherwise.
fn format_hex_components(r: u8, g: u8, b: u8, a: u8) -> String {
    if a == 255 {
        format!("#{r:02X}{g:02X}{b:02X}")
    } else {
        format!("#{r:02X}{g:02X}{b:02X}{a:02X}")
    }
}

// ---------------------------------------------------------------------------
// Color<f64> — normalized channels for CSS/GTK operations
// ---------------------------------------------------------------------------

impl Color<f64> {
    /// Creates a new opaque color from RGB components (alpha = 1.0).
    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// Creates a new color from RGBA components.
    pub const fn new_rgba(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a }
    }

    /// Black.
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0);

    /// White.
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0);

    /// The perceived luminance with custom `weights` (0.0-1.0).
    pub fn luminance(&self, weights: LuminanceWeights) -> Luminance {
        Luminance::new(self.r * weights.r + self.g * weights.g + self.b * weights.b)
    }

    /// The perceived luminance with ITU-R BT.709 weights (0.0-1.0).
    pub fn luminance_709(&self) -> Luminance {
        self.luminance(LuminanceWeights::REC_709)
    }

    /// The perceived luminance with ITU-R BT.601 weights (0.0-1.0).
    pub fn luminance_601(&self) -> Luminance {
        self.luminance(LuminanceWeights::REC_601)
    }

    /// Linear blend toward `other` by `t` (0 = self, 1 = other).
    /// All four channels are interpolated; `t` is clamped to `[0, 1]`.
    pub fn lerp(&self, other: &Self, t: f64) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }

    /// Lightens the color by blending each channel toward white by
    /// `factor` (0 = unchanged, 1 = white). Alpha is preserved.
    pub fn lighten(&self, factor: f64) -> Self {
        let factor = factor.clamp(0.0, 1.0);
        Self {
            r: self.r + (1.0 - self.r) * factor,
            g: self.g + (1.0 - self.g) * factor,
            b: self.b + (1.0 - self.b) * factor,
            a: self.a,
        }
    }

    /// The color lifted to at least `min` luminance (BT.709) by
    /// blending toward white - dark colors stay readable on dark
    /// backgrounds. Alpha is preserved.
    pub fn with_min_luminance(&self, min: impl Into<Luminance>) -> Self {
        let min = min.into();
        let luminance = self.luminance_709();
        if luminance >= min {
            return *self;
        }
        self.lighten((min - luminance) / (Luminance::MAX - luminance))
    }

    /// Clamps all four channels to `[0.0, 1.0]`.
    pub fn clamp(&self) -> Self {
        Self {
            r: self.r.clamp(0.0, 1.0),
            g: self.g.clamp(0.0, 1.0),
            b: self.b.clamp(0.0, 1.0),
            a: self.a.clamp(0.0, 1.0),
        }
    }
}

// ---------------------------------------------------------------------------
// Color<f32> — normalized channels for GTK/GDK operations
// ---------------------------------------------------------------------------

impl Color<f32> {
    /// Creates a new opaque color from RGB components (alpha = 1.0).
    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// Creates a new color from RGBA components.
    pub const fn new_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Black.
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0);

    /// White.
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0);

    /// The perceived luminance with custom `weights` (0.0-1.0).
    pub fn luminance(&self, weights: LuminanceWeights) -> Luminance {
        Luminance::new(f64::from(self.r) * weights.r + f64::from(self.g) * weights.g + f64::from(self.b) * weights.b)
    }

    /// The perceived luminance with ITU-R BT.709 weights (0.0-1.0).
    pub fn luminance_709(&self) -> Luminance {
        self.luminance(LuminanceWeights::REC_709)
    }

    /// The perceived luminance with ITU-R BT.601 weights (0.0-1.0).
    pub fn luminance_601(&self) -> Luminance {
        self.luminance(LuminanceWeights::REC_601)
    }

    /// Linear blend toward `other` by `t` (0 = self, 1 = other).
    /// All four channels are interpolated; `t` is clamped to `[0, 1]`.
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }

    /// Lightens the color by blending each channel toward white by
    /// `factor` (0 = unchanged, 1 = white). Alpha is preserved.
    pub fn lighten(&self, factor: f32) -> Self {
        let factor = factor.clamp(0.0, 1.0);
        Self {
            r: self.r + (1.0 - self.r) * factor,
            g: self.g + (1.0 - self.g) * factor,
            b: self.b + (1.0 - self.b) * factor,
            a: self.a,
        }
    }

    /// The color lifted to at least `min` luminance (BT.709) by
    /// blending toward white - dark colors stay readable on dark
    /// backgrounds. Alpha is preserved.
    pub fn with_min_luminance(&self, min: impl Into<Luminance>) -> Self {
        let min = min.into();
        let luminance = self.luminance_709();
        if luminance >= min {
            return *self;
        }
        self.lighten(((min - luminance) / (Luminance::MAX - luminance)) as f32)
    }

    /// Clamps all four channels to `[0.0, 1.0]`.
    pub fn clamp(&self) -> Self {
        Self {
            r: self.r.clamp(0.0, 1.0),
            g: self.g.clamp(0.0, 1.0),
            b: self.b.clamp(0.0, 1.0),
            a: self.a.clamp(0.0, 1.0),
        }
    }
}

// ---------------------------------------------------------------------------
// Color<u8> — pixel-native color for software rendering
// ---------------------------------------------------------------------------

impl Color<u8> {
    /// Creates a new opaque color from RGB components (alpha = 255).
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Creates a new color from RGBA components.
    pub const fn new_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Black.
    pub const BLACK: Self = Self::new(0, 0, 0);

    /// White.
    pub const WHITE: Self = Self::new(255, 255, 255);

    /// Creates an opaque color from a packed `0xRRGGBB` value -
    /// `const`-usable, e.g. in `static_palette!` definitions.
    pub const fn from_rgb_u32(value: u32) -> Self {
        Self {
            r: ((value >> 16) & 0xFF) as u8,
            g: ((value >> 8) & 0xFF) as u8,
            b: (value & 0xFF) as u8,
            a: 255,
        }
    }

    /// Creates a color from a packed `0xRRGGBBAA` value -
    /// `const`-usable, e.g. in `static_palette!` definitions.
    pub const fn from_rgba_u32(value: u32) -> Self {
        Self {
            r: ((value >> 24) & 0xFF) as u8,
            g: ((value >> 16) & 0xFF) as u8,
            b: ((value >> 8) & 0xFF) as u8,
            a: (value & 0xFF) as u8,
        }
    }
}

impl Default for Color<f64> {
    /// Fully transparent black.
    fn default() -> Self {
        Self::new_rgba(0.0, 0.0, 0.0, 0.0)
    }
}

impl Default for Color<f32> {
    /// Fully transparent black.
    fn default() -> Self {
        Self::new_rgba(0.0, 0.0, 0.0, 0.0)
    }
}

impl Default for Color<u8> {
    /// Fully transparent black.
    fn default() -> Self {
        Self::new_rgba(0, 0, 0, 0)
    }
}

// ---------------------------------------------------------------------------
// Parsing / formatting trait implementations
// ---------------------------------------------------------------------------

impl FromStr for Color<f64> {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Color::<u8>::from(parse_hex_components(s)?).into())
    }
}

impl FromStr for Color<f32> {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Color::<u8>::from(parse_hex_components(s)?).into())
    }
}

impl FromStr for Color<u8> {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(parse_hex_components(s)?))
    }
}

impl Display for Color<f64> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = Color::<u8>::from(*self);
        f.write_str(&format_hex_components(c.r, c.g, c.b, c.a))
    }
}

impl Display for Color<f32> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = Color::<u8>::from(*self);
        f.write_str(&format_hex_components(c.r, c.g, c.b, c.a))
    }
}

impl Display for Color<u8> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format_hex_components(self.r, self.g, self.b, self.a))
    }
}

// ---------------------------------------------------------------------------
// Cross-type conversions
// ---------------------------------------------------------------------------

impl From<[u8; 4]> for Color<u8> {
    fn from(c: [u8; 4]) -> Self {
        Self {
            r: c[0],
            g: c[1],
            b: c[2],
            a: c[3],
        }
    }
}

impl From<Color<u8>> for [u8; 4] {
    fn from(c: Color<u8>) -> Self {
        [c.r, c.g, c.b, c.a]
    }
}

impl From<Color<u8>> for Color<f64> {
    fn from(c: Color<u8>) -> Self {
        Self {
            r: c.r as f64 / 255.0,
            g: c.g as f64 / 255.0,
            b: c.b as f64 / 255.0,
            a: c.a as f64 / 255.0,
        }
    }
}

impl From<Color<f64>> for Color<u8> {
    fn from(c: Color<f64>) -> Self {
        Self {
            r: (c.r * 255.0).round() as u8,
            g: (c.g * 255.0).round() as u8,
            b: (c.b * 255.0).round() as u8,
            a: (c.a * 255.0).round() as u8,
        }
    }
}

impl From<Color<u8>> for Color<f32> {
    fn from(c: Color<u8>) -> Self {
        Self {
            r: c.r as f32 / 255.0,
            g: c.g as f32 / 255.0,
            b: c.b as f32 / 255.0,
            a: c.a as f32 / 255.0,
        }
    }
}

impl From<Color<f32>> for Color<u8> {
    fn from(c: Color<f32>) -> Self {
        Self {
            r: (c.r * 255.0).round() as u8,
            g: (c.g * 255.0).round() as u8,
            b: (c.b * 255.0).round() as u8,
            a: (c.a * 255.0).round() as u8,
        }
    }
}

impl From<Color<f32>> for Color<f64> {
    fn from(c: Color<f32>) -> Self {
        Self {
            r: c.r as f64,
            g: c.g as f64,
            b: c.b as f64,
            a: c.a as f64,
        }
    }
}

impl From<Color<f64>> for Color<f32> {
    fn from(c: Color<f64>) -> Self {
        Self {
            r: c.r as f32,
            g: c.g as f32,
            b: c.b as f32,
            a: c.a as f32,
        }
    }
}

// ---------------------------------------------------------------------------
// Serde — canonical wire format is the hex string `"#rrggbb(aa)"`
// ---------------------------------------------------------------------------

impl Serialize for Color<f64> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl Serialize for Color<f32> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl Serialize for Color<u8> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Color<f64> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer)?.parse().map_err(serde::de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for Color<f32> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer)?.parse().map_err(serde::de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for Color<u8> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer)?.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f64_new_sets_alpha_to_one() {
        assert_eq!(Color::<f64>::new(0.5, 0.25, 0.75).a, 1.0);
    }

    #[test]
    fn f64_new_rgba_preserves_alpha() {
        assert_eq!(Color::<f64>::new_rgba(0.1, 0.2, 0.3, 0.4).a, 0.4);
    }

    #[test]
    fn f64_black_and_white_constants() {
        assert_eq!(Color::<f64>::BLACK, Color::<f64>::new(0.0, 0.0, 0.0));
        assert_eq!(Color::<f64>::WHITE, Color::<f64>::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn f64_to_u8_round_trip() {
        let color = Color::<f64>::new_rgba(1.0, 0.0, 1.0, 0.0);
        let bytes: Color<u8> = color.into();
        assert_eq!(bytes, Color::<u8>::new_rgba(255, 0, 255, 0));
        let restored: Color<f64> = bytes.into();
        assert_eq!(restored, color);
    }

    #[test]
    fn f32_to_u8_conversion() {
        let c: Color<u8> = Color::<f32>::new(1.0, 0.0, 1.0).into();
        assert_eq!(c, Color::<u8>::new_rgba(255, 0, 255, 255));
    }

    #[test]
    fn u8_from_and_to_array() {
        let color: Color<u8> = [128, 64, 32, 255].into();
        assert_eq!(<[u8; 4]>::from(color), [128, 64, 32, 255]);
    }

    #[test]
    fn u8_to_f64_conversion() {
        let f: Color<f64> = Color::<u8>::new_rgba(128, 64, 32, 255).into();
        assert_eq!(f.r, 128.0 / 255.0);
        assert_eq!(f.a, 1.0);
    }

    #[test]
    fn from_str_rrggbb() {
        let color: Color<f64> = "#FF6B4A".parse().unwrap();
        assert_eq!(color, Color::<f64>::new(1.0, 0x6B as f64 / 255.0, 0x4A as f64 / 255.0));
    }

    #[test]
    fn from_str_rrggbbaa() {
        let color: Color<f64> = "#FF6B4A80".parse().unwrap();
        assert_eq!(color.a, 0x80 as f64 / 255.0);
    }

    #[test]
    fn from_str_shorthand_rgb() {
        let color: Color<f64> = "#F0A".parse().unwrap();
        assert_eq!(color, Color::<f64>::new(1.0, 0.0, 0xAA as f64 / 255.0));
    }

    #[test]
    fn from_str_shorthand_rgba() {
        let color: Color<f64> = "#F0AF".parse().unwrap();
        assert_eq!(color.a, 1.0);
    }

    #[test]
    fn from_str_without_hash() {
        let with: Color<f64> = "#FF6B4A".parse().unwrap();
        let without: Color<f64> = "FF6B4A".parse().unwrap();
        assert_eq!(with, without);
    }

    #[test]
    fn from_str_f32() {
        let color: Color<f32> = "#FF0000".parse().unwrap();
        assert_eq!(color, Color::<f32>::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn from_str_u8() {
        let color: Color<u8> = "#FF6B4A80".parse().unwrap();
        assert_eq!(color, Color::<u8>::new_rgba(0xFF, 0x6B, 0x4A, 0x80));
    }

    #[test]
    fn from_str_invalid() {
        assert!(Color::<f64>::from_str("#FF").is_err());
        assert!(Color::<f64>::from_str("#FFGGFF").is_err());
        assert!(Color::<f64>::from_str("invalid").is_err());
    }

    #[test]
    fn display_opaque() {
        assert_eq!(Color::<f64>::new(1.0, 0.0, 1.0).to_string(), "#FF00FF");
    }

    #[test]
    fn display_with_alpha() {
        assert_eq!(Color::<f64>::new_rgba(1.0, 0.0, 1.0, 0.5).to_string(), "#FF00FF80");
    }

    #[test]
    fn display_u8() {
        assert_eq!(Color::<u8>::new_rgba(255, 0, 255, 128).to_string(), "#FF00FF80");
    }

    #[test]
    fn round_trip_parse_display() {
        let original = Color::<f64>::new_rgba(0x12 as f64 / 255.0, 0x34 as f64 / 255.0, 0x56 as f64 / 255.0, 0x78 as f64 / 255.0);
        let parsed: Color<f64> = original.to_string().parse().unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn luminance_709_black_and_white() {
        assert_eq!(Color::<f64>::BLACK.luminance_709(), Luminance::new(0.0));
        assert!((Color::<f64>::WHITE.luminance_709().as_f64() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn luminance_601_weights() {
        let color = Color::<f64>::new(1.0, 0.0, 0.0);
        assert!((color.luminance_601().as_f64() - 0.299).abs() < 1e-9);
        assert!((Color::<f64>::WHITE.luminance_601().as_f64() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn lerp_midpoint() {
        let half = Color::<f64>::BLACK.lerp(&Color::<f64>::WHITE, 0.5);
        assert!((half.r - 0.5).abs() < 1e-9);
        assert!((half.a - 1.0).abs() < 1e-9);
    }

    #[test]
    fn lerp_clamps_t() {
        assert_eq!(Color::<f64>::BLACK.lerp(&Color::<f64>::WHITE, 2.0), Color::<f64>::WHITE);
        assert_eq!(Color::<f64>::BLACK.lerp(&Color::<f64>::WHITE, -1.0), Color::<f64>::BLACK);
    }

    #[test]
    fn lighten_preserves_alpha() {
        let color = Color::<f64>::new_rgba(0.0, 0.0, 0.0, 0.5).lighten(0.5);
        assert!((color.r - 0.5).abs() < 1e-9);
        assert_eq!(color.a, 0.5);
    }

    #[test]
    fn with_min_luminance_lifts_dark_color() {
        let lifted = Color::<f64>::BLACK.with_min_luminance(0.5);
        assert!(lifted.luminance_709() >= Luminance::new(0.5));
    }

    #[test]
    fn with_min_luminance_keeps_bright_color() {
        let bright = Color::<f64>::new(0.9, 0.9, 0.9);
        assert_eq!(bright.with_min_luminance(0.5), bright);
    }

    #[test]
    fn f32_lerp_and_luminance() {
        let half = Color::<f32>::BLACK.lerp(&Color::<f32>::WHITE, 0.5);
        assert!((half.r - 0.5).abs() < 1e-6);
        assert!((Color::<f32>::WHITE.luminance_709().as_f64() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn serde_hex_round_trip() {
        let color = Color::<f64>::new(1.0, 0.0, 0.0);
        let json = serde_json::to_string(&color).unwrap();
        assert_eq!(json, "\"#FF0000\"");
        let parsed: Color<f64> = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, color);
    }

    #[test]
    fn from_rgb_u32_unpacks_channels() {
        assert_eq!(Color::<u8>::from_rgb_u32(0xEDD49E), Color::<u8>::new(0xED, 0xD4, 0x9E));
        assert_eq!(Color::<u8>::from_rgb_u32(0x000000), Color::<u8>::BLACK);
    }

    #[test]
    fn from_rgba_u32_unpacks_channels() {
        assert_eq!(Color::<u8>::from_rgba_u32(0xEDD49E80), Color::<u8>::new_rgba(0xED, 0xD4, 0x9E, 0x80));
    }

    #[test]
    fn from_rgb_u32_is_const_usable() {
        const COLOR: Color<u8> = Color::<u8>::from_rgb_u32(0xEDD49E);
        assert_eq!(COLOR, Color::<u8>::new(0xED, 0xD4, 0x9E));
    }
}
