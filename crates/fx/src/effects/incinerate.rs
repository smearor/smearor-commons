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
use smearor_commons_noise::Simplex2DFractal;

/// The window burns away behind a fiery edge, trailing smoke and
/// embers - BMW's most involved effect.
///
/// Behavioral spec (source shader: `incinerate.frag`, re-implemented
/// from semantics): a noise-mixed radial `gradient` sweeps the window
/// from `start_pos` outward; `hide_threshold` travels through it,
/// carving four zones - scorch (brown tint + heat distortion), burn
/// (fiery edge), flame (trailing fire), smoke (gray wisps + embers).
/// `start_pos` is the pointer uv when `use_pointer` (schema
/// `incinerate-use-pointer`, `Incinerate.js` sets it from the
/// actor-local pointer), else a seed-random point on a window edge.
/// The `dFdx`/`dFdy` scorch distortion is approximated by forward
/// differences of the gradient.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Incinerate {
    /// Effect scale - schema `incinerate-scale`, default `1.0`.
    #[serde(default = "Incinerate::default_scale")]
    pub scale: Factor,
    /// Noise weight in the burn gradient - schema `incinerate-turbulence`, default `0.3`.
    #[serde(default = "Incinerate::default_turbulence")]
    pub turbulence: Factor,
    /// Fire tint - schema `incinerate-color`, default `rgb(255, 180, 127)`.
    #[serde(default = "Incinerate::default_color")]
    pub color: Color<f64>,
    /// Burn origin at the pointer instead of a random edge - schema
    /// `incinerate-use_pointer`, default `true`.
    #[serde(default = "Incinerate::default_use_pointer")]
    pub use_pointer: bool,
}

impl Incinerate {
    const fn default_scale() -> Factor {
        Factor::new(1.0)
    }
    const fn default_turbulence() -> Factor {
        Factor::new(0.3)
    }
    const fn default_color() -> Color<f64> {
        Color::<f64>::new(1.0, 180.0 / 255.0, 127.0 / 255.0)
    }
    const fn default_use_pointer() -> bool {
        true
    }

    /// BMW's boundary fallback: `seed[0] > seed[1]` picks the axis,
    /// the other coordinate snaps to 0/1 - a random point on a
    /// random window edge.
    fn start_pos(&self, ctx: &FxContext<'_>, seed: Vec2<()>) -> Vec2<()> {
        if self.use_pointer
            && let Some(pointer) = ctx.pointer
        {
            return pointer;
        }
        if seed.x > seed.y {
            Vec2::<()>::new(seed.x, (seed.y + 0.5).floor())
        } else {
            Vec2::<()>::new((seed.x + 0.5).floor(), seed.y)
        }
    }
}

impl Default for Incinerate {
    fn default() -> Self {
        Self {
            scale: Self::default_scale(),
            turbulence: Self::default_turbulence(),
            color: Self::default_color(),
            use_pointer: Self::default_use_pointer(),
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
        key: "turbulence",
        label: "Turbulence",
        kind: ParamKind::Float {
            min: 0.0,
            max: 1.0,
            step: 0.05,
            default: 0.3,
        },
    },
    ParamSpec {
        key: "color",
        label: "Color",
        kind: ParamKind::Color {
            default: Incinerate::default_color(),
        },
    },
    ParamSpec {
        key: "use_pointer",
        label: "Use Pointer",
        kind: ParamKind::Toggle { default: true },
    },
];

/// `getFireColor` - tritone through `black -> uColor -> white`,
/// alpha = the fire value itself.
fn fire_color(val: f64, color: Color<f64>) -> Color<f64> {
    let t = crate::tritone(val, Color::<f64>::new(0.0, 0.0, 0.0), color, Color::<f64>::new(1.0, 1.0, 1.0));
    Color::<f64>::new_rgba(t.r, t.g, t.b, val)
}

impl WindowEffect for Incinerate {
    fn nick(&self) -> &'static str {
        "incinerate"
    }

