use crate::Circle;
use crate::Vec2;
use serde::Deserialize;
use serde::Serialize;
use std::hash::Hash;
use std::hash::Hasher;

/// A ring / annulus - the area between two concentric circles of
/// radii `inner` and `outer` in the coordinate unit `U`.
///
/// `T` defaults to `f64` for domain math. The type is a technical
/// primitive without validation: `0 <= inner <= outer` for
/// meaningful geometry, stricter bounds belong to consumer wrappers.
/// An `inner` of `0` degenerates to a disk.
#[derive(Debug, Serialize, Deserialize)]
pub struct Annulus<U = (), T = f64> {
    /// The shared center of both boundary circles.
    pub center: Vec2<U, T>,
    /// The inner radius - the hole.
    pub inner: T,
    /// The outer radius - the rim.
    pub outer: T,
}

impl<U, T: Copy> Copy for Annulus<U, T> {}

impl<U, T: Clone> Clone for Annulus<U, T> {
    fn clone(&self) -> Self {
        Self::new(self.center.clone(), self.inner.clone(), self.outer.clone())
    }
}

impl<U, T: PartialEq> PartialEq for Annulus<U, T> {
    fn eq(&self, other: &Self) -> bool {
        self.center == other.center && self.inner == other.inner && self.outer == other.outer
    }
}

impl<U, T: Eq> Eq for Annulus<U, T> {}

impl<U, T: Hash> Hash for Annulus<U, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.center.hash(state);
        self.inner.hash(state);
        self.outer.hash(state);
    }
}

impl<U, T> Annulus<U, T> {
    /// Creates a ring from its center and both radii.
    pub const fn new(center: Vec2<U, T>, inner: T, outer: T) -> Self {
        Self { center, inner, outer }
    }
}

impl<U> Annulus<U, f64> {
    /// Whether `point` lies in the ring - both boundaries count as
    /// inside (`inner <= distance <= outer`).
    pub fn contains(&self, point: &Vec2<U, f64>) -> bool {
        let distance = self.center.distance_f64(point);
        distance >= self.inner && distance <= self.outer
    }

    /// The radial thickness `outer - inner`, in the unit `U`.
    pub fn width(&self) -> U
    where
        U: From<f64>,
    {
        U::from(self.width_f64())
    }

    /// The radial thickness, as a plain `f64` - the unit-erased
    /// counterpart for `Annulus<()>` and unit-free math.
    pub fn width_f64(&self) -> f64 {
        self.outer - self.inner
    }

    /// The radius halfway between `inner` and `outer` - the natural
    /// placement line for content centered on the ring, in the
    /// unit `U`.
    pub fn mid_radius(&self) -> U
    where
        U: From<f64>,
    {
        U::from(self.mid_radius_f64())
    }

    /// The middle radius, as a plain `f64` - see
    /// [`Annulus::width_f64`].
    pub fn mid_radius_f64(&self) -> f64 {
        (self.inner + self.outer) / 2.0
    }

    /// The inner boundary circle.
    pub fn inner_circle(&self) -> Circle<U, f64> {
        Circle::new(self.center, self.inner)
    }

    /// The outer boundary circle.
    pub fn outer_circle(&self) -> Circle<U, f64> {
        Circle::new(self.center, self.outer)
    }
}

#[cfg(test)]
mod tests {
    use super::Annulus;
    use crate::Pixels;
    use crate::Vec2;

    /// Points in the hole, on the rim and in the ring.
    #[test]
    fn contains_includes_both_boundaries() {
        let ring = Annulus::<Pixels>::new(Vec2::new(0.0, 0.0), 10.0, 20.0);
        assert!(!ring.contains(&Vec2::new(0.0, 0.0)));
        assert!(ring.contains(&Vec2::new(10.0, 0.0)));
        assert!(ring.contains(&Vec2::new(15.0, 0.0)));
        assert!(ring.contains(&Vec2::new(0.0, 20.0)));
        assert!(!ring.contains(&Vec2::new(0.0, 21.0)));
    }

    /// `inner == 0` degenerates to the full disk.
    #[test]
    fn zero_inner_is_a_disk() {
        let disk = Annulus::<()>::new(Vec2::new(0.0, 0.0), 0.0, 5.0);
        assert!(disk.contains(&Vec2::new(0.0, 0.0)));
        assert_eq!(disk.width_f64(), 5.0);
        assert_eq!(disk.mid_radius_f64(), 2.5);
    }

    /// The boundary circles carry the ring's center and radii.
    #[test]
    fn boundary_circles() {
        let ring = Annulus::<()>::new(Vec2::new(3.0, 4.0), 1.0, 2.0);
        assert_eq!(ring.inner_circle().radius, 1.0);
        assert_eq!(ring.outer_circle().radius, 2.0);
        assert_eq!(ring.outer_circle().center, Vec2::new(3.0, 4.0));
    }
}
