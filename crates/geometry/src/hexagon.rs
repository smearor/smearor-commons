use crate::Polygon;
use crate::PolygonOrientation;
use crate::Rect;
use crate::RotationRad;
use crate::Size;
use crate::Vec2;
use serde::Deserialize;
use serde::Serialize;
use std::f64::consts::FRAC_PI_3;
use std::f64::consts::FRAC_PI_6;
use std::hash::Hash;
use std::hash::Hasher;

/// `sin(60°) = sqrt(3) / 2` - the inradius (apothem) of a regular
/// hexagon with circumradius `1`.
const SIN_60: f64 = 0.866_025_403_784_438_6;

/// A regular hexagon tagged with its coordinate unit `U` and
/// numeric component type `T`.
///
/// A regular hexagon is fully determined by a single extent: the
/// **circumradius** (center to vertex) equals its edge length, and
/// the **inradius** (center to edge, also called *apothem*) is
/// `circumradius * sin(60°)`. Use [`Self::with_inradius`] or
/// [`Self::with_side_length`] when the hexagon is specified by one
/// of the other extents.
///
/// `T` defaults to `f64` for domain math. The type is a technical
/// primitive without validation: `circumradius` should be
/// non-negative, stricter bounds belong to consumer wrappers.
#[derive(Debug, Serialize, Deserialize)]
pub struct Hexagon<U = (), T = f64> {
    /// The center point.
    pub center: Vec2<U, T>,
    /// The circumradius - distance from `center` to each vertex.
    /// For a regular hexagon this equals the edge length.
    pub circumradius: T,
    /// Whether the hexagon has a horizontal top edge
    /// ([`PolygonOrientation::FlatTop`]) or a vertex on top
    /// ([`PolygonOrientation::PointyTop`]).
    pub orientation: PolygonOrientation,
}

impl<U, T: Copy> Copy for Hexagon<U, T> {}

impl<U, T: Clone> Clone for Hexagon<U, T> {
    fn clone(&self) -> Self {
        Self::new(self.center.clone(), self.circumradius.clone(), self.orientation)
    }
}

impl<U, T: PartialEq> PartialEq for Hexagon<U, T> {
    fn eq(&self, other: &Self) -> bool {
        self.center == other.center && self.circumradius == other.circumradius && self.orientation == other.orientation
    }
}

impl<U, T: Eq> Eq for Hexagon<U, T> {}

impl<U, T: Hash> Hash for Hexagon<U, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.center.hash(state);
        self.circumradius.hash(state);
        self.orientation.hash(state);
    }
}

impl<U, T> Hexagon<U, T> {
    /// Creates a hexagon from its center, circumradius and
    /// orientation.
    pub const fn new(center: Vec2<U, T>, circumradius: T, orientation: PolygonOrientation) -> Self {
        Self {
            center,
            circumradius,
            orientation,
        }
    }

    /// Creates a [`PolygonOrientation::FlatTop`] hexagon.
    pub const fn flat_top(center: Vec2<U, T>, circumradius: T) -> Self {
        Self::new(center, circumradius, PolygonOrientation::FlatTop)
    }

    /// Creates a [`PolygonOrientation::PointyTop`] hexagon.
    pub const fn pointy_top(center: Vec2<U, T>, circumradius: T) -> Self {
        Self::new(center, circumradius, PolygonOrientation::PointyTop)
    }
}

impl<U> Hexagon<U, f64> {
    /// Creates a hexagon from its center, edge length and
    /// orientation. The edge length of a regular hexagon equals
    /// its circumradius.
    pub const fn with_side_length(center: Vec2<U, f64>, side_length: f64, orientation: PolygonOrientation) -> Self {
        Self::new(center, side_length, orientation)
    }

    /// Creates a hexagon from its center, inradius and
    /// orientation. The inradius (apothem, center to edge) is
    /// `circumradius * sin(60°)`.
    pub fn with_inradius(center: Vec2<U, f64>, inradius: f64, orientation: PolygonOrientation) -> Self {
        Self::new(center, inradius / SIN_60, orientation)
    }

