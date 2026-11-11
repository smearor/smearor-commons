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

mod units;

pub use units::Millimeters;
pub use units::Pixels;
pub use units::PixelsPerMm;
pub use units::SquareMillimeters;
pub use units::UnitsPerMm;
