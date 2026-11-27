//! Scalar quantity newtypes - measured values with semantics.
//!
//! Unlike the coordinate-space tags in [`crate::units`] (which mark
//! which space a `Vec2<U>` lives in), these types wrap plain `f64`
//! scalars so quantities cannot be confused with each other or with
//! raw numbers flowing through the pipeline: an [`AspectRatio`], a
//! [`ScaleFactor`] and a [`RotationRad`] all contain `f64` yet are
//! never interchangeable.

mod aspect_ratio;
mod factor;
mod fraction;
mod quarter_turn;
mod rotation;
mod rotation_direction;
mod rotation_rad;
mod scale_factor;
mod sixth_turn;
mod smoothing_alpha;

pub use aspect_ratio::AspectRatio;
pub use factor::Factor;
pub use fraction::Fraction;
pub use quarter_turn::QuarterTurn;
pub use rotation::Rotation;
pub use rotation_direction::RotationDirection;
pub use rotation_rad::RotationRad;
pub use scale_factor::ScaleFactor;
pub use sixth_turn::SixthTurn;
pub use smoothing_alpha::SmoothingAlpha;
