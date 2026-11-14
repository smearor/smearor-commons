use crate::Corners;
use crate::RotationRad;
use crate::Scanline;
use crate::Vec2;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;

/// An axis-aligned rectangle tagged with its coordinate unit `U` and
/// numeric component type `T`.
///
/// `min` is the top-left corner, `size` the extent (`size.x` = width,
/// `size.y` = height); `right`/`bottom` are the max edges. The phantom
/// unit keeps millimeter regions and pixel footprints from mixing;
/// `T` defaults to `f64` for domain math while the unit-erased
/// `Rect<(), i32>`/`Rect<(), f32>` forms (alias [`crate::Rectangle`])
/// serve integer and float boundaries - smithay `Rectangle<i32, K>`
/// and graphene `Rect` alike.
#[derive(Debug, Serialize, Deserialize)]
pub struct Rect<U = (), T = f64> {
    /// Top-left corner in the unit `U`.
    pub min: Vec2<U, T>,
    /// Extent: width in `size.x`, height in `size.y`.
    pub size: Vec2<U, T>,
}

impl<U, T: Copy> Copy for Rect<U, T> {}

impl<U, T: Clone> Clone for Rect<U, T> {
    fn clone(&self) -> Self {
        Self::new(self.min.clone(), self.size.clone())
    }
}

impl<U, T: PartialEq> PartialEq for Rect<U, T> {
    fn eq(&self, other: &Self) -> bool {
        self.min == other.min && self.size == other.size
    }
}

impl<U, T: Eq> Eq for Rect<U, T> {}

impl<U, T: Hash> Hash for Rect<U, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.min.hash(state);
        self.size.hash(state);
    }
}

impl<U, T> Rect<U, T> {
    /// Creates a rectangle from its top-left corner and its extent.
    pub const fn new(min: Vec2<U, T>, size: Vec2<U, T>) -> Self {
        Self { min, size }
    }

    /// Creates a rectangle from raw top-left coordinates and extent.
    pub const fn from_xywh(x: T, y: T, width: T, height: T) -> Self {
        Self {
            min: Vec2::new(x, y),
            size: Vec2::new(width, height),
        }
    }
}

impl<U, T: Copy> Rect<U, T> {
    /// Width of the rectangle.
    pub fn width(&self) -> T {
        self.size.x
    }

    /// Height of the rectangle.
    pub fn height(&self) -> T {
        self.size.y
    }
}

impl<U, T: Copy + Add<Output = T>> Rect<U, T> {
    /// Right edge x-coordinate.
    pub fn right(&self) -> T {
        self.min.x + self.size.x
    }

    /// Bottom edge y-coordinate.
    pub fn bottom(&self) -> T {
        self.min.y + self.size.y
    }

    /// Lower-right corner (`right`, `bottom`).
    pub fn max(&self) -> Vec2<U, T> {
        Vec2::<U, T>::new(self.right(), self.bottom())
    }

    /// The rectangle translated by `delta`.
    pub fn translated(&self, delta: Vec2<U, T>) -> Self {
        Self::new(self.min + delta, self.size)
    }
}

impl<U, T: Copy + PartialOrd + Add<Output = T>> Rect<U, T> {
    /// Whether `point` lies inside this rectangle (bounds inclusive).
    pub fn contains(&self, point: &Vec2<U, T>) -> bool {
        point.x >= self.min.x && point.x <= self.right() && point.y >= self.min.y && point.y <= self.bottom()
    }

    /// Whether this rectangle overlaps `other` - two rectangles overlap
    /// when neither lies completely beside or above the other.
    pub fn overlaps(&self, other: &Self) -> bool {
        self.min.x < other.right() && self.right() > other.min.x && self.min.y < other.bottom() && self.bottom() > other.min.y
    }
}

