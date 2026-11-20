use crate::PantoneNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Fantasyland" palette - 1950s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum FantasylandColor {
        /// Pantone 211 C.
        Pms211 => "pms-211",
        /// Pantone 7655 C.
        Pms7655 => "pms-7655",
        /// Pantone 285 C.
        Pms285 => "pms-285",
        /// Pantone 7768 C.
        Pms7768 => "pms-7768",
        /// Pantone Black 6 C.
        Black6 => "black-6",
        /// Pantone White C.
        White => "white"
    }
}

/// The sRGB values of the [`FantasylandColor`] slots.
pub static FANTASYLAND: StaticPalette<FantasylandColor, u8> = static_palette! {
    FantasylandColor::Pms211 => PantoneNamedColor::Pms211C.to_color(),
    FantasylandColor::Pms7655 => PantoneNamedColor::Pms7655C.to_color(),
    FantasylandColor::Pms285 => PantoneNamedColor::Pms285C.to_color(),
    FantasylandColor::Pms7768 => PantoneNamedColor::Pms7768C.to_color(),
    FantasylandColor::Black6 => PantoneNamedColor::Black6C.to_color(),
    FantasylandColor::White => PantoneNamedColor::WhiteC.to_color(),
};
