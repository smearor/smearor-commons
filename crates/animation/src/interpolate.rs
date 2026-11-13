use smearor_commons_geometry::Degrees;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Millimeters;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Radians;
use smearor_commons_geometry::RotationRad;
use smearor_commons_geometry::ScaleFactor;
use smearor_commons_geometry::Vec2;

/// A value that can be interpolated between two endpoints along a
/// normalized progress `t` (`0` = `self`, `1` = `target`).
///
/// `t` is not clamped - easing curves like
/// [`crate::EasingFunction::Overshoot`] deliberately extrapolate, and
/// each type decides what extrapolation means: plain scalars just
/// keep going, [`RotationRad`] keeps turning past the target on the
/// shortest arc and settles back.
pub trait Interpolate {
    /// The interpolated value at progress `t` between `self` and
    /// `target`.
    fn interpolate(self, target: Self, t: Fraction) -> Self;
}

impl Interpolate for f64 {
    fn interpolate(self, target: Self, t: Fraction) -> Self {
        self + (target - self) * t
    }
}

impl Interpolate for f32 {
    fn interpolate(self, target: Self, t: Fraction) -> Self {
        (f64::from(self) + (f64::from(target) - f64::from(self)) * t) as f32
    }
}

impl Interpolate for Pixels {
    fn interpolate(self, target: Self, t: Fraction) -> Self {
        Self(self.0 + (target.0 - self.0) * t)
    }
}

impl Interpolate for Millimeters {
    fn interpolate(self, target: Self, t: Fraction) -> Self {
        Self(self.0 + (target.0 - self.0) * t)
    }
}

impl Interpolate for Degrees {
    fn interpolate(self, target: Self, t: Fraction) -> Self {
        Self(self.0 + (target.0 - self.0) * t)
    }
}

/// Linear interpolation of the radian value - `Radians` is a scalar
/// unit, not a rotation; for wrap-aware shortest-arc interpolation
/// use [`RotationRad`].
impl Interpolate for Radians {
    fn interpolate(self, target: Self, t: Fraction) -> Self {
        Self(self.0 + (target.0 - self.0) * t)
    }
}

impl Interpolate for ScaleFactor {
    fn interpolate(self, target: Self, t: Fraction) -> Self {
        Self::lerp(self, target, t)
    }
}

/// Wrap-aware interpolation: the shortest arc between the two
/// angles, normalized into `[0, 2π)` - a rotation animation never
/// sweeps the long way around.
impl Interpolate for RotationRad {
    fn interpolate(self, target: Self, t: Fraction) -> Self {
        Self::ema(self, target, t.into()).normalized()
    }
}

/// Component-wise interpolation - animating a position, size or
/// offset animates both axes on one clock.
impl<U, T: Interpolate + Copy> Interpolate for Vec2<U, T> {
    fn interpolate(self, target: Self, t: Fraction) -> Self {
        Self::new(self.x.interpolate(target.x, t), self.y.interpolate(target.y, t))
    }
}

/// Pair-wise interpolation - two animated values on one clock,
/// e.g. `(RotationRad, ScaleFactor)` for a rotation-zoom composite.
impl<A: Interpolate, B: Interpolate> Interpolate for (A, B) {
    fn interpolate(self, target: Self, t: Fraction) -> Self {
        (self.0.interpolate(target.0, t), self.1.interpolate(target.1, t))
    }
}

#[cfg(test)]
mod tests {
    use super::Interpolate;
    use smearor_commons_geometry::Fraction;
    use smearor_commons_geometry::Pixels;
    use smearor_commons_geometry::RotationRad;
    use smearor_commons_geometry::Vec2;
    use std::f64::consts::PI;

    /// `t` maps `from` and `to` exactly onto the endpoints.
    #[test]
    fn endpoints_are_exact() {
        assert_eq!(2.0_f64.interpolate(10.0, Fraction::new(0.0)), 2.0);
        assert_eq!(2.0_f64.interpolate(10.0, Fraction::new(1.0)), 10.0);
        assert_eq!(2.0_f64.interpolate(10.0, Fraction::new(0.5)), 6.0);
    }

    /// `350° -> 10°` goes forward across zero (20° arc), not -340°
    /// the long way around.
    #[test]
    fn rotation_takes_the_shortest_arc() {
        let from = RotationRad::from_degrees(350.0);
        let to = RotationRad::from_degrees(10.0);
        let mid = from.interpolate(to, Fraction::new(0.5));
        assert!((mid.to_degrees() - 0.0).abs() < 1e-6, "midpoint should be 0°, got {mid}");
    }

    /// Both components of a vector interpolate on the same `t`.
    #[test]
    fn vec2_interpolates_component_wise() {
        let from = Vec2::<Pixels>::new(0.0, 10.0);
        let to = Vec2::<Pixels>::new(100.0, 20.0);
        let mid = from.interpolate(to, Fraction::new(0.5));
        assert_eq!(mid.x, 50.0);
        assert_eq!(mid.y, 15.0);
    }

    /// Pairs interpolate both elements independently.
    #[test]
    fn tuples_interpolate_pair_wise() {
        let from = (0.0_f64, RotationRad::new(0.0));
        let to = (100.0_f64, RotationRad::new(PI));
        let mid = from.interpolate(to, Fraction::new(0.5));
        assert_eq!(mid.0, 50.0);
        assert!((mid.1.as_f64() - PI / 2.0).abs() < 1e-9);
    }
}