    /// The edge length - for a regular hexagon equal to the
    /// circumradius.
    pub fn side_length(&self) -> f64 {
        self.circumradius
    }

    /// The inradius (apothem) - distance from `center` to each
    /// edge, `circumradius * sin(60°)`.
    pub fn inradius(&self) -> f64 {
        self.circumradius * SIN_60
    }

    /// The six vertices in counter-clockwise order starting from
    /// angle `0°` (`FlatTop`) resp. `30°` (`PointyTop`) in steps of
    /// `60°`.
    pub fn vertices(&self) -> [Vec2<U, f64>; 6] {
        let offset = match self.orientation {
            PolygonOrientation::FlatTop => 0.0,
            PolygonOrientation::PointyTop => FRAC_PI_6,
        };
        [0, 1, 2, 3, 4, 5].map(|i| self.center + Vec2::from_polar(self.circumradius, RotationRad::new(offset + f64::from(i) * FRAC_PI_3)))
    }

    /// The hexagon as a generic closed [`Polygon`] - the bridge to
    /// ring algorithms like `segments` and `offset`.
    pub fn to_polygon(&self) -> Polygon<U> {
        Polygon::new(self.vertices().to_vec())
    }

    /// Whether `point` lies inside the hexagon - the boundary
    /// counts as inside.
    ///
    /// A regular hexagon is the intersection of three strips
    /// perpendicular to its three edge normals: `y` (`FlatTop`)
    /// resp. `x` (`PointyTop`) and the two `±60°` axes.
    pub fn contains(&self, point: &Vec2<U, f64>) -> bool {
        let (dx, dy) = (point.x - self.center.x, point.y - self.center.y);
        let inradius = self.inradius();
        match self.orientation {
            PolygonOrientation::FlatTop => dy.abs() <= inradius && (SIN_60 * dx + 0.5 * dy).abs() <= inradius && (SIN_60 * dx - 0.5 * dy).abs() <= inradius,
            PolygonOrientation::PointyTop => dx.abs() <= inradius && (0.5 * dx + SIN_60 * dy).abs() <= inradius && (0.5 * dx - SIN_60 * dy).abs() <= inradius,
        }
    }

    /// The axis-aligned bounding box: `2R x sqrt(3)*R` for
    /// `FlatTop`, `sqrt(3)*R x 2R` for `PointyTop`.
    pub fn bounding_box(&self) -> Rect<U, f64> {
        let (half_width, half_height) = match self.orientation {
            PolygonOrientation::FlatTop => (self.circumradius, self.inradius()),
            PolygonOrientation::PointyTop => (self.inradius(), self.circumradius),
        };
        Rect::new(self.center - Vec2::new(half_width, half_height), Vec2::new(half_width * 2.0, half_height * 2.0))
    }

    /// Side of the largest centered axis-aligned square inscribed
    /// in the hexagon: `(3 - sqrt(3)) * circumradius`, its corners
    /// resting on the four slanted edges. The factor is the same
    /// for both orientations - a square turned a quarter turn is a
    /// square.
    pub fn inscribed_square_side(&self) -> f64 {
        Self::inscribed_square_side_for_circumradius(self.circumradius)
    }

    /// Side of the largest centered axis-aligned square inscribed
    /// in a hexagon of `circumradius` -
    /// [`Hexagon::inscribed_square_side`] without constructing the
    /// hexagon.
    pub fn inscribed_square_side_for_circumradius(circumradius: f64) -> f64 {
        circumradius * (3.0 - 2.0 * SIN_60)
    }

    /// The hexagon whose inscribed square has `side` - the exact
    /// inverse of [`Hexagon::inscribed_square_side`].
    pub fn with_inscribed_square(center: Vec2<U>, side: f64, orientation: PolygonOrientation) -> Self {
        Self::new(center, Self::circumradius_for_inscribed_square(side), orientation)
    }

    /// The circumradius of the hexagon whose inscribed square has
    /// `side` - [`Hexagon::with_inscribed_square`] without
    /// constructing the hexagon.
    pub fn circumradius_for_inscribed_square(side: f64) -> f64 {
        side / (3.0 - 2.0 * SIN_60)
    }

