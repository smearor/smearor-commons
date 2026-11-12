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

mod corners;
mod rect;
mod rotation_rad;
mod scanline;
mod smoothing_alpha;
mod units;
mod vec2;

pub use corners::Corners;
pub use rect::Rect;
pub use rotation_rad::RotationRad;
pub use scanline::Scanline;
pub use smoothing_alpha::SmoothingAlpha;
pub use units::Millimeters;
pub use units::Pixels;
pub use units::PixelsPerMm;
pub use units::SquareMillimeters;
pub use units::UnitsPerMm;
pub use vec2::Vec2;
pub use vec2::serde_pair;
pub use vec2::serde_points;
