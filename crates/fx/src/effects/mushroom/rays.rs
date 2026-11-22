use smearor_commons_color::Color;
use smearor_commons_geometry::Vec2;
use smearor_commons_noise::Field;
use smearor_commons_noise::Simplex2D;

use super::scale::zero_start_end;

/// BMW `getRays` - diagonal light streaks: `uv` stretched to
/// `(10, 0.5)`, scrolling down over progress, seeded horizontally;
/// attenuated to zero at all edges (`zeroStartEnd` in both axes)
/// and at the animation's ends, remapped `[-5, 1]` and tinted.
pub(super) fn rays(uv: Vec2<()>, progress: f64, seed_y: f64, color: Color<f64>) -> Color<f64> {
    let ray_uv = Vec2::<()>::new(uv.x * 10.0 + seed_y, uv.y * 0.5 - progress);
    let mut ray = Simplex2D.at(ray_uv);
    ray *= zero_start_end(uv.y, 1.0, 8.0) * zero_start_end(uv.x, 1.0, 8.0) * zero_start_end(progress, 1.0, 8.0);
    ray = crate::remap(ray * 1.10, 0.0, 1.0, -5.0, 1.0);
    Color::<f64>::new_rgba(color.r, color.g, color.b, (color.a * ray).clamp(0.0, 1.0))
}
