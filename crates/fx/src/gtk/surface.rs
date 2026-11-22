//! The surface-source abstraction for layered effects.

use smearor_commons_buffer::PixelBuffer;
use smearor_commons_buffer::Rgba;

/// Supplies the `ctx.surface` texture for
/// [`crate::Layers::SurfaceAndWindow`] effects.
///
/// Called once per rendered frame by [`crate::gtk::FxPlayer`], so the
/// provider may return a fresh snapshot of whatever sits under the
/// animated window each frame (e.g. the floating board under a card).
/// Returning `None` degrades the layered pass to transparent surface
/// reads - the same contract as [`crate::WindowEffect::check`]
/// reporting [`crate::FxError::MissingSurface`].
pub type SurfaceProvider = Box<dyn Fn() -> Option<PixelBuffer<Rgba>>>;