    /// The axis-aligned bounding size of the hexagon of
    /// `circumradius`: `2R x sqrt(3) * R` for `FlatTop`, transposed
    /// for `PointyTop`.
    pub fn bounding_size_for_circumradius(circumradius: f64, orientation: PolygonOrientation) -> Size<f64> {
        let (width, height) = match orientation {
            PolygonOrientation::FlatTop => (2.0 * circumradius, 2.0 * circumradius * SIN_60),
            PolygonOrientation::PointyTop => (2.0 * circumradius * SIN_60, 2.0 * circumradius),
        };
        Size::new(width, height)
    }

    /// The circumradius of the hexagon whose axis-aligned bounding
    /// size fits `size` - the exact inverse of
    /// [`Hexagon::bounding_size_for_circumradius`]. A `0.0`
    /// component leaves that axis unconstrained.
    pub fn circumradius_for_bounding_size(size: Size<f64>, orientation: PolygonOrientation) -> f64 {
        match orientation {
            PolygonOrientation::FlatTop => (size.x / 2.0).max(size.y / (2.0 * SIN_60)),
            PolygonOrientation::PointyTop => (size.x / (2.0 * SIN_60)).max(size.y / 2.0),
        }
    }

    /// How far the outline extends along `direction` measured from
    /// `center` - the exact support function of the convex hexagon:
    /// the largest projection of any vertex.
    pub fn support(&self, direction: RotationRad) -> f64 {
        let (sin, cos) = direction.sin_cos();
        let ray = Vec2::<U>::new(cos, sin);
        self.vertices().iter().map(|vertex| (*vertex - self.center).dot(&ray)).fold(0.0, f64::max)
    }
}

#[cfg(test)]
mod tests {
    use super::Hexagon;
    use crate::PolygonOrientation;
    use crate::RotationRad;
    use crate::Vec2;

    /// The three extent specifications produce the same hexagon.
    #[test]
    fn circumradius_inradius_and_side_length_agree() {
        let center = Vec2::<()>::new(10.0, 20.0);
        let by_radius = Hexagon::flat_top(center, 100.0);
        let by_side = Hexagon::with_side_length(center, 100.0, PolygonOrientation::FlatTop);
        let by_inradius = Hexagon::with_inradius(center, 100.0 * super::SIN_60, PolygonOrientation::FlatTop);

        assert_eq!(by_radius, by_side);
        assert!((by_inradius.circumradius - 100.0).abs() < 1e-9);
        assert!((by_radius.inradius() - 100.0 * super::SIN_60).abs() < 1e-9);
        assert!((by_radius.side_length() - 100.0).abs() < 1e-9);
    }

    /// `FlatTop` vertices sit on the circumcircle at `0°`, `60°`, ...
    /// and `PointyTop` vertices sit on it offset by `30°`.
    #[test]
    fn vertices_follow_the_orientation() {
        let flat = Hexagon::<()>::flat_top(Vec2::new(0.0, 0.0), 10.0).vertices();
        assert!((flat[0].x - 10.0).abs() < 1e-9 && flat[0].y.abs() < 1e-9, "flat-top vertex at 0 degrees");
        assert!(flat.iter().all(|v| v.magnitude_f64() - 10.0 < 1e-9), "all on the circumcircle");

        let pointy = Hexagon::<()>::pointy_top(Vec2::new(0.0, 0.0), 10.0).vertices();
        assert!((pointy[1].x.abs() < 1e-9) && (pointy[1].y - 10.0).abs() < 1e-9, "pointy-top vertex at 90 degrees");
        assert!(pointy.iter().all(|v| v.magnitude_f64() - 10.0 < 1e-9), "all on the circumcircle");
    }

