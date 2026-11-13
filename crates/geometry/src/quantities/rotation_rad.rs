use crate::Degrees;
use crate::Radians;
use crate::SmoothingAlpha;
use crate::Vec2;
use serde::Deserialize;
use serde::Serialize;
use std::f64::consts::FRAC_PI_2;
use std::f64::consts::PI;
use std::f64::consts::TAU;
use std::fmt::Display;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

/// A rotation angle in radians.
///
/// Wraps the [`Radians`] scalar unit with rotation semantics -
/// wrapping, deltas, axes, trigonometry - so angles cannot be
/// confused with distances or other scalar values flowing through
/// the pipeline. Plain scalar uses (thresholds, wire values) use
/// [`Radians`]/[`Degrees`] directly.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RotationRad(pub Radians);

impl RotationRad {
    /// No rotation.
    pub const ZERO: Self = Self(Radians::ZERO);

    /// 30° - an hour mark on a clock face, 12-item radial layouts.
    pub const TWELFTH_TURN: Self = Self(Radians::TWELFTH_TURN);

    /// 45° - a compass diagonal, 8-item radial layouts.
    pub const EIGHTH_TURN: Self = Self(Radians::EIGHTH_TURN);

    /// 60° - an equilateral angle, hexagonal and 6-item radial layouts.
    pub const SIXTH_TURN: Self = Self(Radians::SIXTH_TURN);

    /// 90° - a right angle, the `QuarterTurn` increments.
    pub const QUARTER_TURN: Self = Self(Radians::QUARTER_TURN);

    /// 120° - three-way symmetric layouts.
    pub const THIRD_TURN: Self = Self(Radians::THIRD_TURN);

    /// 180° - a straight angle, the direction opposite `ZERO`.
    pub const HALF_TURN: Self = Self(Radians::HALF_TURN);

    /// 270° - the last of the `QuarterTurn` increments.
    pub const THREE_QUARTER_TURN: Self = Self(Radians::THREE_QUARTER_TURN);

    /// A full turn - every direction, an arc of `2*PI` radians.
    pub const FULL_TURN: Self = Self(Radians::FULL_TURN);
    /// Creates a rotation from a radians value.
    pub const fn new(radians: f64) -> Self {
        Self(Radians::new(radians))
    }

    /// Creates a rotation from the [`Radians`] scalar unit.
    pub const fn from_radians(radians: Radians) -> Self {
        Self(radians)
    }

    /// Creates a rotation from degrees.
    ///
    /// Accepts any scalar that widens losslessly to `f64` (`f32`, `i32`,
    /// `u32`, ...); narrower-than-`f64` sources like `usize` still
    /// require an explicit cast.
    pub fn from_degrees(degrees: impl Into<f64>) -> Self {
        Self(Radians::new(degrees.into().to_radians()))
    }

    /// The angle in radians.
    pub const fn as_f64(self) -> f64 {
        self.0.as_f64()
    }

    /// Whether the rotation is zero (no turn).
    pub fn is_zero(self) -> bool {
        self.0 == Radians::ZERO
    }

