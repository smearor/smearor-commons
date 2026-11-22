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

mod bezier;
mod bounce_path;
mod catmull_rom;
mod conical_helix;
mod delay;
mod eased;
mod easing;
mod ellipse;
mod figure8;
#[cfg(feature = "gtk4")]
mod frame_driver;
mod helix;
mod interpolate;
mod keyframes;
mod lemniskate;
mod lissajous;
mod lorenz;
mod mirror;
mod mobius;
mod orbit;
mod polygon_path;
mod repeat;
mod rose;
mod rotation_snap;
mod snap_update;
mod span;
mod sphere_lissajous;
mod spherical_orbit;
mod spiral;
mod spiro;
mod star_path;
mod steps;
mod superellipse;
mod three_phase_span;
mod timed_animation;
mod torus_knot;
mod trajectory;
mod viviani;
mod wave_trail;

pub use bezier::BezierPath;
pub use bounce_path::BouncePath;
pub use catmull_rom::CatmullRom;
pub use conical_helix::ConicalHelix;
pub use delay::Delay;
pub use eased::Eased;
pub use easing::EasingFunction;
pub use easing::StepsJump;
pub use ellipse::Ellipse;
pub use figure8::Figure8;
#[cfg(feature = "gtk4")]
pub use frame_driver::drive;
pub use helix::Helix;
pub use interpolate::Interpolate;
pub use keyframes::Keyframes;
pub use lemniskate::Lemniskate;
pub use lissajous::Lissajous;
pub use lorenz::Lorenz;
pub use mirror::Mirror;
pub use mobius::Mobius;
pub use orbit::Orbit;
pub use polygon_path::PolygonPath;
pub use repeat::Repeat;
pub use rose::Rose;
pub use rotation_snap::RotationSnap;
use smearor_commons_geometry::RotationRad;
use smearor_commons_geometry::ScaleFactor;
use smearor_commons_geometry::Vec2;
pub use snap_update::SnapUpdate;
pub use span::Span;
pub use sphere_lissajous::SphereLissajous;
pub use spherical_orbit::SphericalOrbit;
pub use spiral::Spiral;
pub use spiro::Spiro;
pub use star_path::StarPath;
pub use steps::Steps;
pub use superellipse::Superellipse;
pub use three_phase_span::HoldMode;
pub use three_phase_span::ThreePhaseSpan;
pub use timed_animation::TimedAnimation;
pub use torus_knot::TorusKnot;
pub use trajectory::Trajectory;
pub use viviani::Viviani;
pub use wave_trail::WaveTrail;

/// An eased, timed rotation animation - the shortest-arc lerp
/// between two [`RotationRad`] values.
pub type RotationAnimation = TimedAnimation<Span<RotationRad>>;

/// An eased, timed zoom animation - the plain lerp between two
/// [`ScaleFactor`] values.
pub type ZoomAnimation = TimedAnimation<Span<ScaleFactor>>;

/// A pulse animation - zoom out, hold, zoom back in via
/// [`ThreePhaseSpan`].
pub type PulseAnimation = TimedAnimation<ThreePhaseSpan<ScaleFactor>>;

/// An eased, timed animation of any vector value - position, size
/// or offset, in whatever coordinate space `U` tags.
pub type Vec2Animation<U> = TimedAnimation<Span<Vec2<U>>>;

/// A rotation-zoom composite - shortest-arc rotation plus a
/// three-phase zoom (out, hold, back in) on one clock.
pub type RotationZoomAnimation = TimedAnimation<(Span<RotationRad>, ThreePhaseSpan<ScaleFactor>)>;
