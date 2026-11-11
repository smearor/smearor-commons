//! Physical unit types used by the geometry primitives.
//!
//! The types in this module are unit-tagged `f64` newtypes, giving
//! dimensional safety at zero runtime cost: `Millimeters` cannot be mixed up
//! with `Pixels`, and `PixelsPerMm`/`UnitsPerMm` convert between device and
//! physical coordinates.

mod millimeters;
mod pixels;
mod pixels_per_mm;
mod square_millimeters;
mod units_per_mm;

pub use millimeters::Millimeters;
pub use pixels::Pixels;
pub use pixels_per_mm::PixelsPerMm;
pub use square_millimeters::SquareMillimeters;
pub use units_per_mm::UnitsPerMm;
