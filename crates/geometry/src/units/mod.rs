//! Scalar unit types used by the geometry primitives.
//!
//! The types in this module are unit-tagged `f64` newtypes, giving
//! dimensional safety at zero runtime cost: `Millimeters` cannot be mixed up
//! with `Pixels`, `Degrees` not with `Radians`, and `PixelsPerMm`/`UnitsPerMm`
//! convert between device and physical coordinates. The coordinate-space
//! units double as `Vec2<U>` unit markers; the angular units
//! ([`Degrees`], [`Radians`]) are plain scalar boundaries for the
//! quantity type [`crate::RotationRad`].

mod degrees;
mod millimeters;
mod pixels;
mod pixels_per_mm;
mod radians;
mod square_millimeters;
mod units_per_mm;

pub use degrees::Degrees;
pub use millimeters::Millimeters;
pub use pixels::Pixels;
pub use pixels_per_mm::PixelsPerMm;
pub use radians::Radians;
pub use square_millimeters::SquareMillimeters;
pub use units_per_mm::UnitsPerMm;
