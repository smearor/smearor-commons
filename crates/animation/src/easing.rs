use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Fraction;

/// Progress curve of an animation: maps linear time progress
/// (`0.0..=1.0`) to eased progress. Eased values may exceed the
/// range (e.g. [`Overshoot`](Self::Overshoot)) - trajectories decide
/// whether extrapolation is meaningful for their value type.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum EasingFunction {
    /// Constant speed - eased progress equals time progress.
    Linear,
    /// Slow start, fast finish (quadratic ease-in).
    EaseIn,
    /// Fast start, slow finish (quadratic ease-out).
    EaseOut,
    /// Slow start and end, fast middle (quadratic ease-in-out).
    EaseInOut,
    /// Goes beyond the target and settles back.
    Overshoot {
        /// Overshoot strength - higher values overshoot further.
        overshoot_amount: f64,
    },
}

impl EasingFunction {
    /// Penner's canonical back-easing strength (`s = 1.70158`) -
    /// the standard "visible but tasteful" overshoot from the
    /// easing literature.
    pub const DEFAULT_OVERSHOOT: f64 = 1.70158;

    /// The canonical overshoot easing - [`Self::Overshoot`] with
    /// [`Self::DEFAULT_OVERSHOOT`].
    pub const fn overshoot() -> Self {
        Self::Overshoot {
            overshoot_amount: Self::DEFAULT_OVERSHOOT,
        }
    }

    /// The eased progress for linear time progress `t`.
    pub fn apply(&self, t: Fraction) -> Fraction {
        let t = t.as_f64();
        Fraction::new(match self {
            Self::Linear => t,
            Self::EaseIn => t * t,
            Self::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            Self::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
            Self::Overshoot { overshoot_amount } => {
                let s = overshoot_amount * 1.525;
                if t < 0.5 {
                    let p = t * 2.0;
                    0.5 * (p * p * ((s + 1.0) * p - s))
                } else {
                    let p = t * 2.0 - 2.0;
                    0.5 * (p * p * ((s + 1.0) * p + s) + 2.0)
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::EasingFunction;
    use smearor_commons_geometry::Fraction;

    /// Every easing passes through the endpoints unchanged.
    #[test]
    fn endpoints_are_fixed() {
        for easing in [
            EasingFunction::Linear,
            EasingFunction::EaseIn,
            EasingFunction::EaseOut,
            EasingFunction::EaseInOut,
            EasingFunction::Overshoot { overshoot_amount: 1.7 },
            EasingFunction::overshoot(),
        ] {
            assert_eq!(easing.apply(Fraction::ZERO), Fraction::ZERO, "{easing:?} at t=0");
            assert_eq!(easing.apply(Fraction::ONE), Fraction::ONE, "{easing:?} at t=1");
        }
    }

    /// Overshoot exceeds 1.0 somewhere in the second half - the
    /// "settle back" is the point of the curve.
    #[test]
    fn overshoot_exceeds_the_target() {
        let easing = EasingFunction::Overshoot { overshoot_amount: 1.7 };
        let exceeded = (1..100).any(|i| easing.apply(Fraction::new(i as f64 / 100.0)) > Fraction::ONE);
        assert!(exceeded);
    }

    /// EaseIn is slow at the start, EaseOut is fast - mirror curves.
    #[test]
    fn ease_in_and_out_mirror_each_other() {
        for i in 0..=10 {
            let t = i as f64 / 10.0;
            let sum = EasingFunction::EaseIn.apply(Fraction::new(t)).as_f64() + EasingFunction::EaseOut.apply(Fraction::new(1.0 - t)).as_f64();
            assert!((sum - 1.0).abs() < 1e-9, "t={t}");
        }
    }
}
