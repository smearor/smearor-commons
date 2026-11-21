use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Fraction;
use std::f64::consts::FRAC_PI_2;
use std::f64::consts::PI;
use std::f64::consts::TAU;

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
    /// Slow start, fast finish (cubic ease-in).
    EaseInCubic,
    /// Fast start, slow finish (cubic ease-out) - the classic
    /// "arrive" curve for camera and pose transitions.
    EaseOutCubic,
    /// Slow start and end, fast middle (cubic ease-in-out).
    EaseInOutCubic,
    /// Goes below 0, then beyond the target and settles back
    /// (Penner's back-in-out).
    Overshoot {
        /// Overshoot strength - higher values overshoot further.
        overshoot_amount: f64,
    },
    /// Slow start, fast finish (quartic ease-in, `t^4`).
    EaseInQuart,
    /// Fast start, slow finish (quartic ease-out).
    EaseOutQuart,
    /// Slow start and end, fast middle (quartic ease-in-out).
    EaseInOutQuart,
    /// Slow start, fast finish (quintic ease-in, `t^5`).
    EaseInQuint,
    /// Fast start, slow finish (quintic ease-out).
    EaseOutQuint,
    /// Slow start and end, fast middle (quintic ease-in-out).
    EaseInOutQuint,
    /// Slow start, fast finish (sinusoidal ease-in).
    EaseInSine,
    /// Fast start, slow finish (sinusoidal ease-out) - the gentlest
    /// curve of the family.
    EaseOutSine,
    /// Slow start and end, fast middle (sinusoidal ease-in-out).
    EaseInOutSine,
    /// Very slow start, abrupt finish (exponential ease-in).
    EaseInExpo,
    /// Abrupt start, very slow finish (exponential ease-out).
    EaseOutExpo,
    /// Abrupt middle between very slow ends (exponential
    /// ease-in-out).
    EaseInOutExpo,
    /// Slow start, fast finish (circular ease-in).
    EaseInCirc,
    /// Fast start, slow finish (circular ease-out).
    EaseOutCirc,
    /// Slow start and end, fast middle (circular ease-in-out).
    EaseInOutCirc,
    /// Dips below the start value, then accelerates (Penner's
    /// back-in).
    BackIn {
        /// Overshoot strength - higher values dip further.
        overshoot_amount: f64,
    },
    /// Overshoots the target, then settles back (Penner's back-out).
    /// [`Self::Overshoot`] is the in-out combination of this and
    /// [`Self::BackIn`].
    BackOut {
        /// Overshoot strength - higher values overshoot further.
        overshoot_amount: f64,
    },
    /// Oscillates around the target and settles - the springy
    /// ease-out of the easing literature.
    Elastic {
        /// Oscillation amplitude - `1.0` is the classic curve.
        amplitude: f64,
        /// Oscillation period in progress units - the classic
        /// curve uses `0.3`.
        period: f64,
    },
    /// Bounces off the target with decreasing heights - the
    /// physical drop-in of the easing literature.
    Bounce,
    /// A custom cubic bezier curve - CSS `cubic-bezier()`
    /// compatible: `x1`, `y1` are the outgoing control point, `x2`,
    /// `y2` the incoming one. `y` may exceed `0..1`, `x` is clamped
    /// to it.
    CubicBezier {
        /// X of the outgoing control point.
        x1: f64,
        /// Y of the outgoing control point.
        y1: f64,
        /// X of the incoming control point.
        x2: f64,
        /// Y of the incoming control point.
        y2: f64,
    },
    /// Discrete progress: `steps` equally sized jumps, CSS
    /// `steps()` compatible.
    Steps {
        /// The number of discrete steps.
        steps: u32,
        /// When the jump happens within each step interval.
        jump: StepsJump,
    },
}

impl Default for EasingFunction {
    /// The identity curve - `Default` makes a plain
    /// `#[serde(default)]` mean "no easing".
    fn default() -> Self {
        Self::Linear
    }
}

