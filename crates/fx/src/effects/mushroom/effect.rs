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
use smearor_commons_color::Palette;
use smearor_commons_geometry::Factor;
use smearor_commons_geometry::Vec2;

use super::color::StarColor;

/// The window shrinks with a jerky 8-bit staircase while a mushroom
/// of effects erupts: four-point sparkles rise, diagonal light rays
/// streak down, and rings of five-point stars circle the window
/// (behind and in front).
///
/// Behavioral spec (source shader: `mushroom.frag`, re-implemented
/// from semantics): `progress` inverts for `Open`. The window scales
/// around its center by `mix(eightBitScale, (easeInOutSine,
/// easeInQuad), scaleStyle)` - the staircase gives the "8-bit"
/// growth. `spark_count` sparkles rise on `hash21` offsets with an
/// `xEdge` envelope and shared `progress*spark_rotation` spin; `rays`
/// streak on a `(10, 0.5)`-stretched simplex scrolling down; `ring_
/// count` rings of `star_count` stars orbit the `aspect*0.33`
/// ellipse, each composited behind (`depth < 0`) or in front of the
/// window and colored by the six-stop `star_colors` ramp at their
/// ring offset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Mushroom {
    /// 0 = pure 8-bit staircase, 1 = smooth sine/quad growth - schema
    /// `mushroom-scale_style`, default `1.0`.
    #[serde(default = "Mushroom::default_scale_style")]
    pub scale_style: Factor,
    /// Rising four-point sparkles - schema `mushroom-spark_count`, default `4`.
    #[serde(default = "Mushroom::default_spark_count")]
    pub spark_count: u32,
    /// Sparkle tint - schema `mushroom-spark_color`, default `rgba(255, 255, 255, 1.0)`.
    #[serde(default = "Mushroom::default_spark_color")]
    pub spark_color: Color<f64>,
    /// Sparkle spin - schema `mushroom-spark_rotation`, default `0.3`.
    #[serde(default = "Mushroom::default_spark_rotation")]
    pub spark_rotation: Factor,
    /// Light-ray tint (`a = 0` disables rays) - schema `mushroom-rays_color`, default `rgba(255, 255, 255, 1.0)`.
    #[serde(default = "Mushroom::default_rays_color")]
    pub rays_color: Color<f64>,
    /// Star ring count - schema `mushroom-ring_count`, default `3`.
    #[serde(default = "Mushroom::default_ring_count")]
    pub ring_count: u32,
    /// Ring orbit speed - schema `mushroom-ring_rotation`, default `1.33`.
    #[serde(default = "Mushroom::default_ring_rotation")]
    pub ring_rotation: Factor,
    /// Stars per ring - schema `mushroom-star_count`, default `5`.
    #[serde(default = "Mushroom::default_star_count")]
    pub star_count: u32,
    /// The six-stop star ramp - the named [`Palette`] equivalent of
    /// BMW's `mushroom-star-color-0` .. `mushroom-star-color-5` keys.
    /// Slots are [`StarColor`] in interpolation order; missing slots
    /// fall back to the default ramp.
    #[serde(default = "Mushroom::default_star_colors")]
    #[cfg_attr(feature = "schemars", schemars(with = "std::collections::HashMap<String, Color<f64>>"))]
    pub star_colors: Palette<StarColor, f64>,
}

/// The default six-stop ramp, in [`StarColor`] order - the schema's
/// yellow/green/cyan stops.
const DEFAULT_RAMP: [(StarColor, Color<f64>); 6] = [
    (StarColor::Star0, Color::<f64>::new_rgba(233.0 / 255.0, 249.0 / 255.0, 1.0 / 255.0, 1.0)),
    (StarColor::Star1, Color::<f64>::new_rgba(233.0 / 255.0, 249.0 / 255.0, 1.0 / 255.0, 1.0)),
    (StarColor::Star2, Color::<f64>::new_rgba(91.0 / 255.0, 1.0, 1.0 / 255.0, 1.0)),
    (StarColor::Star3, Color::<f64>::new_rgba(91.0 / 255.0, 1.0, 1.0 / 255.0, 1.0)),
    (StarColor::Star4, Color::<f64>::new_rgba(0.0, 240.0 / 255.0, 236.0 / 255.0, 1.0)),
    (StarColor::Star5, Color::<f64>::new_rgba(0.0, 240.0 / 255.0, 236.0 / 255.0, 1.0)),
];

