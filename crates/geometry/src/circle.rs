use crate::Vec2;
use serde::Deserialize;
use serde::Serialize;
use std::f64::consts::PI;
use std::hash::Hash;
use std::hash::Hasher;

/// A closed disk / circle tagged with its coordinate unit `U` and
/// numeric component type `T`.
///
/// `T` defaults to `f64` for domain math. The type is a technical
/// primitive without validation: `radius` should be non-negative,
/// stricter bounds belong to consumer wrappers.
#[derive(Debug, Serialize, Deserialize)]
pub struct Circle<U = (), T = f64> {
    /// The center point.
    pub center: Vec2<U, T>,
    /// The radius.
    pub radius: T,
}

impl<U, T: Copy> Copy for Circle<U, T> {}

impl<U, T: Clone> Clone for Circle<U, T> {
    fn clone(&self) -> Self {
        Self::new(self.center.clone(), self.radius.clone())
    }
}

impl<U, T: PartialEq> PartialEq for Circle<U, T> {
    fn eq(&self, other: &Self) -> bool {
        self.center == other.center && self.radius == other.radius
    }
}

impl<U, T: Eq> Eq for Circle<U, T> {}

impl<U, T: Hash> Hash for Circle<U, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.center.hash(state);
        self.radius.hash(state);
    }
}

impl<U, T> Circle<U, T> {
    /// Creates a circle from its center and radius.
    pub const fn new(center: Vec2<U, T>, radius: T) -> Self {
        Self { center, radius }
    }
}

impl<U> Circle<U, f64> {
    /// Whether `point` lies inside the disk - the boundary counts
    /// as inside (`distance <= radius`).
    pub fn contains(&self, point: &Vec2<U, f64>) -> bool {
        self.center.distance_f64(point) <= self.radius
    }

    /// Whether the two disks share interior points - boundary touch
    /// alone does not count (`distance < r1 + r2`).
    pub fn overlaps(&self, other: &Self) -> bool {
        self.center.distance_f64(&other.center) < self.radius + other.radius
    }

    /// The radius in the unit `U` - the unit-carrying counterpart of
    /// the raw `radius` field, like [`Vec2::x_unit`].
    pub fn radius_unit(&self) -> U
    where
        U: From<f64>,
    {
        U::from(self.radius)
    }

    /// `2 * radius`, in the unit `U`.
    pub fn diameter(&self) -> U
    where
        U: From<f64>,
    {
        U::from(self.diameter_f64())
    }

    /// `2 * radius`, as a plain `f64` - the unit-erased counterpart
    /// for `Circle<()>` and unit-free math.
    pub fn diameter_f64(&self) -> f64 {
        self.radius * 2.0
    }

    /// `PI * radius^2` - the caller picks the area type `P` (like
    /// [`Vec2::dot`]'s product type): [`crate::SquareMillimeters`] for
    /// `Circle<Millimeters>`, `f64` for `Circle<()>` and unit-free math.
    pub fn area<P: From<f64>>(&self) -> P {
        P::from(PI * self.radius * self.radius)
    }

    /// `2 * PI * radius`, in the unit `U`.
    pub fn circumference(&self) -> U
    where
        U: From<f64>,
    {
        U::from(self.circumference_f64())
    }

    /// `2 * PI * radius`, as a plain `f64` - see
    /// [`Circle::diameter_f64`].
    pub fn circumference_f64(&self) -> f64 {
        2.0 * PI * self.radius
    }
}

#[cfg(test)]
mod tests {
    use super::Circle;
    use crate::Pixels;
    use crate::SquareMillimeters;
    use crate::Vec2;
    use std::f64::consts::FRAC_1_SQRT_2;
    use std::f64::consts::PI;

    /// Points inside, on and outside the disk.
    #[test]
    fn contains_includes_boundary() {
        let circle = Circle::<Pixels>::new(Vec2::new(10.0, 20.0), 5.0);
        assert!(circle.contains(&Vec2::new(10.0, 20.0)));
        assert!(circle.contains(&Vec2::new(15.0, 20.0)));
        assert!(!circle.contains(&Vec2::new(16.0, 20.0)));
    }

    /// Strict interior overlap: touching boundaries do not count.
    #[test]
    fn overlaps_is_strict() {
        let a = Circle::<()>::new(Vec2::new(0.0, 0.0), 1.0);
        let touching = Circle::<()>::new(Vec2::new(2.0, 0.0), 1.0);
        let overlapping = Circle::<()>::new(Vec2::new(1.9, 0.0), 1.0);
        let separate = Circle::<()>::new(Vec2::new(3.0, 0.0), 1.0);
        assert!(!a.overlaps(&touching));
        assert!(a.overlaps(&overlapping));
        assert!(!a.overlaps(&separate));
        assert!(a.overlaps(&a));
    }

    /// The classic measures follow the radius - plain `f64` on the
    /// unit-erased `Circle<()>`.
    #[test]
    fn circle_measures() {
        let circle = Circle::<()>::new(Vec2::new(0.0, 0.0), 2.0);
        assert_eq!(circle.diameter_f64(), 4.0);
        let area: f64 = circle.area();
        assert!((area - 4.0 * PI).abs() < 1e-9);
        assert!((circle.circumference_f64() - 4.0 * PI).abs() < 1e-9);
        let unit = Circle::<()>::new(Vec2::new(0.0, 0.0), FRAC_1_SQRT_2);
        let unit_area: f64 = unit.area();
        assert!((unit_area - PI / 2.0).abs() < 1e-9);
    }

    /// Measures carry the coordinate unit when `U` is a unit newtype.
    #[test]
    fn measures_carry_the_unit() {
        let circle = Circle::<Pixels>::new(Vec2::new(0.0, 0.0), 2.0);
        assert_eq!(circle.radius_unit(), Pixels::new(2.0));
        assert_eq!(circle.diameter(), Pixels::new(4.0));
        assert_eq!(circle.circumference(), Pixels::new(4.0 * PI));
        let area = circle.area::<SquareMillimeters>();
        assert!((area.as_f64() - 4.0 * PI).abs() < 1e-9);
    }
}
