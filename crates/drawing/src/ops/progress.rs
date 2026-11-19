//! The [`DrawProgressOps`] capability trait.

use smearor_commons_color::Color;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Vec2;

/// Fraction-based indicator drawing operations: progress bars and
/// circular gauges. Values are clamped to `[0.0, 1.0]`.
pub trait DrawProgressOps {
    /// Draw a horizontal progress bar at the bottom of the view.
    fn draw_progress_bar(&mut self, value: Fraction, color: impl Into<Color<u8>>);

    /// Draw a circular gauge: a faint `track` ring plus a `color` arc
    /// starting at 12 o'clock and sweeping `value` of a full turn
    /// clockwise (sysinfo `draw_gauge` lineage, generalized to
    /// `Fraction` semantics).
    ///
    /// `radius` is the centerline radius of the ring; `line_width` is its
    /// stroke width — the ring occupies `radius ± line_width / 2`.
    /// Multi-arc gauges (network up/down) and threshold markers
    /// (temperature) stay consumer-side policy built on this primitive.
    #[allow(clippy::too_many_arguments)]
    fn draw_gauge(
        &mut self,
        center: Vec2<Pixels, f32>,
        radius: Pixels<f32>,
        line_width: Pixels<f32>,
        value: Fraction,
        color: impl Into<Color<f64>>,
        track: impl Into<Color<f64>>,
    );
}

impl<T: DrawProgressOps + ?Sized> DrawProgressOps for &mut T {
    fn draw_progress_bar(&mut self, value: Fraction, color: impl Into<Color<u8>>) {
        (**self).draw_progress_bar(value, color);
    }

    fn draw_gauge(
        &mut self,
        center: Vec2<Pixels, f32>,
        radius: Pixels<f32>,
        line_width: Pixels<f32>,
        value: Fraction,
        color: impl Into<Color<f64>>,
        track: impl Into<Color<f64>>,
    ) {
        (**self).draw_gauge(center, radius, line_width, value, color, track);
    }
}
