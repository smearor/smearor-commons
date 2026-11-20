use crate::PantoneNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Warhol" palette - 1960s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum WarholColor {
        /// Pantone 7621 C.
        Pms7621 => "pms-7621",
        /// Pantone 158 C.
        Pms158 => "pms-158",
        /// Pantone 7713 C.
        Pms7713 => "pms-7713",
        /// Pantone 340 C.
        Pms340 => "pms-340",
        /// Pantone 318 C.
        Pms318 => "pms-318",
        /// Pantone 516 C.
        Pms516 => "pms-516",
        /// Pantone 7662 C.
        Pms7662 => "pms-7662",
        /// Pantone 7744 C.
        Pms7744 => "pms-7744",
        /// Pantone 1235 C.
        Pms1235 => "pms-1235"
    }
}

/// The sRGB values of the [`WarholColor`] slots.
pub static WARHOL: StaticPalette<WarholColor, u8> = static_palette! {
    WarholColor::Pms7621 => PantoneNamedColor::Pms7621C.to_color(),
    WarholColor::Pms158 => PantoneNamedColor::Pms158C.to_color(),
    WarholColor::Pms7713 => PantoneNamedColor::Pms7713C.to_color(),
    WarholColor::Pms340 => PantoneNamedColor::Pms340C.to_color(),
    WarholColor::Pms318 => PantoneNamedColor::Pms318C.to_color(),
    WarholColor::Pms516 => PantoneNamedColor::Pms516C.to_color(),
    WarholColor::Pms7662 => PantoneNamedColor::Pms7662C.to_color(),
    WarholColor::Pms7744 => PantoneNamedColor::Pms7744C.to_color(),
    WarholColor::Pms1235 => PantoneNamedColor::Pms1235C.to_color(),
};
