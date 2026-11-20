use crate::Color;
use crate::PantoneNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Deco Architecture" palette - 1930s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum DecoArchitectureColor {
        /// Pantone `Silver` metallic chip (closest catalog color is Pantone 877 C); not in the PANTONE solid-coated catalog - the sRGB value is sampled from the book page.
        Silver => "silver",
        /// Pantone 10366 C; not in the PANTONE solid-coated catalog - the sRGB value is sampled from the book page.
        Pms10366 => "pms-10366",
        /// Pantone 7533 C.
        Pms7533 => "pms-7533",
        /// Pantone 5635 C.
        Pms5635 => "pms-5635",
        /// Pantone 4725 C.
        Pms4725 => "pms-4725"
    }
}

/// The sRGB values of the [`DecoArchitectureColor`] slots.
pub static DECO_ARCHITECTURE: StaticPalette<DecoArchitectureColor, u8> = static_palette! {
    DecoArchitectureColor::Silver => Color::from_rgb_u32(0xABAFB0),
    DecoArchitectureColor::Pms10366 => Color::from_rgb_u32(0xAA8161),
    DecoArchitectureColor::Pms7533 => PantoneNamedColor::Pms7533C.to_color(),
    DecoArchitectureColor::Pms5635 => PantoneNamedColor::Pms5635C.to_color(),
    DecoArchitectureColor::Pms4725 => PantoneNamedColor::Pms4725C.to_color(),
};
