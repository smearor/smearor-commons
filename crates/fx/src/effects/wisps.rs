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
use smearor_commons_noise::Field;
use smearor_commons_noise::Hash;
use smearor_commons_noise::Simplex2DFractal;

/// Glaring dots orbit on ellipses while the window dissolves under a
/// shrinking edge mask - "wisps" carrying the window away.
///
/// Behavioral spec (source shader: `wisps.frag`, re-implemented from
/// semantics): progress is `easeOutQuad`-eased and inverted for
/// `Open`. The window scales down slightly toward the center; eight
/// layers of cell-hashed orbiting wisps (`getWisps`) accumulate via
/// `alphaOver`, gated by `wispsIn`/`wispsOut`; the window dissolves
/// through a piecewise noise `windowMask` plus the relative edge
/// mask. Wisp orbits run on raw `progress * duration`, so they keep
/// moving through the whole animation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Wisps {
    /// Wisp grid scale - schema `wisps-scale`, default `1.0`.
    #[serde(default = "Wisps::default_scale")]
    pub scale: Factor,
    /// Shadow stop of the tritone ramp - schema `wisps-color_1`, default `rgb(255, 100, 200)`.
    #[serde(default = "Wisps::default_color1")]
    pub color1: Color<f64>,
    /// Midtone stop - schema `wisps-color_2`, default `rgb(100, 100, 255)`.
    #[serde(default = "Wisps::default_color2")]
    pub color2: Color<f64>,
    /// Highlight stop - schema `wisps-color_3`, default `rgb(100, 200, 255)`.
    #[serde(default = "Wisps::default_color3")]
    pub color3: Color<f64>,
}

impl Wisps {
    const fn default_scale() -> Factor {
        Factor::new(1.0)
    }
    const fn default_color1() -> Color<f64> {
        Color::<f64>::new(1.0, 100.0 / 255.0, 200.0 / 255.0)
    }
    const fn default_color2() -> Color<f64> {
        Color::<f64>::new(100.0 / 255.0, 100.0 / 255.0, 1.0)
    }
    const fn default_color3() -> Color<f64> {
        Color::<f64>::new(100.0 / 255.0, 200.0 / 255.0, 1.0)
    }
}

impl Default for Wisps {
    fn default() -> Self {
        Self {
            scale: Self::default_scale(),
            color1: Self::default_color1(),
            color2: Self::default_color2(),
            color3: Self::default_color3(),
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
        key: "color1",
        label: "Color 1",
        kind: ParamKind::Color {
            default: Wisps::default_color1(),
        },
    },
    ParamSpec {
        key: "color2",
        label: "Color 2",
        kind: ParamKind::Color {
            default: Wisps::default_color2(),
        },
    },
    ParamSpec {
        key: "color3",
        label: "Color 3",
        kind: ParamKind::Color {
            default: Wisps::default_color3(),
        },
    },
];

/// Wisp grid constants from the shader.
const WISPS_RADIUS: f64 = 20.0;
const WISPS_SPEED: f64 = 10.0;
const WISPS_SPACING: f64 = 40.0 + WISPS_RADIUS;
const WISPS_LAYERS: usize = 8;
const WISPS_IN_TIME: f64 = 0.5;
const WINDOW_OUT_TIME: f64 = 1.0;
const SCALING: f64 = 0.9;

/// BMW `getWisps` - one orbiting dot per grid cell. `texcoords` is
/// the scaled uv, `grid_size` the cell pitch in px, `seed` the
/// per-layer `uSeed * (i+1)`.
fn wisp_layer(texcoords: Vec2<()>, grid_size: f64, seed: Vec2<()>, size_px: Vec2<()>, time: f64, colors: [Color<f64>; 3]) -> Color<f64> {
    let cell_offset = Hash::<Vec2<()>>::hash(&seed);
    let coords = Vec2::<()>::new((texcoords.x + cell_offset.x) * size_px.x / grid_size, (texcoords.y + cell_offset.y) * size_px.y / grid_size);
    let mut cell_uv = Vec2::<()>::new(crate::fract(coords.x), crate::fract(coords.y));
    let cell_id = Vec2::<()>::new(coords.x - cell_uv.x + 362.456, coords.y - cell_uv.y + 362.456);
    // Component-wise `cellID * seed * k` hashing - BMW hash12.
    let hash12 = |k: f64| Hash::<f64>::hash(&Vec2::<()>::new(cell_id.x * seed.x * k, cell_id.y * seed.y * k));
    let speed = crate::mix(10.0, 15.0, hash12(134.451)) / grid_size * WISPS_SPEED;
    let rotation = crate::mix(0.0, std::f64::consts::TAU, hash12(54.4129));
    let radius = crate::mix(0.5, 1.0, hash12(19.1249)) * WISPS_RADIUS;
    let roundness = crate::mix(-1.0, 1.0, hash12(7.51949));
    let mut offset = Vec2::<()>::new((speed * (time + 1.0)).sin() * roundness, (speed * (time + 1.0)).cos());
    offset = offset * (0.5 - 0.5 * radius / grid_size);
    offset = Vec2::<()>::new(offset.x * rotation.cos() - offset.y * rotation.sin(), offset.x * rotation.sin() + offset.y * rotation.cos());
    cell_uv += offset;
    let color = crate::tritone(hash12(1.256), colors[0], colors[1], colors[2]);
    let dx = (cell_uv.x - 0.5) * grid_size / radius;
    let dy = (cell_uv.y - 0.5) * grid_size / radius;
    let dist = (dx * dx + dy * dy).sqrt();
    if dist < 1.0 {
        let alpha = 5.0_f64.min(0.01 / (dist * dist));
        return Color::<f64>::new_rgba(color.r * alpha, color.g * alpha, color.b * alpha, alpha);
    }
    Color::<f64>::new_rgba(0.0, 0.0, 0.0, 0.0)
}

impl WindowEffect for Wisps {
    fn nick(&self) -> &'static str {
        "wisps"
    }

