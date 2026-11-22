use smearor_commons_animation::EasingFunction;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Vec2;

/// BMW `scaleUV` - scales `uv` around the window center by
/// `scale` per axis (`scale = 0` keeps the window, `1` shrinks it
/// to a point). `mix(1, 0, scale)` is the divisor.
pub(super) fn scale_uv(uv: Vec2<()>, scale: Vec2<()>) -> Vec2<()> {
    let centered = Vec2::<()>::new(uv.x * 2.0 - 1.0, uv.y * 2.0 - 1.0);
    let div = Vec2::<()>::new(crate::mix(1.0, 0.0, scale.x).max(f64::EPSILON), crate::mix(1.0, 0.0, scale.y).max(f64::EPSILON));
    Vec2::<()>::new(centered.x / div.x * 0.5 + 0.5, centered.y / div.y * 0.5 + 0.5)
}

/// BMW `zeroStartEnd` - a symmetric bump `1 - |2t-1|^power` clamped
/// and scaled by `max_size`; zero at both animation ends.
pub(super) fn zero_start_end(t: f64, max_size: f64, power: f64) -> f64 {
    (1.0 - (2.0 * t - 1.0).abs().powf(power)).clamp(0.0, 1.0) * max_size
}

/// BMW `eightBitScale` - the jerky staircase growth (quarter/half/
/// full steps) which gives the effect its name.
pub(super) fn eight_bit_scale(progress: f64) -> f64 {
    match progress {
        p if p <= 0.1 => 0.25,
        p if p <= 0.2 => 0.5,
        p if p <= 0.3 => 0.25,
        p if p <= 0.4 => 0.5,
        p if p <= 0.5 => 0.25,
        p if p <= 0.6 => 0.5,
        p if p <= 0.7 => 1.0,
        p if p <= 0.8 => 0.25,
        p if p <= 0.9 => 0.5,
        _ => 1.0,
    }
}

/// The window scale vector - `mix(8bit, (easeInOutSine,
/// easeInQuad), scaleStyle)` between staircase and smooth.
pub(super) fn window_scale(progress: f64, scale_style: f64) -> Vec2<()> {
    let stair = eight_bit_scale(progress);
    let smooth = Vec2::<()>::new(
        EasingFunction::EaseInOutSine.apply(Fraction::new(progress)).as_f64(),
        EasingFunction::EaseIn.apply(Fraction::new(progress)).as_f64(),
    );
    Vec2::<()>::new(crate::mix(stair, smooth.x, scale_style), crate::mix(stair, smooth.y, scale_style))
}
