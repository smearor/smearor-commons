//! Pixel format markers - the byte layout of a pixel at the type level.
//!
//! The marker types carry no data; they tag [`crate::PixelBuffer`] and
//! the views so a `BGRA` buffer cannot flow into an `RGBA` code path.
//! Downstream formats (RGB565, YUV, ...) implement [`PixelFormat`]
//! themselves - the set is deliberately open.

mod bgr;
mod bgra;
mod bgrx;
mod gray;
mod pixel_format;
mod pixel_format_id;
mod rgb;
mod rgba;
mod rgbx;

pub use bgr::Bgr;
pub use bgra::Bgra;
pub use bgrx::Bgrx;
pub use gray::Gray;
pub use pixel_format::PixelFormat;
pub use pixel_format_id::PixelFormatId;
pub use rgb::Rgb;
pub use rgba::Rgba;
pub use rgbx::Rgbx;
