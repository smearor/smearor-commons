//! Pixel and graphics buffer types - ownership, format, extent and stride.
//!
//! [`PixelBuffer`] owns a packed pixel buffer, [`BufferView`] and
//! [`BufferViewMut`] borrow strided foreign memory (Wayland SHM,
//! framework surfaces), and [`BufferMetadata`] carries the
//! format-agnostic width/height/stride triple. The pixel format is a
//! type-level marker ([`Rgba`], [`Bgra`]) implementing [`PixelFormat`],
//! so format mismatches are compile errors rather than runtime checks.
//!
//! Drawing operations live downstream (`pixel-drawing`); framework
//! bridges (`image`, `cairo`, `gtk4`, `smithay`) are added behind
//! dependency-named features.

mod error;
mod formats;
mod metadata;
mod pixel_buffer;
mod stride;
mod view;
mod view_mut;

pub use error::BufferError;
pub use formats::Bgr;
pub use formats::Bgra;
pub use formats::Bgrx;
pub use formats::Gray;
pub use formats::PixelFormat;
pub use formats::PixelFormatId;
pub use formats::Rgb;
pub use formats::Rgba;
pub use formats::Rgbx;
pub use metadata::BufferMetadata;
pub use pixel_buffer::PixelBuffer;
pub use stride::Stride;
pub use view::BufferView;
pub use view_mut::BufferViewMut;
