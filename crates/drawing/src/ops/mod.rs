//! The drawing capability traits and the [`DrawOps`] umbrella trait.
//!
//! The rendering vocabulary is split into per-capability traits so that
//! backends can implement them incrementally and consumers can bound on
//! exactly the capabilities they need:
//!
//! - [`DrawBackend`]: the per-target error type shared by all capability traits
//! - [`DrawPrimitiveOps`]: surface fills and shape primitives
//! - [`DrawIconOps`]: icon and glyph rendering
//! - [`DrawTextOps`]: label and centered text rendering
//! - [`DrawProgressOps`]: progress and gauge indicators
//!
//! [`DrawOps`] bundles all of them as a supertrait with a blanket impl —
//! use it as a bound when a consumer needs the full vocabulary. Method
//! calls still require the declaring trait in scope; importing
//! [`crate::prelude`] brings in all capability traits at once.
//!
//! Every capability trait is also implemented for `&mut T` where `T`
//! implements it, so mutable references compose transparently — e.g.
//! `&mut PixelBuffer` can serve as a [`DrawTarget`].

mod backend;
mod icon;
mod primitive;
mod progress;
mod target;
mod text;

pub use backend::DrawBackend;
pub use icon::Codepoint;
pub use icon::DrawIconOps;
pub use icon::Icon;
pub use icon::IconGrid;
pub use primitive::Cap;
pub use primitive::DrawPrimitiveOps;
pub use primitive::FillCircle;
pub use primitive::FillPolygon;
pub use primitive::FillRect;
pub use primitive::FillSector;
pub use primitive::FillSectors;
pub use primitive::StrokeArc;
pub use primitive::StrokeCircle;
pub use primitive::StrokePolygon;
pub use primitive::StrokePolyline;
pub use primitive::StrokeRect;
pub use progress::DrawProgressOps;
pub use progress::Gauge;
pub use progress::PROGRESS_BAR_BOTTOM_MARGIN;
pub use progress::ProgressBar;
pub use target::DrawTarget;
pub use text::DrawTextOps;

/// Umbrella trait bundling all drawing capability traits.
///
/// Blanket-implemented for every type that implements all capability
/// traits; useful as a bound (`T: DrawOps`) when a consumer needs the
/// full vocabulary. It declares no methods itself — import the
/// individual capability traits (or [`crate::prelude`]) to call them.
pub trait DrawOps: DrawIconOps + DrawPrimitiveOps + DrawProgressOps + DrawTextOps {}

impl<T> DrawOps for T where T: DrawIconOps + DrawPrimitiveOps + DrawProgressOps + DrawTextOps {}
