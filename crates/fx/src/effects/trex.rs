use crate::FxContext;
use crate::FxError;
use crate::ParamKind;
use crate::ParamSpec;
use crate::ParamValue;
use crate::WindowEffect;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_animation::EasingFunction;
use smearor_commons_buffer::BufferViewMut;
use smearor_commons_buffer::Rgba;
use smearor_commons_color::Color;
use smearor_commons_geometry::Factor;
use smearor_commons_geometry::Vec2;
use smearor_commons_noise::Hash;

/// T-Rex claw attack: the window is slashed open by timed scratch
/// marks with a flash at the wound.
///
/// Behavioral spec (source shader: `trex.frag`, re-implemented from
/// semantics): `progress` is `easeOutQuad`, inverted for `Open`. The
/// window uv warps radially (`pow(dist, 1+warp)`) and scales back; N
/// claw layers each sample a scratch map through a cell-randomized
/// grid (`getClawUV`) at staggered delays - the map's `r` channel is
/// the per-pixel scratch time, `scratchMap > progress` keeps the
/// window; `dFdx/dFdy(scratchMap)` (finite differences here) adds a
/// fold distortion; a `uFlashColor` flash hugs the wound edge;
/// `sqrt(1 - fade²)` fades survivors at the end.
///
/// `ctx.assets.claw` overrides the procedural scratch map
/// (BMW `claws.png`).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Trex {
    /// Scratch flash color - schema `trex-scratch_color`, default `rgb(170, 0, 0)`.
    #[serde(default = "Trex::default_color")]
    pub color: Color<f64>,
    /// Claw mark scale - schema `trex-scratch_scale`, default `1.0`.
    #[serde(default = "Trex::default_scale")]
    pub scale: Factor,
    /// Radial blow-up intensity - schema `trex-scratch_warp`, default `0.15`.
    #[serde(default = "Trex::default_warp")]
    pub warp: Factor,
    /// Number of claw strikes - schema `trex-scratch_count`, default `5`.
    #[serde(default = "Trex::default_count")]
    pub count: u32,
}

impl Trex {
    const fn default_color() -> Color<f64> {
        Color::<f64>::new(170.0 / 255.0, 0.0, 0.0)
    }
    const fn default_scale() -> Factor {
        Factor::new(1.0)
    }
    const fn default_warp() -> Factor {
        Factor::new(0.15)
    }
    const fn default_count() -> u32 {
        5
    }
}

impl Default for Trex {
    fn default() -> Self {
        Self {
            color: Self::default_color(),
            scale: Self::default_scale(),
            warp: Self::default_warp(),
            count: Self::default_count(),
        }
    }
}

const PARAMS: &[ParamSpec] = &[
    ParamSpec {
        key: "color",
        label: "Scratch Color",
        kind: ParamKind::Color {
            default: Trex::default_color(),
        },
    },
    ParamSpec {
        key: "scale",
        label: "Scale",
        kind: ParamKind::Float {
            min: 0.1,
            max: 5.0,
            step: 0.1,
            default: 1.0,
        },
    },
    ParamSpec {
        key: "warp",
        label: "Warp",
        kind: ParamKind::Float {
            min: 0.0,
            max: 1.0,
            step: 0.05,
            default: 0.15,
        },
    },
    ParamSpec {
        key: "count",
        label: "Claw Count",
        kind: ParamKind::Int { min: 1, max: 10, default: 5 },
    },
];

const FLASH_INTENSITY: f64 = 0.1;
const MAX_SPAWN_TIME: f64 = 0.6;
const FF_TIME: f64 = 0.6;

