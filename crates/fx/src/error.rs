use thiserror::Error;

/// Errors of the fx spec/dispatch layer - unknown effect tags,
/// missing layered context, parameter mismatches.
#[derive(Debug, Error)]
pub enum FxError {
    /// The `"effect"` tag names no shipped variant.
    #[error("unknown effect `{0}`")]
    UnknownEffect(String),

    /// A [`crate::Layers::SurfaceAndWindow`] effect ran a pass without
    /// `ctx.surface`.
    #[error("layered effect requires ctx.surface, got none")]
    MissingSurface,

    /// Live [`crate::ParamValue`]s don't line up with the effect's
    /// `params()` (wrong length or wrong kind).
    #[error("parameter mismatch: {0}")]
    ParamMismatch(String),
}
