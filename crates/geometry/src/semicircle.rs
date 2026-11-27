use crate::Rect;
use crate::RotationRad;
use crate::Vec2;
use serde::Deserialize;
use serde::Serialize;
use std::hash::Hash;
use std::hash::Hasher;

/// A half-disk ("dome") - the upper half of a circle, bounded below
/// by the diameter edge through [`Semicircle::center`].
///
/// `center` is the midpoint of the flat (diameter) edge - the center
/// of the virtual full circle - and the dome extends to smaller `y`
/// in the usual screen convention (`point.y <= center.y`). Rotate or
/// mirror the coordinate frame for domes opening in other directions.
///
/// `T` defaults to `f64` for domain math. The type is a technical
/// primitive without validation: `radius >= 0` for meaningful
/// geometry, stricter bounds belong to consumer wrappers.
#[derive(Debug, Serialize, Deserialize)]
pub struct Semicircle<U = (), T = f64> {
    /// The center of the virtual full circle - the midpoint of the
    /// flat diameter edge the dome sits on.
    pub center: Vec2<U, T>,
    /// The radius of the virtual full circle.
    pub radius: T,
}

impl<U, T: Copy> Copy for Semicircle<U, T> {}

impl<U, T: Clone> Clone for Semicircle<U, T> {
    fn clone(&self) -> Self {
        Self::new(self.center.clone(), self.radius.clone())
    }
}

impl<U, T: PartialEq> PartialEq for Semicircle<U, T> {
    fn eq(&self, other: &Self) -> bool {
        self.center == other.center && self.radius == other.radius
    }
}

impl<U, T: Eq> Eq for Semicircle<U, T> {}

impl<U, T: Hash> Hash for Semicircle<U, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.center.hash(state);
        self.radius.hash(state);
    }
}

impl<U, T> Semicircle<U, T> {
    /// Creates a dome sitting on the diameter edge through `center`.
    pub const fn new(center: Vec2<U, T>, radius: T) -> Self {
        Self { center, radius }
    }
}

impl<U> Semicircle<U, f64> {
    /// Whether `point` lies in the dome - inside the virtual circle
    /// and on its flat edge's dome side (`point.y <= center.y`,
    /// boundary inclusive).
    pub fn contains(&self, point: &Vec2<U, f64>) -> bool {
        self.center.distance_f64(point) <= self.radius && point.y <= self.center.y
    }

    /// The axis-aligned bounding box: `2r` wide and `r` tall, the
    /// The largest dome centered on the bottom edge of `bounds` -
    /// its radius is `min(width / 2, height)`: half the width or
    /// the full height, whichever is tighter. Inset the bound
    /// beforehand where a margin applies (`bounds.inset(...)`).
    pub fn inscribed_in(bounds: &Rect<U, f64>) -> Self {
        Self::new(Vec2::new(bounds.center().x, bounds.min.y + bounds.height()), (bounds.width() / 2.0).min(bounds.height()))
    }

    /// diameter edge as its bottom side.
    pub fn bounding_box(&self) -> Rect<U, f64> {
        Rect::new(self.center - Vec2::new(self.radius, self.radius), Vec2::new(self.radius * 2.0, self.radius))
    }

    /// How far the outline extends along `direction` measured from
    /// `center` - the support function of the half-disk: `r` for any
    /// direction into the dome half (the arc is part of the full
    /// circle), `r * |cos|` along the flat edge and below it (only
    /// the diameter endpoints bound those directions).
    pub fn support(&self, direction: RotationRad) -> f64 {
        let (sin, cos) = direction.sin_cos();
        if sin < 0.0 { self.radius } else { self.radius * cos.abs() }
    }

    /// Side of the largest axis-aligned square standing on the
    /// diameter edge with both top corners on the arc: `2r / sqrt(5)`.
    pub fn inscribed_square_side(&self) -> f64 {
        self.radius * 2.0 / 5.0_f64.sqrt()
    }

    /// The dome whose inscribed square stands on the diameter edge
    /// through `center` with `side` - the exact inverse of
    /// [`Semicircle::inscribed_square_side`].
    pub fn with_inscribed_square(center: Vec2<U>, side: f64) -> Self {
        Self::new(center, side * 5.0_f64.sqrt() / 2.0)
    }
}

#[cfg(test)]
mod tests {
    use super::Semicircle;
    use crate::Pixels;
    use crate::RotationRad;
    use crate::Vec2;

    /// Points inside the dome, past the flat edge and off the sides.
    #[test]
    fn contains_uses_the_diameter_edge() {
        let dome = Semicircle::<Pixels>::new(Vec2::new(0.0, 0.0), 10.0);
        assert!(dome.contains(&Vec2::new(0.0, 0.0)));
        assert!(dome.contains(&Vec2::new(0.0, -10.0)));
        assert!(dome.contains(&Vec2::new(9.9, -0.5)));
        assert!(!dome.contains(&Vec2::new(0.0, 1.0)));
        assert!(!dome.contains(&Vec2::new(11.0, -0.5)));
    }

    /// The bounding box is `2r x r` with the flat edge at the bottom.
    #[test]
    fn bounding_box_is_wide_and_half_tall() {
        let dome = Semicircle::<()>::new(Vec2::new(50.0, 60.0), 20.0);
        let bounds = dome.bounding_box();
        assert_eq!(bounds.width(), 40.0);
        assert_eq!(bounds.height(), 20.0);
        assert_eq!(bounds.bottom(), 60.0);
    }

    /// The inscribed square and its inverse agree.
    #[test]
    fn inscribed_square_roundtrips() {
        let dome = Semicircle::<()>::new(Vec2::new(0.0, 0.0), 10.0);
        let side = dome.inscribed_square_side();
        // Top corners rest on the arc: (s/2)^2 + s^2 = r^2.
        assert!((side.powi(2) / 4.0 + side.powi(2) - 100.0).abs() < 1e-9);
        let back = Semicircle::<()>::with_inscribed_square(Vec2::new(0.0, 0.0), side);
        assert!((back.radius - 10.0).abs() < 1e-9);
    }

    /// Support reaches `r` upward and `r|cos|` along the flat edge.
    #[test]
    fn support_follows_the_dome_side() {
        let dome = Semicircle::<()>::new(Vec2::new(0.0, 0.0), 10.0);
        assert!((dome.support(RotationRad::new(-std::f64::consts::FRAC_PI_2)) - 10.0).abs() < 1e-9);
        assert!((dome.support(RotationRad::ZERO) - 10.0).abs() < 1e-9);
        assert!(dome.support(RotationRad::QUARTER_TURN).abs() < 1e-9);
    }
}
