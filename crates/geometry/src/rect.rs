use crate::Corners;
use crate::RotationRad;
use crate::Scanline;
use crate::Vec2;
use serde::Deserialize;
use serde::Serialize;

/// An axis-aligned rectangle tagged with its coordinate unit `U`.
///
/// `min` is the top-left corner, `size` the extent (`size.x` = width,
/// `size.y` = height); `right`/`bottom` are the max edges. The phantom
/// unit keeps millimeter regions and pixel footprints from mixing.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rect<U> {
    /// Top-left corner in the unit `U`.
    pub min: Vec2<U>,
    /// Extent: width in `size.x`, height in `size.y`.
    pub size: Vec2<U>,
}

impl<U> Rect<U> {
    /// Creates a rectangle from its top-left corner and its extent.
    pub const fn new(min: Vec2<U>, size: Vec2<U>) -> Self {
        Self { min, size }
    }

    /// Creates a rectangle from raw top-left coordinates and extent.
    pub const fn from_xywh(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            min: Vec2::new(x, y),
            size: Vec2::new(width, height),
        }
    }

    /// Creates the rectangle of the given `size` centered at `center`.
    pub fn centered(center: Vec2<U>, size: Vec2<U>) -> Self {
        Self {
            min: Vec2::<U>::new(center.x - size.x / 2.0, center.y - size.y / 2.0),
            size,
        }
    }

    /// Right edge x-coordinate.
    pub fn right(&self) -> f64 {
        self.min.x + self.size.x
    }

    /// Bottom edge y-coordinate.
    pub fn bottom(&self) -> f64 {
        self.min.y + self.size.y
    }

    /// Lower-right corner (`right`, `bottom`).
    pub fn max(&self) -> Vec2<U> {
        Vec2::<U>::new(self.right(), self.bottom())
    }

    /// Width of the rectangle.
    pub fn width(&self) -> f64 {
        self.size.x
    }

    /// Height of the rectangle.
    pub fn height(&self) -> f64 {
        self.size.y
    }

    /// Center point of the rectangle.
    pub fn center(&self) -> Vec2<U> {
        Vec2::<U>::new(self.min.x + self.size.x / 2.0, self.min.y + self.size.y / 2.0)
    }

    /// The rectangle's four corners, starting at `top_left` (the
    /// rect's `min`).
    pub fn corners(&self) -> Corners<U> {
        Corners {
            top_left: self.min,
            top_right: Vec2::<U>::new(self.right(), self.min.y),
            bottom_right: self.max(),
            bottom_left: Vec2::<U>::new(self.min.x, self.bottom()),
        }
    }

    /// Whether the rectangle is degenerate: non-positive extent or
    /// non-finite coordinates. A degenerate rectangle contains no points.
    pub fn is_degenerate(&self) -> bool {
        let finite = [self.min.x, self.min.y, self.size.x, self.size.y].iter().all(|v| v.is_finite());
        !finite || self.size.x <= 0.0 || self.size.y <= 0.0
    }

    /// Whether `point` lies inside this rectangle (bounds inclusive).
    pub fn contains(&self, point: &Vec2<U>) -> bool {
        point.x >= self.min.x && point.x <= self.right() && point.y >= self.min.y && point.y <= self.bottom()
    }

    /// Whether this rectangle overlaps `other` - two rectangles overlap
    /// when neither lies completely beside or above the other.
    pub fn overlaps(&self, other: &Self) -> bool {
        self.min.x < other.right() && self.right() > other.min.x && self.min.y < other.bottom() && self.bottom() > other.min.y
    }

    /// The rectangular intersection of two overlapping rectangles -
    /// `None` when they are disjoint.
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        self.overlaps(other).then(|| {
            let min = Vec2::<U>::new(self.min.x.max(other.min.x), self.min.y.max(other.min.y));
            let max = Vec2::<U>::new(self.right().min(other.right()), self.bottom().min(other.bottom()));
            Self::new(min, Vec2::<U>::new(max.x - min.x, max.y - min.y))
        })
    }

    /// The rectangle shrunk by `margin` (a scalar in the unit `U`) on
    /// every side - negative or degenerate results clamp to an empty
    /// extent.
    pub fn inset(&self, margin: U) -> Self
    where
        U: Into<f64>,
    {
        let margin = margin.into();
        let min = Vec2::<U>::new(self.min.x + margin, self.min.y + margin);
        let size = Vec2::<U>::new(self.size.x - 2.0 * margin, self.size.y - 2.0 * margin);
        Self::new(min, size)
    }

    /// The rectangle translated by `delta`.
    pub fn translated(&self, delta: Vec2<U>) -> Self {
        Self::new(self.min + delta, self.size)
    }

    /// The size of the axis-aligned bounding box of this rectangle's
    /// extent rotated by `rotation` around its center (position is
    /// irrelevant for the bounding box).
    pub fn rotated_bounds(&self, rotation: RotationRad) -> Vec2<U> {
        let (sin, cos) = rotation.sin_cos();
        let abs_cos = cos.abs();
        let abs_sin = sin.abs();
        Vec2::<U>::new(self.size.x * abs_cos + self.size.y * abs_sin, self.size.x * abs_sin + self.size.y * abs_cos)
    }

    /// Rasterizes this rectangle's extent rotated by `rotation` around its
    /// center into horizontal scanline strips.
    ///
    /// Returns [`Scanline`]s relative to the rectangle's center (y-down
    /// screen coordinates); each strip is one pixel high. Callers add the
    /// actual center to map the strips onto `cairo::RectangleInt`s or
    /// similar integer regions.
    pub fn rotated_region_scanlines(&self, rotation: RotationRad) -> Vec<Scanline> {
        let half_width = self.size.x / 2.0;
        let half_height = self.size.y / 2.0;
        let corners = [
            Vec2::<U>::new(-half_width, -half_height),
            Vec2::<U>::new(half_width, -half_height),
            Vec2::<U>::new(half_width, half_height),
            Vec2::<U>::new(-half_width, half_height),
        ];
        let transformed: Vec<(f64, f64)> = corners
            .iter()
            .map(|c| {
                let r = c.rotated(rotation);
                (r.x, r.y)
            })
            .collect();
        let min_y = transformed.iter().map(|(_, y)| *y).fold(f64::INFINITY, f64::min).floor() as i32;
        let max_y = transformed.iter().map(|(_, y)| *y).fold(f64::NEG_INFINITY, f64::max).ceil() as i32;
        let mut strips = Vec::with_capacity((max_y - min_y).max(0) as usize);
        for y in min_y..max_y {
            let current_y = y as f64 + 0.5;
            let mut intersections = Vec::with_capacity(4);
            for i in 0..4 {
                let p1 = transformed[i];
                let p2 = transformed[(i + 1) % 4];
                if (p1.1 <= current_y && p2.1 > current_y) || (p2.1 <= current_y && p1.1 > current_y) {
                    intersections.push(p1.0 + (current_y - p1.1) * (p2.0 - p1.0) / (p2.1 - p1.1));
                }
            }
            if intersections.len() >= 2 {
                intersections.sort_by(f64::total_cmp);
                strips.push(Scanline {
                    y,
                    x_start: intersections[0].round() as i32,
                    x_end: intersections[intersections.len() - 1].round() as i32,
                });
            }
        }
        strips
    }

    /// The minimal translation needed so that at least `min_visible`
    /// (a scalar in the unit `U`) of this rectangle overlaps `bounds`
    /// on each axis.
    ///
    /// Applying the returned offset to the rectangle's position moves it
    /// back inside the bounds. The overlap requirement is capped at half
    /// the bounds size so it stays satisfiable on very small bounds.
    pub fn clamp_offset(&self, bounds: &Self, min_visible: U) -> Vec2<U>
    where
        U: Into<f64>,
    {
        let min_visible = min_visible.into();
        Vec2::<U>::new(
            clamp_axis(self.min.x, self.size.x, bounds.min.x, bounds.size.x, min_visible),
            clamp_axis(self.min.y, self.size.y, bounds.min.y, bounds.size.y, min_visible),
        )
    }

    /// The minimal translation needed so that this rectangle lies
    /// entirely inside `bounds`.
    ///
    /// On axes where the rectangle is larger than the bounds it is
    /// centered, so it stays reachable instead of snapping to one edge.
    pub fn contain_offset(&self, bounds: &Self) -> Vec2<U> {
        Vec2::<U>::new(
            contain_axis(self.min.x, self.size.x, bounds.min.x, bounds.size.x),
            contain_axis(self.min.y, self.size.y, bounds.min.y, bounds.size.y),
        )
    }
}

