//! The [`DrawBackend`] base trait: one error type per render target.

/// Base trait that ties a target's capability impls to a single error
/// type, shared across all capability traits via the supertrait bound.
///
/// Capability methods return `Result<(), Self::Error>` so error
/// handling becomes a property of the target type rather than a tax on
/// every call: the software-raster target uses
/// [`core::convert::Infallible`] (slice writes cannot fail), while the
/// cairo backend propagates `cairo::Error`. This preserves cairo's
/// deferred-error semantics — a failed surface keeps reporting the same
/// sticky error through every subsequent call.
pub trait DrawBackend {
    /// The error produced by this target's draw operations.
    type Error;
}

impl<T: DrawBackend + ?Sized> DrawBackend for &mut T {
    type Error = T::Error;
}
