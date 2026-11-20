use crate::PantoneNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Santa Fe" palette - 1980s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum SantaFeColor {
        /// Pantone 4735 C.
        Pms4735 => "pms-4735",
        /// Pantone 7629 C.
        Pms7629 => "pms-7629",
        /// Pantone 7606 C.
        Pms7606 => "pms-7606",
        /// Pantone 7618 C.
        Pms7618 => "pms-7618",
        /// Pantone 7536 C.
        Pms7536 => "pms-7536",
        /// Pantone 5145 C.
        Pms5145 => "pms-5145",
        /// Pantone 5155 C.
        Pms5155 => "pms-5155"
    }
}

/// The sRGB values of the [`SantaFeColor`] slots.
pub static SANTA_FE: StaticPalette<SantaFeColor, u8> = static_palette! {
    SantaFeColor::Pms4735 => PantoneNamedColor::Pms4735C.to_color(),
    SantaFeColor::Pms7629 => PantoneNamedColor::Pms7629C.to_color(),
    SantaFeColor::Pms7606 => PantoneNamedColor::Pms7606C.to_color(),
    SantaFeColor::Pms7618 => PantoneNamedColor::Pms7618C.to_color(),
    SantaFeColor::Pms7536 => PantoneNamedColor::Pms7536C.to_color(),
    SantaFeColor::Pms5145 => PantoneNamedColor::Pms5145C.to_color(),
    SantaFeColor::Pms5155 => PantoneNamedColor::Pms5155C.to_color(),
};
