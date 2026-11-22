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
use smearor_commons_geometry::Factor;
use smearor_commons_geometry::Vec2;
use smearor_commons_noise::Hash;

/// The window shatters into shards that blow apart and sink -
/// broken glass from an epicenter.
///
/// Behavioral spec (source shader: `broken-glass.frag`, re-implemented
/// from semantics): five shard layers each blow out from `epicenter`
/// (`uBlowForce` scale, alternating rotation, `gravity` drift - the
/// transforms compose into the lookup coords); the shard map's `g`
/// bins shards into layers, `r - (progress + 0.1)² > 0` keeps the
/// shard; `progress` inverts for `Open`. `epicenter` is the pointer uv
/// when `use_pointer` on close (schema `broken-glass-use-pointer`,
/// `BrokenGlass.js`), else the window center.
///
/// `ctx.assets.shard` overrides the procedural Voronoi shard map
/// (BMW `shards.png`: `r` = per-shard random, `g` = layer id).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct BrokenGlass {
    /// Shard map scale - schema `broken-glass-scale`, default `1.0`.
    #[serde(default = "BrokenGlass::default_scale")]
    pub scale: Factor,
    /// Downward drift - schema `broken-glass-gravity`, default `-1.0`.
    #[serde(default = "BrokenGlass::default_gravity")]
    pub gravity: Factor,
    /// Blow-apart force - schema `broken-glass-blow_force`, default `0.5`.
    #[serde(default = "BrokenGlass::default_blow_force")]
    pub blow_force: Factor,
    /// Epicenter at the pointer on close - schema
    /// `broken-glass-use_pointer`, default `false`.
    #[serde(default)]
    pub use_pointer: bool,
}

impl BrokenGlass {
    const fn default_scale() -> Factor {
        Factor::new(1.0)
    }
    const fn default_gravity() -> Factor {
        Factor::new(-1.0)
    }
    const fn default_blow_force() -> Factor {
        Factor::new(0.5)
    }
}

impl Default for BrokenGlass {
    fn default() -> Self {
        Self {
            scale: Self::default_scale(),
            gravity: Self::default_gravity(),
            blow_force: Self::default_blow_force(),
            use_pointer: false,
        }
    }
}

const PARAMS: &[ParamSpec] = &[
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
        key: "gravity",
        label: "Gravity",
        kind: ParamKind::Float {
            min: -5.0,
            max: 5.0,
            step: 0.1,
            default: -1.0,
        },
    },
    ParamSpec {
        key: "blow_force",
        label: "Blow Force",
        kind: ParamKind::Float {
            min: 0.0,
            max: 2.0,
            step: 0.1,
            default: 0.5,
        },
    },
    ParamSpec {
        key: "use_pointer",
        label: "Use Pointer",
        kind: ParamKind::Toggle { default: false },
    },
];

const SHARD_LAYERS: usize = 5;
const ACTOR_SCALE: f64 = 2.0;
const PADDING: f64 = ACTOR_SCALE / 2.0 - 0.5;