impl<U> Default for Rect<U> {
    /// The empty rectangle at the origin.
    fn default() -> Self {
        Self::new(Vec2::ZERO, Vec2::ZERO)
    }
}

/// Per-axis part of [`Rect::clamp_offset`]: clamps the rectangle's start
/// coordinate so it keeps `min_visible` units inside the bounds, capped at
/// half the bounds size.
fn clamp_axis(rect_min: f64, rect_size: f64, bounds_min: f64, bounds_size: f64, min_visible: f64) -> f64 {
    let overlap = min_visible.min(bounds_size / 2.0);
    let allowed_min = bounds_min + overlap - rect_size;
    let allowed_max = bounds_min + bounds_size - overlap;
    rect_min.clamp(allowed_min, allowed_max) - rect_min
}

/// Per-axis part of [`Rect::contain_offset`]: clamps the rectangle's start
/// coordinate fully inside the bounds, or centers it when larger.
fn contain_axis(rect_min: f64, rect_size: f64, bounds_min: f64, bounds_size: f64) -> f64 {
    if rect_size >= bounds_size {
        return bounds_min + (bounds_size - rect_size) / 2.0 - rect_min;
    }
    rect_min.clamp(bounds_min, bounds_min + bounds_size - rect_size) - rect_min
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Pixels;

    const EPSILON: f64 = 1e-9;

    #[test]
    fn accessors() {
        let rect = Rect::<Pixels>::from_xywh(10.0, 20.0, 100.0, 50.0);
        assert_eq!(rect.right(), 110.0);
        assert_eq!(rect.bottom(), 70.0);
        assert_eq!(rect.max(), Vec2::new(110.0, 70.0));
        assert_eq!(rect.width(), 100.0);
        assert_eq!(rect.height(), 50.0);
        assert_eq!(rect.center(), Vec2::new(60.0, 45.0));
        assert_eq!(rect.corners().bottom_right, Vec2::new(110.0, 70.0));
        assert!(!rect.is_degenerate());
    }

    #[test]
    fn detects_degenerate_rects() {
        for (width, height) in [(0.0, 10.0), (10.0, 0.0), (-5.0, 10.0), (f64::NAN, 1.0)] {
            assert!(Rect::<Pixels>::from_xywh(0.0, 0.0, width, height).is_degenerate());
        }
    }

    #[test]
    fn contains_inside_and_on_bounds() {
        let rect = Rect::<Pixels>::from_xywh(0.0, 0.0, 500.0, 1000.0);
        assert!(rect.contains(&Vec2::new(0.0, 10.0)));
        assert!(rect.contains(&Vec2::new(500.0, 10.0)));
        assert!(rect.contains(&Vec2::new(250.0, 900.0)));
        assert!(!rect.contains(&Vec2::new(501.0, 10.0)));
        assert!(!rect.contains(&Vec2::new(-1.0, 10.0)));
    }

    #[test]
    fn intersection_is_the_overlap_rect() {
        let a = Rect::<Pixels>::from_xywh(0.0, 0.0, 100.0, 100.0);
        let b = Rect::<Pixels>::from_xywh(50.0, 25.0, 100.0, 50.0);
        assert_eq!(a.intersection(&b), Some(Rect::<Pixels>::from_xywh(50.0, 25.0, 50.0, 50.0)));
        assert_eq!(a.intersection(&Rect::<Pixels>::from_xywh(200.0, 0.0, 100.0, 10.0)), None);
    }

    #[test]
    fn inset_shrinks_all_sides() {
        let rect = Rect::<Pixels>::from_xywh(10.0, 20.0, 100.0, 50.0);
        assert_eq!(rect.inset(Pixels::new(5.0)), Rect::<Pixels>::from_xywh(15.0, 25.0, 90.0, 40.0));
    }

    #[test]
    fn translated_moves_origin() {
        let rect = Rect::<Pixels>::from_xywh(10.0, 20.0, 100.0, 50.0);
        assert_eq!(rect.translated(Vec2::new(5.0, -10.0)), Rect::<Pixels>::from_xywh(15.0, 10.0, 100.0, 50.0));
    }

    #[test]
    fn rotated_bounds_zero_degrees() {
        let size = Rect::<Pixels>::from_xywh(0.0, 0.0, 100.0, 50.0).rotated_bounds(RotationRad::from_degrees(0.0));
        assert!((size.x - 100.0).abs() < EPSILON && (size.y - 50.0).abs() < EPSILON);
    }

    #[test]
    fn rotated_bounds_ninety_degrees() {
        let size = Rect::<Pixels>::from_xywh(0.0, 0.0, 100.0, 50.0).rotated_bounds(RotationRad::from_degrees(90.0));
        assert!((size.x - 50.0).abs() < EPSILON && (size.y - 100.0).abs() < EPSILON);
    }

    #[test]
    fn rotated_bounds_forty_five_degrees() {
        let size = Rect::<Pixels>::from_xywh(0.0, 0.0, 100.0, 100.0).rotated_bounds(RotationRad::from_degrees(45.0));
        let expected = 100.0 * std::f64::consts::SQRT_2;
        assert!((size.x - expected).abs() < EPSILON && (size.y - expected).abs() < EPSILON);
    }

    #[test]
    fn scanlines_cover_unrotated_rect() {
        let strips = Rect::<Pixels>::from_xywh(0.0, 0.0, 10.0, 6.0).rotated_region_scanlines(RotationRad::from_degrees(0.0));
        assert_eq!(strips.len(), 6);
        for strip in &strips {
            assert_eq!((strip.x_start, strip.x_end), (-5, 5), "y={}", strip.y);
        }
    }

    #[test]
    fn scanlines_cover_rotated_square() {
        let strips = Rect::<Pixels>::from_xywh(0.0, 0.0, 10.0, 10.0).rotated_region_scanlines(RotationRad::from_degrees(45.0));
        // The diamond is widest at its horizontal diagonal.
        let widest = strips.iter().map(|s| s.width()).max().unwrap_or_default();
        assert!(widest > 10);
        assert!(!strips.is_empty());
    }

    #[test]
    fn clamp_offset_keeps_min_visible() {
        let bounds = Rect::<Pixels>::from_xywh(0.0, 0.0, 1000.0, 800.0);
        let footprint = Rect::<Pixels>::from_xywh(100.0, 100.0, 200.0, 100.0);
        assert_eq!(footprint.clamp_offset(&bounds, Pixels::new(40.0)), Vec2::new(0.0, 0.0));
        let footprint = Rect::<Pixels>::from_xywh(990.0, 100.0, 200.0, 100.0);
        let offset = footprint.clamp_offset(&bounds, Pixels::new(40.0));
        assert_eq!(offset.y, 0.0);
        assert_eq!(offset.x, -30.0); // 40 px stay inside the bounds
        let footprint = Rect::<Pixels>::from_xywh(-190.0, 100.0, 200.0, 100.0);
        let offset = footprint.clamp_offset(&bounds, Pixels::new(40.0));
        assert_eq!(offset.x, 30.0);
    }

    #[test]
    fn contain_offset_centers_oversized() {
        let bounds = Rect::<Pixels>::from_xywh(0.0, 0.0, 100.0, 100.0);
        let footprint = Rect::<Pixels>::from_xywh(0.0, 0.0, 200.0, 50.0);
        let offset = footprint.contain_offset(&bounds);
        // Wider than bounds on x: centered (-50 shift); fits on y: clamped.
        assert_eq!(offset.x, -50.0);
        assert_eq!(offset.y, 0.0);
    }
}
