use crate::Interpolate;
use crate::Trajectory;
use serde::Deserialize;
use serde::Serialize;
use smearor_commons_geometry::Fraction;

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
        Self { from, mid, to, lead, trail }
    }
}

impl<T: Interpolate + Copy> Trajectory for ThreePhaseSpan<T> {
    type Value = T;

    fn value_at(&self, t: Fraction) -> T {
        if t < self.lead {
            // `t < lead` implies `lead > 0` - the division is safe.
            self.from.interpolate(self.mid, Fraction::new(t / self.lead))
        } else if t <= self.trail {
            self.mid
        } else {
            let tail = Fraction::new((t - self.trail) / (Fraction::ONE - self.trail));
            self.mid.interpolate(self.to, tail)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ThreePhaseSpan;
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
}