    /// The absolute magnitude of the angle - the unsigned arc length,
    /// e.g. of a [`signed_delta_to`](Self::signed_delta_to) result.
    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }

    /// The angle converted to degrees.
    pub fn to_degrees(self) -> f64 {
        self.0.as_f64().to_degrees()
    }

    /// The angle as the [`Degrees`] scalar unit - the unit-carrying
    /// counterpart of [`RotationRad::to_degrees`] for typed
    /// boundaries.
    pub fn degrees(self) -> Degrees {
        self.0.to_degrees()
    }

    /// The angle as the [`Radians`] scalar unit - the unit-carrying
    /// counterpart of [`RotationRad::as_f64`] for typed boundaries.
    pub const fn radians(self) -> Radians {
        self.0
    }

    /// The angle in degrees as `f32` for `f32`-based framework APIs
    /// (e.g. GTK4 transforms).
    pub fn to_degrees_f32(self) -> f32 {
        self.to_degrees() as f32
    }

    /// `(sin, cos)` of the angle - trigonometry for transforms and
    /// geometry (the `f64::sin_cos` delegate).
    pub fn sin_cos(self) -> (f64, f64) {
        self.0.sin_cos()
    }

    /// The interior angle between two consecutive edge directions
    /// of a positive-area polygon: `PI - signed turn` from
    /// `previous` to `next` (sharp corners < ~PI/2).
    pub fn interior<U>(previous: Vec2<U>, next: Vec2<U>) -> Self {
        Self::new(PI - previous.cross::<f64>(&next).atan2(previous.dot::<f64>(&next)))
    }

    /// The angle normalized to `[0, 2*PI)` - the canonical
    /// representative of the same direction. The full-turn
    /// counterpart of [`Self::axis`], which folds onto a
    /// half-turn axis instead.
    pub fn normalized(self) -> RotationRad {
        Self(self.0.normalized())
    }

    /// The shortest signed arc from `self` to `to`, in [-π, π].
    ///
    /// The ±PI wrap is crossed instead of going the long way around;
    /// the sign tells the turning direction.
    pub fn signed_delta_to(self, to: RotationRad) -> RotationRad {
        let mut delta = to.0 - self.0;
        while delta.as_f64() > PI {
            delta = delta - Radians::new(TAU);
        }
        while delta.as_f64() < -PI {
            delta = delta + Radians::new(TAU);
        }
        Self(delta)
    }

    /// Wraparound-aware EMA blend - `old + alpha * diff(new, old)`,
    /// where `diff` is the shortest signed arc between the angles
    /// (modulo 2π). `alpha` is the weight of `new`.
    pub fn ema(old: RotationRad, new: RotationRad, alpha: SmoothingAlpha) -> RotationRad {
        Self(old.0 + old.signed_delta_to(new).0 * alpha)
    }

    /// The direction normalized to an undirected axis in
    /// `[-pi/2, pi/2)` - `self` and `self + pi` (the antipodal
    /// direction) share an axis, so heading-ambiguous footprints
    /// like two-pin dumbbells fold onto one stable value.
    pub fn axis(&self) -> RotationRad {
        Self::new(self.0.as_f64().rem_euclid(PI) - FRAC_PI_2)
    }

    /// The shortest signed turn between two undirected axes, in
    /// `[-pi/2, pi/2)` - the [`Self::axis`]-space counterpart of
    /// [`Self::signed_delta_to`]: axes wrap modulo `pi`, so two
    /// nearly-parallel lines on opposite sides of the `+-pi/2`
    /// fold keep a small delta instead of a nearly-pi swing.
    pub fn axis_delta_to(self, to: RotationRad) -> RotationRad {
        Self::new((to.0 - self.0 + Radians::new(FRAC_PI_2)).as_f64().rem_euclid(PI) - FRAC_PI_2)
    }

    /// Axis-space [`Self::ema`]: wraps the blend along the `pi`
    /// fold instead of the `2pi` wrap, so smoothing a
    /// heading-ambiguous axis cannot swing a full half-turn when a
    /// measurement crosses `+-pi/2`. `old` and `new` are axis
    /// values; the result is folded back into `[-pi/2, pi/2)`.
    pub fn ema_axis(old: RotationRad, new: RotationRad, alpha: SmoothingAlpha) -> RotationRad {
        let blended = Self(old.0 + old.axis_delta_to(new).0 * alpha);
        RotationRad::ZERO.axis_delta_to(blended)
    }
}

/// A degrees scalar becomes a rotation - the typed counterpart of
/// [`RotationRad::from_degrees`].
impl From<Degrees> for RotationRad {
    fn from(degrees: Degrees) -> Self {
        Self::from_degrees(degrees.as_f64())
    }
}

/// A radians scalar becomes a rotation - the typed counterpart of
/// [`RotationRad::new`].
impl From<Radians> for RotationRad {
    fn from(radians: Radians) -> Self {
        Self(radians)
    }
}

impl From<f64> for RotationRad {
    fn from(radians: f64) -> Self {
        Self::new(radians)
    }
}

impl Add for RotationRad {
    type Output = RotationRad;

    /// Composes two rotations.
    fn add(self, other: RotationRad) -> RotationRad {
        Self(self.0 + other.0)
    }
}

impl AddAssign for RotationRad {
    /// Adds a rotation in place.
    fn add_assign(&mut self, other: RotationRad) {
        self.0 = self.0 + other.0;
    }
}

impl Sub for RotationRad {
    type Output = RotationRad;

