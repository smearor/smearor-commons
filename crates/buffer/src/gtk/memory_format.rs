//! `PixelFormatId` ⇄ `gdk::MemoryFormat` — a byte-exact mapping for
//! every 8-bit format; 16/32-bit and float memory formats have no
//! `PixelFormatId` counterpart and fail the fallible direction.

use gtk4::gdk::MemoryFormat;

use crate::PixelFormatId;

impl From<PixelFormatId> for MemoryFormat {
    fn from(format: PixelFormatId) -> Self {
        match format {
            PixelFormatId::Rgba => MemoryFormat::R8g8b8a8,
            PixelFormatId::Bgra => MemoryFormat::B8g8r8a8,
            PixelFormatId::RgbaPremultiplied => MemoryFormat::R8g8b8a8Premultiplied,
            PixelFormatId::BgraPremultiplied => MemoryFormat::B8g8r8a8Premultiplied,
            PixelFormatId::Rgb => MemoryFormat::R8g8b8,
            PixelFormatId::Bgr => MemoryFormat::B8g8r8,
            PixelFormatId::Rgbx => MemoryFormat::R8g8b8x8,
            PixelFormatId::Bgrx => MemoryFormat::B8g8r8x8,
            PixelFormatId::Gray => MemoryFormat::G8,
        }
    }
}

impl TryFrom<MemoryFormat> for PixelFormatId {
    /// The unsupported [`MemoryFormat`], handed back to the caller.
    type Error = MemoryFormat;

    fn try_from(format: MemoryFormat) -> Result<Self, MemoryFormat> {
        Ok(match format {
            MemoryFormat::R8g8b8a8 => PixelFormatId::Rgba,
            MemoryFormat::B8g8r8a8 => PixelFormatId::Bgra,
            MemoryFormat::R8g8b8a8Premultiplied => PixelFormatId::RgbaPremultiplied,
            MemoryFormat::B8g8r8a8Premultiplied => PixelFormatId::BgraPremultiplied,
            MemoryFormat::R8g8b8 => PixelFormatId::Rgb,
            MemoryFormat::B8g8r8 => PixelFormatId::Bgr,
            MemoryFormat::R8g8b8x8 => PixelFormatId::Rgbx,
            MemoryFormat::B8g8r8x8 => PixelFormatId::Bgrx,
            MemoryFormat::G8 => PixelFormatId::Gray,
            _ => return Err(format),
        })
    }
}
