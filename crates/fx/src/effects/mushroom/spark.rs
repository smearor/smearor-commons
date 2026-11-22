use smearor_commons_animation::EasingFunction;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Vec2;

use super::scale::scale_uv;
use super::scale::zero_start_end;

/// BMW `rotate(a, angle, center)` - rotation around a pivot.
fn rotate(a: Vec2<()>, angle: f64, center: Vec2<()>) -> Vec2<()> {
    let (sin, cos) = angle.sin_cos();
    Vec2::<()>::new(
        cos * (a.x - center.x) + sin * (a.y - center.y) + center.x,
        cos * (a.y - center.y) - sin * (a.x - center.x) + center.y,
    )
}

/// BMW `getSpark` - a four-point sparkle: a sharp `easeInExpo`
/// diamond/cross mask rotated over progress.
fn get_spark(uv: Vec2<()>, center: Vec2<()>, brightness: f64, size: f64, rotation: f64) -> f64 {
    let brightness = brightness.clamp(0.001, 1.0);
    let size = size.clamp(0.001, 1.0);
    let bn = crate::mix(0.0, 0.07, brightness);
    let uv = Vec2::<()>::new(uv.x + 0.5 - center.x, uv.y + 0.5 - center.y);
    let uv = scale_uv(uv, Vec2::<()>::new(1.0 - size, 1.0 - size));
    let uv = rotate(uv, rotation, Vec2::<()>::new(0.5, 0.5));
    let p = crate::mix(-1.0, 1000.0, EasingFunction::EaseInExpo.apply(Fraction::new(bn)).as_f64());
    let m = (((uv.x - 0.5).abs() * 2.0).powf(p) + ((uv.y - 0.5).abs() * 2.0).powf(p)).clamp(0.0, 1.0);
    (EasingFunction::EaseInSine.apply(Fraction::new(1.0 - (m - bn))).as_f64() - 0.004).clamp(0.0, 1.0)
}

/// BMW `getSparks` - all `spark_count` sparkles composited in
/// order: each rises on its own `hash21` offset with an
/// `xEdge`-attenuated horizontal envelope, a `sin`-jittered x
/// position and the shared `progress * spark_rotation` spin.
#[allow(clippy::too_many_arguments)]
pub(super) fn sparks(
    uv_aspect: Vec2<()>,
    aspect: f64,
    progress: f64,
    count: u32,
    rotation: f64,
    seed_x: f64,
    color: smearor_commons_color::Color<f64>,
) -> smearor_commons_color::Color<f64> {
    let x_edge = (1.0 - ((uv_aspect.x - aspect * 0.5) / (aspect * 0.5)).abs().powi(8)).clamp(0.0, 1.0);
    let mut result = smearor_commons_color::Color::<f64>::new_rgba(0.0, 0.0, 0.0, 0.0);
    for s in 0..count {
        let h = smearor_commons_noise::Hash::<Vec2<()>>::hash(&(s as f64 + seed_x));
        let y = crate::mix(0.0 - h.y, 1.0 + (1.0 - h.y), progress).clamp(0.0, 1.0);
        let x = 0.66 * (h.x * std::f64::consts::TAU).sin() + 0.5 * aspect;
        let mask = get_spark(
            uv_aspect,
            Vec2::<()>::new(x, y),
            zero_start_end(y, 1.0, 4.0) * x_edge,
            zero_start_end(y, 0.5, 4.0) * x_edge,
            progress * std::f64::consts::TAU * rotation,
        );
        result = crate::alpha_over(result, smearor_commons_color::Color::<f64>::new_rgba(color.r, color.g, color.b, color.a * mask));
    }
    result
}
