use crate::PantoneNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Anime" palette - 1990s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum AnimeColor {
        /// Pantone 230 C.
        Pms230 => "pms-230",
        /// Pantone 231 C.
        Pms231 => "pms-231",
        /// Pantone 318 C.
        Pms318 => "pms-318",
        /// Pantone 293 C.
        Pms293 => "pms-293",
        /// Pantone 361 C.
        Pms361 => "pms-361",
        /// Pantone 382 C.
        Pms382 => "pms-382",
        /// Pantone 1675 C.
        Pms1675 => "pms-1675",
        /// Pantone 107 C.
        Pms107 => "pms-107",
        /// Pantone 528 C.
        Pms528 => "pms-528"
    }
}

/// The sRGB values of the [`AnimeColor`] slots.
pub static ANIME: StaticPalette<AnimeColor, u8> = static_palette! {
    AnimeColor::Pms230 => PantoneNamedColor::Pms230C.to_color(),
    AnimeColor::Pms231 => PantoneNamedColor::Pms231C.to_color(),
    AnimeColor::Pms318 => PantoneNamedColor::Pms318C.to_color(),
    AnimeColor::Pms293 => PantoneNamedColor::Pms293C.to_color(),
    AnimeColor::Pms361 => PantoneNamedColor::Pms361C.to_color(),
    AnimeColor::Pms382 => PantoneNamedColor::Pms382C.to_color(),
    AnimeColor::Pms1675 => PantoneNamedColor::Pms1675C.to_color(),
    AnimeColor::Pms107 => PantoneNamedColor::Pms107C.to_color(),
    AnimeColor::Pms528 => PantoneNamedColor::Pms528C.to_color(),
};
