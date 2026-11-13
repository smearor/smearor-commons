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

mod aspect_ratio;
mod corners;
mod position;
mod quarter_turn;
mod rect;
mod rectangle;
mod rotation;
mod rotation_rad;
mod scale_factor;
mod scanline;
mod size;
mod smoothing_alpha;
mod units;
mod vec2;

pub use aspect_ratio::AspectRatio;
pub use corners::Corners;
pub use position::Position;
pub use quarter_turn::QuarterTurn;
pub use rect::Rect;
pub use rectangle::Rectangle;
pub use rotation::Rotation;
pub use rotation_rad::RotationRad;
pub use scale_factor::ScaleFactor;
pub use scanline::Scanline;
pub use size::Size;
#[cfg(feature = "gtk4")]
pub use size::WidgetSizeExt;
pub use smoothing_alpha::SmoothingAlpha;
pub use units::Millimeters;
pub use units::Pixels;
pub use units::PixelsPerMm;
pub use units::SquareMillimeters;
pub use units::UnitsPerMm;
pub use vec2::Vec2;
pub use vec2::serde_pair;
pub use vec2::serde_points;
