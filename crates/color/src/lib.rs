//! Generic color types for the Smearor ecosystem.
//!
//! Channel-generic [`Color<T>`] (normalized `f64`/`f32` and byte `u8`),
//! hex parsing/formatting via `FromStr`/`Display` and consolidated color
//! operations. `gtk4`/`cairo` conversions live behind feature gates.
//!
//! Semantic color types (`FigureColor`, `LineColor`, `PawnColor`, ...)
//! stay in consumer crates as thin newtype wrappers around [`Color<T>`].

mod color;
#[cfg(feature = "cairo")]
mod cairo;
mod parse_color_error;

pub use color::Color;
pub use parse_color_error::ParseColorError;
