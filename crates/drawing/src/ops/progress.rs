//! The [`DrawProgressOps`] capability trait.

use smearor_commons_color::Color;
use smearor_commons_geometry::Fraction;

/// Fraction-based indicator drawing operations: progress bars and
/// (in later phases) gauges. Values are clamped to `[0.0, 1.0]`.
pub trait DrawProgressOps {
    /// Draw a horizontal progress bar at the bottom of the view.
    fn draw_progress_bar(&mut self, value: Fraction, color: impl Into<Color<u8>>);
}
