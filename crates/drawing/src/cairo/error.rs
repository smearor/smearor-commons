//! The cairo backend's own error type.

/// Errors the cairo backend can surface beyond per-draw `cairo::Error`s:
/// surface bridging and pixel readback can fail for reasons cairo does
/// not report through its context status.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum DrawingError {
    /// A cairo runtime error from surface or context creation.
    #[error("cairo error: {0}")]
    Cairo(#[from] cairo::Error),
    /// `ImageSurface::data()` refused access because a `Context` or
    /// another surface handle still borrows the pixel memory
    /// (`BorrowError::NonExclusive`). Drop all contexts and clones that
    /// draw into the surface before reading it back.
    #[error("surface pixel data is still borrowed by a live cairo handle")]
    SurfaceBorrowed,
    /// A `BufferViewMut` layout cairo cannot wrap — e.g. a stride too
    /// small for the declared width, or dimensions exceeding `i32`.
    #[error("invalid surface parameters: {0}")]
    InvalidSurface(&'static str),
}
