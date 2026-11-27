use crate::Polygon;
use crate::PolygonOrientation;
use crate::Rect;
use crate::RotationRad;
use crate::Vec2;
use serde::Deserialize;
use serde::Serialize;
use std::f64::consts::FRAC_PI_4;
use std::f64::consts::FRAC_PI_8;
use std::hash::Hash;
use std::hash::Hasher;

/// `cos(22.5°) ≈ 0.9239` - the inradius (apothem) of a regular
/// octagon with circumradius `1`.
const COS_22_5: f64 = 0.923_879_532_511_286_7;

/// `sin(22.5°) ≈ 0.3827` - half the edge length of a regular
/// octagon with circumradius `1`.
const SIN_22_5: f64 = 0.382_683_432_365_089_8;

/// `sqrt(2)` - the diagonal of the unit square.
const SQRT_2: f64 = std::f64::consts::SQRT_2;

/// A regular octagon tagged with its coordinate unit `U` and
/// numeric component type `T`.
///
/// A regular octagon is fully determined by a single extent: the
/// **circumradius** (center to vertex), the **inradius** (center
/// to edge, also called *apothem*, `circumradius * cos(22.5°)`)
/// and the **edge length** (`2 * circumradius * sin(22.5°)`). Use
/// [`Self::with_inradius`] or [`Self::with_side_length`] when the
/// octagon is specified by one of the other extents.
///
/// `T` defaults to `f64` for domain math. The type is a technical
/// primitive without validation: `circumradius` should be
/// non-negative, stricter bounds belong to consumer wrappers.
#[derive(Debug, Serialize, Deserialize)]
pub struct Octagon<U = (), T = f64> {
    /// The center point.
    pub center: Vec2<U, T>,
    /// The circumradius - distance from `center` to each vertex.
    pub circumradius: T,
    /// Whether the octagon has a horizontal top edge
    /// ([`PolygonOrientation::FlatTop`]) or a vertex on top
    /// ([`PolygonOrientation::PointyTop`]).
    pub orientation: PolygonOrientation,
}

impl<U, T: Copy> Copy for Octagon<U, T> {}

impl<U, T: Clone> Clone for Octagon<U, T> {
    fn clone(&self) -> Self {
        Self::new(self.center.clone(), self.circumradius.clone(), self.orientation)
    }
}

impl<U, T: PartialEq> PartialEq for Octagon<U, T> {
    fn eq(&self, other: &Self) -> bool {
        self.center == other.center && self.circumradius == other.circumradius && self.orientation == other.orientation
    }
}

impl<U, T: Eq> Eq for Octagon<U, T> {}

impl<U, T: Hash> Hash for Octagon<U, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.center.hash(state);
        self.circumradius.hash(state);
        self.orientation.hash(state);
    }
}

impl<U, T> Octagon<U, T> {
    /// Creates an octagon from its center, circumradius and
    /// orientation.
    pub const fn new(center: Vec2<U, T>, circumradius: T, orientation: PolygonOrientation) -> Self {
        Self {
            center,
            circumradius,
            orientation,
        }
    }

    /// Creates a [`PolygonOrientation::FlatTop`] octagon.
    pub const fn flat_top(center: Vec2<U, T>, circumradius: T) -> Self {
        Self::new(center, circumradius, PolygonOrientation::FlatTop)
    }

    /// Creates a [`PolygonOrientation::PointyTop`] octagon.
    pub const fn pointy_top(center: Vec2<U, T>, circumradius: T) -> Self {
        Self::new(center, circumradius, PolygonOrientation::PointyTop)
    }
}

impl<U> Octagon<U, f64> {
    /// Creates an octagon from its center, edge length and
    /// orientation. The edge length of a regular octagon is
    /// `2 * circumradius * sin(22.5°)`.
    pub fn with_side_length(center: Vec2<U, f64>, side_length: f64, orientation: PolygonOrientation) -> Self {
        Self::new(center, side_length / (2.0 * SIN_22_5), orientation)
    }

    /// Creates an octagon from its center, inradius and
    /// orientation. The inradius (apothem, center to edge) is
    /// `circumradius * cos(22.5°)`.
    pub fn with_inradius(center: Vec2<U, f64>, inradius: f64, orientation: PolygonOrientation) -> Self {
        Self::new(center, inradius / COS_22_5, orientation)
    }

    /// The edge length - `2 * circumradius * sin(22.5°)`.
    pub fn side_length(&self) -> f64 {
        self.circumradius * 2.0 * SIN_22_5
    }

    /// The inradius (apothem) - distance from `center` to each
    /// edge, `circumradius * cos(22.5°)`.
    pub fn inradius(&self) -> f64 {
        self.circumradius * COS_22_5
    }

