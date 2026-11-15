use crate::Degrees;
use crate::Pixels;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;

/// Rotation sensitivity: degrees of turn per scrolled pixel.
///
/// Maps a scroll length onto an angle - the input-side counterpart
/// of [`crate::PixelsPerMm`] (length → length). Wrapping the bare
/// `f64` keeps the rate unconfusable with a plain angle or a pixel
/// count. Serializes as a bare number.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DegreesPerPixel(pub f64);

impl DegreesPerPixel {
    /// Creates a sensitivity from degrees per pixel.
    pub const fn new(degrees_per_pixel: f64) -> Self {
        Self(degrees_per_pixel)
    }

    /// The sensitivity in degrees per pixel.
    pub const fn as_f64(self) -> f64 {
        self.0
    }

    /// Maps a scroll length in pixels onto a turn in degrees.
    pub fn scale(&self, px: Pixels<f64>) -> Degrees {
        Degrees::new(px.as_f64() * self.0)
    }
}

impl From<f64> for DegreesPerPixel {
    fn from(degrees_per_pixel: f64) -> Self {
        Self(degrees_per_pixel)
    }
}

impl From<DegreesPerPixel> for f64 {
    fn from(degrees_per_pixel: DegreesPerPixel) -> Self {
        degrees_per_pixel.0
    }
}

impl Display for DegreesPerPixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::DegreesPerPixel;
    use super::Pixels;

    /// `scale` maps pixels onto degrees via the rate.
    #[test]
    fn scale_maps_pixels_to_degrees() {
        let step = DegreesPerPixel::new(5.0);
        assert!((step.scale(Pixels::new(10.0)).as_f64() - 50.0).abs() < 1e-9);
        assert!((step.scale(Pixels::new(-2.0)).as_f64() - (-10.0)).abs() < 1e-9);
    }
}
