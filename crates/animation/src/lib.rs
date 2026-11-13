//! Timed animations for the Smearor ecosystem.
//!
//! Pure `std` core: an [`TimedAnimation`] pairs a [`Trajectory`] - the
//! shape of the animated output - with a [`Duration`] and an
//! [`EasingFunction`]. Trajectories compose: [`Span`] interpolates
//! between two values of any [`Interpolate`] type, [`ThreePhaseSpan`]
//! eases out, holds and eases back, and `(S1, S2)` tuples animate
//! composite values on one clock. The `gtk4` feature gate adds the
//! frame-clock driver that pumps an animation on a widget.
//!
//! `serde` is unconditional; `schemars` schema derives live behind
//! their feature gate.

mod easing;
#[cfg(feature = "gtk4")]
mod frame_driver;
mod interpolate;
mod span;
mod three_phase_span;
mod timed_animation;
mod trajectory;

pub use easing::EasingFunction;
#[cfg(feature = "gtk4")]
pub use frame_driver::drive;
pub use interpolate::Interpolate;
pub use span::Span;
pub use three_phase_span::ThreePhaseSpan;
pub use timed_animation::TimedAnimation;
pub use trajectory::Trajectory;

/// An eased, timed rotation animation - the shortest-arc lerp
/// between two [`smearor_commons_geometry::RotationRad`] values.
pub type RotationAnimation = TimedAnimation<Span<smearor_commons_geometry::RotationRad>>;
