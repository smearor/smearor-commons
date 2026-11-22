use crate::CycleSpeed;
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
use smearor_commons_color::Saturation;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Vec2;
use smearor_commons_noise::Field;
use smearor_commons_noise::Hash;
use smearor_commons_noise::Simplex2D;

/// A glowing aura blooms around the window while it shrinks into a
/// soft-edged shape - radial blur, cos-palette hue cycling and an
/// aspect-corrected superellipse crop.
///
/// Behavioral spec (source shader: `aura-glow.frag`, re-implemented
/// from semantics): for `Close` the progress is inverted (the glow
/// grows while the window leaves). The content uv is de-padded by
/// `uPadding` and blended toward a 1:1-aspect `oneToOneUV` as progress
/// advances, so the final crop is circular regardless of window
/// aspect. `gradient` is a superellipse distance (`|uv-0.5|^shape`,
/// `shape` sweeping 2 -> 100 over `progress⁵`) plus a
/// `Simplex2D` noise term keyed by the seed. `glowMask` rides the
/// gradient edge; the window is radial-blurred while it zooms slightly;
/// the glow color is a `cos(progress*speed + uv.xyx + (0,2,4))`
/// palette hue-offset by `start_hue` (or a seed hash). The crop blends
/// soft/hard `smoothstep` edges by `edge_hardness` and is guarded by
/// in/out `easeInSine`/`easeOutSine` tails.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct AuraGlow {
    /// Cos-palette cycles over progress - schema `aura-glow-speed`, default `2.5`.
    #[serde(default = "AuraGlow::default_speed")]
    pub speed: CycleSpeed,
    /// Hash the start hue from the seed - schema `aura-glow-random-color`, default `false`.
    #[serde(default)]
    pub random_color: bool,
    /// Start hue as a fraction of the hue circle - schema
    /// `aura-glow-start-hue`, default `0.0`. BMW's `hueOffset` is a
    /// `0..1` fraction, which is why this is `Fraction`, not `Hue`
    /// (degrees).
    #[serde(default = "AuraGlow::default_start_hue")]
    pub start_hue: Fraction,
    /// Glow saturation multiplier - schema `aura-glow-saturation`, default `0.75`.
    #[serde(default = "AuraGlow::default_saturation")]
    pub saturation: Saturation,
    /// Glow width `0..=1` - schema `aura-glow-edge-size`, default `0.75`.
    #[serde(default = "AuraGlow::default_edge_size")]
    pub edge_size: Fraction,
    /// Soft/hard crop blend `0..=1` - schema `aura-glow-edge-hardness`, default `0.25`.
    #[serde(default = "AuraGlow::default_edge_hardness")]
    pub edge_hardness: Fraction,
    /// Radial blur radius in px at progress 0 - schema `aura-glow-blur`, default `15.0`.
    #[serde(default = "AuraGlow::default_blur")]
    pub blur: Pixels<f64>,
}

impl AuraGlow {
    const fn default_speed() -> CycleSpeed {
        CycleSpeed::new(2.5)
    }
    const fn default_start_hue() -> Fraction {
        Fraction::new(0.0)
    }
    fn default_saturation() -> Saturation {
        Saturation::new(0.75)
    }
    const fn default_edge_size() -> Fraction {
        Fraction::new(0.75)
    }
    const fn default_edge_hardness() -> Fraction {
        Fraction::new(0.25)
    }
    const fn default_blur() -> Pixels<f64> {
        Pixels::new(15.0)
    }
}

impl Default for AuraGlow {
    fn default() -> Self {
        Self {
            speed: Self::default_speed(),
            random_color: false,
            start_hue: Self::default_start_hue(),
            saturation: Self::default_saturation(),
            edge_size: Self::default_edge_size(),
            edge_hardness: Self::default_edge_hardness(),
            blur: Self::default_blur(),
        }
    }
}

const PARAMS: &[ParamSpec] = &[
    ParamSpec {
        key: "speed",
        label: "Speed",
        kind: ParamKind::Float {
            min: 0.0,
            max: 10.0,
            step: 0.1,
            default: 2.5,
        },
    },
    ParamSpec {
        key: "random_color",
        label: "Random Color",
        kind: ParamKind::Toggle { default: false },
    },
    ParamSpec {
        key: "start_hue",
        label: "Start Hue",
        kind: ParamKind::Float {
            min: 0.0,
            max: 1.0,
            step: 0.05,
            default: 0.0,
        },
    },
    ParamSpec {
        key: "saturation",
        label: "Saturation",
        kind: ParamKind::Float {
            min: 0.0,
            max: 1.0,
            step: 0.05,
            default: 0.75,
        },
    },
    ParamSpec {
        key: "edge_size",
        label: "Edge Size",
        kind: ParamKind::Float {
            min: 0.0,
            max: 1.0,
            step: 0.05,
            default: 0.75,
        },
    },
    ParamSpec {
        key: "edge_hardness",
        label: "Edge Hardness",
        kind: ParamKind::Float {
            min: 0.0,
            max: 1.0,
            step: 0.05,
            default: 0.25,
        },
    },
    ParamSpec {
        key: "blur",
        label: "Blur",
        kind: ParamKind::Float {
            min: 0.0,
            max: 50.0,
            step: 1.0,
            default: 15.0,
        },
    },
];

