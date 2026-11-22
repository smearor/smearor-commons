use crate::FxContext;
use crate::FxError;
use crate::ParamKind;
use crate::ParamSpec;
use crate::ParamValue;
use crate::WindowEffect;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_buffer::BufferViewMut;
use smearor_commons_buffer::Rgba;
use smearor_commons_color::Color;
use smearor_commons_geometry::Vec2;

/// The window is (un)painted by a brush stroke sweep, with a soft
/// drop shadow trailing the painted edge.
///
/// Behavioral spec (source shader: `paint-brush.frag`, re-implemented
/// from semantics): the brush texture's channel `brush_size - 1`
/// holds a sweep-time field per pixel; `getMask` is `step(brush,
/// progress)` for `Open` and `step(progress, brush)` for `Close`.
/// Masked-out pixels accumulate a 10-step downward drop shadow of the
/// remaining window (`SHADOW_WIDTH 0.03`, alpha `* 0.2`).
///
/// `ctx.assets.brush` overrides the procedural sweep
/// (BMW `brush.png`: channels 0..3 = brush sizes 1..4).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct PaintBrush {
    /// Brush thickness - schema `paint-brush-size`, default `3`.
    #[serde(default = "PaintBrush::default_size")]
    pub size: u32,
}

impl PaintBrush {
    const fn default_size() -> u32 {
        3
    }
}

impl Default for PaintBrush {
    fn default() -> Self {
        Self { size: Self::default_size() }
    }
}

const PARAMS: &[ParamSpec] = &[ParamSpec {
    key: "size",
    label: "Brush Size",
    kind: ParamKind::Int { min: 1, max: 4, default: 3 },
}];

const SHADOW_WIDTH: f64 = 0.03;
const SHADOW_STEPS: usize = 10;

impl WindowEffect for PaintBrush {
    fn nick(&self) -> &'static str {
        "paint-brush"
    }

    fn label(&self) -> &'static str {
        "Paint Brush"
    }

    fn params(&self) -> &'static [ParamSpec] {
        PARAMS
    }

    fn values(&self) -> Vec<ParamValue> {
        vec![ParamValue::Int(self.size as i64)]
    }

    fn set_values(&mut self, values: &[ParamValue]) -> Result<(), FxError> {
        match values {
            [ParamValue::Int(size)] => {
                self.size = (*size).clamp(1, 4) as u32;
                Ok(())
            }
            _ => Err(FxError::ParamMismatch(format!("paint-brush takes 1 int parameter, got {}", values.len()))),
        }
    }

    fn apply(&self, ctx: &FxContext<'_>, out: &mut BufferViewMut<'_, Rgba>) {
        let size = ctx.window.size();
        let size_px = Vec2::<()>::new(size.x as f64, size.y as f64);
        let opening = matches!(ctx.direction, crate::Direction::Open);
        let progress = ctx.progress.as_f64();
        let channel = (self.size.clamp(1, 4) - 1) as usize;
        let window = ctx.window;
        let brush_texture = ctx.assets.brush;
        // The sweep-time field at `uv` - texture channel or
        // procedural fallback.
        let sweep = |uv: Vec2<()>| match brush_texture {
            Some(t) => {
                let s = t.sample_bilinear(uv);
                [s.r, s.g, s.b, s.a][channel]
            }
            None => crate::assets::procedural::brush(uv, channel),
        };
        let mask = |uv: Vec2<()>| {
            let brush = sweep(uv);
            if opening {
                (progress >= brush) as u8 as f64
            } else {
                (brush >= progress) as u8 as f64
            }
        };
        crate::render::fill(out, |x, y| {
            let uv = Vec2::<()>::new((x as f64 + 0.5) / size_px.x, (y as f64 + 0.5) / size_px.y);
            let mut c = window.sample_bilinear(uv);
            if mask(uv) == 0.0 {
                // Drop shadow below the remaining parts.
                for i in 0..SHADOW_STEPS {
                    let shadow_uv = Vec2::<()>::new(uv.x, uv.y - (i as f64) * SHADOW_WIDTH / SHADOW_STEPS as f64);
                    let shadow_alpha = (1.0 - i as f64 / SHADOW_STEPS as f64) * mask(shadow_uv) * window.sample_bilinear(shadow_uv).a;
                    if shadow_alpha > 0.0 {
                        c = Color::<f64>::new_rgba(0.0, 0.0, 0.0, shadow_alpha * 0.2);
                        break;
                    }
                    if i == SHADOW_STEPS - 1 {
                        c.a = 0.0;
                    }
                }
            }
            c
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Direction;
    use crate::Layer;
    use smearor_commons_buffer::PixelBuffer;
    use smearor_commons_geometry::Fraction;
    use smearor_commons_geometry::Pixels;
    use smearor_commons_geometry::Rect;
    use smearor_commons_geometry::Size;
    use smearor_commons_noise::Seed;
    use std::time::Duration;

    fn solid(size: u32) -> PixelBuffer<Rgba> {
        let mut buffer = PixelBuffer::<Rgba>::new(Size::new(size, size));
        let mut view = buffer.view_mut();
        let c = Color::<f64>::new_rgba(0.5, 0.5, 0.5, 1.0);
        for y in 0..size {
            for x in 0..size {
                view.put_color(Pixels::new(x), Pixels::new(y), c);
            }
        }
        buffer
    }

    fn ctx_of<'a>(window: &'a PixelBuffer<Rgba>, progress: f64, direction: Direction) -> FxContext<'a> {
        let size = window.size();
        FxContext {
            progress: Fraction::new(progress),
            duration: Duration::from_secs_f64(2.0),
            seed: Seed(0xABCD_EF01_2345_6789),
            direction,
            layer: Layer::Window,
            window: window.view(),
            surface: None,
            assets: crate::FxAssets::default(),
            pointer: None,
            fullscreen: false,
            padding: Pixels::new(0.0),
            window_rect: Rect::from_xywh(0.0, 0.0, f64::from(size.x), f64::from(size.y)),
        }
    }

    /// Open end fully painted, close end mostly erased.
    #[test]
    fn endpoints() {
        let window = solid(32);
        let mut out = PixelBuffer::<Rgba>::new(Size::new(32, 32));
        let fx = PaintBrush::default();
        fx.apply(&ctx_of(&window, 1.0, Direction::Open), &mut out.view_mut());
        assert_eq!(out.view().sample_bilinear(Vec2::<()>::new(0.5, 0.5)).a, 1.0);
        // The sweep erases left-to-right: mid-close the left side is
        // more gone than the right side.
        fx.apply(&ctx_of(&window, 0.5, Direction::Close), &mut out.view_mut());
        let left = out.view().sample_bilinear(Vec2::<()>::new(0.1, 0.5)).a;
        let right = out.view().sample_bilinear(Vec2::<()>::new(0.9, 0.5)).a;
        assert!(left < right, "left {left} should trail right {right}");
    }
}
