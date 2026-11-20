//! The [`DrawPrimitiveOps`] capability trait and its draw specs.

mod fill_circle;
mod fill_polygon;
mod fill_rect;
mod fill_sector;
mod stroke_polyline;
mod stroke_rect;

pub use fill_circle::FillCircle;
pub use fill_polygon::FillPolygon;
pub use fill_rect::FillRect;
pub use fill_sector::FillSector;
pub use stroke_polyline::StrokePolyline;
pub use stroke_rect::StrokeRect;

use crate::ops::DrawBackend;
use smearor_commons_color::Color;

/// Primitive surface operations: background fills plus the highlight
/// primitives harvested from consumer code — rect fills, ring strokes and
/// dots over `Rect`/`Circle` geometry.
///
/// The methods only write the declared pixel area; row padding and memory
/// beyond the view extent stay untouched. `alpha` composes with the
/// color's own alpha channel; the rendered result is source-over blended
/// onto existing pixels, matching what a cairo backend produces with
/// `set_source_rgba` + `fill`.
pub trait DrawPrimitiveOps: DrawBackend {
    /// Fill the entire view with a solid background color.
    fn fill_background(&mut self, color: impl Into<Color<u8>>) -> Result<(), Self::Error>;

    /// Fill `spec`'s rect (chess `fill_highlight` lineage — square
    /// flashes, selection overlays).
    fn fill_rect(&mut self, spec: FillRect) -> Result<(), Self::Error>;

    /// Stroke `spec`'s rect outline (chess `ring_highlight` lineage —
    /// centered on the path edge; the consumer computes any inset policy
    /// itself).
    fn stroke_rect(&mut self, spec: StrokeRect) -> Result<(), Self::Error>;

    /// Fill `spec`'s circle (chess `target_dot` lineage — legal-target
    /// dots, indicators).
    fn fill_circle(&mut self, spec: FillCircle) -> Result<(), Self::Error>;

    /// Fill `spec`'s polygon, even-odd (gamut outlines, Ostwald/NCS
    /// triangle cells).
    fn fill_polygon(&mut self, spec: FillPolygon) -> Result<(), Self::Error>;

    /// Stroke `spec`'s polyline segments — the path stays open
    /// (spectral and Planckian loci, wireframe edges).
    fn stroke_polyline(&mut self, spec: StrokePolyline) -> Result<(), Self::Error>;

    /// Fill `spec`'s annulus sector (pie-ring pieces: color-wheel
    /// wedges, fan chips).
    fn fill_sector(&mut self, spec: FillSector) -> Result<(), Self::Error>;
}

impl<T: DrawPrimitiveOps + ?Sized> DrawPrimitiveOps for &mut T {
    fn fill_background(&mut self, color: impl Into<Color<u8>>) -> Result<(), Self::Error> {
        (**self).fill_background(color)
    }

    fn fill_rect(&mut self, spec: FillRect) -> Result<(), Self::Error> {
        (**self).fill_rect(spec)
    }

    fn stroke_rect(&mut self, spec: StrokeRect) -> Result<(), Self::Error> {
        (**self).stroke_rect(spec)
    }

    fn fill_circle(&mut self, spec: FillCircle) -> Result<(), Self::Error> {
        (**self).fill_circle(spec)
    }

    fn fill_polygon(&mut self, spec: FillPolygon) -> Result<(), Self::Error> {
        (**self).fill_polygon(spec)
    }

    fn stroke_polyline(&mut self, spec: StrokePolyline) -> Result<(), Self::Error> {
        (**self).stroke_polyline(spec)
    }

    fn fill_sector(&mut self, spec: FillSector) -> Result<(), Self::Error> {
        (**self).fill_sector(spec)
    }
}
