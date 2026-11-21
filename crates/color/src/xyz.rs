use crate::Color;
use serde::Deserialize;
use serde::Serialize;

/// A CIE 1931 XYZ tristimulus value.
///
/// The sRGB conversions assume the D65 white point used by the sRGB
/// specification (IEC 61966-2-1). Values outside the sRGB gamut are
/// clamped channel-wise when converting back to [`Color`].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Xyz {
    /// The X tristimulus value (`Y = 100.0` for the white point).
    pub x: f64,
    /// The Y tristimulus value - the luminance.
    pub y: f64,
    /// The Z tristimulus value.
    pub z: f64,
}

impl Xyz {
    /// The CIE D65 (daylight, ~6504K) white point at `Y = 100`.
    pub const WHITE_D65: Self = Self {
        x: 95.047,
        y: 100.0,
        z: 108.883,
    };

    /// The CIE D50 (horizon light, ~5003K) white point at `Y = 100` -
    /// the standard illuminant of print and graphic arts (ICC PCS,
    /// ISO 12647 / FOGRA characterization data).
    pub const WHITE_D50: Self = Self { x: 96.42, y: 100.0, z: 82.49 };

    /// The CIE illuminant C (average daylight, ~6774K) white point at
    /// `Y = 100` - the white point of the Munsell renotation data.
    pub const WHITE_C: Self = Self {
        x: 98.074,
        y: 100.0,
        z: 118.232,
    };

    /// Creates a tristimulus value from the components.
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Creates a tristimulus value from chromaticity coordinates and a
    /// luminance (`xyY` notation).
    ///
    /// `y_chroma` must be positive; for `y_chroma <= 0.0` (no defined
    /// chromaticity, e.g. black) the result is black.
    pub fn from_xyy(x: f64, y_chroma: f64, luminance: f64) -> Self {
        if y_chroma <= 0.0 || luminance <= 0.0 {
            return Self::new(0.0, 0.0, 0.0);
        }
        Self {
            x: x * luminance / y_chroma,
            y: luminance,
            z: (1.0 - x - y_chroma) * luminance / y_chroma,
        }
    }

    /// The chromaticity coordinates of the value (`x`, `y`, `Y`).
    pub fn to_xyy(&self) -> (f64, f64, f64) {
        let sum = self.x + self.y + self.z;
        if sum <= 0.0 {
            return (0.0, 0.0, self.y);
        }
        (self.x / sum, self.y / sum, self.y)
    }

    /// Adapts the tristimulus value from one white point to another
    /// using the Bradford chromatic adaptation transform.
    pub fn adapted(&self, source_white: Self, target_white: Self) -> Self {
        const BRADFORD: [[f64; 3]; 3] = [[0.8951, 0.2664, -0.1614], [-0.7502, 1.7135, 0.0367], [0.0389, -0.0685, 1.0296]];
        const BRADFORD_INV: [[f64; 3]; 3] = [
            [0.9869929, -0.1470543, 0.1599627],
            [0.4323053, 0.5183603, 0.0492912],
            [-0.0085287, 0.0400428, 0.9684867],
        ];
        let (sx, sy, sz) = (mat_mul(BRADFORD, self), mat_mul(BRADFORD, &source_white), mat_mul(BRADFORD, &target_white));
        let scale = Self::new(
            if sy.x != 0.0 { sz.x / sy.x } else { 1.0 },
            if sy.y != 0.0 { sz.y / sy.y } else { 1.0 },
            if sy.z != 0.0 { sz.z / sy.z } else { 1.0 },
        );
        let scaled = Self::new(sx.x * scale.x, sx.y * scale.y, sx.z * scale.z);
        mat_mul(BRADFORD_INV, &scaled)
    }
}

/// Multiplies a 3×3 matrix with a tristimulus value.
fn mat_mul(m: [[f64; 3]; 3], v: &Xyz) -> Xyz {
    Xyz::new(
        m[0][0] * v.x + m[0][1] * v.y + m[0][2] * v.z,
        m[1][0] * v.x + m[1][1] * v.y + m[1][2] * v.z,
        m[2][0] * v.x + m[2][1] * v.y + m[2][2] * v.z,
    )
}

/// Linearizes one sRGB channel (IEC 61966-2-1).
fn srgb_to_linear(channel: f64) -> f64 {
    if channel <= 0.04045 {
        channel / 12.92
    } else {
        ((channel + 0.055) / 1.055).powf(2.4)
    }
}