    fn label(&self) -> &'static str {
        "Incinerate"
    }

    fn params(&self) -> &'static [ParamSpec] {
        PARAMS
    }

    fn values(&self) -> Vec<ParamValue> {
        vec![
            ParamValue::Float(self.scale.0),
            ParamValue::Float(self.turbulence.0),
            ParamValue::Color(self.color),
            ParamValue::Bool(self.use_pointer),
        ]
    }

    fn set_values(&mut self, values: &[ParamValue]) -> Result<(), FxError> {
        let mismatch = |index: usize| FxError::ParamMismatch(format!("incinerate param {index}"));
        if values.len() != 4 {
            return Err(FxError::ParamMismatch(format!("incinerate takes 4 parameters, got {}", values.len())));
        }
        self.scale = Factor::new(values[0].as_f64().ok_or(mismatch(0))?.max(0.01));
        self.turbulence = Factor::new(values[1].as_f64().ok_or(mismatch(1))?);
        self.color = values[2].as_color().ok_or(mismatch(2))?;
        self.use_pointer = values[3].as_bool().ok_or(mismatch(3))?;
        Ok(())
    }

    fn apply(&self, ctx: &FxContext<'_>, out: &mut BufferViewMut<'_, Rgba>) {
        let size = ctx.window.size();
        let size_px = Vec2::<()>::new(size.x as f64, size.y as f64);
        let opening = matches!(ctx.direction, crate::Direction::Open);
        let seed = Hash::<Vec2<()>>::hash(&ctx.seed.as_f64());
        let start = self.start_pos(ctx, seed);
        let duration = ctx.duration.as_secs_f64();
        let progress = ctx.progress.as_f64();
        let max_dim = size_px.x.max(size_px.y);
        let aspect = Vec2::<()>::new(size_px.x / max_dim, size_px.y / max_dim);
        let fire = self.color;
        // Zone widths - all scale-relative.
        let scorch_width = 0.2 * self.scale.0;
        let burn_width = 0.03 * self.scale.0;
        let smoke_width = 0.9 * self.scale.0;
        let flame_width = 0.2 * self.scale.0;
        let hide_threshold = crate::mix(if opening { 0.0 } else { -scorch_width }, 1.0 + smoke_width, progress);
        let scorch_range = if opening {
            (hide_threshold - scorch_width, hide_threshold)
        } else {
            (hide_threshold, hide_threshold + scorch_width)
        };
        let burn_range = (hide_threshold - burn_width, hide_threshold + burn_width);
        let flame_range = (hide_threshold - flame_width, hide_threshold);
        let smoke_range = (hide_threshold - smoke_width, hide_threshold);
        // The shared smoke noise - evaluated per pixel for the
        // gradient and reused for embers/flames.
        let smoke_noise = |uv: Vec2<()>| {
            let p = Vec2::<()>::new(uv.x / self.scale.0 * size_px.x / 1.5 * 0.01, uv.y / self.scale.0 * size_px.y / 1.5 * 0.01);
            Simplex2DFractal.at(Vec2::<()>::new(p.x + seed.x, p.y + seed.y + progress * 0.3 * duration))
        };
        let gradient = |uv: Vec2<()>| {
            let dx = (uv.x - start.x) * aspect.x;
            let dy = (uv.y - start.y) * aspect.y;
            let circle = (dx * dx + dy * dy).sqrt();
            crate::mix(circle, smoke_noise(uv), 200.0 * self.turbulence.0 * self.scale.0 / max_dim)
        };
        let window = ctx.window;
        let fullscreen = ctx.fullscreen;
        let padding = ctx.padding;
        crate::render::fill(out, |x, y| {
            let uv = Vec2::<()>::new((x as f64 + 0.5) / size_px.x, (y as f64 + 0.5) / size_px.y);
            let px = Vec2::<()>::new(x as f64 + 0.5, y as f64 + 0.5);
            let g = gradient(uv);
            let uv_noise = Vec2::<()>::new(uv.x / self.scale.0 * size_px.x / 1.5, uv.y / self.scale.0 * size_px.y / 1.5);
            let noise = smoke_noise(uv);
            // Zone masks.
            let px_mask =
                |fade: f64, offset: f64| crate::absolute_edge_mask(px, size_px, padding, smearor_commons_geometry::Pixels::new(fade), offset, fullscreen);
            let smoke_mask = crate::smoothstep(0.0, 1.0, (g - smoke_range.0) / smoke_width) * px_mask(100.0, 0.3);
            let flame_mask = crate::smoothstep(0.0, 1.0, (g - flame_range.0) / flame_width) * px_mask(20.0, 0.0);
            let fire_mask = crate::smoothstep(1.0, 0.0, (g - hide_threshold).abs() / burn_width);
            let mut scorch_mask = crate::smoothstep(1.0, 0.0, (g - scorch_range.0) / scorch_width);
            if opening {
                scorch_mask = 1.0 - scorch_mask;
            }
            let mut c = Color::<f64>::new_rgba(0.0, 0.0, 0.0, 0.0);
            // The window shows on the unburned side of the threshold.
            if (!opening && g > hide_threshold) || (opening && g < hide_threshold) {
                // Scorch-zone heat distortion - forward differences of
                // the gradient replace `dFdx`/`dFdy`.
                let mut distort = Vec2::<()>::new(0.0, 0.0);
                if scorch_range.0 < g && g < scorch_range.1 {
                    let dgx = gradient(Vec2::<()>::new(uv.x + 1.0 / size_px.x, uv.y)) - g;
                    let dgy = gradient(Vec2::<()>::new(uv.x, uv.y + 1.0 / size_px.y)) - g;
                    distort = Vec2::<()>::new(dgx * scorch_mask * 5.0, dgy * scorch_mask * 5.0);
                }
                c = window.sample_bilinear(uv + distort);
            }
            // Smoke + embers.
            if smoke_range.0 < g && g < smoke_range.1 {
                let smoke = smoke_mask * noise;
                let smoke_c = Color::<f64>::new_rgba(0.5 * smoke, 0.5 * smoke, 0.5 * smoke, smoke);
                c = crate::alpha_over(c, smoke_c);
                let ember_shift = 0.3 * smoke_mask * duration;
                let ember_noise = Simplex2DFractal.at(Vec2::<()>::new(uv_noise.x * 0.05 + seed.x, uv_noise.y * 0.05 + seed.y - ember_shift * noise));
                let embers = (ember_noise + 0.3).powf(100.0).clamp(0.0, 2.0) * smoke;
                let e = fire_color(embers, fire);
                c = Color::<f64>::new_rgba(c.r + e.r, c.g + e.g, c.b + e.b, c.a + e.a);
            }
            // Scorch tint.
            if scorch_range.0 < g && g < scorch_range.1 {
                let r = crate::mix(c.r, crate::mix(c.r, 0.1, 0.4), scorch_mask);
                let gg = crate::mix(c.g, crate::mix(c.g, 0.05, 0.4), scorch_mask);
                let b = crate::mix(c.b, crate::mix(c.b, 0.02, 0.4), scorch_mask);
                c = Color::<f64>::new_rgba(r, gg, b, c.a);
            }
            // Flames + burning edge.
            if burn_range.0.min(flame_range.0) < g && g < burn_range.1.max(flame_range.1) {
                let flame_noise = Simplex2DFractal.at(Vec2::<()>::new(
                    uv_noise.x * 0.02 + seed.x,
                    uv_noise.y * 0.02 + seed.y + noise * duration + progress * duration,
                ));
                if flame_range.0 < g && g < flame_range.1 {
                    let mut flame = (flame_noise + 0.3).powf(20.0).clamp(0.0, 2.0) * flame_mask;
                    flame += (flame_noise + 0.4).powf(10.0).clamp(0.0, 2.0) * flame_mask * flame_mask * 0.1;
                    let f = fire_color(flame, fire);
                    c = Color::<f64>::new_rgba(c.r + f.r, c.g + f.g, c.b + f.b, c.a + f.a);
                }
                if burn_range.0 < g && g < burn_range.1 {
                    let edge = fire_mask * (flame_noise + 0.4).powf(4.0) * c.a;
                    let f = fire_color(edge, fire);
                    c = Color::<f64>::new_rgba(c.r + f.r, c.g + f.g, c.b + f.b, c.a + f.a);
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

    /// Full open untouched, full close gone.
    #[test]
    fn endpoints() {
        let window = solid(32);
        let mut out = PixelBuffer::<Rgba>::new(Size::new(32, 32));
        let fx = Incinerate::default();
        fx.apply(&ctx_of(&window, 1.0, Direction::Open), &mut out.view_mut());
        assert_eq!(out.view().sample_bilinear(Vec2::<()>::new(0.5, 0.5)).a, 1.0);
        fx.apply(&ctx_of(&window, 1.0, Direction::Close), &mut out.view_mut());
        assert_eq!(out.view().sample_bilinear(Vec2::<()>::new(0.5, 0.5)).a, 0.0);
    }
}
