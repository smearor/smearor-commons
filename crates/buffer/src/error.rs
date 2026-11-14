use crate::Stride;
use smearor_commons_geometry::Pixels;
use thiserror::Error;

/// Errors from buffer construction and view validation.
#[derive(Debug, Error)]
pub enum BufferError {
    /// The data length does not match the extent's packed size.
    #[error("buffer length {actual} does not match {width}x{height} x {bytes_per_pixel} = {expected} bytes")]
    LengthMismatch {
        /// The expected packed length (`width * height * bytes_per_pixel`).
        expected: usize,
        /// The actual data length.
        actual: usize,
        /// The declared width.
        width: Pixels<u32>,
        /// The declared height.
        height: Pixels<u32>,
        /// The format's bytes per pixel.
        bytes_per_pixel: usize,
    },

    /// The stride is smaller than a packed pixel row.
    #[error("stride {stride} is smaller than the packed row width {row_bytes}")]
    StrideTooSmall {
        /// The declared stride in bytes per row.
        stride: Stride,
        /// The packed row width (`width * bytes_per_pixel`).
        row_bytes: usize,
    },

    /// The data is too short for the declared metadata.
    #[error("data length {actual} is smaller than the required {required} bytes")]
    DataTooShort {
        /// The minimal length covering every pixel row.
        required: usize,
        /// The actual data length.
        actual: usize,
    },
}