impl WindowEffect for AuraGlow {
    fn nick(&self) -> &'static str {
        "aura-glow"
    }

    fn label(&self) -> &'static str {
        "Aura Glow"
    }

    fn params(&self) -> &'static [ParamSpec] {
        PARAMS
    }

    fn values(&self) -> Vec<ParamValue> {
        vec![
            ParamValue::Float(self.speed.0),
            ParamValue::Bool(self.random_color),
            ParamValue::Float(self.start_hue.0),
            ParamValue::Float(self.saturation.as_f64()),
            ParamValue::Float(self.edge_size.0),
            ParamValue::Float(self.edge_hardness.0),
            ParamValue::Float(self.blur.0),
        ]
    }

    fn set_values(&mut self, values: &[ParamValue]) -> Result<(), FxError> {
        let mismatch = |index: usize| FxError::ParamMismatch(format!("aura-glow param {index}"));
        if values.len() != 7 {
            return Err(FxError::ParamMismatch(format!("aura-glow takes 7 parameters, got {}", values.len())));
        }
        let float = |index: usize| values[index].as_f64().ok_or(mismatch(index));
        self.speed = CycleSpeed(float(0)?);
        self.random_color = values[1].as_bool().ok_or(mismatch(1))?;
        self.start_hue = Fraction::new(float(2)?);
        self.saturation = Saturation::new(float(3)?);
        self.edge_size = Fraction::new(float(4)?);
        self.edge_hardness = Fraction::new(float(5)?);
        self.blur = Pixels::new(float(6)?);
        Ok(())
    }

    fn apply(&self, ctx: &FxContext<'_>, out: &mut BufferViewMut<'_, Rgba>) {
        let size = ctx.window.size();
        let size_px = Vec2::<()>::new(size.x as f64, size.y as f64);
        let progress = match ctx.direction {
            crate::Direction::Open => ctx.progress.as_f64(),
            crate::Direction::Close => 1.0 - ctx.progress.as_f64(),
        };
        let seed_uv = ctx.seed.offset_2d();
        for y in 0..size.y {
            for x in 0..size.x {
                let i_tex = Vec2::<()>::new((x as f64 + 0.5) / size_px.x, (y as f64 + 0.5) / size_px.y);
                // De-padded content uv: drop the padding margin, re-normalize.
                let content_fraction = Vec2::<()>::new(1.0, 1.0) - Vec2::<()>::new(2.0 * ctx.padding.0, 2.0 * ctx.padding.0) / size_px;
                let mut uv = (i_tex - Vec2::<()>::new(0.5, 0.5)).component_mul(&size_px) / content_fraction;
                uv = uv / size_px + Vec2::<()>::new(0.5, 0.5);

                // 1:1 aspect blend - the crop ends circular.
                let mut one_to_one = uv - Vec2::<()>::new(0.5, 0.5);
                if size_px.x > size_px.y {
                    one_to_one.y *= size_px.y / size_px.x;
                } else {
                    one_to_one.x *= size_px.x / size_px.y;
                }
                let one_to_one = one_to_one + Vec2::<()>::new(0.5, 0.5);
                uv = one_to_one * (1.0 - progress) + uv * progress;

                // Superellipse gradient + noise wobble.
                let shape = crate::mix(2.0, 100.0, progress.powi(5));
                let mut gradient = (uv.x - 0.5).abs().mul_add(2.0, 0.0).powf(shape) + (uv.y - 0.5).abs().mul_add(2.0, 0.0).powf(shape);
                gradient += Simplex2D.at(i_tex + seed_uv) * 0.5;

                let mut glow_mask = 1.0 - ((progress - gradient) / (self.edge_size.0 + 0.1)).clamp(0.0, 1.0);
                glow_mask *= EasingFunction::EaseOutSine.apply(Fraction::new(((1.0 - progress) * 4.0).min(1.0))).as_f64();

                // Zoom + progressive blur.
                let zoom = crate::mix(1.1, 1.0, EasingFunction::EaseOutCubic.apply(Fraction::new(progress)).as_f64());
                let window_uv = (i_tex - Vec2::<()>::new(0.5, 0.5)) * zoom + Vec2::<()>::new(0.5, 0.5);
                let mut window_color = match self.blur.0 > 0.0 {
                    true => ctx.window.sample_radial_blur(
                        window_uv,
                        Vec2::<()>::new((1.0 - progress) * self.blur.0 / size_px.x, (1.0 - progress) * self.blur.0 / size_px.y),
                        15,
                        3,
                    ),
                    false => ctx.window.sample_bilinear(window_uv),
                };
                glow_mask *= window_color.a;

                // Cos palette + hue offset, saturation-scaled.
                let mut glow_color = Color::<f64>::new_rgba(
                    (progress * self.speed.0 + uv.x + 0.0).cos(),
                    (progress * self.speed.0 + uv.y + 2.0).cos(),
                    (progress * self.speed.0 + uv.x + 4.0).cos(),
                    1.0,
                );
                let color_offset = if self.random_color {
                    Hash::<f64>::hash(&ctx.seed.offset_2d())
                } else {
                    self.start_hue.0
                };
                glow_color = crate::offset_hue(glow_color, color_offset + 0.1);
                let glow = Color::<f64>::new_rgba(
                    (glow_color.r * self.saturation.as_f64()).clamp(0.0, 1.0),
                    (glow_color.g * self.saturation.as_f64()).clamp(0.0, 1.0),
                    (glow_color.b * self.saturation.as_f64()).clamp(0.0, 1.0),
                    1.0,
                );

                window_color = Color::<f64>::new_rgba(
                    window_color.r + glow.r * glow_mask,
                    window_color.g + glow.g * glow_mask,
                    window_color.b + glow.b * glow_mask,
                    window_color.a,
                );
                window_color = Color::<f64>::new_rgba(glow.r, glow.g, glow.b, glow_mask * 0.2).alpha_over(&window_color);

                // Superellipse crop, soft/hard blended.
                let soft_crop = 1.0 - crate::smoothstep(progress - 0.5, progress + 0.5, gradient);
                let hard_crop = 1.0 - crate::smoothstep(progress - 0.05, progress + 0.05, gradient);
                let mut crop_mask = crate::mix(soft_crop, hard_crop, self.edge_hardness.0);
                crop_mask *= EasingFunction::EaseInSine.apply(Fraction::new((progress * 2.0).min(1.0))).as_f64();
                crop_mask = crop_mask.max(1.0 - EasingFunction::EaseOutSine.apply(Fraction::new(((1.0 - progress) * 4.0).min(1.0))).as_f64());
                window_color.a *= crop_mask;

                out.put_color(Pixels::new(x), Pixels::new(y), window_color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Direction;
    use crate::Layer;
    use smearor_commons_buffer::BufferView;
    use smearor_commons_buffer::PixelBuffer;
    use smearor_commons_geometry::Rect;
    use smearor_commons_geometry::Size;
    use smearor_commons_noise::Seed;
    use std::time::Duration;

    fn ctx_for<'a>(window: BufferView<'a, Rgba>, progress: f64) -> FxContext<'a> {
        FxContext {
            progress: Fraction::new(progress),
            duration: Duration::from_millis(1500),
            seed: Seed(1),
            direction: Direction::Open,
            layer: Layer::Window,
            window,
            surface: None,
            pointer: None,
            fullscreen: false,
            padding: Pixels::new(0.0),
            window_rect: Rect::from_xywh(0.0, 0.0, 32.0, 32.0),
        }
    }

    /// `edge_hardness = 1` yields a hard crop: interior alpha is either
    /// fully kept or fully cut (no half-transparent crop zone).
    #[test]
    fn aura_glow_crop() {
        let data = [200, 200, 200, 255].repeat(32 * 32);
        let window = BufferView::<Rgba>::from_packed(&data, Size::new(32, 32)).unwrap();
        let mut out = PixelBuffer::<Rgba>::new(Size::new(32, 32));
        let glow = AuraGlow {
            edge_hardness: Fraction::new(1.0),
            ..Default::default()
        };
        glow.apply(&ctx_for(window, 0.6), &mut out.view_mut());
        // At 0.6 progress the crop is hard: alpha is ~0 or ~255 modulo
        // the glow overlay's small additive alpha.
        let alphas: Vec<u8> = out.view().data().iter().skip(3).step_by(4).copied().collect();
        let extreme = alphas.iter().filter(|a| **a < 40 || **a > 215).count();
        assert!(extreme as f64 / alphas.len() as f64 > 0.6, "hard crop: most alphas near 0/255");
    }

    /// `blur = 0` bypasses radial blur: on a checkerboard input the
    /// sharp sample differs from the blurred one.
    #[test]
    fn aura_glow_blur_radius() {
        let mut data = Vec::with_capacity(32 * 32 * 4);
        for y in 0..32 {
            for x in 0..32 {
                let v = if (x / 4 + y / 4) % 2 == 0 { 255 } else { 0 };
                data.extend_from_slice(&[v, v, v, 255]);
            }
        }
        let window = BufferView::<Rgba>::from_packed(&data, Size::new(32, 32)).unwrap();
        let mut out_sharp = PixelBuffer::<Rgba>::new(Size::new(32, 32));
        let mut out_blur = PixelBuffer::<Rgba>::new(Size::new(32, 32));
        AuraGlow {
            blur: Pixels::new(0.0),
            ..Default::default()
        }
        .apply(&ctx_for(window, 0.5), &mut out_sharp.view_mut());
        AuraGlow {
            blur: Pixels::new(30.0),
            ..Default::default()
        }
        .apply(&ctx_for(window, 0.5), &mut out_blur.view_mut());
        assert_ne!(out_sharp.view().data(), out_blur.view().data(), "blur=0 must differ from blur=30 on sharp input");
    }
}
