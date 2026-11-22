use crate::FxContext;
use crate::FxError;
use crate::ParamKind;
use crate::ParamSpec;
use crate::ParamValue;
use crate::StripCount;
use crate::WindowEffect;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_buffer::BufferViewMut;
use smearor_commons_buffer::Rgba;
use smearor_commons_color::Color;
use smearor_commons_geometry::Factor;
use smearor_commons_geometry::Vec2;
use smearor_commons_noise::Field;
use smearor_commons_noise::Simplex2D;

/// The window is cut into vertical strips which fall away like paper
/// through a shredder - each strip delays by a seeded offset,
/// accelerates downward, sways slightly and darkens.
///
/// Behavioral spec (source shader: `shredder.frag`, re-implemented
/// from semantics): `prog` inverts for `Open`. `coords.y` covers the
/// actor scaled by `ACTOR_SCALE = 2.5` (`Shredder.js`); strips may
/// fall by `maxFall = 0.5 * (scale - 1)`. Strip `index =
/// floor(x * strips)`; a `fract(sin())` hash spreads strip starts by
/// `min(0.5 * randomness, 0.95)`; `t` normalizes each strip's own
/// run. `fall = t² * maxFall`, `sway = sin(t*6 + index) * SWAY * t /
/// strips` (never shows the neighbor). The strip narrows by `GAP`
/// over `t < 0.25`, `SOFT` edges appear after `t > 0.03`; with
/// `rough_edges`, simplex noise tears `ROUGH` into the limit (left
/// and right edges hashed differently). Strips darken by `DARKEN`
/// and fade from `t = 0.4`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Shredder {
    /// Strip count - schema `shredder-strips`, default `20`.
    #[serde(default = "Shredder::default_strips")]
    pub strips: StripCount,
    /// Start-time spread - schema `shredder-randomness`, default `0.7`.
    #[serde(default = "Shredder::default_randomness")]
    pub randomness: Factor,
    /// Torn paper edges - schema `shredder-rough_edges`, default `true`.
    #[serde(default = "Shredder::default_rough_edges")]
    pub rough_edges: bool,
}

impl Shredder {
    const fn default_strips() -> StripCount {
        StripCount::new(20)
    }
    const fn default_randomness() -> Factor {
        Factor::new(0.7)
    }
    const fn default_rough_edges() -> bool {
        true
    }
}

impl Default for Shredder {
    fn default() -> Self {
        Self {
            strips: Self::default_strips(),
            randomness: Self::default_randomness(),
            rough_edges: Self::default_rough_edges(),
        }
    }
}

/// `ACTOR_SCALE` from `Shredder.js` - vertical overscan for falling strips.
const ACTOR_SCALE: f64 = 2.5;
/// Sideways sway in strip widths - `SWAY`.
const SWAY: f64 = 0.08;
/// Strip narrowing while falling - `GAP`.
const GAP: f64 = 0.2;
/// Fall darkening - `DARKEN`.
const DARKEN: f64 = 0.3;
/// Edge softness in strip widths - `SOFT`.
const SOFT: f64 = 0.04;
/// Rough-edge cut depth in strip widths - `ROUGH`.
const ROUGH: f64 = 0.16;

const PARAMS: &[ParamSpec] = &[
    ParamSpec {
        key: "strips",
        label: "Strips",
        kind: ParamKind::Int { min: 1, max: 100, default: 20 },
    },
    ParamSpec {
        key: "randomness",
        label: "Randomness",
        kind: ParamKind::Float {
            min: 0.0,
            max: 1.0,
            step: 0.05,
            default: 0.7,
        },
    },
    ParamSpec {
        key: "rough_edges",
        label: "Rough Edges",
        kind: ParamKind::Toggle { default: true },
    },
];

impl WindowEffect for Shredder {
    fn nick(&self) -> &'static str {
        "shredder"
    }

    fn label(&self) -> &'static str {
        "Shredder"
    }

    fn params(&self) -> &'static [ParamSpec] {
        PARAMS
    }

    fn values(&self) -> Vec<ParamValue> {
        vec![
            ParamValue::Int(i64::from(self.strips.as_u32())),
            ParamValue::Float(self.randomness.0),
            ParamValue::Bool(self.rough_edges),
        ]
    }

    fn set_values(&mut self, values: &[ParamValue]) -> Result<(), FxError> {
        let mismatch = |index: usize| FxError::ParamMismatch(format!("shredder param {index}"));
        if values.len() != 3 {
            return Err(FxError::ParamMismatch(format!("shredder takes 3 parameters, got {}", values.len())));
        }
        self.strips = StripCount::new(values[0].as_i64().ok_or(mismatch(0))?.max(1) as u32);
        self.randomness = Factor::new(values[1].as_f64().ok_or(mismatch(1))?);
        self.rough_edges = values[2].as_bool().ok_or(mismatch(2))?;
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
        let strips = f64::from(self.strips.as_u32());
        let max_fall = 0.5 * (ACTOR_SCALE - 1.0);
        let spread = (0.5 * self.randomness.0).min(0.95);
        let inv_norm = 1.0 / (1.0 - spread).max(f64::EPSILON);
        crate::render::fill(out, |x, y| {
            let uv = Vec2::<()>::new((x as f64 + 0.5) / w, (y as f64 + 0.5) / h);
            let coords = Vec2::<()>::new(uv.x, (uv.y - 0.5) * ACTOR_SCALE + 0.5);
            let index = (coords.x * strips).floor();
            // Same-time fall, seeded start spread.
            let random = crate::fract((index * 91.7 + 13.1).sin() * 4375.5);
            let t = ((prog - random * spread) * inv_norm).clamp(0.0, 1.0);
            let fall = t * t * max_fall;
            let sway = (t * 6.0 + index).sin() * SWAY * t / strips;
            let local = crate::fract((coords.x - sway) * strips) - 0.5;
            // Narrowing + optional torn edges.
            let width = crate::mix(1.0, 1.0 - GAP, crate::smoothstep(0.0, 0.25, t));
            let mut limit = 0.5 * width;
            if self.rough_edges {
                let side = if local > 0.0 { 3.1 } else { 0.0 };
                limit -= (Simplex2D.at(Vec2::<()>::new((coords.y - fall) * 55.0, index * 7.3 + side)) - 0.5) * ROUGH * crate::smoothstep(0.0, 0.25, t);
            }
            let edge = 1.0 - crate::smoothstep(limit - SOFT, limit, local.abs());
            let cover = crate::mix(1.0, edge, crate::smoothstep(0.0, 0.03, t));
            // Sample where the strip came from; out-of-window stays hidden.
            let source = Vec2::<()>::new(coords.x - sway, coords.y - fall);
            let mut c = ctx.window.sample_bilinear(source);
            if !(0.0..=1.0).contains(&source.x) || !(0.0..=1.0).contains(&source.y) {
                c.a = 0.0;
            }
            let darken = 1.0 - DARKEN * t;
            Color::<f64>::new_rgba(c.r * darken, c.g * darken, c.b * darken, c.a * cover * (1.0 - crate::smoothstep(0.4, 1.0, t)))
        });
    }
}
