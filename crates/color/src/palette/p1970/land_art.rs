use crate::PantoneNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Land Art" palette - 1970s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum LandArtColor {
        /// Pantone 284 C.
        Pms284 => "pms-284",
        /// Pantone 454 C.
        Pms454 => "pms-454",
        /// Pantone 7612 C.
        Pms7612 => "pms-7612",
        /// Pantone 406 C.
        Pms406 => "pms-406",
        /// Pantone 7674 C.
        Pms7674 => "pms-7674",
        /// Pantone 408 C.
        Pms408 => "pms-408",
        /// Pantone 267 C.
        Pms267 => "pms-267"
    }
}

/// The sRGB values of the [`LandArtColor`] slots.
pub static LAND_ART: StaticPalette<LandArtColor, u8> = static_palette! {
    LandArtColor::Pms284 => PantoneNamedColor::Pms284C.to_color(),
    LandArtColor::Pms454 => PantoneNamedColor::Pms454C.to_color(),
    LandArtColor::Pms7612 => PantoneNamedColor::Pms7612C.to_color(),
    LandArtColor::Pms406 => PantoneNamedColor::Pms406C.to_color(),
    LandArtColor::Pms7674 => PantoneNamedColor::Pms7674C.to_color(),
    LandArtColor::Pms408 => PantoneNamedColor::Pms408C.to_color(),
    LandArtColor::Pms267 => PantoneNamedColor::Pms267C.to_color(),
};
