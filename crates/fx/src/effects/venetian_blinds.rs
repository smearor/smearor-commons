use crate::FxContext;
use crate::FxError;
use crate::ParamKind;
use crate::ParamSpec;
use crate::ParamValue;
use crate::SlatCount;
use crate::WindowEffect;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_animation::EasingFunction;
use smearor_commons_buffer::BufferView;
use smearor_commons_buffer::BufferViewMut;
use smearor_commons_buffer::Rgba;
use smearor_commons_color::Color;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Vec2;

/// Horizontal slats turn shut like a venetian blind - or fold up to
/// the top when `pull_up` is set. Turned slats darken, catch light on
/// their edges and soften while they rotate.
///
/// Behavioral spec (source shader: `venetian-blinds.frag`,
/// re-implemented from semantics): `prog` inverts for `Open`; `pos =
/// uv.y * slats` is the position in slat heights. Slats start
/// top-to-bottom with `index / (slats-1) * min(delay, 0.95)` spread
/// (`getSlatProgress`). Turn mode: slat height is `cos(easeInOutCubic
/// (t) * π/2)`; inside the visible band the texture is stretched
/// back via `local / height`, darkened by `turn * (SHADE + 0.25 *
/// rel)`, rim-lit at `|rel| > 0.8` and soft-edged (`SOFT` after
/// `t > 0.05`); `perspective` narrows the lower edge by
/// `PERSPECTIVE * turn`. Pull-up mode: each slat folds to `FOLDED`
/// and rises to `0.5 * FOLDED` by `easeInOutCubic(t)`; the first
/// covering slat wins (upper in front); all fade after `FOLD_END`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct VenetianBlinds {
    /// Slat count (`1..=15`, shader `MAX_SLATS`) - schema `venetian-blinds-slats`, default `9`.
    #[serde(default = "VenetianBlinds::default_slats")]
    pub slats: SlatCount,
    /// Top-to-bottom start delay - schema `venetian-blinds-delay`, default `0.4`.
    #[serde(default = "VenetianBlinds::default_delay")]
    pub delay: Fraction,
    /// Narrow lower edge while turning - schema `venetian-blinds-perspective`, default `false`.
    #[serde(default = "VenetianBlinds::default_perspective")]
    pub perspective: bool,
    /// Fold up instead of turning - schema `venetian-blinds-pull_up`, default `false`.
    #[serde(default = "VenetianBlinds::default_pull_up")]
    pub pull_up: bool,
}

impl VenetianBlinds {
    const fn default_slats() -> SlatCount {
        SlatCount::new(9)
    }
    const fn default_delay() -> Fraction {
        Fraction::new(0.4)
    }
    const fn default_perspective() -> bool {
        false
    }
    const fn default_pull_up() -> bool {
        false
    }
}

impl Default for VenetianBlinds {
    fn default() -> Self {
        Self {
            slats: Self::default_slats(),
            delay: Self::default_delay(),
            perspective: Self::default_perspective(),
            pull_up: Self::default_pull_up(),
        }
    }
}

/// Darkening of an almost closed slat - `SHADE`.
const SHADE: f64 = 0.55;
/// Edge highlight - `RIM`.
const RIM: f64 = 0.25;
/// Slat edge softness - `SOFT`.
const SOFT: f64 = 0.03;
/// Perspective narrowing - `PERSPECTIVE`.
const PERSPECTIVE: f64 = 0.2;
/// Folded slat height - `FOLDED`.
const FOLDED: f64 = 0.22;
/// Pull-up complete at this progress - `FOLD_END`.
const FOLD_END: f64 = 0.88;

/// Per-slat progress: top-to-bottom delay spread, same duration each.
fn slat_progress(index: f64, slats: f64, delay: f64, prog: f64) -> f64 {
    let total = delay.min(0.95);
    let slat_delay = index / (slats - 1.0).max(1.0) * total;
    ((prog - slat_delay) / (1.0 - total).max(f64::EPSILON)).clamp(0.0, 1.0)
}

/// `getSlatColor` - samples the window for a pixel `local` slat
/// heights from the slat center; `height` is the turned slat's
/// apparent height, `t` its progress.
#[allow(clippy::too_many_arguments)]
fn slat_color(window: BufferView<'_, Rgba>, x: f64, slats: f64, index: f64, local: f64, height: f64, t: f64, perspective: bool) -> Color<f64> {
    let limit = 0.5 * height;
    if local.abs() >= limit {
        return Color::<f64>::new_rgba(0.0, 0.0, 0.0, 0.0);
    }
    // turn = sin(angle), since height = cos(angle).
    let turn = (1.0 - height * height).max(0.0).sqrt();
    let rel = local / limit.max(0.01);
    // Perspective narrows the far (lower) edge.
    let mut sx = x;
    if perspective {
        let shrink = PERSPECTIVE * turn * (0.5 * rel + 0.5);
        sx = 0.5 + (x - 0.5) / (1.0 - shrink).max(f64::EPSILON);
        if !(0.0..=1.0).contains(&sx) {
            return Color::<f64>::new_rgba(0.0, 0.0, 0.0, 0.0);
        }
    }
    // Stretch the visible band back to the slat's full height.
    let sy = (index + 0.5 + local / height.max(0.02)) / slats;
    let mut c = window.sample_bilinear(Vec2::<()>::new(sx, sy));
    let darken = 1.0 - turn * SHADE - turn * 0.25 * rel;
    let rim = crate::smoothstep(0.8, 1.0, rel.abs()) * turn * RIM;
    let edge = 1.0 - crate::smoothstep(limit - SOFT, limit, local.abs());
    c.r = c.r * darken + rim;
    c.g = c.g * darken + rim;
    c.b = c.b * darken + rim;
    c.a *= crate::mix(1.0, edge, crate::smoothstep(0.0, 0.05, t));
    c
}

