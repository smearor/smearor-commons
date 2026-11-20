//! The [`ProgressBar`] draw spec and its shared bottom margin.

use smearor_commons_color::Color;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Pixels;

/// Bottom-edge clearance under the progress bar — part of the
/// shared visual contract so both backends render the strip at the
/// same position.
pub const PROGRESS_BAR_BOTTOM_MARGIN: f32 = 2.0;

/// A progress bar's draw specification: fill fraction, color and
/// strip height. The backends anchor the strip at the target's
/// bottom edge, raised by [`PROGRESS_BAR_BOTTOM_MARGIN`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProgressBar {
    /// Fill fraction — the backends clamp it to `[0.0, 1.0]`.
    pub value: Fraction,
    /// Fill color; the unfilled rest of the strip is darkened to a
    /// third of it.
    pub color: Color<f64>,
    /// Strip height in pixels.
    pub height: Pixels<f32>,
}

impl ProgressBar {
    /// Creates a progress bar spec.
    pub fn new(value: Fraction, color: impl Into<Color<f64>>, height: Pixels<f32>) -> Self {
        Self {
            value,
            color: color.into(),
            height,
        }
    }
}