    /// The signed difference between two rotations.
    fn sub(self, other: RotationRad) -> RotationRad {
        Self(self.0 - other.0)
    }
}

impl Neg for RotationRad {
    type Output = RotationRad;

    /// The inverse rotation.
    fn neg(self) -> RotationRad {
        Self(-self.0)
    }
}

impl Display for RotationRad {
    /// Formats via [`Degrees`] (`45°`) - the display convention
    /// across the ecosystem's logs and UIs (the stored radians are
    /// an implementation detail for humans).
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.degrees().fmt(f)
    }
}

/// A blend weight scales a rotation - the EMA/lerp-weighted
/// counterpart of `RotationRad * f64`.
impl Mul<SmoothingAlpha> for RotationRad {
    type Output = RotationRad;

    fn mul(self, weight: SmoothingAlpha) -> RotationRad {
        Self(self.0 * weight)
    }
}

impl Mul<RotationRad> for SmoothingAlpha {
    type Output = RotationRad;

    fn mul(self, rotation: RotationRad) -> Self::Output {
        rotation * self
    }
}

impl Mul<f64> for RotationRad {
    type Output = RotationRad;

    /// Scales the angle - fractional spans (`width * t`), sign flips
    /// (`angle * -1.0`) and weighted blends.
    fn mul(self, factor: f64) -> RotationRad {
        Self(self.0 * factor)
    }
}

#[cfg(feature = "async-graphql")]
async_graphql::scalar!(RotationRad);

#[cfg(test)]
mod tests {
    use super::RotationRad;
    use crate::SmoothingAlpha;
    use std::f64::consts::FRAC_PI_2;
    use std::f64::consts::PI;
    use std::f64::consts::TAU;

    /// `normalized` wraps any angle into `[0, 2*PI)` without
    /// changing the direction it points to.
    #[test]
    fn normalized_wraps_into_full_turn() {
        assert_eq!(RotationRad::new(0.5).normalized().as_f64(), 0.5);
        assert!((RotationRad::new(-0.5).normalized().as_f64() - (TAU - 0.5)).abs() < 1e-9);
        assert!((RotationRad::new(3.0 * PI).normalized().as_f64() - PI).abs() < 1e-9);
        assert!(RotationRad::new(TAU).normalized().as_f64().abs() < 1e-9);
    }

    /// `alpha = 1` makes the blend pure `new` - history has no weight.
    #[test]
    fn ema_full_weight_is_new() {
        let result = RotationRad::ema(RotationRad::new(1.0), RotationRad::new(2.0), SmoothingAlpha::new(1.0));
        assert_eq!(result.as_f64(), 2.0);
    }

    /// `alpha = 0` keeps `old` - a frozen pose ignores measurement.
    #[test]
    fn ema_zero_weight_is_old() {
        let result = RotationRad::ema(RotationRad::new(1.0), RotationRad::new(2.0), SmoothingAlpha::new(0.0));
        assert_eq!(result.as_f64(), 1.0);
    }

    /// The shortest arc crosses ±π upward: `3.0 → -3.0` passes over
    /// π, not back through 0.
    #[test]
    fn ema_crosses_pi_boundary_shortest_way() {
        let result = RotationRad::ema(RotationRad::new(3.0), RotationRad::new(-3.0), SmoothingAlpha::new(0.5));
        assert!(result.as_f64() > 3.0);
        assert!(result.as_f64() <= PI + 1e-9);
    }

    /// Within ±PI the arc is the plain difference.
    #[test]
    fn signed_delta_inside_bounds() {
        let delta = RotationRad::new(1.0).signed_delta_to(RotationRad::new(2.0));
        assert_eq!(delta.as_f64(), 1.0);
    }

    /// `3.0 -> -3.0` wraps across +PI (delta ~= +0.28), not -6 the
    /// long way.
    #[test]
    fn signed_delta_wraps_shortest_way() {
        let delta = RotationRad::new(3.0).signed_delta_to(RotationRad::new(-3.0));
        assert!((delta.as_f64() - (TAU - 6.0)).abs() < 1e-9);
    }

    /// The mirrored crossing reports the negative direction.
    #[test]
    fn signed_delta_wraps_negative_side() {
        let delta = RotationRad::new(-3.0).signed_delta_to(RotationRad::new(3.0));
        assert!((delta.as_f64() - (6.0 - TAU)).abs() < 1e-9);
    }