    /// The eight vertices in counter-clockwise order starting from
    /// angle `22.5°` (`FlatTop`) resp. `0°` (`PointyTop`) in steps
    /// of `45°`.
    pub fn vertices(&self) -> [Vec2<U, f64>; 8] {
        let offset = match self.orientation {
            PolygonOrientation::FlatTop => FRAC_PI_8,
            PolygonOrientation::PointyTop => 0.0,
        };
        [0, 1, 2, 3, 4, 5, 6, 7].map(|i| self.center + Vec2::from_polar(self.circumradius, RotationRad::new(offset + f64::from(i) * FRAC_PI_4)))
    }

    /// The octagon as a generic closed [`Polygon`] - the bridge to
    /// ring algorithms like `segments` and `offset`.
    pub fn to_polygon(&self) -> Polygon<U> {
        Polygon::new(self.vertices().to_vec())
    }

    /// Whether `point` lies inside the octagon - the boundary
    /// counts as inside.
    ///
    /// A regular octagon is the intersection of four strips
    /// perpendicular to its edge normals: `x` and `y` plus the two
    /// `±45°` diagonals for `FlatTop`, the `22.5°`-rotated
    /// counterparts for `PointyTop`.
    pub fn contains(&self, point: &Vec2<U, f64>) -> bool {
        let (dx, dy) = (point.x - self.center.x, point.y - self.center.y);
        let inradius = self.inradius();
        match self.orientation {
            PolygonOrientation::FlatTop => {
                dx.abs() <= inradius && dy.abs() <= inradius && (dx + dy).abs() <= inradius * SQRT_2 && (dx - dy).abs() <= inradius * SQRT_2
            }
            PolygonOrientation::PointyTop => {
                (COS_22_5 * dx + SIN_22_5 * dy).abs() <= inradius
                    && (SIN_22_5 * dx + COS_22_5 * dy).abs() <= inradius
                    && (SIN_22_5 * dx - COS_22_5 * dy).abs() <= inradius
                    && (COS_22_5 * dx - SIN_22_5 * dy).abs() <= inradius
            }
        }
    }

    /// The axis-aligned bounding box: a square `2*inradius x
    /// 2*inradius` for `FlatTop`, `2R x 2R` for `PointyTop`.
    pub fn bounding_box(&self) -> Rect<U, f64> {
        let half = match self.orientation {
            PolygonOrientation::FlatTop => self.inradius(),
            PolygonOrientation::PointyTop => self.circumradius,
        };
        Rect::new(self.center - Vec2::new(half, half), Vec2::new(half * 2.0, half * 2.0))
    }

    /// Side of the largest centered axis-aligned square inscribed
    /// in the octagon, expressed in the inradius.
    ///
    /// `FlatTop` keeps a square's corners on the four diagonal
    /// edges (`side = inradius * sqrt(2)`); `PointyTop` puts the
    /// corners against the 22.5-degree-slanted edges and fits the
    /// larger `2 * inradius / (cos(22.5deg) + sin(22.5deg))`.
    pub fn inscribed_square_side(&self) -> f64 {
        match self.orientation {
            PolygonOrientation::FlatTop => self.inradius() * SQRT_2,
            PolygonOrientation::PointyTop => 2.0 * self.inradius() / (COS_22_5 + SIN_22_5),
        }
    }

    /// The octagon whose inscribed square has `side` - the exact
    /// inverse of [`Octagon::inscribed_square_side`].
    pub fn with_inscribed_square(center: Vec2<U>, side: f64, orientation: PolygonOrientation) -> Self {
        let inradius = match orientation {
            PolygonOrientation::FlatTop => side / SQRT_2,
            PolygonOrientation::PointyTop => side * (COS_22_5 + SIN_22_5) / 2.0,
        };
        Self::with_inradius(center, inradius, orientation)
    }

    /// How far the outline extends along `direction` measured from
    /// `center` - the exact support function of the convex octagon:
    /// the largest projection of any vertex.
    pub fn support(&self, direction: RotationRad) -> f64 {
        let (sin, cos) = direction.sin_cos();
        let ray = Vec2::<U>::new(cos, sin);
        self.vertices().iter().map(|vertex| (*vertex - self.center).dot(&ray)).fold(0.0, f64::max)
    }
}

#[cfg(test)]
mod tests {
    use super::COS_22_5;
    use super::Octagon;
    use super::SIN_22_5;
    use super::SQRT_2;
    use crate::PolygonOrientation;
    use crate::RotationRad;
    use crate::Vec2;

