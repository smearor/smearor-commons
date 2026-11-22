use serde::Deserialize;
use serde::Serialize;

/// What a [`crate::WindowEffect`] needs beyond the animated texture.
///
/// `SurfaceAndWindow` makes the consumer run two passes - `Surface`
/// first (the effect composes backdrop + content), then `Window` -
/// and supply `ctx.surface` (BMW's `LayeredEffect`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum Layers {
    /// The effect only reads `ctx.window` - one pass.
    #[default]
    WindowOnly,
    /// The effect renders the surface pass and the window pass -
    /// the consumer supplies `ctx.surface` and runs both.
    SurfaceAndWindow,
}
