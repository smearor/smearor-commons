//! The Natural Colour System (NCS) - the Swedish appearance-based
//! color notation (SIS, 1979; ISO/CIE standard).
//!
//! Every NCS color is a resemblance recipe against the six
//! elementary percepts - white, black and the four chromatic hues
//! yellow (`Y`), red (`R`), green (`G`), blue (`B`):
//!
//! - [`NcsHue`] is the position on the hue circle (`0` = `Y`,
//!   `100` = `R`, `200` = `B`, `300` = `G`), where the two digits
//!   after a base letter give the resemblance toward the next
//!   elementary hue: `Y90R` is yellow that resembles red to 90%.
//! - [`Ncs`] is the full `S sccc-HUE` specification: blackness `s`,
//!   chromaticness `c` and the implicit whiteness `w` with
//!   `s + c + w = 100`.
//!
//! The modeled colors are a *documented approximation*: the official
//! NCS atlas values are proprietary, so hue resemblance is mapped
//! onto CIELAB hue angles between the four elementary anchors and
//! blackness/chromaticness onto `L*`/`C*` ranges. The geometry and
//! the notation are the real NCS ones.

use crate::Color;
use crate::Hue;
use crate::Lab;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::fmt::Formatter;

/// Approximate CIELAB hue angles of the elementary hues in circle
/// order `Y`, `R`, `B`, `G` in degrees - a documented
/// approximation, see the module documentation.
const ELEMENTARY_ANGLES: [f64; 4] = [90.0, 25.0, 255.0, 155.0];

/// A position on the NCS hue circle (`0` = `Y`, `100` = `R`,
/// `200` = `B`, `300` = `G`).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(transparent)]
pub struct NcsHue(f64);

impl NcsHue {
    /// Creates a hue position, wrapped into `0..400`.
    pub fn new(position: f64) -> Self {
        Self(position.rem_euclid(400.0))
    }

    /// The numeric circle position (`0..400`).
    pub const fn as_f64(self) -> f64 {
        self.0
    }

    /// The approximate CIELAB hue angle, interpolated along the
    /// shortest arc between the elementary anchors.
    pub fn lab_hue(&self) -> Hue {
        let index = (self.0 / 100.0).floor() as usize % 4;
        let t = self.0.rem_euclid(100.0) / 100.0;
        let a0 = ELEMENTARY_ANGLES[index];
        let a1 = ELEMENTARY_ANGLES[(index + 1) % 4];
        let span = (a1 - a0 + 180.0).rem_euclid(360.0) - 180.0;
        Hue::new(a0 + span * t)
    }

    /// The approximate sRGB value of an NCS cell of this hue: the
    /// elementary anchor hue at `c%` chroma, lightened by whiteness
    /// and darkened by blackness. `blackness` and `chromaticness`
    /// are percents (`0..=100`).
    pub fn color(&self, blackness: f64, chromaticness: f64) -> Color<u8> {
        let lightness = 95.0 - 0.7 * blackness - 0.3 * chromaticness;
        let (sin, cos) = self.lab_hue().as_f64().to_radians().sin_cos();
        let chroma = 0.9 * chromaticness;
        Color::from(Lab::new(lightness, chroma * cos, chroma * sin))
    }
}

impl Display for NcsHue {
    /// The hue notation, e.g. `"Y"`, `"Y90R"`, `"B10Y"`.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let sector = (self.0 / 100.0).floor() as usize % 4;
        let pct = self.0.rem_euclid(100.0).round() as i32;
        let (from, to) = (["Y", "R", "B", "G"][sector], ["R", "B", "G", "Y"][sector]);
        if pct == 0 { write!(f, "{from}") } else { write!(f, "{from}{pct}{to}") }
    }
}

impl From<f64> for NcsHue {
    /// Wraps a circle position into `0..400`.
    fn from(position: f64) -> Self {
        Self::new(position)
    }
}

impl From<NcsHue> for f64 {
    /// The numeric circle position (`0..400`).
    fn from(hue: NcsHue) -> Self {
        hue.0
    }
}

