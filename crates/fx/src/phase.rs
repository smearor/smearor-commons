/// A `[start, end]` window on the normalized effect timeline.
/// Effects compose their motion out of overlapping phases; `at`
/// evaluates the window as a smoothstepped `0..1` envelope.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Phase {
    /// Timeline position where the phase starts fading in.
    pub start: f64,
    /// Timeline position where the phase is fully in.
    pub end: f64,
}

impl Phase {
    /// Creates a phase spanning `[start, end]`.
    pub const fn new(start: f64, end: f64) -> Self {
        Self { start, end }
    }

    /// The phase length (`end - start`).
    pub fn len(self) -> f64 {
        self.end - self.start
    }

    /// The smoothstepped `0..1` envelope at timeline `progress`.
    pub fn at(self, progress: f64) -> f64 {
        crate::smoothstep(self.start, self.end, progress)
    }

    /// The linearly interpolated `0..1` envelope at `progress` -
    /// for envelopes shaped by the caller (e.g. squared falls).
    pub fn at_linear(self, progress: f64) -> f64 {
        ((progress - self.start) / self.len()).clamp(0.0, 1.0)
    }
}
