use crate::Color;
use crate::PantoneNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Hotel California" palette - 1970s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum HotelCaliforniaColor {
        /// Pantone 3005 C.
        Pms3005 => "pms-3005",
        /// Pantone 293 C.
        Pms293 => "pms-293",
        /// Pantone 555 C.
        Pms555 => "pms-555",
        /// Pantone 7776 C; not in the PANTONE solid-coated catalog - the sRGB value is sampled from the book page.
        Pms7776 => "pms-7776",
        /// Pantone 156 C.
        Pms156 => "pms-156",
        /// Pantone 7622 C.
        Pms7622 => "pms-7622",
        /// Pantone 539 C.
        Pms539 => "pms-539"
    }
}

/// The sRGB values of the [`HotelCaliforniaColor`] slots.
pub static HOTEL_CALIFORNIA: StaticPalette<HotelCaliforniaColor, u8> = static_palette! {
    HotelCaliforniaColor::Pms3005 => PantoneNamedColor::Pms3005C.to_color(),
    HotelCaliforniaColor::Pms293 => PantoneNamedColor::Pms293C.to_color(),
    HotelCaliforniaColor::Pms555 => PantoneNamedColor::Pms555C.to_color(),
    HotelCaliforniaColor::Pms7776 => Color::from_rgb_u32(0xC2C016),
    HotelCaliforniaColor::Pms156 => PantoneNamedColor::Pms156C.to_color(),
    HotelCaliforniaColor::Pms7622 => PantoneNamedColor::Pms7622C.to_color(),
    HotelCaliforniaColor::Pms539 => PantoneNamedColor::Pms539C.to_color(),
};