    /// `contains` accepts center, edge midpoints and vertices and
    /// rejects points outside the strips both orientations span.
    #[test]
    fn contains_the_strips() {
        let hex = Hexagon::<()>::flat_top(Vec2::new(0.0, 0.0), 10.0);
        assert!(hex.contains(&Vec2::new(0.0, 0.0)));
        assert!(hex.contains(&Vec2::new(10.0, 0.0)), "vertex counts as inside");
        assert!(!hex.contains(&Vec2::new(10.1, 0.0)));
        let inradius = hex.inradius();
        assert!(!hex.contains(&Vec2::new(0.0, inradius + 0.1)), "above the top edge");
        assert!(hex.contains(&Vec2::new(0.0, inradius)), "top edge midpoint counts as inside");
        // The upper-right edge at x = 7 sits at y = 5.196:
        // (7, 4) inside, (7, 5.5) outside.
        assert!(hex.contains(&Vec2::new(7.0, 4.0)));
        assert!(!hex.contains(&Vec2::new(7.0, 5.5)));
        assert!(!hex.contains(&Vec2::new(9.0, inradius - 1.0)));

        let pointy = Hexagon::<()>::pointy_top(Vec2::new(0.0, 0.0), 10.0);
        assert!(pointy.contains(&Vec2::new(0.0, 10.0)), "top vertex inside");
        assert!(!pointy.contains(&Vec2::new(0.0, 10.1)));
        assert!(pointy.contains(&Vec2::new(pointy.inradius(), 0.0)), "right edge midpoint inside");
    }

    /// `bounding_box` covers all vertices and matches the flat/pointy
    /// extents.
    #[test]
    fn bounding_box_covers_the_vertices() {
        let flat = Hexagon::<()>::flat_top(Vec2::new(0.0, 0.0), 10.0);
        let bbox = flat.bounding_box();
        assert!((bbox.size.x - 20.0).abs() < 1e-9);
        assert!((bbox.size.y - 20.0 * super::SIN_60).abs() < 1e-9);
        assert!(
            flat.vertices()
                .iter()
                .all(|v| v.x >= bbox.min.x - 1e-9 && v.x <= bbox.right() + 1e-9 && v.y >= bbox.min.y - 1e-9 && v.y <= bbox.bottom() + 1e-9),
            "vertices inside the bbox (float boundary tolerance)"
        );

        let pointy = Hexagon::<()>::pointy_top(Vec2::new(0.0, 0.0), 10.0).bounding_box();
        assert!((pointy.size.x - 20.0 * super::SIN_60).abs() < 1e-9);
        assert!((pointy.size.y - 20.0).abs() < 1e-9);
    }

    /// `support` equals the circumradius along vertex directions and
    /// the inradius along edge normals.
    #[test]
    fn support_reaches_the_outline() {
        let flat = Hexagon::<()>::flat_top(Vec2::new(0.0, 0.0), 10.0);
        assert!((flat.support(RotationRad::new(0.0)) - 10.0).abs() < 1e-9, "vertex direction");
        assert!((flat.support(RotationRad::from_degrees(90.0)) - flat.inradius()).abs() < 1e-9, "edge normal");
        assert!((flat.support(RotationRad::from_degrees(30.0)) - flat.inradius()).abs() < 1e-9, "upper-right edge normal");
    }

    /// `to_polygon` hands the vertices to the generic ring.
    #[test]
    fn to_polygon_yields_the_ring() {
        let hex = Hexagon::<()>::flat_top(Vec2::new(0.0, 0.0), 10.0);
        let polygon = hex.to_polygon();
        assert_eq!(polygon.0.len(), 6);
        assert!(polygon.contains(&Vec2::new(0.0, 0.0)));
    }

    /// The flat-top inscribed square: `(3 - sqrt(3)) * R`, and its
    /// inverse constructor agrees.
    #[test]
    fn inscribed_square_roundtrips() {
        let hexagon = Hexagon::flat_top(Vec2::<()>::new(0.0, 0.0), 100.0);
        let side = hexagon.inscribed_square_side();
        assert!((side - 100.0 * (3.0 - 3.0_f64.sqrt())).abs() < 1e-9);
        let back = Hexagon::with_inscribed_square(Vec2::<()>::new(0.0, 0.0), side, PolygonOrientation::FlatTop);
        assert!((back.circumradius - 100.0).abs() < 1e-9);
        // The square's corners rest on the slanted edges.
        let half = side / 2.0;
        assert!(hexagon.contains(&Vec2::new(half - 1e-9, half - 1e-9)));
    }
}
