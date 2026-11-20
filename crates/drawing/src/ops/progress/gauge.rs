//! The [`Gauge`] draw spec.

use smearor_commons_color::Color;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Vec2;

/// A circular gauge's draw specification: ring geometry, fill
/// fraction, arc color and track color.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Gauge {
    /// Center of the gauge ring.
    pub center: Vec2<Pixels, f32>,
    /// Centerline radius of the ring — the ring occupies
    /// `radius ± line_width / 2`.
    pub radius: Pixels<f32>,
    /// Stroke width of the ring.
    pub line_width: Pixels<f32>,
    /// Fill fraction — the backends clamp it to `[0.0, 1.0]`.
    pub value: Fraction,
    /// Arc color for the filled portion.
    pub color: Color<f64>,
    /// Track color for the unfilled ring.
    pub track: Color<f64>,
}

impl Gauge {
    /// Creates a gauge spec.
    pub fn new(
        center: Vec2<Pixels, f32>,
        radius: Pixels<f32>,
        line_width: Pixels<f32>,
        value: Fraction,
        color: impl Into<Color<f64>>,
        track: impl Into<Color<f64>>,
    ) -> Self {
        Self {
            center,
            radius,
            line_width,
            value,
            color: color.into(),
            track: track.into(),
        }
    }
}
