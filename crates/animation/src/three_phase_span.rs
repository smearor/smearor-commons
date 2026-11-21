use crate::EasingFunction;
use crate::Interpolate;
use crate::Trajectory;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Fraction;

/// How the hold phase applies its eased curve between `mid` and
/// `hold_to`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum HoldMode {
    /// One-way drift: `mid -> hold_to` over the hold duration.
    #[default]
    Drift,
    /// Out and back: `mid -> hold_to -> mid` - the eased curve is
    /// mirrored so the hold animates around its hold point and
    /// returns to `mid` for the `mid -> to` phase.
    Pulse,
}

/// A three-phase trajectory: interpolates `from -> mid` over the
/// leading `lead` fraction of progress, holds `mid` until `trail`,
/// then interpolates `mid -> to` over the remainder.
///
/// The zoom-during-rotation pattern (zoom out, hold, zoom back in)
/// is `ThreePhaseSpan<ScaleFactor>` with `from = to = 1.0`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ThreePhaseSpan<T> {
    /// The value at progress `0`.
    pub from: T,
    /// The value held between `lead` and `trail`.
    pub mid: T,
    /// The value at progress `1`.
    pub to: T,
    /// End of the leading phase (progress fraction `0.0..trail`).
    pub lead: Fraction,
    /// End of the hold phase (progress fraction `lead..1.0`).
    pub trail: Fraction,
    /// Easing applied to the `from -> mid` phase progress - kept
    /// inside the phase so `lead` stays a real time fraction.
    #[serde(default)]
    pub lead_easing: EasingFunction,
    /// Easing applied to the `mid -> to` phase progress.
    #[serde(default)]
    pub trail_easing: EasingFunction,
    /// Optional end value of the hold phase - `None` holds `mid`
    /// constant, `Some` interpolates `mid -> hold_to` over the hold.
    #[serde(default)]
    pub hold_to: Option<T>,
    /// Easing applied to the `mid -> hold_to` hold progress - only
    /// meaningful with [`Self::hold_to`] set.
    #[serde(default)]
    pub hold_easing: EasingFunction,
    /// Whether the hold drifts one way to `hold_to` or pulses out
    /// and back - only meaningful with [`Self::hold_to`] set.
    #[serde(default)]
    pub hold_mode: HoldMode,
}

impl<T> ThreePhaseSpan<T> {
    /// Creates a three-phase trajectory. `lead` ends the `from ->
    /// mid` phase; `trail` ends the `mid` hold and starts `mid ->
    /// to`. Both fractions clamp into `0.0..=1.0` with
    /// `lead <= trail`.
    pub fn new(from: T, mid: T, to: T, lead: Fraction, trail: Fraction) -> Self {
        let lead = lead.clamped();
        let trail = trail.clamped();
        let trail = if trail < lead { lead } else { trail };
        Self {
            from,
            mid,
            to,
            lead,
            trail,
            lead_easing: EasingFunction::Linear,
            trail_easing: EasingFunction::Linear,
            hold_to: None,
            hold_easing: EasingFunction::Linear,
            hold_mode: HoldMode::Drift,
        }
    }

    /// Returns the span with easing curves for the two
    /// interpolating phases - the phase boundaries stay real time
    /// fractions, the curves only shape the interpolation inside
    /// each phase.
    pub fn with_easing(mut self, lead_easing: EasingFunction, trail_easing: EasingFunction) -> Self {
        self.lead_easing = lead_easing;
        self.trail_easing = trail_easing;
        self
    }

    /// Returns the span with a drifting hold: the middle phase
    /// interpolates `mid -> hold_to` shaped by `hold_easing` instead
    /// of holding `mid` constant.
    pub fn with_hold(self, hold_to: T, hold_easing: EasingFunction) -> Self {
        self.with_hold_mode(hold_to, hold_easing, HoldMode::Drift)
    }

    /// Returns the span with a shaped hold of the given mode - see
    /// [`HoldMode`].
    pub fn with_hold_mode(mut self, hold_to: T, hold_easing: EasingFunction, hold_mode: HoldMode) -> Self {
        self.hold_to = Some(hold_to);
        self.hold_easing = hold_easing;
        self.hold_mode = hold_mode;
        self
    }
}

impl<T: Interpolate + Copy> Trajectory for ThreePhaseSpan<T> {
    type Value = T;

