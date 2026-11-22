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
use smearor_commons_noise::Field;
use smearor_commons_noise::Hash;
use smearor_commons_noise::Simplex2D;

/// Thanos-snap disintegration: the window crumbles into four
/// wind-blown dust layers.
///
/// Behavioral spec (source shader: `snap.frag`, re-implemented from
/// semantics): progress is *un*inverted for `Open` (unlike most
/// effects). A vertical+noise `progress` field (`^2`) is the per-layer
/// dissolve threshold against the dust map's `r` channel; each of 4
/// layers gets a seed-rotated grow/shrink direction (winding-flipped
/// per side, flipped for half the layers) plus a small wind drift;
/// `dustMap.g` bins particles into layers. `uDustColor` tints the
/// window toward dust as progress grows.
///
/// `ctx.assets.dust` overrides the procedural Voronoi dust map
/// (BMW `dust.png`: `r` = dissolve threshold, `g` = layer id).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Snap {
    /// Dust particle scale - schema `snap-scale`, default `3.0`.
    #[serde(default = "Snap::default_scale")]
    pub scale: Factor,
    /// Dust tint - schema `snap-color`, default `rgb(50, 40, 35)`.
    #[serde(default = "Snap::default_color")]
    pub color: Color<f64>,
}

impl Snap {
    const fn default_scale() -> Factor {
        Factor::new(3.0)
    }
    const fn default_color() -> Color<f64> {
        Color::<f64>::new(50.0 / 255.0, 40.0 / 255.0, 35.0 / 255.0)
    }
}

impl Default for Snap {
    fn default() -> Self {
        Self {
            scale: Self::default_scale(),
            color: Self::default_color(),
        }
    }
}

const PARAMS: &[ParamSpec] = &[
    ParamSpec {
        key: "scale",
        label: "Scale",
        kind: ParamKind::Float {
            min: 0.5,
            max: 10.0,
            step: 0.5,
            default: 3.0,
        },
    },
    ParamSpec {
        key: "color",
        label: "Color",
        kind: ParamKind::Color {
            default: Snap::default_color(),
        },
    },
];

const DUST_LAYERS: f64 = 4.0;
const GROW_INTENSITY: f64 = 0.05;
const SHRINK_INTENSITY: f64 = 0.05;
const WIND_INTENSITY: f64 = 0.05;
const ACTOR_SCALE: f64 = 1.2;
const PADDING: f64 = ACTOR_SCALE / 2.0 - 0.5;

