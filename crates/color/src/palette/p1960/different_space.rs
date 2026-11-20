use crate::PantoneNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "A Different Space" palette - 1960s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum DifferentSpaceColor {
        /// Pantone Black C.
        Black => "black",
        /// Pantone 2747 C.
        Pms2747 => "pms-2747",
        /// Pantone 405 C.
        Pms405 => "pms-405",
        /// Pantone Cool Gray 8 C.
        CoolGray8 => "cool-gray-8",
        /// Pantone 1797 C.
        Pms1797 => "pms-1797",
        /// Pantone 7740 C.
        Pms7740 => "pms-7740",
        /// Pantone 715 C.
        Pms715 => "pms-715"
    }
}

/// The sRGB values of the [`DifferentSpaceColor`] slots.
pub static DIFFERENT_SPACE: StaticPalette<DifferentSpaceColor, u8> = static_palette! {
    DifferentSpaceColor::Black => PantoneNamedColor::BlackC.to_color(),
    DifferentSpaceColor::Pms2747 => PantoneNamedColor::Pms2747C.to_color(),
    DifferentSpaceColor::Pms405 => PantoneNamedColor::Pms405C.to_color(),
    DifferentSpaceColor::CoolGray8 => PantoneNamedColor::CoolGray8C.to_color(),
    DifferentSpaceColor::Pms1797 => PantoneNamedColor::Pms1797C.to_color(),
    DifferentSpaceColor::Pms7740 => PantoneNamedColor::Pms7740C.to_color(),
    DifferentSpaceColor::Pms715 => PantoneNamedColor::Pms715C.to_color(),
};