/// When a [`EasingFunction::Steps`] interval jumps to the next
/// value - the CSS `steps()` jump terms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum StepsJump {
    /// The step value holds at the interval start - the first jump
    /// happens at `t = 0`.
    Start,
    /// The step value holds at the interval end - the last jump
    /// happens at `t = 1`.
    End,
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
        // The endpoints are contractually fixed - trigonometric
        // curves would otherwise return rounding residue.
        if t <= 0.0 {
            return Fraction::ZERO;
        }
        if t >= 1.0 {
            return Fraction::ONE;
        }
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
            Self::EaseInCubic => t * t * t,
            Self::EaseOutCubic => 1.0 - (1.0 - t).powi(3),
            Self::EaseInOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
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
            Self::EaseInQuart => t.powi(4),
            Self::EaseOutQuart => 1.0 - (1.0 - t).powi(4),
            Self::EaseInOutQuart => {
                if t < 0.5 {
                    8.0 * t.powi(4)
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(4) / 2.0
                }
            }
            Self::EaseInQuint => t.powi(5),
            Self::EaseOutQuint => 1.0 - (1.0 - t).powi(5),
            Self::EaseInOutQuint => {
                if t < 0.5 {
                    16.0 * t.powi(5)
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(5) / 2.0
                }
            }
            Self::EaseInSine => 1.0 - (t * FRAC_PI_2).cos(),
            Self::EaseOutSine => (t * FRAC_PI_2).sin(),
            Self::EaseInOutSine => -((t * PI).cos() - 1.0) / 2.0,
            Self::EaseInExpo => {
                if t <= 0.0 {
                    0.0
                } else {
                    (10.0 * t - 10.0).exp2()
                }
            }
            Self::EaseOutExpo => {
                if t >= 1.0 {
                    1.0
                } else {
                    1.0 - (-10.0 * t).exp2()
                }
            }
            Self::EaseInOutExpo => {
                if t <= 0.0 {
                    0.0
                } else if t >= 1.0 {
                    1.0
                } else if t < 0.5 {
                    (20.0 * t - 10.0).exp2() / 2.0
                } else {
                    (2.0 - (-20.0 * t + 10.0).exp2()) / 2.0
                }
            }
            Self::EaseInCirc => 1.0 - (1.0 - t * t).sqrt(),
            Self::EaseOutCirc => (1.0 - (t - 1.0).powi(2)).sqrt(),
            Self::EaseInOutCirc => {
                if t < 0.5 {
                    (1.0 - (1.0 - 4.0 * t * t).sqrt()) / 2.0
                } else {
                    ((1.0 - (-2.0 * t + 2.0).powi(2)).sqrt() + 1.0) / 2.0
                }
            }
            Self::BackIn { overshoot_amount } => {
                let s = overshoot_amount + 1.0;
                s * t * t * t - overshoot_amount * t * t
            }
            Self::BackOut { overshoot_amount } => {
                let s = overshoot_amount + 1.0;
                let p = t - 1.0;
                1.0 + s * p * p * p + overshoot_amount * p * p
            }
            Self::Elastic { amplitude, period } => {
                if t <= 0.0 {
                    0.0
                } else if t >= 1.0 {
                    1.0
                } else {
                    // The phase shift starts the oscillation on the
                    // target side so the curve passes through 0.
                    let phase = period / TAU * (amplitude.max(1.0).asin());
                    amplitude * (-10.0 * t).exp2() * (TAU / period * (t - phase)).sin() + 1.0
                }
            }
            Self::Bounce => {
                // The classic ease-out bounce: parabolic arcs of
                // decreasing height, mirrored into `0..1`.
                let t = 1.0 - t;
                1.0 - if t < 1.0 / 2.75 {
                    7.5625 * t * t
                } else if t < 2.0 / 2.75 {
                    let t = t - 1.5 / 2.75;
                    7.5625 * t * t + 0.75
                } else if t < 2.5 / 2.75 {
                    let t = t - 2.25 / 2.75;
                    7.5625 * t * t + 0.9375
                } else {
                    let t = t - 2.625 / 2.75;
                    7.5625 * t * t + 0.984375
                }
            }
            Self::CubicBezier { x1, y1, x2, y2 } => {
                let bezier = |s: f64, a: f64, b: f64| (1.0 - s).powi(3) * 0.0 + 3.0 * (1.0 - s).powi(2) * s * a + 3.0 * (1.0 - s) * s * s * b + s.powi(3);
                // Invert x(s) = t via Newton iterations, bisection
                // fallback - the CSS `cubic-bezier()` semantics.
                let mut param = t;
                for _ in 0..8 {
                    let x = bezier(param, *x1, *x2) - t;
                    if x.abs() < 1e-6 {
                        break;
                    }
                    let dx = 3.0 * (1.0 - param).powi(2) * x1 + 6.0 * (1.0 - param) * param * (x2 - x1) + 3.0 * param * param * (1.0 - x2);
                    if dx.abs() < 1e-6 {
                        break;
                    }
                    param = (param - x / dx).clamp(0.0, 1.0);
                }
                bezier(param, *y1, *y2)
            }
            Self::Steps { steps, jump } => {
                let n = (*steps).max(1) as f64;
                let step = (t * n).floor() / n;
                match jump {
                    StepsJump::End => step,
                    StepsJump::Start => {
                        if t <= 0.0 {
                            1.0 / n
                        } else {
                            (step + 1.0 / n).min(1.0)
                        }
                    }
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
            EasingFunction::EaseInCubic,
            EasingFunction::EaseOutCubic,
            EasingFunction::EaseInOutCubic,
            EasingFunction::Overshoot { overshoot_amount: 1.7 },
            EasingFunction::overshoot(),
            EasingFunction::EaseInQuart,
            EasingFunction::EaseOutQuart,
            EasingFunction::EaseInOutQuart,
            EasingFunction::EaseInQuint,
            EasingFunction::EaseOutQuint,
            EasingFunction::EaseInOutQuint,
            EasingFunction::EaseInSine,
            EasingFunction::EaseOutSine,
            EasingFunction::EaseInOutSine,
            EasingFunction::EaseInExpo,
            EasingFunction::EaseOutExpo,
            EasingFunction::EaseInOutExpo,
            EasingFunction::EaseInCirc,
            EasingFunction::EaseOutCirc,
            EasingFunction::EaseInOutCirc,
            EasingFunction::BackIn { overshoot_amount: 1.7 },
            EasingFunction::BackOut { overshoot_amount: 1.7 },
            EasingFunction::Elastic { amplitude: 1.0, period: 0.3 },
            EasingFunction::Bounce,
            EasingFunction::CubicBezier {
                x1: 0.25,
                y1: 0.1,
                x2: 0.25,
                y2: 1.0,
            },
            EasingFunction::Steps {
                steps: 4,
                jump: crate::easing::StepsJump::End,
            },
            EasingFunction::Steps {
                steps: 4,
                jump: crate::easing::StepsJump::Start,
            },
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

/// BackIn dips below the start, BackOut overshoots the target -
/// the two halves of Overshoot.
#[test]
fn back_curves_exceed_their_ends() {
    let back_in = EasingFunction::BackIn { overshoot_amount: 1.7 };
    let back_out = EasingFunction::BackOut { overshoot_amount: 1.7 };
    assert!((1..50).any(|i| back_in.apply(Fraction::new(i as f64 / 100.0)) < Fraction::ZERO));
    assert!((50..100).any(|i| back_out.apply(Fraction::new(i as f64 / 100.0)) > Fraction::ONE));
}

/// The CSS `ease` curve `cubic-bezier(0.25, 0.1, 0.25, 1.0)` is
/// a gentle ease-out - its midpoint sits above 0.5.
#[test]
fn cubic_bezier_matches_the_css_ease_curve() {
    let bezier = EasingFunction::CubicBezier {
        x1: 0.25,
        y1: 0.1,
        x2: 0.25,
        y2: 1.0,
    };
    let mid = bezier.apply(Fraction::HALF).as_f64();
    assert!((mid - 0.8024).abs() < 0.01, "css ease midpoint, got {mid}");
}

/// Steps only ever produce the discrete jump values.
#[test]
fn steps_quantizes_the_progress() {
    let steps = EasingFunction::Steps {
        steps: 4,
        jump: crate::easing::StepsJump::End,
    };
    for i in 0..100 {
        let eased = steps.apply(Fraction::new(i as f64 / 100.0)).as_f64();
        assert!((eased * 4.0).fract().abs() < 1e-9, "t={i}: {eased}");
    }
    assert_eq!(steps.apply(Fraction::ZERO), Fraction::ZERO);
    assert_eq!(steps.apply(Fraction::new(0.99)), Fraction::new(0.75));
}