    #[test]
    fn interior_angle_of_a_right_turn() {
        let angle = RotationRad::interior(crate::Vec2::<crate::Millimeters>::new(1.0, 0.0), crate::Vec2::new(0.0, 1.0));
        assert!((angle.as_f64() - FRAC_PI_2).abs() < 1e-9);
    }

    /// `axis` folds `a` and `a + pi` onto the same undirected angle.
    #[test]
    fn axis_ignores_direction() {
        let (a, b) = (RotationRad::new(0.3), RotationRad::new(0.3 + PI));
        assert!((a.axis().as_f64() - b.axis().as_f64()).abs() < 1e-9);
        assert!((a.axis().as_f64() - (0.3 - FRAC_PI_2)).abs() < 1e-9);
    }

    /// `axis` keeps every direction inside `[-pi/2, pi/2)`, even for
    /// non-normalized inputs - `rem_euclid` wraps full turns first.
    #[test]
    fn axis_stays_in_half_turn_range() {
        for angle in [0.0, 1.0, -1.0, FRAC_PI_2, -FRAC_PI_2, PI, -PI, 4.0, -4.0, 42.0] {
            let axis = RotationRad::new(angle).axis().as_f64();
            assert!((-FRAC_PI_2..FRAC_PI_2).contains(&axis), "axis({angle}) = {axis} outside [-pi/2, pi/2)");
        }
    }

    /// `axis` measures the line's tilt off vertical: `0` and `pi`
    /// (horizontal) fold to `-pi/2`, `+-pi/2` (vertical) folds to `0`.
    #[test]
    fn axis_boundary_values() {
        assert!((RotationRad::new(0.0).axis().as_f64() + FRAC_PI_2).abs() < 1e-9);
        assert!(RotationRad::new(FRAC_PI_2).axis().as_f64().abs() < 1e-9);
        assert!(RotationRad::new(-FRAC_PI_2).axis().as_f64().abs() < 1e-9);
        assert!((RotationRad::new(PI).axis().as_f64() + FRAC_PI_2).abs() < 1e-9);
    }

    /// Ordinary axis deltas pass through unchanged; an antipodal
    /// turn (the same line) reports a zero delta.
    #[test]
    fn axis_delta_inside_bounds() {
        let a = RotationRad::new(0.2);
        let b = RotationRad::new(0.9);
        assert!((a.axis_delta_to(b).as_f64() - 0.7).abs() < 1e-9);
        assert!((b.axis_delta_to(a).as_f64() + 0.7).abs() < 1e-9);
        assert!(a.axis_delta_to(RotationRad::new(a.as_f64() + PI)).as_f64().abs() < 1e-9);
    }

    /// `axis_delta_to` wraps modulo `pi`: two nearly-parallel lines
    /// on opposite sides of the `+-pi/2` fold keep a small turn
    /// (~0.14 rad) instead of the ~3 rad `signed_delta_to` would
    /// report for the raw values.
    #[test]
    fn axis_delta_wraps_across_the_fold() {
        let old = RotationRad::new(-1.5);
        let new = RotationRad::new(1.5);
        let delta = old.axis_delta_to(new).as_f64();
        assert!((delta - (3.0 - PI)).abs() < 1e-9);
        // antisymmetric, like signed_delta_to
        assert!((new.axis_delta_to(old).as_f64() + delta).abs() < 1e-9);
    }

    /// Plain axis blends stay linear inside the range.
    #[test]
    fn ema_axis_inside_bounds() {
        let blended = RotationRad::ema_axis(RotationRad::new(0.0), RotationRad::new(0.4), SmoothingAlpha::new(0.5));
        assert!((blended.as_f64() - 0.2).abs() < 1e-9);
    }

    /// `ema_axis` blends through the `+-pi/2` fold: the midpoint of
    /// two near-horizontal axes on opposite fold sides lands on
    /// the horizontal boundary (`-pi/2`) instead of sweeping the
    /// long way around.
    #[test]
    fn ema_axis_blends_across_the_fold() {
        let blended = RotationRad::ema_axis(RotationRad::new(-1.5), RotationRad::new(1.5), SmoothingAlpha::new(0.5));
        assert!((blended.as_f64() + FRAC_PI_2).abs() < 1e-9);
    }
}
