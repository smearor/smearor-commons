use crate::Rect;
use crate::Vec2;
use serde::Deserialize;
use serde::Serialize;
use std::f64::consts::PI;
use std::f64::consts::SQRT_2;
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

    /// The axis-aligned square fully containing the disk
    /// (`center ± radius` on both axes).
    pub fn bounding_box(&self) -> Rect<U, f64> {
        let diameter = self.diameter_f64();
        Rect::centered(self.center, Vec2::new(diameter, diameter))
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

    /// Side of the largest axis-aligned square inscribed in the
    /// circle: its four corners rest on the outline, so
    /// `side = radius * sqrt(2)`.
    pub fn inscribed_square_side(&self) -> f64 {
        self.radius * SQRT_2
    }

    /// The circle circumscribing a centered axis-aligned square of
    /// `side` - the exact inverse of
    /// [`Circle::inscribed_square_side`].
    pub fn with_inscribed_square(center: Vec2<U>, side: f64) -> Self {
        Self::new(center, side / SQRT_2)
    }

    /// Rasterizes the disk into 1-px-high integer rectangles - one
    /// [`Rect<U, i32>`] per covered scanline, in the circle's own
    /// coordinate space.
    ///
    /// Each row uses the chord at its center-nearest edge (the widest
    /// the circle reaches inside that row), so the union covers every
    /// point [`contains`](Self::contains) would hit - an outer
    /// rasterization suitable for rectangle-union regions like
    /// `cairo::Region`.
    pub fn region_rects(&self) -> Vec<Rect<U, i32>> {
        if self.radius <= 0.0 {
            return Vec::new();
        }
        let bounds = self.bounding_box();
        let top = bounds.min.y.floor() as i32;
        // `floor + 1` keeps the row containing the bottom edge inside
        // the range even when it lands exactly on a scanline boundary
        // (`ceil` would exclude it there).
        let bottom = bounds.bottom().floor() as i32 + 1;
        (top..bottom)
            .filter_map(|y| {
                // Distance from the center to the row's nearest edge -
                // zero when the row straddles the center line.
                let nearest = (f64::from(y) - self.center.y).max(self.center.y - f64::from(y + 1)).max(0.0);
                if nearest > self.radius {
                    return None;
                }
                let half = (self.radius * self.radius - nearest * nearest).sqrt().max(0.5);
                let left = (self.center.x - half).floor() as i32;
                // Same edge rule as `bottom`: include the column the
                // right edge lands on when it is exactly integer.
                let right = (self.center.x + half).floor() as i32 + 1;
                Some(Rect::from_xywh(left, y, right - left, 1))
            })
            .collect()
    }

    /// Rasterizes the disk into a cairo [`Region`](cairo::Region) in
    /// the circle's own coordinate space - one
    /// [`RectangleInt`](cairo::RectangleInt) per
    /// [`region_rects`](Self::region_rects) strip. Used e.g. for
    /// circular dead zones inside a wayland surface's input region.
    ///
    /// # Errors
    ///
    /// Propagates [`cairo::Error`]s from `Region` construction.
    #[cfg(feature = "cairo")]
    pub fn region(&self) -> Result<cairo::Region, cairo::Error> {
        let region = cairo::Region::create();
        for rect in self.region_rects() {
            region.union_rectangle(&rect.into())?;
        }
        Ok(region)
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

    /// The bounding box is the square `center ± radius`.
    #[test]
    fn bounding_box_is_circumscribed_square() {
        let circle = Circle::<Pixels>::new(Vec2::new(10.0, 20.0), 5.0);
        let bounds = circle.bounding_box();
        assert_eq!(bounds.min, Vec2::new(5.0, 15.0));
        assert_eq!(bounds.max(), Vec2::new(15.0, 25.0));
        assert_eq!(bounds.center(), Vec2::new(10.0, 20.0));
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

    /// The scanline rasterization covers every point `contains`
    /// reports inside - an outer rasterization, so every integer point
    /// of a contained point's pixel is unioned.
    #[test]
    fn region_rects_cover_contained_points() {
        let circle = Circle::<Pixels>::new(Vec2::new(10.0, 10.0), 5.0);
        let rects = circle.region_rects();
        assert!(!rects.is_empty());
        for dy in -6..=6 {
            for dx in -6..=6 {
                let point = Vec2::new(10.0 + f64::from(dx), 10.0 + f64::from(dy));
                if circle.contains(&point) {
                    let covered = rects.iter().any(|rect| {
                        point.x >= f64::from(rect.min.x)
                            && point.x < f64::from(rect.min.x + rect.size.x)
                            && point.y >= f64::from(rect.min.y)
                            && point.y < f64::from(rect.min.y + rect.size.y)
                    });
                    assert!(covered, "contained point {point:?} not covered");
                }
            }
        }
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

    /// The inscribed square's corners rest on the circle.
    #[test]
    fn inscribed_square_corners_touch_the_circle() {
        let circle = Circle::<()>::new(Vec2::new(0.0, 0.0), 10.0);
        let side = circle.inscribed_square_side();
        // Corner distance: sqrt(2) * side/2 = radius.
        assert!((side * std::f64::consts::FRAC_1_SQRT_2 - 10.0).abs() < 1e-9);
        let back = Circle::<()>::with_inscribed_square(Vec2::new(0.0, 0.0), side);
        assert!((back.radius - 10.0).abs() < 1e-9);
    }
}