impl Mushroom {
    const fn default_scale_style() -> Factor {
        Factor::new(1.0)
    }
    const fn default_spark_count() -> u32 {
        4
    }
    const fn default_spark_color() -> Color<f64> {
        Color::<f64>::new_rgba(1.0, 1.0, 1.0, 1.0)
    }
    const fn default_spark_rotation() -> Factor {
        Factor::new(0.3)
    }
    const fn default_rays_color() -> Color<f64> {
        Color::<f64>::new_rgba(1.0, 1.0, 1.0, 1.0)
    }
    const fn default_ring_count() -> u32 {
        3
    }
    const fn default_ring_rotation() -> Factor {
        Factor::new(1.33)
    }
    const fn default_star_count() -> u32 {
        5
    }

    /// The palette form of [`DEFAULT_RAMP`].
    fn default_star_colors() -> Palette<StarColor, f64> {
        Palette::from_iter(DEFAULT_RAMP)
    }

    /// The six ramp colors in slot order, slot-missing falls back.
    fn ramp_colors(&self) -> [Color<f64>; 6] {
        DEFAULT_RAMP.map(|(slot, fallback)| self.star_colors.get_or(&slot, fallback))
    }
}

impl Default for Mushroom {
    fn default() -> Self {
        Self {
            scale_style: Self::default_scale_style(),
            spark_count: Self::default_spark_count(),
            spark_color: Self::default_spark_color(),
            spark_rotation: Self::default_spark_rotation(),
            rays_color: Self::default_rays_color(),
            ring_count: Self::default_ring_count(),
            ring_rotation: Self::default_ring_rotation(),
            star_count: Self::default_star_count(),
            star_colors: Self::default_star_colors(),
        }
    }
}

const PARAMS: &[ParamSpec] = &[
    ParamSpec {
        key: "scale_style",
        label: "8-bit Scale",
        kind: ParamKind::Float {
            min: 0.0,
            max: 1.0,
            step: 0.05,
            default: 1.0,
        },
    },
    ParamSpec {
        key: "spark_count",
        label: "Spark Count",
        kind: ParamKind::Int { min: 0, max: 20, default: 4 },
    },
    ParamSpec {
        key: "spark_color",
        label: "Spark Color",
        kind: ParamKind::Color {
            default: Mushroom::default_spark_color(),
        },
    },
    ParamSpec {
        key: "spark_rotation",
        label: "Spark Rotation",
        kind: ParamKind::Float {
            min: -2.0,
            max: 2.0,
            step: 0.05,
            default: 0.3,
        },
    },
    ParamSpec {
        key: "rays_color",
        label: "Rays Color",
        kind: ParamKind::Color {
            default: Mushroom::default_rays_color(),
        },
    },
    ParamSpec {
        key: "ring_count",
        label: "Ring Count",
        kind: ParamKind::Int { min: 0, max: 10, default: 3 },
    },
    ParamSpec {
        key: "ring_rotation",
        label: "Ring Rotation",
        kind: ParamKind::Float {
            min: -5.0,
            max: 5.0,
            step: 0.05,
            default: 1.33,
        },
    },
    ParamSpec {
        key: "star_count",
        label: "Star Count",
        kind: ParamKind::Int { min: 0, max: 15, default: 5 },
    },
    ParamSpec {
        key: "star_colors.star-0",
        label: "Star Color 1",
        kind: ParamKind::Color { default: DEFAULT_RAMP[0].1 },
    },
    ParamSpec {
        key: "star_colors.star-1",
        label: "Star Color 2",
        kind: ParamKind::Color { default: DEFAULT_RAMP[1].1 },
    },
    ParamSpec {
        key: "star_colors.star-2",
        label: "Star Color 3",
        kind: ParamKind::Color { default: DEFAULT_RAMP[2].1 },
    },
    ParamSpec {
        key: "star_colors.star-3",
        label: "Star Color 4",
        kind: ParamKind::Color { default: DEFAULT_RAMP[3].1 },
    },
    ParamSpec {
        key: "star_colors.star-4",
        label: "Star Color 5",
        kind: ParamKind::Color { default: DEFAULT_RAMP[4].1 },
    },
    ParamSpec {
        key: "star_colors.star-5",
        label: "Star Color 6",
        kind: ParamKind::Color { default: DEFAULT_RAMP[5].1 },
    },
];

