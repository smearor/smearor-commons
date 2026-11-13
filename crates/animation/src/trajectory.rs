use smearor_commons_geometry::Fraction;

/// Maps eased progress `t` (conventionally `0.0..=1.0`, easing may
/// exceed it) to an animated value - the shape of an animation's
/// output over time.
///
/// Implementations: [`crate::Span`] for a two-endpoint interpolation,
/// [`crate::ThreePhaseSpan`] for an ease-out / hold / ease-in
/// pattern, and `(S1, S2)` tuples for composite values animated on
/// one clock.
pub trait Trajectory {
    /// The animated value type.
    type Value;

    /// The value at eased progress `t`.
    fn value_at(&self, t: Fraction) -> Self::Value;
}

/// Two trajectories animate in lockstep on the same progress - a
/// composite animation like "rotate while zooming" needs no
/// dedicated animation struct.
impl<A: Trajectory, B: Trajectory> Trajectory for (A, B) {
    type Value = (A::Value, B::Value);

    fn value_at(&self, t: Fraction) -> Self::Value {
        (self.0.value_at(t), self.1.value_at(t))
    }
}