    fn value_at(&self, t: Fraction) -> T {
        if t < self.lead {
            if self.lead <= Fraction::ZERO {
                // a zero-length lead jumps to `mid` - eased progress
                // below 0 (back-in, elastic) lands here.
                return self.mid;
            }
            self.from.interpolate(self.mid, self.lead_easing.apply(Fraction::new(t / self.lead)))
        } else if t <= self.trail {
            match self.hold_to {
                None => self.mid,
                Some(hold_to) if self.trail > self.lead => {
                    let p = (t - self.lead) / (self.trail - self.lead);
                    // Pulse mirrors the hold progress: the excursion
                    // peaks at the hold's midpoint and returns.
                    let p = match self.hold_mode {
                        HoldMode::Drift => p,
                        HoldMode::Pulse => {
                            if p <= 0.5 {
                                2.0 * p
                            } else {
                                2.0 * (1.0 - p)
                            }
                        }
                    };
                    let hold = self.hold_easing.apply(Fraction::new(p));
                    self.mid.interpolate(hold_to, hold)
                }
                Some(hold_to) => hold_to,
            }
        } else {
            if self.trail >= Fraction::ONE {
                // a zero-length tail jumps to `to` - eased progress
                // above 1 (overshoot, elastic) lands here.
                return self.to;
            }
            let tail = self.trail_easing.apply(Fraction::new((t - self.trail) / (Fraction::ONE - self.trail)));
            self.mid.interpolate(self.to, tail)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HoldMode;
    use super::ThreePhaseSpan;
    use crate::EasingFunction;
    use crate::Trajectory;
    use smearor_commons_geometry::Fraction;

    /// `from -> mid` over the lead, `mid` hold, `mid -> to` over
    /// the tail - each phase interpolates linearly.
    #[test]
    fn phases_interpolate_and_hold() {
        let span = ThreePhaseSpan::new(1.0_f64, 0.9, 1.0, Fraction::new(1.0 / 3.0), Fraction::new(2.0 / 3.0));
        assert_eq!(span.value_at(Fraction::new(0.0)), 1.0);
        assert!((span.value_at(Fraction::new(1.0 / 6.0)) - 0.95).abs() < 1e-9);
        assert!((span.value_at(Fraction::new(1.0 / 3.0)) - 0.9).abs() < 1e-9);
        assert!((span.value_at(Fraction::new(0.5)) - 0.9).abs() < 1e-9);
        assert!((span.value_at(Fraction::new(2.0 / 3.0)) - 0.9).abs() < 1e-9);
        assert!((span.value_at(Fraction::new(5.0 / 6.0)) - 0.95).abs() < 1e-9);
        assert!((span.value_at(Fraction::new(1.0)) - 1.0).abs() < 1e-9);
    }

    /// Out-of-range fractions clamp so `lead <= trail` always.
    #[test]
    fn out_of_range_phase_fractions_clamp() {
        let span = ThreePhaseSpan::new(0.0_f64, 1.0, 2.0, Fraction::new(0.8), Fraction::new(0.4));
        assert_eq!(span.lead, Fraction::new(0.8));
        assert_eq!(span.trail, Fraction::new(0.8));
    }

    /// A zero-length tail returns `to` for eased progress above 1
    /// instead of dividing by zero.
    #[test]
    fn degenerate_tail_survives_overshoot() {
        let span = ThreePhaseSpan::new(0.0_f64, 1.0, 2.0, Fraction::new(0.4), Fraction::ONE);
        assert_eq!(span.value_at(Fraction::new(1.1)), 2.0);
    }

    /// A zero-length lead returns `mid` for eased progress below 0
    /// instead of dividing by zero.
    #[test]
    fn degenerate_lead_survives_undershoot() {
        let span = ThreePhaseSpan::new(0.0_f64, 1.0, 2.0, Fraction::ZERO, Fraction::new(0.5));
        assert_eq!(span.value_at(Fraction::new(-0.1)), 1.0);
    }

    /// `with_easing` shapes the phase interpolation while the
    /// boundaries keep their time fractions.
    #[test]
    fn phase_easings_shape_within_boundaries() {
        let span = ThreePhaseSpan::new(0.0_f64, 0.5, 1.0, Fraction::new(0.4), Fraction::new(0.6)).with_easing(EasingFunction::EaseIn, EasingFunction::EaseOut);
        // lead phase midpoint t=0.2 -> phase progress 0.5, eased to 0.25
        assert!((span.value_at(Fraction::new(0.2)) - 0.5 * 0.25).abs() < 1e-9);
        // trail phase midpoint t=0.8 -> phase progress 0.5, eased to 0.75
        assert!((span.value_at(Fraction::new(0.8)) - (0.5 + 0.5 * 0.75)).abs() < 1e-9);
    }

    /// `Pulse` mirrors the hold curve: the excursion peaks at the
    /// hold midpoint and returns to `mid` for the trail phase.
    #[test]
    fn pulse_hold_returns_to_mid() {
        let span = ThreePhaseSpan::new(0.0_f64, 0.0, 0.0, Fraction::new(0.2), Fraction::new(0.8)).with_hold_mode(1.0, EasingFunction::EaseIn, HoldMode::Pulse);
        // hold midpoint t=0.5 -> mirrored progress 1.0 -> hold_to
        assert!((span.value_at(Fraction::new(0.5)) - 1.0).abs() < 1e-9);
        // hold quarter t=0.35 -> mirrored progress 0.5 -> eased 0.25
        assert!((span.value_at(Fraction::new(0.35)) - 0.25).abs() < 1e-9);
        // returns to mid at the trail boundary
        assert!((span.value_at(Fraction::new(0.8)) - 0.0).abs() < 1e-9);
    }

    /// `with_hold` turns the hold into an eased `mid -> hold_to`
    /// drift; a plain span still holds `mid` constant.
    #[test]
    fn hold_easing_shapes_the_drift() {
        let span = ThreePhaseSpan::new(0.0_f64, 0.0, 0.0, Fraction::new(0.2), Fraction::new(0.8)).with_hold(1.0, EasingFunction::EaseIn);
        // hold midpoint t=0.5 -> hold progress 0.5, eased to 0.25
        assert!((span.value_at(Fraction::new(0.5)) - 0.25).abs() < 1e-9);
        assert!((span.value_at(Fraction::new(0.8)) - 1.0).abs() < 1e-9);
        let plain = ThreePhaseSpan::new(0.0_f64, 0.0, 0.0, Fraction::new(0.2), Fraction::new(0.8));
        assert!((plain.value_at(Fraction::new(0.5)) - 0.0).abs() < 1e-9);
    }

    /// Older payloads without the easing fields still deserialize.
    #[test]
    fn missing_easing_fields_default_to_linear() {
        let json = r#"{"from":0.0,"mid":1.0,"to":2.0,"lead":0.3,"trail":0.7}"#;
        let span: ThreePhaseSpan<f64> = serde_json::from_str(json).expect("valid three-phase span");
        assert_eq!(span.lead_easing, EasingFunction::Linear);
        assert_eq!(span.trail_easing, EasingFunction::Linear);
    }
}
