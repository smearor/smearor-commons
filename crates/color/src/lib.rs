//! Generic color types for the Smearor ecosystem.
//!
//! Channel-generic `Color<T>` (normalized `f64`/`f32` and byte `u8`),
//! hex parsing/formatting via `FromStr`/`Display` and consolidated color
//! operations. `gtk4`/`cairo` conversions live behind feature gates.
