//! Wilhelm Ostwald's color order system (*Die Farbenfibel*, 1916)
//! - the first systematic mixture model.
//!
//! Ostwald's premise: every color is a recipe
//! `v * Vollfarbe + w * Weiss + s * Schwarz` with
//! `v + w + s = 1` - a full color ([`Hue`]), a whiteness and a
//! blackness part. [`Ostwald`] models exactly that triple.
//!
//! The modeled colors are a *documented approximation*: the atlas
//! chips were pigment mixtures, so `color()` computes the idealized
//! linear sRGB mixture `v * srgb(hue) + w * 255` - the geometry and
//! the recipe model are the real Ostwald ones.

use crate::Color;
use crate::Hsv;
use crate::Hue;
use serde::Deserialize;
use serde::Serialize;

/// An Ostwald color: a [`Hue`] plus the mixture weights `v`
/// (full color), `w` (white) and `s` (black) with `v + w + s = 1`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Ostwald {
    /// The hue of the full color.
    pub hue: Hue,
    /// The full-color fraction `v` (`0..=1`).
    pub full: f64,
    /// The white fraction `w` (`0..=1`).
    pub white: f64,
    /// The black fraction `s` (`0..=1`).
    pub black: f64,
}

impl Ostwald {
    /// Creates an Ostwald color; the weights are scaled down
    /// proportionally if `full + white + black > 1`.
    pub fn new(hue: Hue, full: f64, white: f64, black: f64) -> Self {
        let sum = full + white + black;
        if sum > 1.0 {
            Self {
                hue,
                full: full / sum,
                white: white / sum,
                black: black / sum,
            }
        } else {
            Self { hue, full, white, black }
        }
    }

    /// The pure full color `v = 1` of `hue`.
    pub fn full(hue: Hue) -> Self {
        Self {
            hue,
            full: 1.0,
            white: 0.0,
            black: 0.0,
        }
    }

    /// The unmixed full color at full saturation and value.
    pub fn full_color(hue: Hue) -> Color<u8> {
        Color::from(Hsv::new(hue, 1.0, 1.0))
    }

    /// The approximate sRGB value of the recipe: the affine mixture
    /// `v * full + w * white` (the black part is implicit -
    /// `v + w + s = 1` leaves `1 - v - w` dark).
    pub fn color(&self) -> Color<u8> {
        let pure = Self::full_color(self.hue);
        let mix = |channel: u8| (f64::from(channel) * self.full + 255.0 * self.white).round() as u8;
        Color::<u8>::new(mix(pure.r), mix(pure.g), mix(pure.b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_weights_sum_to_one() {
        let ostwald = Ostwald::new(Hue::new(30.0), 0.4, 0.4, 0.4);
        assert!((ostwald.full + ostwald.white + ostwald.black - 1.0).abs() < 1e-9);
    }

    #[test]
    fn the_full_color_is_the_saturated_hue() {
        assert_eq!(Ostwald::full(Hue::new(0.0)).color(), Color::<u8>::new(255, 0, 0));
    }

    #[test]
    fn pure_white_and_pure_black_are_the_endpoints() {
        assert_eq!(Ostwald::new(Hue::new(210.0), 0.0, 1.0, 0.0).color(), Color::<u8>::new(255, 255, 255));
        assert_eq!(Ostwald::new(Hue::new(210.0), 0.0, 0.0, 1.0).color(), Color::<u8>::new(0, 0, 0));
    }

    #[test]
    fn the_mixture_is_affine() {
        // v = 0.5, w = 0.25, s = 0.25 of pure red -> (127 + 64, 64, 64).
        let ostwald = Ostwald::new(Hue::new(0.0), 0.5, 0.25, 0.25);
        assert_eq!(ostwald.color(), Color::<u8>::new(191, 64, 64));
    }
}
