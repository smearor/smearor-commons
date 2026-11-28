//! GTK adapter - attaches the headless effect engine to GTK widgets.
//!
//! Three pieces ([phase 5](../../../concepts/window-effects.md)):
//!
//! * [`snapshot`] - renders a widget into a frozen
//!   [`PixelBuffer<Rgba>`] texture once per animation.
//! * [`FxPlayer`] - a `DrawingArea` subclass that owns the clock and
//!   buffers and plays any [`EffectSpec`].
//! * [`SurfaceProvider`] - the callback layered
//!   (`Layers::SurfaceAndWindow`) effects pull their backdrop texture
//!   from.
//!
//! The adapter never owns the real widget: hiding/showing it around
//! the animation stays consumer policy. The `fx.rs` experimenter is
//! the working reference for the same frame pipeline.

mod player;
mod pointer;
mod snapshot;
mod surface;

pub use player::FxPlayOptions;
pub use player::FxPlayer;
pub use snapshot::snapshot;
pub use snapshot::snapshot_to_png;
pub use surface::SurfaceProvider;
