use crate::Direction;
use crate::Layer;
use smearor_commons_buffer::BufferView;
use smearor_commons_buffer::Rgba;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Rect;
use smearor_commons_geometry::Vec2;
use smearor_commons_noise::Seed;
use std::time::Duration;

/// The uniform block one effect sees for one frame.
///
/// Every field is a deliberate analogue of a Burn-My-Windows uniform
/// (documented per field); the consumer assembles the context per pass
/// and the crate stays clock-free - progress arrives pre-eased.
pub struct FxContext<'a> {
    /// Linear animation progress `0..=1` - the `TimedAnimation`
    /// applies easing before handing this over (BMW's `uProgress`).
    pub progress: Fraction,

    /// The wall-clock run duration - BMW's `uDuration` (some shaders
    /// scale movement speed by it).
    pub duration: Duration,

    /// Deterministic per-run seed - BMW's `Math.random()` uniform.
    /// Fix it to replay an animation bitwise.
    pub seed: Seed,

    /// Open or close - BMW's `uForOpening`.
    pub direction: Direction,

    /// Which layer this pass renders - BMW's `uLayer`
    /// ([`Layer::Window`] for single-layer effects).
    pub layer: Layer,

    /// The animated content - BMW's `sampler2D`, straight-alpha RGBA.
    /// `window.size()` is BMW's `uSize`; it is not a separate field.
    pub window: BufferView<'a, Rgba>,

    /// What sits behind the content for layered effects (BMW's
    /// `uLayer = 0` source) - `None` for [`crate::Layers::WindowOnly`].
    pub surface: Option<BufferView<'a, Rgba>>,

    /// Pointer position in uv space - BMW's `broken-glass-use-pointer`
    /// (`uEpicenter`).
    pub pointer: Option<Vec2<()>>,

    /// Whether the content covers the whole output - BMW's
    /// `uIsFullscreen` (effects may clamp edge fades differently).
    pub fullscreen: bool,

    /// Shadow/decoration margin around the content in px - BMW's
    /// `uPadding`. `ops::absolute_edge_mask` subtracts it before
    /// fading; plain widget snapshots pass `0.0`.
    pub padding: Pixels<f64>,

    /// Visible window rect in layer pixels - BMW's `uFrame`/
    /// `uFrameRect`/`uCenter`/`uAxes`/`uFrameHalf`. Single-layer
    /// consumers pass the full buffer rect; layered effects use it to
    /// find the content inside `window`/`surface`.
    pub window_rect: Rect<Pixels, f64>,
}
