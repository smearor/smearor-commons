//! The [`DrawPrimitiveOps`] capability trait.

use crate::ops::DrawBackend;
use smearor_commons_color::Alpha;
use smearor_commons_color::Color;
use smearor_commons_geometry::Circle;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Rect;

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

    /// Fill `rect` with `color` at `alpha` (chess `fill_highlight`
    /// lineage — square flashes, selection overlays).
    fn fill_rect(&mut self, rect: Rect<Pixels>, color: Color<f64>, alpha: impl Into<Alpha>) -> Result<(), Self::Error>;

    /// Stroke the outline of `rect` with `color` at `alpha`, centered on
    /// the path edge with the given `line_width` (chess `ring_highlight`
    /// lineage — the consumer computes any inset policy itself).
    fn stroke_rect(&mut self, rect: Rect<Pixels>, color: Color<f64>, line_width: Pixels<f32>, alpha: impl Into<Alpha>) -> Result<(), Self::Error>;

    /// Fill `circle` with `color` at `alpha` (chess `target_dot` lineage —
    /// legal-target dots, indicators).
    fn fill_circle(&mut self, circle: Circle<Pixels>, color: Color<f64>, alpha: impl Into<Alpha>) -> Result<(), Self::Error>;
}

impl<T: DrawPrimitiveOps + ?Sized> DrawPrimitiveOps for &mut T {
    fn fill_background(&mut self, color: impl Into<Color<u8>>) -> Result<(), Self::Error> {
        (**self).fill_background(color)
    }

    fn fill_rect(&mut self, rect: Rect<Pixels>, color: Color<f64>, alpha: impl Into<Alpha>) -> Result<(), Self::Error> {
        (**self).fill_rect(rect, color, alpha)
    }

    fn stroke_rect(&mut self, rect: Rect<Pixels>, color: Color<f64>, line_width: Pixels<f32>, alpha: impl Into<Alpha>) -> Result<(), Self::Error> {
        (**self).stroke_rect(rect, color, line_width, alpha)
    }

    fn fill_circle(&mut self, circle: Circle<Pixels>, color: Color<f64>, alpha: impl Into<Alpha>) -> Result<(), Self::Error> {
        (**self).fill_circle(circle, color, alpha)
    }
}