impl WindowEffect for BrokenGlass {
    fn nick(&self) -> &'static str {
        "broken-glass"
    }

    fn label(&self) -> &'static str {
        "Broken Glass"
    }

    fn params(&self) -> &'static [ParamSpec] {
        PARAMS
    }

    fn values(&self) -> Vec<ParamValue> {
        vec![
            ParamValue::Float(self.scale.0),
            ParamValue::Float(self.gravity.0),
            ParamValue::Float(self.blow_force.0),
            ParamValue::Bool(self.use_pointer),
        ]
    }

    fn set_values(&mut self, values: &[ParamValue]) -> Result<(), FxError> {
        let mismatch = |index: usize| FxError::ParamMismatch(format!("broken-glass param {index}"));
        if values.len() != 4 {
            return Err(FxError::ParamMismatch(format!("broken-glass takes 4 parameters, got {}", values.len())));
        }
        self.scale = Factor::new(values[0].as_f64().ok_or(mismatch(0))?.max(0.01));
        self.gravity = Factor::new(values[1].as_f64().ok_or(mismatch(1))?);
        self.blow_force = Factor::new(values[2].as_f64().ok_or(mismatch(2))?);
        self.use_pointer = values[3].as_bool().ok_or(mismatch(3))?;
        Ok(())
    }

    fn apply(&self, ctx: &FxContext<'_>, out: &mut BufferViewMut<'_, Rgba>) {
        let size = ctx.window.size();
        let size_px = Vec2::<()>::new(size.x as f64, size.y as f64);
        let opening = matches!(ctx.direction, crate::Direction::Open);
        let seed = Hash::<Vec2<()>>::hash(&ctx.seed.as_f64());
        // Pointer epicenter only applies when closing (BrokenGlass.js).
        let epicenter = if self.use_pointer && !opening {
            ctx.pointer.unwrap_or(Vec2::<()>::new(0.5, 0.5))
        } else {
            Vec2::<()>::new(0.5, 0.5)
        };
        let progress = ctx.progress.as_f64();
        let shard_scale = self.scale.0;
        let gravity_f = self.gravity.0;
        let blow = self.blow_force.0;
        let window = ctx.window;
        let shard_texture = ctx.assets.shard;
        crate::render::fill(out, |x, y| {
            let uv0 = Vec2::<()>::new((x as f64 + 0.5) / size_px.x, (y as f64 + 0.5) / size_px.y);
            let progress = if opening { 1.0 - progress } else { progress };
            let mut c = Color::<f64>::new_rgba(0.0, 0.0, 0.0, 0.0);
            for i in 0..SHARD_LAYERS {
                let i = i as f64;
                // Undo the ACTOR_SCALE margin, then scale/rotate/fall
                // around the epicenter.
                let mut coords = Vec2::<()>::new(uv0.x * ACTOR_SCALE - PADDING - epicenter.x, uv0.y * ACTOR_SCALE - PADDING - epicenter.y);
                let blow_scale = crate::mix(1.0, 1.0 + blow * (i + 2.0) / SHARD_LAYERS as f64, progress);
                coords = Vec2::<()>::new(coords.x / blow_scale, coords.y / blow_scale);
                let rotation = (i % 2.0 - 0.5) * 0.2 * progress;
                let (sin, cos) = rotation.sin_cos();
                coords = Vec2::<()>::new(coords.x * cos - coords.y * sin, coords.x * sin + coords.y * cos);
                let gravity = if opening { -1.0 } else { 1.0 } * gravity_f * 0.1 * (i + 1.0) * progress * progress;
                coords = Vec2::<()>::new(coords.x, coords.y + gravity);
                coords = Vec2::<()>::new(coords.x + epicenter.x, coords.y + epicenter.y);
                // Shard map lookup + layer binning.
                let shard_uv = Vec2::<()>::new((coords.x + seed.x) * size_px.x / shard_scale / 500.0, (coords.y + seed.y) * size_px.y / shard_scale / 500.0);
                let (shard_r, shard_g) = match shard_texture {
                    Some(t) => {
                        let s = t.sample_bilinear(shard_uv);
                        (s.r, s.g)
                    }
                    None => crate::assets::procedural::shard(shard_uv),
                };
                let shard_group = (shard_g * SHARD_LAYERS as f64 * 0.999).floor();
                if shard_group == i && shard_r - (progress + 0.1).powi(2) > 0.0 {
                    c = window.sample_bilinear(coords);
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

    /// Open end reconstructed, close end fully shattered.
    #[test]
    fn endpoints() {
        let window = solid(32);
        let mut out = PixelBuffer::<Rgba>::new(Size::new(32, 32));
        let fx = BrokenGlass::default();
        fx.apply(&ctx_of(&window, 1.0, Direction::Open), &mut out.view_mut());
        assert_eq!(out.view().sample_bilinear(Vec2::<()>::new(0.5, 0.5)).a, 1.0);
        fx.apply(&ctx_of(&window, 1.0, Direction::Close), &mut out.view_mut());
        assert_eq!(out.view().sample_bilinear(Vec2::<()>::new(0.5, 0.5)).a, 0.0);
    }

    /// Mid-shatter shards scatter - the frame differs from the window.
    #[test]
    fn shards_scatter() {
        let window = solid(64);
        let mut out = PixelBuffer::<Rgba>::new(Size::new(64, 64));
        BrokenGlass::default().apply(&ctx_of(&window, 0.5, Direction::Close), &mut out.view_mut());
        let view = out.view();
        let mut gone = 0;
        for y in 0..64 {
            for x in 0..64 {
                if view.sample_bilinear(Vec2::<()>::new((x as f64 + 0.5) / 64.0, (y as f64 + 0.5) / 64.0)).a < 0.5 {
                    gone += 1
                }
            }
        }
        assert!(gone > 0, "shards should leave holes");
    }
}
