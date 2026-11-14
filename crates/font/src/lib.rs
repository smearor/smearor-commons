//! Font metric utilities on [`ab_glyph`] fonts.
//!
//! Glyph scale fitting ([`glyph_fit_scale`]), bounds centering
//! ([`centered_glyph`]) and width truncation ([`truncate_text_to_width`]),
//! expressed in `smearor_commons_geometry` units ([`Pixels`], [`Vec2`],
//! [`Rect`]). These are the pure computations — font loading and caching
//! policy stay with the consumer.

mod glyph;
mod text;

pub use glyph::centered_glyph;
pub use glyph::glyph_fit_scale;
pub use text::truncate_text_to_width;