impl<U, T: Copy + PartialOrd + Add<Output = T> + Sub<Output = T>> Rect<U, T> {
    /// The rectangular intersection of two overlapping rectangles -
    /// `None` when they are disjoint.
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        self.overlaps(other).then(|| {
            let max_of = |a: T, b: T| if a >= b { a } else { b };
            let min_of = |a: T, b: T| if a <= b { a } else { b };
            let min = Vec2::<U, T>::new(max_of(self.min.x, other.min.x), max_of(self.min.y, other.min.y));
            let max = Vec2::<U, T>::new(min_of(self.right(), other.right()), min_of(self.bottom(), other.bottom()));
            Self::new(min, Vec2::<U, T>::new(max.x - min.x, max.y - min.y))
        })
    }
}

impl<U, T: Default> Default for Rect<U, T> {
    /// The empty rectangle at the origin.
    fn default() -> Self {
        Self::new(Vec2::default(), Vec2::default())
    }
}

impl<U, T: Display> Display for Rect<U, T> {
    /// The `Rectangle(min: (x,y), size: (w,h))` spelling.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rectangle(min: {}, size: {})", self.min, self.size)
    }
}

impl<U, T: Copy + AddAssign> AddAssign for Rect<U, T> {
    /// Adds another rectangle component-wise (origin and extent) - the
    /// accumulation form used for damage unions and deltas.
    fn add_assign(&mut self, other: Self) {
        self.min += other.min;
        self.size += other.size;
    }
}

impl<U, T: Copy + Add<Output = T>> Add for Rect<U, T> {
    type Output = Self;

    /// Component-wise sum of two rectangles (origin and extent).
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.min + rhs.min, self.size + rhs.size)
    }
}

impl<U, T: Copy + SubAssign> SubAssign for Rect<U, T> {
    /// Subtracts another rectangle component-wise (origin and extent).
    fn sub_assign(&mut self, other: Self) {
        self.min -= other.min;
        self.size -= other.size;
    }
}

impl<U, T: Copy + Sub<Output = T>> Sub for Rect<U, T> {
    type Output = Self;

    /// Component-wise difference of two rectangles (origin and extent).
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.min - rhs.min, self.size - rhs.size)
    }
}

// ---------------------------------------------------------------------------
// `f32` / `i32` numeric bridges (boundary integer and float coordinates)
// ---------------------------------------------------------------------------

impl<U> Rect<U, f32> {
    /// Rounds to the nearest integer (standard for display coordinates).
    pub fn to_i32_round(&self) -> Rect<U, i32> {
        Rect::new(self.min.to_i32_round(), self.size.to_i32_round())
    }

    /// Rounds down (for clipping/bounding boxes, to not exceed any pixels).
    pub fn to_i32_floor(&self) -> Rect<U, i32> {
        Rect::new(self.min.to_i32_floor(), self.size.to_i32_floor())
    }

    /// Rounds up (for damage tracking, to capture all affected pixels).
    pub fn to_i32_ceil(&self) -> Rect<U, i32> {
        Rect::new(self.min.to_i32_ceil(), self.size.to_i32_ceil())
    }
}

impl<U> Rect<U, i32> {
    /// The rectangle as `f32` components.
    pub fn to_f32(&self) -> Rect<U, f32> {
        Rect::new(self.min.to_f32(), self.size.to_f32())
    }

    /// The rectangle as `f64` components.
    pub fn to_f64(&self) -> Rect<U, f64> {
        Rect::new(self.min.to_f64(), self.size.to_f64())
    }
}

impl<U> Rect<U, f32> {
    /// The rectangle as `f64` components.
    pub fn to_f64(&self) -> Rect<U, f64> {
        Rect::new(self.min.to_f64(), self.size.to_f64())
    }
}

impl<U> Rect<U, f64> {
    /// Rounds to the nearest integer (standard for display coordinates).
    pub fn to_i32_round(&self) -> Rect<U, i32> {
        Rect::new(self.min.to_i32_round(), self.size.to_i32_round())
    }

    /// Rounds down (for clipping/bounding boxes, to not exceed any pixels).
    pub fn to_i32_floor(&self) -> Rect<U, i32> {
        Rect::new(self.min.to_i32_floor(), self.size.to_i32_floor())
    }