impl WindowEffect for Snap {
    fn nick(&self) -> &'static str {
        "snap"
    }

    fn label(&self) -> &'static str {
        "Snap"
    }

    fn params(&self) -> &'static [ParamSpec] {
        PARAMS
    }

    fn values(&self) -> Vec<ParamValue> {
        vec![ParamValue::Float(self.scale.0), ParamValue::Color(self.color)]
    }

    fn set_values(&mut self, values: &[ParamValue]) -> Result<(), FxError> {
        let mismatch = |index: usize| FxError::ParamMismatch(format!("snap param {index}"));
        if values.len() != 2 {
            return Err(FxError::ParamMismatch(format!("snap takes 2 parameters, got {}", values.len())));
        }
        self.scale = Factor::new(values[0].as_f64().ok_or(mismatch(0))?.max(0.01));
        self.color = values[1].as_color().ok_or(mismatch(1))?;
        Ok(())
    }

    fn apply(&self, ctx: &FxContext<'_>, out: &mut BufferViewMut<'_, Rgba>) {
        let size = ctx.window.size();
        let size_px = Vec2::<()>::new(size.x as f64, size.y as f64);
        let opening = matches!(ctx.direction, crate::Direction::Open);
        let seed = Hash::<Vec2<()>>::hash(&ctx.seed.as_f64());
        let raw_progress = ctx.progress.as_f64();
        let dust_scale = self.scale.0;
        let dust_color = self.color;
        let window = ctx.window;
        let dust_texture = ctx.assets.dust;
        crate::render::fill(out, |x, y| {
            let uv = Vec2::<()>::new((x as f64 + 0.5) / size_px.x, (y as f64 + 0.5) / size_px.y);
            // Un-inverted for opening (the snap runs 1 -> 0 there).
            let progress = if opening { raw_progress } else { 1.0 - raw_progress };
            let gradient = uv.y * ACTOR_SCALE - PADDING;
            let mut progress = 2.0 - gradient - 2.0 * progress;
            progress += 0.25 - 0.5 * Simplex2D.at((uv + seed) * 2.0);
            progress = progress.max(0.0).powi(2);
            let mut c = Color::<f64>::new_rgba(0.0, 0.0, 0.0, 0.0);
            for i in 0..DUST_LAYERS as usize {
                let i = i as f64;
                let factor = i / (DUST_LAYERS - 1.0);
                let angle = 123.123 * (seed.x + factor);
                let (sin, cos) = angle.sin_cos();
                let mut direction = Vec2::<()>::new(cos, sin);
                // coords relative to the actor center.
                let mut coords = Vec2::<()>::new(uv.x * ACTOR_SCALE - PADDING - 0.5, uv.y * ACTOR_SCALE - PADDING - 0.5);
                // Winding: flip the direction on one side of the window.
                if direction.x * coords.y - direction.y * coords.x > 0.0 {
                    direction = direction * -1.0;
                }
                if factor > 0.5 {
                    direction = direction * -1.0;
                }
                // Grow along, shrink orthogonally to the direction.
                let perp = Vec2::<()>::new(direction.y, -direction.x);
                let plen = (perp.x * perp.x + perp.y * perp.y).sqrt();
                let dist = ((0.0 - coords.x) * perp.x / plen + (0.0 - coords.y) * perp.y / plen).abs();
                let grow = direction * (dist * crate::mix(0.0, GROW_INTENSITY, progress));
                let shrink = perp * (dist * crate::mix(0.0, SHRINK_INTENSITY, progress));
                let scale = crate::mix(1.0, 1.05, factor * progress);
                coords = Vec2::<()>::new((coords.x + grow.x + shrink.x) / scale, (coords.y + grow.y + shrink.y) / scale);
                // Wind.
                let wind = WIND_INTENSITY * progress * if opening { 1.0 } else { -1.0 };
                coords = Vec2::<()>::new(coords.x + wind, coords.y);
                // Dust layer lookup.
                let dust_uv = Vec2::<()>::new((coords.x + seed.x) * size_px.x / dust_scale / 100.0, (coords.y + seed.y) * size_px.y / dust_scale / 100.0);
                let (dust_r, dust_g) = match dust_texture {
                    Some(t) => {
                        let s = t.sample_bilinear(dust_uv);
                        (s.r, s.g)
                    }
                    None => crate::assets::procedural::dust(dust_uv),
                };
                let dust_group = (dust_g * DUST_LAYERS * 0.999).floor();
                if dust_group == i {
                    let mut window_color = window.sample_bilinear(Vec2::<()>::new(coords.x + 0.5, coords.y + 0.5));
                    // Fade the window color to the dust color.
                    let tint = crate::mix(window_color.r, dust_color.r, dust_color.a);
                    let tint_g = crate::mix(window_color.g, dust_color.g, dust_color.a);
                    let tint_b = crate::mix(window_color.b, dust_color.b, dust_color.a);
                    window_color = Color::<f64>::new_rgba(
                        crate::mix(window_color.r, tint, progress),
                        crate::mix(window_color.g, tint_g, progress),
                        crate::mix(window_color.b, tint_b, progress),
                        window_color.a,
                    );
                    if dust_r - progress > 0.0 {
                        c = window_color;
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

    /// Full close leaves nothing.
    #[test]
    fn close_endpoint() {
        let window = solid(32);
        let mut out = PixelBuffer::<Rgba>::new(Size::new(32, 32));
        Snap::default().apply(&ctx_of(&window, 1.0, Direction::Close), &mut out.view_mut());
        assert_eq!(out.view().sample_bilinear(Vec2::<()>::new(0.5, 0.5)).a, 0.0);
    }

    /// Mid-snap some dust cells survive, others are gone.
    #[test]
    fn dust_cells_pop() {
        let window = solid(64);
        let mut out = PixelBuffer::<Rgba>::new(Size::new(64, 64));
        Snap::default().apply(&ctx_of(&window, 0.4, Direction::Close), &mut out.view_mut());
        let view = out.view();
        let mut gone = 0;
        let mut lit = 0;
        for y in 0..64 {
            for x in 0..64 {
                if view.sample_bilinear(Vec2::<()>::new((x as f64 + 0.5) / 64.0, (y as f64 + 0.5) / 64.0)).a < 0.5 {
                    gone += 1
                } else {
                    lit += 1
                }
            }
        }
        assert!(gone > 0 && lit > 0, "dust should dissolve cell-wise: {gone} gone, {lit} lit");
    }
}
