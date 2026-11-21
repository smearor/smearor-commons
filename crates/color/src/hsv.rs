use crate::Color;
use crate::Hue;
use crate::Saturation;
use crate::Value;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::fmt::Formatter;

/// An HSV color: hue in degrees (`0-360`), saturation and value in
/// `0.0-1.0`.
///
/// `Hsv` is the technical representation used by color-wheel
/// interfaces: hue walks the wheel, saturation runs from gray to
/// full chroma, value from black to full brightness. The `From`
/// conversions perform the standard sRGB-space HSV round trip; a
/// saturation of `0` has no hue and round-trips as `h = 0`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Hsv {
    /// Hue in degrees (`0-360`, wraps around).
    pub h: Hue,
    /// Saturation (`0.0` gray to `1.0` full chroma).
    pub s: Saturation,
    /// Value (`0.0` black to `1.0` full brightness).
    pub v: Value,
}

impl Hsv {
    /// Creates an HSV color from its channels.
    pub fn new(h: impl Into<Hue>, s: impl Into<Saturation>, v: impl Into<Value>) -> Self {
        Self {
            h: h.into(),
            s: s.into(),
            v: v.into(),
        }
    }
}

impl Display for Hsv {
    /// `hsv(h, s%, v%)` - saturation and value in percent.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "hsv({:.1}, {:.1}%, {:.1}%)", self.h.as_f64(), self.s.as_f64() * 100.0, self.v.as_f64() * 100.0)
    }
}

/// The sRGB value of an HSV color (sector decomposition, alpha
/// opaque).
impl From<Hsv> for Color<f64> {
    fn from(hsv: Hsv) -> Self {
        let (h, s, v) = (hsv.h.as_f64() / 60.0, hsv.s.as_f64(), hsv.v.as_f64());
        let c = v * s;
        let x = c * (1.0 - (h.rem_euclid(2.0) - 1.0).abs());
        let m = v - c;
        let (r, g, b) = match h as u32 {
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

/// The sRGB value of an HSV color, quantized to 8-bit channels.
impl From<Hsv> for Color<u8> {
    fn from(hsv: Hsv) -> Self {
        Color::<f64>::from(hsv).into()
    }
}

/// The sRGB value of an HSV color, as `f32` channels.
impl From<Hsv> for Color<f32> {
    fn from(hsv: Hsv) -> Self {
        Color::<f64>::from(hsv).into()
    }
}

/// The HSV decomposition of an sRGB color: `h` from max-channel
/// sector, `s = delta / max`, `v = max`. Achromatic colors yield
/// `h = 0, s = 0`.
impl From<Color<f64>> for Hsv {
    fn from(color: Color<f64>) -> Self {
        let max = color.r.max(color.g).max(color.b);
        let min = color.r.min(color.g).min(color.b);
        let delta = max - min;
        let h = if delta <= 0.0 {
            0.0
        } else if max == color.r {
            60.0 * ((color.g - color.b) / delta).rem_euclid(6.0)
        } else if max == color.g {
            60.0 * ((color.b - color.r) / delta + 2.0)
        } else {
            60.0 * ((color.r - color.g) / delta + 4.0)
        };
        Self::new(h, if max > 0.0 { delta / max } else { 0.0 }, max)
    }
}

/// The HSV decomposition of an 8-bit sRGB color.
impl From<Color<u8>> for Hsv {
    fn from(color: Color<u8>) -> Self {
        Color::<f64>::from(color).into()
    }
}

/// The HSV decomposition of an `f32` sRGB color.
impl From<Color<f32>> for Hsv {
    fn from(color: Color<f32>) -> Self {
        Color::<f64>::from(color).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primaries_map_to_their_sectors() {
        assert_eq!(Hsv::from(Color::<u8>::new(255, 0, 0)).h.as_f64(), 0.0);
        assert_eq!(Hsv::from(Color::<u8>::new(0, 255, 0)).h.as_f64(), 120.0);
        assert_eq!(Hsv::from(Color::<u8>::new(0, 0, 255)).h.as_f64(), 240.0);
        assert_eq!(Hsv::from(Color::<u8>::new(255, 255, 0)).h.as_f64(), 60.0);
    }

    #[test]
    fn achromatic_has_no_hue() {
        let hsv = Hsv::from(Color::<u8>::new(128, 128, 128));
        assert_eq!(hsv.h, Hue::ZERO);
        assert_eq!(hsv.s, Saturation::MIN);
        assert!((hsv.v.as_f64() - 128.0 / 255.0).abs() < 1e-9);
    }

    #[test]
    fn hue_wraps_and_channels_clamp() {
        assert_eq!(Hsv::new(370.0, 0.5, 0.5).h.as_f64(), 10.0);
        assert_eq!(Hsv::new(-10.0, 0.5, 0.5).h.as_f64(), 350.0);
        assert_eq!(
            Hsv::new(0.0, 2.0, -1.0),
            Hsv {
                h: Hue::ZERO,
                s: Saturation::MAX,
                v: Value::MIN,
            }
        );
    }

    #[test]
    fn round_trip_keeps_the_color() {
        for color in [Color::<u8>::new(255, 0, 0), Color::<u8>::new(18, 130, 200), Color::<u8>::new(200, 40, 180)] {
            let back = Color::<u8>::from(Hsv::from(color));
            assert!((i32::from(back.r) - i32::from(color.r)).abs() <= 1);
            assert!((i32::from(back.g) - i32::from(color.g)).abs() <= 1);
            assert!((i32::from(back.b) - i32::from(color.b)).abs() <= 1);
        }
    }
}
