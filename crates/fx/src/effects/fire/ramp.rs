use super::FireColor;
use smearor_commons_color::Color;

/// Hard ramp stop positions - `fire.frag`'s `getFireColor` `steps`.
pub(super) const RAMP_STOPS: [f64; 5] = [0.0, 0.2, 0.35, 0.5, 0.8];

/// The classic fire ramp - BMW's `fire-color-1` .. `fire-color-5`
/// GSettings defaults keyed by [`FireColor`] slot.
pub(super) const DEFAULT_RAMP: [(FireColor, Color<f64>); 5] = [
    (FireColor::Ember, Color::<f64>::new_rgba(76.0 / 255.0, 51.0 / 255.0, 25.0 / 255.0, 0.0)),
    (FireColor::Red, Color::<f64>::new_rgba(180.0 / 255.0, 55.0 / 255.0, 30.0 / 255.0, 0.7)),
    (FireColor::Orange, Color::<f64>::new_rgba(1.0, 76.0 / 255.0, 38.0 / 255.0, 0.9)),
    (FireColor::Yellow, Color::<f64>::new_rgba(1.0, 166.0 / 255.0, 25.0 / 255.0, 1.0)),
    (FireColor::White, Color::<f64>::new_rgba(1.0, 1.0, 1.0, 1.0)),
];

/// `getFireColor` - piecewise-linear ramp over the five hard stops.
/// Deliberately not `Keyframes`: the stop positions are fixed and the
/// semantics are "map noise level to flame color", not a trajectory.
pub(super) fn fire_ramp(v: f64, colors: &[Color<f64>; 5]) -> Color<f64> {
    if v < RAMP_STOPS[0] {
        return colors[0];
    }
    for i in 0..4 {
        if v <= RAMP_STOPS[i + 1] {
            let t = (v - RAMP_STOPS[i]) / (RAMP_STOPS[i + 1] - RAMP_STOPS[i]);
            return colors[i].lerp(&colors[i + 1], t);
        }
    }
    colors[4]
}
