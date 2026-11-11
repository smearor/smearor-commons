//! `gdk::RGBA` conversions, gated behind the `gtk4` feature.
//!
//! `gdk::RGBA` stores `f32` channels, so `Color<f32>` converts
//! losslessly; `Color<f64>`/`Color<u8>` convert via `Color<f32>`.

use crate::Color;
use gtk4::gdk;

impl From<Color<f32>> for gdk::RGBA {
    fn from(color: Color<f32>) -> Self {
        Self::new(color.r, color.g, color.b, color.a)
    }
}

impl From<&Color<f32>> for gdk::RGBA {
    fn from(color: &Color<f32>) -> Self {
        Self::new(color.r, color.g, color.b, color.a)
    }
}

impl From<gdk::RGBA> for Color<f32> {
    fn from(color: gdk::RGBA) -> Self {
        Self::new_rgba(color.red(), color.green(), color.blue(), color.alpha())
    }
}

impl From<Color<f64>> for gdk::RGBA {
    fn from(color: Color<f64>) -> Self {
        Self::from(Color::<f32>::from(color))
    }
}

impl From<gdk::RGBA> for Color<f64> {
    fn from(color: gdk::RGBA) -> Self {
        Self::from(Color::<f32>::from(color))
    }
}

impl From<Color<u8>> for gdk::RGBA {
    fn from(color: Color<u8>) -> Self {
        Self::from(Color::<f32>::from(color))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f32_round_trip() {
        let color = Color::<f32>::new_rgba(0.25, 0.5, 0.75, 0.8);
        let rgba = gdk::RGBA::from(color);
        assert_eq!(rgba.red(), 0.25);
        assert_eq!(rgba.alpha(), 0.8);
        assert_eq!(Color::<f32>::from(rgba), color);
    }

    #[test]
    fn f64_and_u8_convert() {
        let rgba = gdk::RGBA::from(Color::<u8>::new_rgba(255, 0, 0, 255));
        assert_eq!(rgba.red(), 1.0);
        let color = Color::<f64>::from(gdk::RGBA::new(0.0, 1.0, 0.0, 1.0));
        assert_eq!(color.g, 1.0);
    }
}
