//! Two-dimensional similarity transforms - rotation, uniform scale
//! and translation around anchor points.
//!
//! [`SimilarityTransform`] is *fitted*: least-squares-computed from
//! corresponding point sets (gesture tracking, marker detection).
//! [`CenteredTransform`] is *declared*: an author-specified
//! placement spec for a child extent centered on a point.

#[cfg(feature = "gtk4")]
mod centered;
mod similarity;

#[cfg(feature = "gtk4")]
pub use centered::CenteredTransform;
pub use similarity::SimilarityTransform;
