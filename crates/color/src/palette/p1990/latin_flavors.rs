use crate::PantoneNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Latin Flavors" palette - 1990s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum LatinFlavorsColor {
        /// Pantone 136 C.
        Pms136 => "pms-136",
        /// Pantone 370 C.
        Pms370 => "pms-370",
        /// Pantone 7592 C.
        Pms7592 => "pms-7592",
        /// Pantone 184 C.
        Pms184 => "pms-184",
        /// Pantone 1795 C.
        Pms1795 => "pms-1795",
        /// Pantone 527 C.
        Pms527 => "pms-527"
    }
}

/// The sRGB values of the [`LatinFlavorsColor`] slots.
pub static LATIN_FLAVORS: StaticPalette<LatinFlavorsColor, u8> = static_palette! {
    LatinFlavorsColor::Pms136 => PantoneNamedColor::Pms136C.to_color(),
    LatinFlavorsColor::Pms370 => PantoneNamedColor::Pms370C.to_color(),
    LatinFlavorsColor::Pms7592 => PantoneNamedColor::Pms7592C.to_color(),
    LatinFlavorsColor::Pms184 => PantoneNamedColor::Pms184C.to_color(),
    LatinFlavorsColor::Pms1795 => PantoneNamedColor::Pms1795C.to_color(),
    LatinFlavorsColor::Pms527 => PantoneNamedColor::Pms527C.to_color(),
};
