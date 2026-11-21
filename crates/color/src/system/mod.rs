//! Historical color *order systems* - appearance-based notations
//! that predate or complement the colorimetric spaces.
//!
//! Unlike the color *spaces* at the crate root (`Lab`, `Xyz`,
//! `Hsv` are mathematical coordinate systems), an order system is a
//! human vocabulary: a notation, a sampling of perceptual steps and
//! a mapping to real colors. The modeled colors are documented
//! approximations where the original atlases are proprietary or
//! perceptual; the notations and geometries are the real ones.

mod ncs;
mod ostwald;

pub use ncs::Ncs;
pub use ncs::NcsHue;
pub use ostwald::Ostwald;