const PARAMS: &[ParamSpec] = &[
    ParamSpec {
        key: "slats",
        label: "Slats",
        kind: ParamKind::Int { min: 1, max: 15, default: 9 },
    },
    ParamSpec {
        key: "delay",
        label: "Delay",
        kind: ParamKind::Float {
            min: 0.0,
            max: 1.0,
            step: 0.05,
            default: 0.4,
        },
    },
    ParamSpec {
        key: "perspective",
        label: "Perspective",
        kind: ParamKind::Toggle { default: false },
    },
    ParamSpec {
        key: "pull_up",
        label: "Pull Up",
        kind: ParamKind::Toggle { default: false },
    },
];

impl WindowEffect for VenetianBlinds {
    fn nick(&self) -> &'static str {
        "venetian-blinds"
    }

    fn label(&self) -> &'static str {
        "Venetian Blinds"
    }

    fn params(&self) -> &'static [ParamSpec] {
        PARAMS
    }

    fn values(&self) -> Vec<ParamValue> {
        vec![
            ParamValue::Int(i64::from(self.slats.as_u32())),
            ParamValue::Float(self.delay.as_f64()),
            ParamValue::Bool(self.perspective),
            ParamValue::Bool(self.pull_up),
        ]
    }

    fn set_values(&mut self, values: &[ParamValue]) -> Result<(), FxError> {
        let mismatch = |index: usize| FxError::ParamMismatch(format!("venetian-blinds param {index}"));
        if values.len() != 4 {
            return Err(FxError::ParamMismatch(format!("venetian-blinds takes 4 parameters, got {}", values.len())));
        }
        self.slats = SlatCount::new(values[0].as_i64().ok_or(mismatch(0))?.clamp(1, 15) as u32);
        self.delay = Fraction::new(values[1].as_f64().ok_or(mismatch(1))?);
        self.perspective = values[2].as_bool().ok_or(mismatch(2))?;
        self.pull_up = values[3].as_bool().ok_or(mismatch(3))?;
        Ok(())
    }

    fn apply(&self, ctx: &FxContext<'_>, out: &mut BufferViewMut<'_, Rgba>) {
        let size = ctx.window.size();
        let (w, h) = (size.x as f64, size.y as f64);
        let prog = match ctx.direction {
            crate::Direction::Open => 1.0 - ctx.progress.as_f64(),
            crate::Direction::Close => ctx.progress.as_f64(),
        };
        // Frame constants.
        let slats = f64::from(self.slats.as_u32());
        let delay = self.delay.as_f64();
        let fold = (prog / FOLD_END).clamp(0.0, 1.0);
        let fold_fade = 1.0 - crate::smoothstep(FOLD_END, 1.0, prog);
        crate::render::fill(out, |x, y| {
            let uv = Vec2::<()>::new((x as f64 + 0.5) / w, (y as f64 + 0.5) / h);
            let pos = uv.y * slats;
            if !self.pull_up {
                let index = pos.floor();
                let t = slat_progress(index, slats, delay, prog);
                let height = (EasingFunction::EaseInOutCubic.apply(Fraction::new(t)).as_f64() * std::f64::consts::FRAC_PI_2).cos();
                let mut c = slat_color(ctx.window, uv.x, slats, index, crate::fract(pos) - 0.5, height, t, self.perspective);
                c.a *= 1.0 - crate::smoothstep(0.92, 1.0, t);
                return c;
            }
            // Pull-up: the first covering slat wins, upper in front.
            let mut c = Color::<f64>::new_rgba(0.0, 0.0, 0.0, 0.0);
            for i in 0..self.slats.as_u32() {
                let index = i as f64;
                let t = slat_progress(index, slats, delay, fold);
                let e = EasingFunction::EaseInOutCubic.apply(Fraction::new(t)).as_f64();
                let height = crate::mix(1.0, FOLDED, e);
                let center = crate::mix(index + 0.5, 0.5 * FOLDED, e);
                let local = pos - center;
                if local.abs() < 0.5 * height {
                    c = slat_color(ctx.window, uv.x, slats, index, local, height, t, self.perspective);
                    break;
                }
            }
            c.a *= fold_fade;
            c
        });
    }
}
