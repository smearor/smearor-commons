use crate::Color;
use crate::Hue;
use crate::LabLightness;
use crate::Xyz;
use serde::Deserialize;
use serde::Serialize;

/// A CIE L\*a\*b\* (CIELAB, 1976) color at the D65 white point.
///
/// `l` is the perceptual lightness (`0` = black, `100` = diffuse
/// white), `a` the green-to-red and `b` the blue-to-yellow opponent
/// axis.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Lab {
    /// The perceptual lightness `L*` (conventionally `0.0` - `100.0`,
    /// extrapolated values allowed).
    pub l: LabLightness,
    /// The green (`a < 0`) to red (`a > 0`) opponent component.
    pub a: f64,
    /// The blue (`b < 0`) to yellow (`b > 0`) opponent component.
    pub b: f64,
}

impl Lab {
    /// Creates a CIELAB value from the components.
    pub fn new(l: impl Into<LabLightness>, a: f64, b: f64) -> Self {
        Self { l: l.into(), a, b }
    }

    /// The chroma `C*ab` - the radial distance from the neutral axis.
    pub fn chroma(&self) -> f64 {
        self.a.hypot(self.b)
    }

    /// The hue angle `h_ab` (`0` = red, `90` = yellow, `180` = green,
    /// `270` = blue).
    pub fn hue(&self) -> Hue {
        Hue::new(self.b.atan2(self.a).to_degrees())
    }
}

const DELTA: f64 = 6.0 / 29.0;
const DELTA_CUBE: f64 = DELTA * DELTA * DELTA;

fn lab_f(t: f64) -> f64 {
    if t > DELTA_CUBE {
        t.powf(1.0 / 3.0)
    } else {
        t / (3.0 * DELTA * DELTA) + 4.0 / 29.0
    }
}

fn lab_f_inv(t: f64) -> f64 {
    if t > DELTA { t * t * t } else { 3.0 * DELTA * DELTA * (t - 4.0 / 29.0) }
}

impl Lab {
    /// Creates a CIELAB value from CIE XYZ relative to `white` - the
    /// white point is part of the Lab color space (D65 for sRGB work,
    /// D50 for print and ICC characterization data).
    pub fn from_xyz(xyz: Xyz, white: Xyz) -> Self {
        let (fx, fy, fz) = (lab_f(xyz.x / white.x), lab_f(xyz.y / white.y), lab_f(xyz.z / white.z));
        Self::new(116.0 * fy - 16.0, 500.0 * (fx - fy), 200.0 * (fy - fz))
    }

    /// Converts to CIE XYZ relative to `white` - the inverse of
    /// [`Self::from_xyz`].
    pub fn to_xyz(self, white: Xyz) -> Xyz {
        let fy = (self.l.as_f64() + 16.0) / 116.0;
        Xyz::new(white.x * lab_f_inv(fy + self.a / 500.0), white.y * lab_f_inv(fy), white.z * lab_f_inv(fy - self.b / 200.0))
    }
}

impl From<Xyz> for Lab {
    /// Converts a CIE XYZ value to CIELAB at the D65 white point.
    fn from(xyz: Xyz) -> Self {
        Self::from_xyz(xyz, Xyz::WHITE_D65)
    }
}

impl From<Lab> for Xyz {
    /// Converts a CIELAB value to CIE XYZ at the D65 white point.
    fn from(lab: Lab) -> Self {
        lab.to_xyz(Xyz::WHITE_D65)
    }
}

impl From<Color<u8>> for Lab {
    /// Converts an sRGB color to CIELAB (via XYZ, D65).
    fn from(color: Color<u8>) -> Self {
        Self::from(Xyz::from(color))
    }
}

impl From<Lab> for Color<u8> {
    /// Converts a CIELAB value to sRGB, clamped to the gamut.
    fn from(lab: Lab) -> Self {
        Self::from(Xyz::from(lab))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn white_point_maps_to_neutral() {
        for white in [Xyz::WHITE_D65, Xyz::WHITE_D50] {
            let lab = Lab::from_xyz(white, white);
            assert!((lab.l.as_f64() - 100.0).abs() < 1e-6);
            assert!(lab.a.abs() < 1e-6 && lab.b.abs() < 1e-6);
            let back = lab.to_xyz(white);
            assert!((back.x - white.x).abs() < 1e-6);
            assert!((back.y - white.y).abs() < 1e-6);
            assert!((back.z - white.z).abs() < 1e-6);
        }
    }

    #[test]
    fn white_has_a_lightness_of_100() {
        let lab = Lab::from(Color::<u8>::new(255, 255, 255));
        assert!((lab.l.as_f64() - 100.0).abs() < 0.1);
        assert!(lab.a.abs() < 0.1 && lab.b.abs() < 0.1);
    }

    #[test]
    fn black_has_a_lightness_of_zero() {
        let lab = Lab::from(Color::<u8>::new(0, 0, 0));
        assert!(lab.l.as_f64().abs() < 0.1);
    }

    #[test]
    fn srgb_primaries_have_the_documented_lab_coordinates() {
        let red = Lab::from(Color::<u8>::new(255, 0, 0));
        assert!((red.l.as_f64() - 53.24).abs() < 0.1 && (red.a - 80.09).abs() < 0.1 && (red.b - 67.20).abs() < 0.1);
    }

    #[test]
    fn lab_round_trip_is_stable() {
        let color = Color::<u8>::new(120, 180, 60);
        let round_tripped = Color::<u8>::from(Lab::from(color));
        assert!((round_tripped.r as i32 - 120).abs() <= 1);
        assert!((round_tripped.g as i32 - 180).abs() <= 1);
        assert!((round_tripped.b as i32 - 60).abs() <= 1);
    }
}
