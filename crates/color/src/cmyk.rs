use crate::Color;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::fmt::Formatter;

/// A CMYK color with channels in whole percent (`0-100`).
///
/// `Cmyk` is a technical representation for print-oriented use:
/// [`PantoneNamedColor::to_cmyk`](crate::PantoneNamedColor::to_cmyk)
/// exposes published CMYK builds of Pantone spot colors, and the
/// `From` conversions perform the naive device conversion between
/// CMYK and [`Color<u8>`] (the standard `K = 1 - max(r, g, b)`
/// undercolor removal). Color-managed conversions with ICC profiles
/// stay in consumer crates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Cmyk {
    /// Cyan channel in percent (`0-100`).
    pub c: u8,
    /// Magenta channel in percent (`0-100`).
    pub m: u8,
    /// Yellow channel in percent (`0-100`).
    pub y: u8,
    /// Black (key) channel in percent (`0-100`).
    pub k: u8,
}

impl Cmyk {
    /// Creates a CMYK color, clamping each channel to `0-100`.
    pub const fn new(c: u8, m: u8, y: u8, k: u8) -> Self {
        Self {
            c: if c > 100 { 100 } else { c },
            m: if m > 100 { 100 } else { m },
            y: if y > 100 { 100 } else { y },
            k: if k > 100 { 100 } else { k },
        }
    }
}

/// Naive device conversion: `K = 1 - max(r, g, b)`, then
/// `C = (1 - r - K) / (1 - K)` (and likewise for `M` and `Y`).
impl From<Color<u8>> for Cmyk {
    fn from(color: Color<u8>) -> Self {
        let r = f64::from(color.r) / 255.0;
        let g = f64::from(color.g) / 255.0;
        let b = f64::from(color.b) / 255.0;
        let k = 1.0 - r.max(g).max(b);
        if k >= 1.0 {
            return Self::new(0, 0, 0, 100);
        }
        let divisor = 1.0 - k;
        let percent = |channel: f64| ((1.0 - channel - k) / divisor * 100.0).round() as u8;
        Self::new(percent(r), percent(g), percent(b), (k * 100.0).round() as u8)
    }
}

/// Naive device conversion: `r = 255 * (1 - C) * (1 - K)` (and
/// likewise for `g` and `b`). The alpha channel is opaque.
impl From<Cmyk> for Color<u8> {
    fn from(cmyk: Cmyk) -> Self {
        let key = f64::from(cmyk.k) / 100.0;
        let channel = |percent: u8| (255.0 * (1.0 - f64::from(percent) / 100.0) * (1.0 - key)).round() as u8;
        Self::new(channel(cmyk.c), channel(cmyk.m), channel(cmyk.y))
    }
}

impl Display for Cmyk {
    /// Formats the channels as `cmyk(c%, m%, y%, k%)`.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "cmyk({}%, {}%, {}%, {}%)", self.c, self.m, self.y, self.k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_clamps_channels_to_100() {
        assert_eq!(Cmyk::new(0, 0, 100, 200), Cmyk { c: 0, m: 0, y: 100, k: 100 });
    }

    #[test]
    fn color_to_cmyk_applies_undercolor_removal() {
        assert_eq!(Cmyk::from(Color::<u8>::new(255, 0, 0)), Cmyk { c: 0, m: 100, y: 100, k: 0 });
        assert_eq!(Cmyk::from(Color::<u8>::new(255, 255, 255)), Cmyk { c: 0, m: 0, y: 0, k: 0 });
        assert_eq!(Cmyk::from(Color::<u8>::new(0, 0, 0)), Cmyk { c: 0, m: 0, y: 0, k: 100 });
    }

    #[test]
    fn cmyk_to_color_applies_the_naive_formula() {
        assert_eq!(Color::<u8>::from(Cmyk::new(0, 100, 100, 0)), Color::<u8>::new(255, 0, 0));
        assert_eq!(Color::<u8>::from(Cmyk::new(0, 0, 0, 100)), Color::<u8>::new(0, 0, 0));
        assert_eq!(Color::<u8>::from(Cmyk::new(0, 0, 0, 0)), Color::<u8>::new(255, 255, 255));
    }

    #[test]
    fn display_formats_percent_channels() {
        assert_eq!(Cmyk::new(0, 90, 86, 0).to_string(), "cmyk(0%, 90%, 86%, 0%)");
    }

    #[test]
    fn serde_round_trip() {
        let cmyk = Cmyk::new(0, 90, 86, 0);
        assert_eq!(serde_json::from_str::<Cmyk>(&serde_json::to_string(&cmyk).unwrap()).unwrap(), cmyk);
    }
}