impl WindowEffect for Trex {
    fn nick(&self) -> &'static str {
        "trex"
    }

    fn label(&self) -> &'static str {
        "T-Rex"
    }

    fn params(&self) -> &'static [ParamSpec] {
        PARAMS
    }

    fn values(&self) -> Vec<ParamValue> {
        vec![
            ParamValue::Color(self.color),
            ParamValue::Float(self.scale.0),
            ParamValue::Float(self.warp.0),
            ParamValue::Int(self.count as i64),
        ]
    }

    fn set_values(&mut self, values: &[ParamValue]) -> Result<(), FxError> {
        let mismatch = |index: usize| FxError::ParamMismatch(format!("trex param {index}"));
        if values.len() != 4 {
            return Err(FxError::ParamMismatch(format!("trex takes 4 parameters, got {}", values.len())));
        }
        self.color = values[0].as_color().ok_or(mismatch(0))?;
        self.scale = Factor::new(values[1].as_f64().ok_or(mismatch(1))?.max(0.01));
        self.warp = Factor::new(values[2].as_f64().ok_or(mismatch(2))?);
        self.count = values[3].as_i64().ok_or(mismatch(3))?.max(1) as u32;
        Ok(())
    }

    fn apply(&self, ctx: &FxContext<'_>, out: &mut BufferViewMut<'_, Rgba>) {
        let size = ctx.window.size();
        let size_px = Vec2::<()>::new(size.x as f64, size.y as f64);
        let opening = matches!(ctx.direction, crate::Direction::Open);
        let seed = Hash::<Vec2<()>>::hash(&ctx.seed.as_f64());
        let progress = EasingFunction::EaseOut.apply(ctx.progress).as_f64();
        let num_claws = self.count.max(1) as f64;
        let grid_scale = 1.0 / self.scale.0;
        let warp = self.warp.0;
        let flash_color = self.color;
        let window = ctx.window;
        let claw_texture = ctx.assets.claw;
        // The aspect fix BMW applies inside getClawUV.
        let aspect_fix = if size_px.x < size_px.y {
            Vec2::<()>::new(1.0, size_px.y / size_px.x)
        } else {
            Vec2::<()>::new(size_px.x / size_px.y, 1.0)
        };
        // getClawUV: cell-randomized sample coords for claw layer `i`.
        let claw_uv = |coords: Vec2<()>, i: f64| {
            let layer_seed = seed * (i + 1.0);
            let offset = Hash::<Vec2<()>>::hash(&layer_seed);
            let coords = Vec2::<()>::new((coords.x + offset.x) * aspect_fix.x * grid_scale, (coords.y + offset.y) * aspect_fix.y * grid_scale);
            let mut cell_uv = Vec2::<()>::new(crate::fract(coords.x), crate::fract(coords.y));
            let cell_id = Vec2::<()>::new(coords.x - cell_uv.x + 362.456, coords.y - cell_uv.y + 362.456);
            let hash12 = |k: f64| Hash::<f64>::hash(&Vec2::<()>::new(cell_id.x * layer_seed.x * k, cell_id.y * layer_seed.y * k));
            let scale = crate::mix(0.8, 1.0, hash12(134.451));
            let offset_x = crate::mix(0.0, 1.0 - scale, hash12(54.4129));
            let offset_y = crate::mix(0.0, 1.0 - scale, hash12(25.3089));
            let rotation = crate::mix(0.0, std::f64::consts::TAU, hash12(2.99837));
            cell_uv = Vec2::<()>::new((cell_uv.x - offset_x) / scale - 0.5, (cell_uv.y - offset_y) / scale - 0.5);
            let (sin, cos) = rotation.sin_cos();
            cell_uv = Vec2::<()>::new(cell_uv.x * cos - cell_uv.y * sin + 0.5, cell_uv.x * sin + cell_uv.y * cos + 0.5);
            Vec2::<()>::new(cell_uv.x.clamp(0.0, 1.0), cell_uv.y.clamp(0.0, 1.0))
        };
        let scratch_map = |coords: Vec2<()>| {
            let mut map = 1.0_f64;
            for i in 0..num_claws as usize {
                let i = i as f64;
                let uv = claw_uv(coords, i);
                let r = match claw_texture {
                    Some(t) => t.sample_bilinear(uv).r,
                    None => crate::assets::procedural::claw(uv),
                };
                map = map.min((r + i / num_claws * MAX_SPAWN_TIME).clamp(0.0, 1.0));
            }
            map
        };
        crate::render::fill(out, |x, y| {
            let uv0 = Vec2::<()>::new((x as f64 + 0.5) / size_px.x, (y as f64 + 0.5) / size_px.y);
            let progress = if opening { 1.0 - progress } else { progress };
            // Radial blow-up warp.
            let c = Vec2::<()>::new(uv0.x * 2.0 - 1.0, uv0.y * 2.0 - 1.0);
            let dist = (c.x * c.x + c.y * c.y).sqrt();
            let warped = if dist > 1e-9 {
                let f = dist.powf(1.0 + warp) / dist * 0.5;
                Vec2::<()>::new(c.x * f + 0.5, c.y * f + 0.5)
            } else {
                Vec2::<()>::new(0.5, 0.5)
            };
            let mut coords = Vec2::<()>::new(crate::mix(uv0.x, warped.x, progress), crate::mix(uv0.y, warped.y, progress));
            let scale = 0.5 * warp * (1.0 - progress);
            coords = Vec2::<()>::new(coords.x * (scale + 1.0) - scale * 0.5, coords.y * (scale + 1.0) - scale * 0.5);
            let scratch = scratch_map(coords);
            // Fold distortion - finite differences of the scratch map
            // (GLSL dFdx/dFdy), scaled by the pixel size.
            let du = 1.0 / size_px.x;
            let dv = 1.0 / size_px.y;
            let offset = Vec2::<()>::new(
                (scratch_map(coords + Vec2::<()>::new(du, 0.0)) - scratch) * progress * 0.5,
                (scratch_map(coords + Vec2::<()>::new(0.0, dv)) - scratch) * progress * 0.5,
            );
            let mut c = window.sample_bilinear(coords + offset);
            // Flash at the wound edge.
            let mut flash_intensity = 1.0 / FLASH_INTENSITY * (scratch - progress) + 1.0;
            if !(0.0..1.0).contains(&flash_intensity) {
                flash_intensity = 0.0;
            }
            let flash = Color::<f64>::new_rgba(flash_color.r, flash_color.g, flash_color.b, flash_color.a * flash_intensity * c.a * (1.0 - progress));
            c.a *= if scratch > progress { 1.0 } else { 0.0 };
            let mut c = crate::alpha_over(c, flash);
            // Final sqrt fade.
            let fade = crate::smoothstep(0.0, 1.0, (progress - 1.0 + FF_TIME) / FF_TIME);
            c.a *= (1.0 - fade * fade).sqrt();
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

    /// Open end untouched, close end scratched away.
    #[test]
    fn endpoints() {
        let window = solid(32);
        let mut out = PixelBuffer::<Rgba>::new(Size::new(32, 32));
        let fx = Trex::default();
        fx.apply(&ctx_of(&window, 1.0, Direction::Open), &mut out.view_mut());
        assert_eq!(out.view().sample_bilinear(Vec2::<()>::new(0.5, 0.5)).a, 1.0);
        fx.apply(&ctx_of(&window, 1.0, Direction::Close), &mut out.view_mut());
        assert_eq!(out.view().sample_bilinear(Vec2::<()>::new(0.5, 0.5)).a, 0.0);
    }
}