impl WindowEffect for Mushroom {
    fn nick(&self) -> &'static str {
        "mushroom"
    }

    fn label(&self) -> &'static str {
        "Mushroom"
    }

    fn params(&self) -> &'static [ParamSpec] {
        PARAMS
    }

    fn values(&self) -> Vec<ParamValue> {
        let mut values = vec![
            ParamValue::Float(self.scale_style.0),
            ParamValue::Int(self.spark_count as i64),
            ParamValue::Color(self.spark_color),
            ParamValue::Float(self.spark_rotation.0),
            ParamValue::Color(self.rays_color),
            ParamValue::Int(self.ring_count as i64),
            ParamValue::Float(self.ring_rotation.0),
            ParamValue::Int(self.star_count as i64),
        ];
        values.extend(DEFAULT_RAMP.map(|(slot, fallback)| ParamValue::Color(self.star_colors.get_or(&slot, fallback))));
        values
    }

    fn set_values(&mut self, values: &[ParamValue]) -> Result<(), FxError> {
        let mismatch = |index: usize| FxError::ParamMismatch(format!("mushroom param {index}"));
        if values.len() != 14 {
            return Err(FxError::ParamMismatch(format!("mushroom takes 14 parameters, got {}", values.len())));
        }
        let float = |index: usize| values[index].as_f64().ok_or(mismatch(index));
        let int = |index: usize| values[index].as_i64().ok_or(mismatch(index));
        let color = |index: usize| values[index].as_color().ok_or(mismatch(index));
        self.scale_style = Factor::new(float(0)?);
        self.spark_count = int(1)?.max(0) as u32;
        self.spark_color = color(2)?;
        self.spark_rotation = Factor::new(float(3)?);
        self.rays_color = color(4)?;
        self.ring_count = int(5)?.max(0) as u32;
        self.ring_rotation = Factor::new(float(6)?);
        self.star_count = int(7)?.max(0) as u32;
        for (offset, (slot, _)) in DEFAULT_RAMP.iter().enumerate() {
            let c = color(8 + offset)?;
            self.star_colors.insert(*slot, c);
        }
        Ok(())
    }

    fn apply(&self, ctx: &FxContext<'_>, out: &mut BufferViewMut<'_, Rgba>) {
        let size = ctx.window.size();
        let (w, h) = (size.x as f64, size.y as f64);
        let progress = match ctx.direction {
            crate::Direction::Open => 1.0 - ctx.progress.as_f64(),
            crate::Direction::Close => ctx.progress.as_f64(),
        };
        // Frame constants.
        let aspect = w / h.max(f64::EPSILON);
        let fscale = super::scale::window_scale(progress, self.scale_style.0);
        let seed = ctx.seed.offset_2d();
        let ramp = self.ramp_colors();
        crate::render::fill(out, |x, y| {
            let uv = Vec2::<()>::new((x as f64 + 0.5) / w, (y as f64 + 0.5) / h);
            let mut c = ctx.window.sample_bilinear(super::scale::scale_uv(uv, fscale));
            let window_alpha = c.a;
            let star_uv = Vec2::<()>::new((uv.x - 0.5) * aspect, 1.0 - uv.y);
            if self.spark_count > 0 {
                c = crate::alpha_over(
                    c,
                    super::spark::sparks(
                        Vec2::<()>::new(uv.x * aspect, uv.y),
                        aspect,
                        progress,
                        self.spark_count,
                        self.spark_rotation.0,
                        seed.x,
                        self.spark_color,
                    ),
                );
            }
            if self.rays_color.a > 0.0 {
                c = crate::alpha_over(c, super::rays::rays(uv, progress, seed.y, self.rays_color));
            }
            if self.ring_count > 0 && self.star_count > 0 {
                c = crate::alpha_over(
                    c,
                    super::star::stars(star_uv, aspect, progress, window_alpha, self.ring_count, self.ring_rotation.0, self.star_count, &ramp),
                );
            }
            c
        });
    }
}