    /// The three extent specifications produce the same octagon.
    #[test]
    fn circumradius_inradius_and_side_length_agree() {
        let center = Vec2::<()>::new(10.0, 20.0);
        let by_radius = Octagon::flat_top(center, 100.0);
        let by_side = Octagon::with_side_length(center, 200.0 * SIN_22_5, PolygonOrientation::FlatTop);
        let by_inradius = Octagon::with_inradius(center, 100.0 * COS_22_5, PolygonOrientation::FlatTop);

        assert!((by_side.circumradius - 100.0).abs() < 1e-9);
        assert!((by_inradius.circumradius - 100.0).abs() < 1e-9);
        assert!((by_radius.inradius() - 100.0 * COS_22_5).abs() < 1e-9);
        assert!((by_radius.side_length() - 200.0 * SIN_22_5).abs() < 1e-9);
    }

    /// `FlatTop` vertices sit on the circumcircle at `22.5°`,
    /// `67.5°`, ... and `PointyTop` vertices on `0°`, `45°`, ...
    #[test]
    fn vertices_follow_the_orientation() {
        let flat = Octagon::flat_top(Vec2::<()>::new(0.0, 0.0), 100.0).vertices();
        assert!((flat[0].x - 100.0 * COS_22_5).abs() < 1e-9 && (flat[0].y - 100.0 * SIN_22_5).abs() < 1e-9);
        let pointy = Octagon::pointy_top(Vec2::<()>::new(0.0, 0.0), 100.0).vertices();
        assert!((pointy[0].x - 100.0).abs() < 1e-9 && pointy[0].y.abs() < 1e-9);
        assert_eq!(flat.len(), 8);
    }

    /// The flat-top bounding box is the square `2*inradius` wide.
    #[test]
    fn flat_top_bounding_box_is_a_square() {
        let octagon = Octagon::flat_top(Vec2::<()>::new(0.0, 0.0), 100.0);
        let bbox = octagon.bounding_box();
        assert!((bbox.size.x - bbox.size.y).abs() < 1e-9);
        assert!((bbox.size.x - 2.0 * octagon.inradius()).abs() < 1e-9);
    }

    /// The four strips cut the flat-top octagon's corners.
    #[test]
    fn contains_the_strips() {
        let octagon = Octagon::flat_top(Vec2::<()>::new(0.0, 0.0), 100.0);
        let apothem = octagon.inradius();
        // The bbox corner (apothem, apothem) lies outside - the
        // diagonal edge cuts it.
        assert!(!octagon.contains(&Vec2::new(apothem, apothem)));
        // The top edge's midpoint and the right edge's midpoint are
        // on the boundary - inside.
        assert!(octagon.contains(&Vec2::new(0.0, apothem)));
        assert!(octagon.contains(&Vec2::new(apothem, 0.0)));
        // A diagonal-edge midpoint: the slant between the right and
        // the top-right vertex centers at 45°.
        assert!(octagon.contains(&Vec2::new(apothem / SQRT_2, apothem / SQRT_2)));
        // The center is inside, a point past the vertex is outside.
        assert!(octagon.contains(&Vec2::new(0.0, 0.0)));
        assert!(!octagon.contains(&Vec2::new(0.0, 101.0)));
    }

    /// `support` matches the largest vertex projection.
    #[test]
    fn support_reaches_the_outline() {
        let octagon = Octagon::flat_top(Vec2::<()>::new(0.0, 0.0), 100.0);
        // Along 0° and 45° an edge is perpendicular to the ray - the
        // support is the apothem; along 22.5° a vertex sits on the
        // ray - the support is the circumradius.
        assert!((octagon.support(RotationRad::new(0.0)) - 100.0 * COS_22_5).abs() < 1e-9);
        assert!((octagon.support(RotationRad::from_degrees(45.0)) - 100.0 * COS_22_5).abs() < 1e-9);
        assert!((octagon.support(RotationRad::from_degrees(22.5)) - 100.0).abs() < 1e-9);
    }

    /// `to_polygon` yields the same eight vertices.
    #[test]
    fn to_polygon_yields_the_ring() {
        let octagon = Octagon::flat_top(Vec2::<()>::new(1.0, 2.0), 50.0);
        let polygon = octagon.to_polygon();
        assert_eq!(polygon.0.len(), 8);
        assert_eq!(polygon.0[0], octagon.vertices()[0]);
    }

    /// FlatTop and PointyTop inscribed squares both rest their
    /// corners on the outline and invert via `with_inscribed_square`.
    #[test]
    fn inscribed_square_roundtrips() {
        for orientation in [PolygonOrientation::FlatTop, PolygonOrientation::PointyTop] {
            let octagon = Octagon::with_inradius(Vec2::<()>::new(0.0, 0.0), 50.0, orientation);
            let side = octagon.inscribed_square_side();
            let back = Octagon::with_inscribed_square(Vec2::<()>::new(0.0, 0.0), side, orientation);
            assert!((back.inradius() - 50.0).abs() < 1e-9, "{orientation:?}");
            let half = side / 2.0;
            assert!(octagon.contains(&Vec2::new(half - 1e-9, half - 1e-9)), "{orientation:?}");
        }
    }
}
