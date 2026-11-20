//! The [`FillSector`] draw spec.

use smearor_commons_color::Alpha;
use smearor_commons_color::Color;
use smearor_commons_geometry::Annulus;
use smearor_commons_geometry::Arc;
use smearor_commons_geometry::Pixels;

/// An annulus-sector ("pie ring") fill's draw specification: ring
/// geometry, angular interval, color and alpha. Directions follow
/// the `Arc`/`RotationRad` convention — measured counterclockwise
/// from the +x axis in a y-up math space, which in y-down screen
/// space sweeps clockwise (chess `annulus_highlight` and Pantone-fan
/// chip lineage).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FillSector {
    /// The ring the sector is cut from.
    pub annulus: Annulus<Pixels>,
    /// The angular interval of the sector.
    pub arc: Arc,
    /// Fill color.
    pub color: Color<f64>,
    /// Alpha composited on top of the color's own alpha channel.
    pub alpha: Alpha,
}

impl FillSector {
    /// Creates an annulus-sector fill spec.
    pub fn new(annulus: Annulus<Pixels>, arc: Arc, color: impl Into<Color<f64>>, alpha: impl Into<Alpha>) -> Self {
        Self {
            annulus,
            arc,
            color: color.into(),
            alpha: alpha.into(),
        }
    }
}
