use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "World War II" palette - 1940s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum WorldWar2Color {
        /// PANTONE 16-1334 TCX `Tan`.
        Tan => "tan",
        /// PANTONE 16-1110 TCX `Olive Gray`.
        OliveGray => "olive-gray",
        /// PANTONE 19-4024 TCX `Dress Blues`.
        DressBlues => "dress-blues",
        /// PANTONE 19-0810 TCX `Major Brown`.
        MajorBrown => "major-brown",
        /// PANTONE 19-0815 TCX `Desert Palm`.
        DesertPalm => "desert-palm",
        /// PANTONE 17-1553 TCX `Paprika`.
        Paprika => "paprika",
        /// PANTONE 17-4336 TCX `Blithe`.
        Blithe => "blithe"
    }
}

/// The sRGB values of the [`WorldWar2Color`] slots.
pub static WORLD_WAR_2: StaticPalette<WorldWar2Color, u8> = static_palette! {
    WorldWar2Color::Tan => PantoneFhiNamedColor::Tan.to_color(),
    WorldWar2Color::OliveGray => PantoneFhiNamedColor::OliveGray.to_color(),
    WorldWar2Color::DressBlues => PantoneFhiNamedColor::DressBlues.to_color(),
    WorldWar2Color::MajorBrown => PantoneFhiNamedColor::MajorBrown.to_color(),
    WorldWar2Color::DesertPalm => PantoneFhiNamedColor::DesertPalm.to_color(),
    WorldWar2Color::Paprika => PantoneFhiNamedColor::Paprika.to_color(),
    WorldWar2Color::Blithe => PantoneFhiNamedColor::Blithe.to_color(),
};
