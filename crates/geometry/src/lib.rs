//! Generic 2D geometry primitives for the Smearor ecosystem.
//!
//! Pure mathematics without framework dependencies: unit-tagged vectors,
//! rotations, rectangles and shapes. `serde` is unconditional;
//! `async-graphql`/`schemars` scalar and schema derives and conversions
//! to `gtk4`, `cairo` and `smithay` types live behind optional feature
//! gates.
//!
//! The unit newtypes ([`Millimeters`], [`Pixels`],
//! [`SquareMillimeters`], [`PixelsPerMm`], [`UnitsPerMm`]) keep raw
//! `f64` values unconfusable and double as the `Vec2<U>` unit markers.
//!
//! One model serves both domain math and framework boundaries:
//! [`Vec2`]`<U, T>` and [`Rect`]`<U, T>` are generic over the
//! coordinate space `U` *and* the numeric component type `T` (default
//! `f64`). `Vec2<Millimeters>` is a tagged domain point;
//! `Vec2<(), i32>` is the *unit-erased* boundary form that converts
//! into smithay `Point<i32, K>` of any space kind `K`, graphene `f32`
//! primitives or `i32` texture extents. [`Position`], [`Size`] and
//! [`Rectangle`] are readable aliases for these erased forms -
//! the same type, not parallel representations.

mod annulus;
mod arc;
#[cfg(feature = "gtk4")]
mod centered_transform;
mod circle;
mod corners;
mod position;
mod quantities;
mod rect;
mod rectangle;
mod scanline;
mod similarity_transform;
mod size;
mod units;
mod vec2;

pub use annulus::Annulus;
pub use arc::Arc;
#[cfg(feature = "gtk4")]
pub use centered_transform::CenteredTransform;
pub use circle::Circle;
pub use corners::Corners;
pub use position::Position;
pub use quantities::AspectRatio;
pub use quantities::Fraction;
pub use quantities::QuarterTurn;
pub use quantities::Rotation;
pub use quantities::RotationRad;
pub use quantities::ScaleFactor;
pub use quantities::SmoothingAlpha;
pub use rect::Rect;
pub use rectangle::Rectangle;
pub use scanline::Scanline;
pub use similarity_transform::SimilarityTransform;
pub use size::Size;
#[cfg(feature = "gtk4")]
pub use size::WidgetSizeExt;
pub use units::Degrees;
pub use units::Millimeters;
pub use units::Pixels;
pub use units::PixelsPerMm;
pub use units::Radians;
pub use units::SquareMillimeters;
pub use units::UnitsPerMm;
pub use vec2::Vec2;
pub use vec2::serde_pair;
pub use vec2::serde_points;
