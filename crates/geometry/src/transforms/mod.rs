//! Two-dimensional similarity transforms - rotation, uniform scale
//! and translation around anchor points.
//!
//! [`SimilarityTransform`] and [`RigidTransform`] are *fitted*:
//! least-squares-computed from corresponding point sets (gesture
//! tracking, marker detection); the rigid variant pins the scale to
//! `1` for same-unit correspondences.
//! [`CenteredTransform`] is *declared*: an author-specified
//! placement spec for a child extent centered on a point.

#[cfg(feature = "gtk4")]
mod centered;
mod rigid;
mod similarity;

#[cfg(feature = "gtk4")]
pub use centered::CenteredTransform;
pub use rigid::RigidTransform;
pub use similarity::SimilarityTransform;
