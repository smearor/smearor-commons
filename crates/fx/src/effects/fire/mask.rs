use crate::FxContext;
use smearor_commons_animation::EasingFunction;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Vec2;

/// The dual-phase mask pair `effectMask` computes: `window` fades the
/// window along the burn direction, `effect` gates the flame (strongest
/// at the burning edge).
#[derive(Debug, Clone, Copy)]
pub(super) struct EffectMask {
    /// Alpha multiplier for the window texture.
    pub(super) window: f64,
    /// Alpha multiplier for the fire front.
    pub(super) effect: f64,
}

/// Edge fade width in pixels - `fire.frag`'s `EDGE_FADE`.
const EDGE_FADE: f64 = 70.0;
/// Relative width of the window-dissolve gradient - `FADE_WIDTH`.
const FADE_WIDTH: f64 = 0.1;
/// The progress fraction spent burning the window - `HIDE_TIME`.
const HIDE_TIME: f64 = 0.4;

impl EffectMask {
    /// The paired window/effect masks at pixel `px` - the `effectMask`
    /// helper's semantics:
    ///
    /// `progress` is `easeOutQuad`-eased inside. `burn` covers the
    /// first `HIDE_TIME` of the animation (the window dissolves), the
    /// remaining `1 - HIDE_TIME` is after-burn (the flame fades). The
    /// burn front runs along `t` - `uv.y` shrunk by `FADE_WIDTH`, or
    /// `1 - uv.y` when `bottom_to_top`. `window` is `1` above the
    /// front; `effect` is densest at the front and fades by a sqrt
    /// curve after the window is gone. The frame edges are softened by
    /// `absolute_edge_mask`. For `Open` the window mask inverts - the
    /// window un-burns.
    pub(super) fn at(ctx: &FxContext<'_>, bottom_to_top: bool, px: Vec2<()>, size_px: Vec2<()>) -> Self {
        let progress = EasingFunction::EaseOut.apply(ctx.progress).as_f64();
        let burn = (progress / HIDE_TIME).clamp(0.0, 1.0);
        let after_burn = ((progress - HIDE_TIME) / (1.0 - HIDE_TIME)).clamp(0.0, 1.0);

        let y01 = px.y / size_px.y;
        let t = if bottom_to_top { 1.0 - y01 } else { y01 } * (1.0 - FADE_WIDTH);

        let mut window = 1.0 - ((burn - t) / FADE_WIDTH).clamp(0.0, 1.0);
        let mut effect = (t * (1.0 - window) / burn.max(f64::EPSILON)).clamp(0.0, 1.0);

        if progress > HIDE_TIME {
            let fade = (1.0 - after_burn * after_burn).sqrt();
            effect *= crate::mix(1.0, 1.0 - t, after_burn) * fade;
        }

        effect *= crate::absolute_edge_mask(px, size_px, ctx.padding, Pixels::new(EDGE_FADE), 0.5, ctx.fullscreen);

        if ctx.direction == crate::Direction::Open {
            window = 1.0 - window;
        }

        EffectMask { window, effect }
    }
}
