//! The [`StrokeSector`] draw spec.

use smearor_commons_color::Alpha;
use smearor_commons_color::Color;
use smearor_commons_geometry::Annulus;
use smearor_commons_geometry::Arc;
use smearor_commons_geometry::Pixels;

/// An annulus-sector outline's draw specification: the closed
/// boundary of `annulus ∩ arc` (both rims plus the two radial edges)
/// stroked with `line_width` - the hovered-wedge / selected-piece
/// outline matching [`FillSector`](crate::ops::FillSector)'s shape.
#[derive(Debug, Clone, PartialEq)]
pub struct StrokeSector {
    /// Annulus the sector is cut from.
    pub annulus: Annulus<Pixels>,
    /// The angular interval of the sector.
    pub arc: Arc,
    /// Stroke color.
    pub color: Color<f64>,
    /// Stroke width in pixels, centered on the boundary.
    pub line_width: Pixels<f32>,
    /// Alpha composited on top of the color's own alpha channel.
    pub alpha: Alpha,
}

impl StrokeSector {
    /// Creates a sector-outline spec.
    pub fn new(annulus: Annulus<Pixels>, arc: Arc, color: impl Into<Color<f64>>, line_width: Pixels<f32>, alpha: impl Into<Alpha>) -> Self {
        Self {
            annulus,
            arc,
            color: color.into(),
            line_width,
            alpha: alpha.into(),
        }
    }
}
