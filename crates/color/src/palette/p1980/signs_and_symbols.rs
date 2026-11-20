use crate::PantoneNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Signs and Symbols" palette - 1980s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum SignsAndSymbolsColor {
        /// Pantone 583 C.
        Pms583 => "pms-583",
        /// Pantone 7480 C.
        Pms7480 => "pms-7480",
        /// Pantone 299 C.
        Pms299 => "pms-299",
        /// Pantone 170 C.
        Pms170 => "pms-170",
        /// Pantone 267 C.
        Pms267 => "pms-267",
        /// Pantone 191 C.
        Pms191 => "pms-191",
        /// Pantone Black C.
        Black => "black"
    }
}

/// The sRGB values of the [`SignsAndSymbolsColor`] slots.
pub static SIGNS_AND_SYMBOLS: StaticPalette<SignsAndSymbolsColor, u8> = static_palette! {
    SignsAndSymbolsColor::Pms583 => PantoneNamedColor::Pms583C.to_color(),
    SignsAndSymbolsColor::Pms7480 => PantoneNamedColor::Pms7480C.to_color(),
    SignsAndSymbolsColor::Pms299 => PantoneNamedColor::Pms299C.to_color(),
    SignsAndSymbolsColor::Pms170 => PantoneNamedColor::Pms170C.to_color(),
    SignsAndSymbolsColor::Pms267 => PantoneNamedColor::Pms267C.to_color(),
    SignsAndSymbolsColor::Pms191 => PantoneNamedColor::Pms191C.to_color(),
    SignsAndSymbolsColor::Black => PantoneNamedColor::BlackC.to_color(),
};