/// Compands one linear channel back to sRGB.
fn linear_to_srgb(channel: f64) -> f64 {
    if channel <= 0.0031308 {
        12.92 * channel
    } else {
        1.055 * channel.powf(1.0 / 2.4) - 0.055
    }
}

impl From<Color<f64>> for Xyz {
    /// Converts a normalized sRGB color to CIE XYZ (D65).
    fn from(color: Color<f64>) -> Self {
        let (r, g, b) = (srgb_to_linear(color.r), srgb_to_linear(color.g), srgb_to_linear(color.b));
        Self::new(
            41.23908 * r + 35.75843 * g + 18.04808 * b,
            21.26390 * r + 71.51687 * g + 7.21923 * b,
            1.93308 * r + 11.91948 * g + 95.05322 * b,
        )
    }
}

impl From<Color<u8>> for Xyz {
    /// Converts an sRGB color to CIE XYZ (D65).
    fn from(color: Color<u8>) -> Self {
        Self::from(Color::<f64>::new(color.r as f64 / 255.0, color.g as f64 / 255.0, color.b as f64 / 255.0))
    }
}

impl From<Xyz> for Color<f64> {
    /// Converts a CIE XYZ (D65) value to normalized sRGB.
    ///
    /// Out-of-gamut results are clamped channel-wise.
    fn from(xyz: Xyz) -> Self {
        let r = 0.032404542 * xyz.x - 0.015371385 * xyz.y - 0.004985314 * xyz.z;
        let g = -0.009692660 * xyz.x + 0.018760108 * xyz.y + 0.000415560 * xyz.z;
        let b = 0.000556434 * xyz.x - 0.002040259 * xyz.y + 0.010572252 * xyz.z;
        Self::new(linear_to_srgb(r).clamp(0.0, 1.0), linear_to_srgb(g).clamp(0.0, 1.0), linear_to_srgb(b).clamp(0.0, 1.0))
    }
}

impl From<Xyz> for Color<u8> {
    /// Converts a CIE XYZ (D65) value to sRGB, clamped to the gamut.
    fn from(xyz: Xyz) -> Self {
        let normalized = Color::<f64>::from(xyz);
        Self::new_rgba(
            (normalized.r * 255.0).round() as u8,
            (normalized.g * 255.0).round() as u8,
            (normalized.b * 255.0).round() as u8,
            255,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn srgb_white_is_the_d65_white_point() {
        let xyz = Xyz::from(Color::<u8>::new(255, 255, 255));
        assert!((xyz.x - 95.047).abs() < 0.1);
        assert!((xyz.y - 100.0).abs() < 0.1);
        assert!((xyz.z - 108.883).abs() < 0.1);
    }

    #[test]
    fn xyz_round_trip_is_stable() {
        let color = Color::<u8>::new(200, 100, 50);
        let round_tripped = Color::<u8>::from(Xyz::from(color));
        assert!((round_tripped.r as i32 - 200).abs() <= 1);
        assert!((round_tripped.g as i32 - 100).abs() <= 1);
        assert!((round_tripped.b as i32 - 50).abs() <= 1);
    }

    #[test]
    fn out_of_gamut_values_are_clamped() {
        let color = Color::<f64>::from(Xyz::new(0.0, 100.0, 0.0));
        assert!(color.r >= 0.0 && color.g <= 1.0 && color.b >= 0.0);
    }

    #[test]
    fn xyy_round_trip_is_stable() {
        let (x, y, yy) = Xyz::new(20.0, 30.0, 50.0).to_xyy();
        let xyz = Xyz::from_xyy(x, y, yy);
        assert!((xyz.x - 20.0).abs() < 0.001 && (xyz.y - 30.0).abs() < 0.001 && (xyz.z - 50.0).abs() < 0.001);
    }

    #[test]
    fn bradford_adaptation_maps_white_to_white() {
        let adapted = Xyz::WHITE_C.adapted(Xyz::WHITE_C, Xyz::WHITE_D65);
        assert!((adapted.x - Xyz::WHITE_D65.x).abs() < 0.05);
        assert!((adapted.y - Xyz::WHITE_D65.y).abs() < 0.05);
        assert!((adapted.z - Xyz::WHITE_D65.z).abs() < 0.05);
    }
}
