use crate::Color;
use crate::Hue;
use crate::Lightness;
use crate::Saturation;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::fmt::Formatter;

/// An HSL color: hue in degrees (`0-360`), saturation and lightness
/// in `0.0-1.0`.
///
/// `Hsl` is the sibling of [`crate::Hsv`] with the symmetric
/// lightness axis: `l = 0.5` is the pure hue, `0` black and `1`
/// white. The `From` conversions perform the standard sRGB-space
/// HSL round trip; a saturation of `0` has no hue and round-trips as
/// `h = 0`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Hsl {
    /// Hue in degrees (`0-360`, wraps around).
    pub h: Hue,
    /// Saturation (`0.0` gray to `1.0` full chroma).
    pub s: Saturation,
    /// Lightness (`0.0` black, `0.5` pure hue, `1.0` white).
    pub l: Lightness,
}

impl Hsl {
    /// Creates an HSL color from its channels.
    pub fn new(h: impl Into<Hue>, s: impl Into<Saturation>, l: impl Into<Lightness>) -> Self {
        Self {
            h: h.into(),
            s: s.into(),
            l: l.into(),
        }
    }
}

impl Display for Hsl {
    /// `hsl(h, s%, l%)` - saturation and lightness in percent.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "hsl({:.1}, {:.1}%, {:.1}%)", self.h.as_f64(), self.s.as_f64() * 100.0, self.l.as_f64() * 100.0)
    }
}

/// The sRGB value of an HSL color (sector decomposition, alpha
/// opaque).
impl From<Hsl> for Color<f64> {
    fn from(hsl: Hsl) -> Self {
        let (h, s, l) = (hsl.h.as_f64(), hsl.s.as_f64(), hsl.l.as_f64());
        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h / 60.0).rem_euclid(2.0) - 1.0).abs());
        let m = l - c / 2.0;
        let (r, g, b) = match (h / 60.0) as u32 {
            0 => (c, x, 0.0),
            1 => (x, c, 0.0),
            2 => (0.0, c, x),
            3 => (0.0, x, c),
            4 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };
        Color::<f64>::new(r + m, g + m, b + m)
    }
}

/// The sRGB value of an HSL color, quantized to 8-bit channels.
impl From<Hsl> for Color<u8> {
    fn from(hsl: Hsl) -> Self {
        Color::<f64>::from(hsl).into()
    }
}

/// The sRGB value of an HSL color, as `f32` channels.
impl From<Hsl> for Color<f32> {
    fn from(hsl: Hsl) -> Self {
        Color::<f64>::from(hsl).into()
    }
}

/// The HSL decomposition of an sRGB color: `h` from max-channel
/// sector, `l = (max + min) / 2`, `s = delta / (1 - |2l - 1|)`.
/// Achromatic colors yield `h = 0, s = 0`.
impl From<Color<f64>> for Hsl {
    fn from(color: Color<f64>) -> Self {
        let max = color.r.max(color.g).max(color.b);
        let min = color.r.min(color.g).min(color.b);
        let delta = max - min;
        let l = (max + min) / 2.0;
        let h = if delta <= 0.0 {
            0.0
        } else if max == color.r {
            60.0 * ((color.g - color.b) / delta).rem_euclid(6.0)
        } else if max == color.g {
            60.0 * ((color.b - color.r) / delta + 2.0)
        } else {
            60.0 * ((color.r - color.g) / delta + 4.0)
        };
        let s = if delta <= 0.0 { 0.0 } else { delta / (1.0 - (2.0 * l - 1.0).abs()) };
        Self::new(h, s, l)
    }
}

/// The HSL decomposition of an 8-bit sRGB color.
impl From<Color<u8>> for Hsl {
    fn from(color: Color<u8>) -> Self {
        Color::<f64>::from(color).into()
    }
}

/// The HSL decomposition of an `f32` sRGB color.
impl From<Color<f32>> for Hsl {
    fn from(color: Color<f32>) -> Self {
        Color::<f64>::from(color).into()
    }
}

/// HSV to HSL: `l = v * (1 - s_v / 2)`, `s_l = (v - l) / min(l, 1 - l)`.
impl From<crate::Hsv> for Hsl {
    fn from(hsv: crate::Hsv) -> Self {
        let (s_v, v) = (hsv.s.as_f64(), hsv.v.as_f64());
        let l = v * (1.0 - s_v / 2.0);
        let s_l = if l > 0.0 && l < 1.0 { (v - l) / l.min(1.0 - l) } else { 0.0 };
        Self::new(hsv.h, s_l, l)
    }
}

/// HSL to HSV: `v = l + s_l * min(l, 1 - l)`, `s_v = 2 * (1 - l / v)`.
impl From<Hsl> for crate::Hsv {
    fn from(hsl: Hsl) -> Self {
        let (s_l, l) = (hsl.s.as_f64(), hsl.l.as_f64());
        let v = l + s_l * l.min(1.0 - l);
        let s_v = if v > 0.0 { 2.0 * (1.0 - l / v) } else { 0.0 };
        Self::new(hsl.h, s_v, v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primaries_have_full_saturation_at_half_lightness() {
        let red = Hsl::from(Color::<u8>::new(255, 0, 0));
        assert_eq!(red.h.as_f64(), 0.0);
        assert_eq!(red.s.as_f64(), 1.0);
        assert_eq!(red.l.as_f64(), 0.5);
        assert_eq!(Hsl::from(Color::<u8>::new(0, 0, 255)).h.as_f64(), 240.0);
    }

    #[test]
    fn achromatic_has_no_hue() {
        let hsl = Hsl::from(Color::<u8>::new(128, 128, 128));
        assert_eq!(hsl.s, Saturation::MIN);
        assert!((hsl.l.as_f64() - 128.0 / 255.0).abs() < 1e-9);
    }

    #[test]
    fn round_trip_keeps_the_color() {
        for color in [Color::<u8>::new(255, 0, 0), Color::<u8>::new(18, 130, 200), Color::<u8>::new(200, 40, 180)] {
            let back = Color::<u8>::from(Hsl::from(color));
            assert!((i32::from(back.r) - i32::from(color.r)).abs() <= 1);
            assert!((i32::from(back.g) - i32::from(color.g)).abs() <= 1);
            assert!((i32::from(back.b) - i32::from(color.b)).abs() <= 1);
        }
    }

    #[test]
    fn hsv_hsl_round_trip() {
        let hsv = crate::Hsv::new(210.0, 0.7, 0.6);
        let back = crate::Hsv::from(Hsl::from(hsv));
        assert!((back.h.as_f64() - hsv.h.as_f64()).abs() < 1e-9);
        assert!((back.s.as_f64() - hsv.s.as_f64()).abs() < 1e-9);
        assert!((back.v.as_f64() - hsv.v.as_f64()).abs() < 1e-9);
    }
}
