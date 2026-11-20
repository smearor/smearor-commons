//! The [`DrawProgressOps`] capability trait and its draw specs.

mod gauge;
mod progress_bar;

pub use gauge::Gauge;
pub use progress_bar::PROGRESS_BAR_BOTTOM_MARGIN;
pub use progress_bar::ProgressBar;

use crate::ops::DrawBackend;

/// Fraction-based indicator drawing operations: progress bars and
/// circular gauges. Values are clamped to `[0.0, 1.0]`.
pub trait DrawProgressOps: DrawBackend {
    /// Draw `bar`'s strip at the bottom of the view.
    fn draw_progress_bar(&mut self, bar: ProgressBar) -> Result<(), Self::Error>;

    /// Draw `gauge`: a faint `track` ring plus a `color` arc starting
    /// at 12 o'clock and sweeping `value` of a full turn clockwise
    /// (sysinfo `draw_gauge` lineage, generalized to `Fraction`
    /// semantics).
    ///
    /// Multi-arc gauges (network up/down) and threshold markers
    /// (temperature) stay consumer-side policy built on this primitive.
    fn draw_gauge(&mut self, gauge: Gauge) -> Result<(), Self::Error>;
}

impl<T: DrawProgressOps + ?Sized> DrawProgressOps for &mut T {
    fn draw_progress_bar(&mut self, bar: ProgressBar) -> Result<(), Self::Error> {
        (**self).draw_progress_bar(bar)
    }

    fn draw_gauge(&mut self, gauge: Gauge) -> Result<(), Self::Error> {
        (**self).draw_gauge(gauge)
    }
}
