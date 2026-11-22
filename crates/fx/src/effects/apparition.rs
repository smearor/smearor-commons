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
use smearor_commons_geometry::Factor;
use smearor_commons_geometry::Vec2;

/// The window is sucked into a random center point - it shakes,
/// twirls and gets pulled inward like a dematerializing apparition,
/// fading as it goes.
///
/// Behavioral spec (source shader: `apparition.frag`, re-implemented
/// from semantics): `progress` inverts for `Open`. The suction
/// `center` is seeded (`seed * randomness + 0.5 * (1 - randomness)`);
/// `coords` covers the actor scaled by `ACTOR_SCALE = 2` (`PADDING =
/// 0.5`) around that center. Shake adds `progress * 0.05 * shake` of
/// phase-shifted sin/cos, suction pulls `coords` toward the center
/// by `progress * coords/dist * 0.5 * suction` (`dist = len/√2`),
/// then `whirl` spins by `twirl * progress`. The displaced sample
/// fades by `1 - progress`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Apparition {
    /// Jitter amplitude - schema `apparition-shake-intensity`, default `3.0`.
    #[serde(default = "Apparition::default_shake")]
    pub shake: Factor,
    /// Swirl strength - schema `apparition-twirl-intensity`, default `5.0`.
    #[serde(default = "Apparition::default_twirl")]
    pub twirl: Factor,
    /// Inward pull - schema `apparition-suction-intensity`, default `0.75`.
    #[serde(default = "Apparition::default_suction")]
    pub suction: Factor,
    /// Center spread - schema `apparition-randomness`, default `0.5`.
    #[serde(default = "Apparition::default_randomness")]
    pub randomness: Factor,
}

impl Apparition {
    const fn default_shake() -> Factor {
        Factor::new(3.0)
    }
    const fn default_twirl() -> Factor {
        Factor::new(5.0)
    }
    const fn default_suction() -> Factor {
        Factor::new(0.75)
    }
    const fn default_randomness() -> Factor {
        Factor::new(0.5)
    }
}

impl Default for Apparition {
    fn default() -> Self {
        Self {
            shake: Self::default_shake(),
            twirl: Self::default_twirl(),
            suction: Self::default_suction(),
            randomness: Self::default_randomness(),
        }
    }
}

/// `ACTOR_SCALE` - the padded coordinate space covering the window.
const ACTOR_SCALE: f64 = 2.0;
/// `PADDING = ACTOR_SCALE / 2 - 0.5`.
const PADDING: f64 = ACTOR_SCALE / 2.0 - 0.5;

const PARAMS: &[ParamSpec] = &[
    ParamSpec {
        key: "shake",
        label: "Shake Intensity",
        kind: ParamKind::Float {
            min: 0.0,
            max: 10.0,
            step: 0.1,
            default: 3.0,
        },
    },
    ParamSpec {
        key: "twirl",
        label: "Twirl Intensity",
        kind: ParamKind::Float {
            min: 0.0,
            max: 10.0,
            step: 0.1,
            default: 5.0,
        },
    },
    ParamSpec {
        key: "suction",
        label: "Suction Intensity",
        kind: ParamKind::Float {
            min: 0.0,
            max: 5.0,
            step: 0.05,
            default: 0.75,
        },
    },
    ParamSpec {
        key: "randomness",
        label: "Randomness",
        kind: ParamKind::Float {
            min: 0.0,
            max: 1.0,
            step: 0.05,
            default: 0.5,
        },
    },
];

impl WindowEffect for Apparition {
    fn nick(&self) -> &'static str {
        "apparition"
    }

    fn label(&self) -> &'static str {
        "Apparition"
    }

    fn params(&self) -> &'static [ParamSpec] {
        PARAMS
    }

    fn values(&self) -> Vec<ParamValue> {
        vec![
            ParamValue::Float(self.shake.0),
            ParamValue::Float(self.twirl.0),
            ParamValue::Float(self.suction.0),
            ParamValue::Float(self.randomness.0),
        ]
    }

    fn set_values(&mut self, values: &[ParamValue]) -> Result<(), FxError> {
        let mismatch = |index: usize| FxError::ParamMismatch(format!("apparition param {index}"));
        if values.len() != 4 {
            return Err(FxError::ParamMismatch(format!("apparition takes 4 parameters, got {}", values.len())));
        }
        let float = |index: usize| values[index].as_f64().ok_or(mismatch(index));
        self.shake = Factor::new(float(0)?);
        self.twirl = Factor::new(float(1)?);
        self.suction = Factor::new(float(2)?);
        self.randomness = Factor::new(float(3)?);
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
        // uSeed is a unit-range random vec2 (the suction center), not a
        // noise offset - hash the normalized seed.
        let seed = smearor_commons_noise::Hash::<Vec2<()>>::hash(&ctx.seed.as_f64());
        let center = Vec2::<()>::new(
            seed.x * self.randomness.0 + 0.5 * (1.0 - self.randomness.0),
            seed.y * self.randomness.0 + 0.5 * (1.0 - self.randomness.0),
        );
        let shake_amp = progress * 0.05 * self.shake.0;
        let shake_x = ((progress + seed.x) * (1.0 + seed.x) * self.shake.0).sin() * shake_amp;
        let shake_y = ((progress + seed.y) * (1.0 + seed.y) * self.shake.0).cos() * shake_amp;
        let alpha = 1.0 - progress;
        crate::render::fill(out, |x, y| {
            let uv = Vec2::<()>::new((x as f64 + 0.5) / w, (y as f64 + 0.5) / h);
            let mut coords = Vec2::<()>::new(uv.x * ACTOR_SCALE - PADDING - center.x + shake_x, uv.y * ACTOR_SCALE - PADDING - center.y + shake_y);
            // Pull toward the center, then whirl.
            let dist = (coords.x.mul_add(coords.x, coords.y * coords.y)).sqrt() / std::f64::consts::SQRT_2;
            if dist > 0.0 {
                coords = Vec2::<()>::new(
                    coords.x + progress * coords.x / dist * 0.5 * self.suction.0,
                    coords.y + progress * coords.y / dist * 0.5 * self.suction.0,
                );
            }
            coords = crate::whirl(coords, self.twirl.0 * progress, 0.0);
            let mut c = ctx.window.sample_bilinear(Vec2::<()>::new(coords.x + center.x, coords.y + center.y));
            c.a *= alpha;
            c
        });
    }
}