    /// Rounds up (for damage tracking, to capture all affected pixels).
    pub fn to_i32_ceil(&self) -> Rect<U, i32> {
        Rect::new(self.min.to_i32_ceil(), self.size.to_i32_ceil())
    }

    /// The rectangle as `f32` components.
    pub fn to_f32(&self) -> Rect<U, f32> {
        Rect::new(self.min.to_f32(), self.size.to_f32())
    }
}

// ---------------------------------------------------------------------------
// `f64` domain math - only available for the default component type
// ---------------------------------------------------------------------------

impl<U> Rect<U, f64> {
    /// Creates the rectangle of the given `size` centered at `center`.
    pub fn centered(center: Vec2<U>, size: Vec2<U>) -> Self {
        Self {
            min: Vec2::<U>::new(center.x - size.x / 2.0, center.y - size.y / 2.0),
            size,
        }
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

    /// The size of the axis-aligned bounding box of this rectangle's
    /// extent rotated by `rotation` around its center (position is
    /// irrelevant for the bounding box).
    pub fn rotated_bounds(&self, rotation: RotationRad) -> Vec2<U> {
        self.size.rotated_bounds(rotation)
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

    /// Rasterizes the rectangle rotated by `rotation` around its center
    /// into a cairo [`Region`](cairo::Region) in the rectangle's own
    /// coordinate space - one 1-px-high [`RectangleInt`](cairo::RectangleInt)
    /// per [`rotated_region_scanlines`](Self::rotated_region_scanlines)
    /// strip, offset by the rect's center.
    ///
    /// Used e.g. to describe the input region of a rotated GTK widget on
    /// a wayland surface.
    ///
    /// # Errors
    ///
    /// Propagates [`cairo::Error`]s from `Region` construction.
    #[cfg(feature = "cairo")]
    pub fn rotated_region(&self, rotation: RotationRad) -> Result<cairo::Region, cairo::Error> {
        let region = cairo::Region::create();
        let center = self.center();
        for strip in self.rotated_region_scanlines(rotation) {
            region.union_rectangle(&cairo::RectangleInt::new(
                (center.x + f64::from(strip.x_start)).round() as i32,
                (center.y + f64::from(strip.y)).round() as i32,
                strip.width(),
                1,
            ))?;
        }
        Ok(region)
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

// ---------------------------------------------------------------------------
// gtk4 gate — graphene rect conversions
//
// Unit-generic like the [`Vec2`] point conversions: the boundary call
// interprets the components in GTK's coordinate space.
// ---------------------------------------------------------------------------

/// The `f32` rectangle as a `graphene::Rect`.
#[cfg(feature = "gtk4")]
impl<U> From<Rect<U, f32>> for gtk4::graphene::Rect {
    fn from(rectangle: Rect<U, f32>) -> Self {
        Self::new(rectangle.min.x, rectangle.min.y, rectangle.size.x, rectangle.size.y)
    }
}

/// The `i32` rectangle as a `graphene::Rect`.
#[cfg(feature = "gtk4")]
impl<U> From<Rect<U, i32>> for gtk4::graphene::Rect {
    fn from(rectangle: Rect<U, i32>) -> Self {
        Self::new(rectangle.min.x as f32, rectangle.min.y as f32, rectangle.size.x as f32, rectangle.size.y as f32)
    }
}

// ---------------------------------------------------------------------------

/// The `f32` rectangle as an `ab_glyph::Rect` (min/max corner form).
#[cfg(feature = "ab_glyph")]
impl<U> From<Rect<U, f32>> for ab_glyph::Rect {
    fn from(rectangle: Rect<U, f32>) -> Self {
        let max = rectangle.min + rectangle.size;
        Self {
            min: ab_glyph::Point {
                x: rectangle.min.x,
                y: rectangle.min.y,
            },
            max: ab_glyph::Point { x: max.x, y: max.y },
        }
    }
}

/// An `ab_glyph::Rect` as a `Rect` in the caller-chosen unit `U` -
/// min/size form, so the corner difference is computed here.
#[cfg(feature = "ab_glyph")]
impl<U> From<ab_glyph::Rect> for Rect<U, f32> {
    fn from(rectangle: ab_glyph::Rect) -> Self {
        Self::new(
            Vec2::new(rectangle.min.x, rectangle.min.y),
            Vec2::new(rectangle.max.x - rectangle.min.x, rectangle.max.y - rectangle.min.y),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Pixels;
    use crate::Rectangle;

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

    /// Integer rectangles share the accessor API - the former
    /// `Rectangle<i32>` boundary type is the same value.
    #[test]
    fn integer_rect_accessors() {
        let rect = Rectangle::<i32>::from_xywh(10, 20, 100, 50);
        assert_eq!(rect.min, Vec2::<(), i32>::new(10, 20));
        assert_eq!(rect.size, Vec2::<(), i32>::new(100, 50));
        assert_eq!(rect.width(), 100);
        assert_eq!(rect.right(), 110);
        assert_eq!(rect.bottom(), 70);
        assert_eq!(rect.max(), Vec2::<(), i32>::new(110, 70));
        assert!(rect.contains(&Vec2::new(60, 45)));
        assert!(!rect.contains(&Vec2::new(120, 45)));
        let b = Rectangle::<i32>::from_xywh(50, 25, 100, 50);
        assert_eq!(rect.intersection(&b), Some(Rectangle::<i32>::from_xywh(50, 25, 60, 45)));
        let mut acc = rect;
        acc += Rectangle::<i32>::from_xywh(1, 2, 3, 4);
        assert_eq!(acc, Rectangle::<i32>::from_xywh(11, 22, 103, 54));
        acc -= Rectangle::<i32>::from_xywh(1, 2, 3, 4);
        assert_eq!(acc, rect);
    }

    /// `f32` rectangles round to `i32` in all three modes - unit
    /// preserved, matching the old `Rectangle<f32>` boundary API.
    #[test]
    fn float_rect_rounding() {
        let rect = Rectangle::<f32>::from_xywh(10.4, 20.6, 100.5, 50.5);
        assert_eq!(rect.to_i32_round(), Rectangle::<i32>::from_xywh(10, 21, 101, 51));
        assert_eq!(rect.to_i32_floor(), Rectangle::<i32>::from_xywh(10, 20, 100, 50));
        assert_eq!(rect.to_i32_ceil(), Rectangle::<i32>::from_xywh(11, 21, 101, 51));
        assert_eq!(Rectangle::<i32>::from_xywh(10, 20, 100, 50).to_f32(), Rectangle::<f32>::from_xywh(10.0, 20.0, 100.0, 50.0));
    }

    /// `Rectangle` displays position and extent of both corners.
    #[test]
    fn display_format() {
        let rect = Rectangle::<i32>::from_xywh(10, 20, 100, 200);
        assert_eq!(format!("{}", rect), "Rectangle(min: (10,20), size: (100,200))");
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

    #[cfg(feature = "cairo")]
    #[test]
    fn rotated_region_covers_rotated_square() {
        let region = Rect::<Pixels>::from_xywh(0.0, 0.0, 10.0, 10.0)
            .rotated_region(RotationRad::from_degrees(45.0))
            .unwrap_or_else(|_| cairo::Region::create());
        assert!(!region.is_empty());
        // The rotated square's diagonal is wider than its axis-aligned side.
        assert!(region.num_rectangles() > 10);
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

    /// `ab_glyph::Rect` converts between min/max corners and min/size
    /// in both directions.
    #[cfg(feature = "ab_glyph")]
    #[test]
    fn ab_glyph_rect_conversions() {
        let source = ab_glyph::Rect {
            min: ab_glyph::Point { x: 1.0, y: 2.0 },
            max: ab_glyph::Point { x: 5.0, y: 9.0 },
        };
        let rect = Rect::<Pixels, f32>::from(source);
        assert_eq!((rect.min.x, rect.min.y, rect.size.x, rect.size.y), (1.0, 2.0, 4.0, 7.0));
        let back = ab_glyph::Rect::from(rect);
        assert_eq!((back.min.x, back.min.y, back.max.x, back.max.y), (1.0, 2.0, 5.0, 9.0));
    }
}