    fn label(&self) -> &'static str {
        "Wisps"
    }

    fn params(&self) -> &'static [ParamSpec] {
        PARAMS
    }

    fn values(&self) -> Vec<ParamValue> {
        vec![
            ParamValue::Float(self.scale.0),
            ParamValue::Color(self.color1),
            ParamValue::Color(self.color2),
            ParamValue::Color(self.color3),
        ]
    }

    fn set_values(&mut self, values: &[ParamValue]) -> Result<(), FxError> {
        let mismatch = |index: usize| FxError::ParamMismatch(format!("wisps param {index}"));
        if values.len() != 4 {
            return Err(FxError::ParamMismatch(format!("wisps takes 4 parameters, got {}", values.len())));
        }
        self.scale = Factor::new(values[0].as_f64().ok_or(mismatch(0))?.max(0.01));
        self.color1 = values[1].as_color().ok_or(mismatch(1))?;
        self.color2 = values[2].as_color().ok_or(mismatch(2))?;
        self.color3 = values[3].as_color().ok_or(mismatch(3))?;
        Ok(())
    }

    fn apply(&self, ctx: &FxContext<'_>, out: &mut BufferViewMut<'_, Rgba>) {
        let size = ctx.window.size();
        let size_px = Vec2::<()>::new(size.x as f64, size.y as f64);
        let opening = matches!(ctx.direction, crate::Direction::Open);
        let eased = EasingFunction::EaseOut.apply(ctx.progress).as_f64();
        let progress = if opening { 1.0 - eased } else { eased };
        // Orbit time runs on raw progress * duration.
        let time = ctx.progress.as_f64() * ctx.duration.as_secs_f64();
        let seed = Hash::<Vec2<()>>::hash(&ctx.seed.as_f64());
        let colors = [self.color1, self.color2, self.color3];
        let scale_factor = self.scale.0;
        let window = ctx.window;
        crate::render::fill(out, |x, y| {
            let uv = Vec2::<()>::new((x as f64 + 0.5) / size_px.x, (y as f64 + 0.5) / size_px.y);
            // Slight center-downscale of the window.
            let s = 1.0 / crate::mix(1.0, SCALING, progress) - 1.0;
            let coords = Vec2::<()>::new(uv.x * (s + 1.0) - s * 0.5, uv.y * (s + 1.0) - s * 0.5);
            let mut c = window.sample_bilinear(coords);
            // Wisp layers.
            let wisp_uv = Vec2::<()>::new(
                ((uv.x - 0.5) / crate::mix(1.0, 0.5, progress) + 0.5) / scale_factor,
                ((uv.y - 0.5) / crate::mix(1.0, 0.5, progress) + 0.5) / scale_factor,
            );
            let mut wisps = Color::<f64>::new_rgba(0.0, 0.0, 0.0, 0.0);
            for i in 0..WISPS_LAYERS {
                let layer_seed = seed * (i as f64 + 1.0);
                let layer = wisp_layer(wisp_uv * 0.3, WISPS_SPACING, layer_seed, size_px, time, colors);
                wisps = crate::alpha_over(wisps, layer);
            }
            let mask = crate::relative_edge_mask(uv, crate::mix(0.01, 0.5, progress));
            let wisps_in = crate::smoothstep(0.0, 1.0, (progress / WISPS_IN_TIME).clamp(0.0, 1.0));
            let wisps_out = crate::smoothstep(0.0, 1.0, ((progress - WISPS_IN_TIME) / (1.0 - WISPS_IN_TIME)).clamp(0.0, 1.0));
            let window_out = crate::smoothstep(0.0, 1.0, (progress / WINDOW_OUT_TIME).clamp(0.0, 1.0));
            let noise = crate::smoothstep(
                1.0,
                0.0,
                (2.0 * Simplex2DFractal.at(Vec2::<()>::new(wisp_uv.x * size_px.x / 250.0, wisp_uv.y * size_px.y / 250.0)) - 1.0).abs(),
            );
            let window_mask = 1.0
                - if window_out < 0.5 {
                    crate::mix(0.0, noise, window_out * 2.0)
                } else {
                    crate::mix(noise, 1.0, window_out * 2.0 - 1.0)
                };
            c.a *= window_mask * mask;
            wisps = Color::<f64>::new_rgba(wisps.r, wisps.g, wisps.b, wisps.a * wisps_in.min(1.0 - wisps_out) * mask);
            crate::alpha_over(c, wisps)
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

    /// Full open untouched, full close gone.
    #[test]
    fn endpoints() {
        let window = solid(32);
        let mut out = PixelBuffer::<Rgba>::new(Size::new(32, 32));
        let fx = Wisps::default();
        fx.apply(&ctx_of(&window, 1.0, Direction::Open), &mut out.view_mut());
        assert_eq!(out.view().sample_bilinear(Vec2::<()>::new(0.5, 0.5)).a, 1.0);
        fx.apply(&ctx_of(&window, 1.0, Direction::Close), &mut out.view_mut());
        assert_eq!(out.view().sample_bilinear(Vec2::<()>::new(0.5, 0.5)).a, 0.0);
    }
}