/// A complete NCS color specification `S sccc-HUE`: a hue plus
/// blackness `s` and chromaticness `c` percents, with the implicit
/// whiteness `w = 100 - s - c`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Ncs {
    /// The position on the NCS hue circle.
    pub hue: NcsHue,
    /// The blackness `s` in percent (`0..=100`).
    pub blackness: f64,
    /// The chromaticness `c` in percent (`0..=100`,
    /// `s + c <= 100` for a valid NCS color).
    pub chromaticness: f64,
}

impl Ncs {
    /// Creates an NCS color; `blackness + chromaticness` is clamped
    /// to `100` by scaling both down proportionally.
    pub fn new(hue: NcsHue, blackness: f64, chromaticness: f64) -> Self {
        let sum = blackness + chromaticness;
        if sum > 100.0 {
            Self {
                hue,
                blackness: blackness * 100.0 / sum,
                chromaticness: chromaticness * 100.0 / sum,
            }
        } else {
            Self { hue, blackness, chromaticness }
        }
    }

    /// The whiteness `w = 100 - s - c` in percent.
    pub fn whiteness(&self) -> f64 {
        100.0 - self.blackness - self.chromaticness
    }

    /// The approximate sRGB value (see [`NcsHue::color`] for the
    /// documented approximation).
    pub fn color(&self) -> Color<u8> {
        self.hue.color(self.blackness, self.chromaticness)
    }
}

impl Display for Ncs {
    /// The standard notation, e.g. `"S 2030-Y90R"`.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "S {:02}{:02}-{}", self.blackness.round() as i32, self.chromaticness.round() as i32, self.hue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_position_wraps_around_the_circle() {
        assert_eq!(NcsHue::new(450.0).as_f64(), 50.0);
        assert_eq!(NcsHue::new(-10.0).as_f64(), 390.0);
    }

    #[test]
    fn the_elementary_positions_have_their_letter_notation() {
        assert_eq!(NcsHue::new(0.0).to_string(), "Y");
        assert_eq!(NcsHue::new(100.0).to_string(), "R");
        assert_eq!(NcsHue::new(200.0).to_string(), "B");
        assert_eq!(NcsHue::new(300.0).to_string(), "G");
    }

    #[test]
    fn the_resemblance_is_the_sector_percentage() {
        assert_eq!(NcsHue::new(90.0).to_string(), "Y90R");
        assert_eq!(NcsHue::new(310.0).to_string(), "G10Y");
    }

    #[test]
    fn the_lab_hue_interpolates_along_the_shortest_arc() {
        // Y->R descends 90 -> 25, so the middle is ~57.5, not 237.5.
        assert!((NcsHue::new(50.0).lab_hue().as_f64() - 57.5).abs() < 1e-6);
        // R->B descends 25 -> 255 through the wrap (the violets).
        let hue = NcsHue::new(150.0).lab_hue().as_f64();
        assert!(hue > 300.0 || hue < 20.0);
        // G->Y descends 155 -> 90, middle is ~122.5.
        assert!((NcsHue::new(350.0).lab_hue().as_f64() - 122.5).abs() < 1e-6);
    }

    #[test]
    fn the_notation_follows_the_s_sccc_hue_form() {
        assert_eq!(Ncs::new(NcsHue::new(90.0), 20.0, 30.0).to_string(), "S 2030-Y90R");
    }

    #[test]
    fn the_parts_sum_to_one_hundred() {
        let ncs = Ncs::new(NcsHue::new(150.0), 20.0, 30.0);
        assert!((ncs.blackness + ncs.chromaticness + ncs.whiteness() - 100.0).abs() < 1e-9);
    }

    #[test]
    fn blackness_is_dark_and_whiteness_is_light() {
        let hue = NcsHue::new(0.0);
        let dark = hue.color(100.0, 0.0);
        let light = hue.color(0.0, 0.0);
        assert!(dark.r < light.r && dark.g < light.g && dark.b < light.b);
    }
}
