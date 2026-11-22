//! The layered `morph` transition - liquid flow, edge-guided melt,
//! or pixel-block morph driven by the [`MorphStyle`] mode.

mod effect;
mod frame;
mod style;

pub use effect::Morph;
pub use style::MorphStyle;
