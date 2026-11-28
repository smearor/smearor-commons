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

    /// [`crate::gtk::snapshot`] got a zero-size or node-less widget.
    #[cfg(feature = "gtk4")]
    #[error("widget snapshot produced no pixels")]
    SnapshotEmpty,

    /// A cairo status error surfaced by the GTK adapter - carried as
    /// text so the error type stays GTK-free.
    #[cfg(feature = "cairo")]
    #[error("cairo error: {0}")]
    Cairo(String),

    /// [`crate::gtk::snapshot_to_png`] failed past the snapshot step -
    /// buffer-to-surface conversion or the PNG file write, carried as
    /// text like [`Self::Cairo`].
    #[cfg(feature = "gtk4")]
    #[error("png dump failed: {0}")]
    PngDump(String),
}
