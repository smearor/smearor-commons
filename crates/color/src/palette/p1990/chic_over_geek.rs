use crate::PantoneNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Chic over Geek" palette - 1990s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum ChicOverGeekColor {
        /// Pantone 1505 C.
        Pms1505 => "pms-1505",
        /// Pantone 1935 C.
        Pms1935 => "pms-1935",
        /// Pantone 2597 C.
        Pms2597 => "pms-2597",
        /// Pantone 361 C.
        Pms361 => "pms-361",
        /// Pantone 285 C.
        Pms285 => "pms-285",
        /// Pantone Black 3 C.
        Black3 => "black-3"
    }
}

/// The sRGB values of the [`ChicOverGeekColor`] slots.
pub static CHIC_OVER_GEEK: StaticPalette<ChicOverGeekColor, u8> = static_palette! {
    ChicOverGeekColor::Pms1505 => PantoneNamedColor::Pms1505C.to_color(),
    ChicOverGeekColor::Pms1935 => PantoneNamedColor::Pms1935C.to_color(),
    ChicOverGeekColor::Pms2597 => PantoneNamedColor::Pms2597C.to_color(),
    ChicOverGeekColor::Pms361 => PantoneNamedColor::Pms361C.to_color(),
    ChicOverGeekColor::Pms285 => PantoneNamedColor::Pms285C.to_color(),
    ChicOverGeekColor::Black3 => PantoneNamedColor::Black3C.to_color(),
};
