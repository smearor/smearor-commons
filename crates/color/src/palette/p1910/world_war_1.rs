use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "World War I" palette - 1910s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum WorldWar1Color {
        /// PANTONE 16-1108 TCX `Twill`.
        Twill => "twill",
        /// PANTONE 19-5411 TCX `Trekking Green`.
        TrekkingGreen => "trekking-green",
        /// PANTONE 19-4024 TCX `Dress Blues`.
        DressBlues => "dress-blues",
        /// PANTONE 18-4225 TCX `Saxony Blue`.
        SaxonyBlue => "saxony-blue",
        /// PANTONE 17-1558 TCX `Grenadine`.
        Grenadine => "grenadine",
        /// PANTONE 11-0601 TCX `Bright White`.
        BrightWhite => "bright-white"
    }
}

/// The sRGB values of the [`WorldWar1Color`] slots.
pub static WORLD_WAR_1: StaticPalette<WorldWar1Color, u8> = static_palette! {
    WorldWar1Color::Twill => PantoneFhiNamedColor::Twill.to_color(),
    WorldWar1Color::TrekkingGreen => PantoneFhiNamedColor::TrekkingGreen.to_color(),
    WorldWar1Color::DressBlues => PantoneFhiNamedColor::DressBlues.to_color(),
    WorldWar1Color::SaxonyBlue => PantoneFhiNamedColor::SaxonyBlue.to_color(),
    WorldWar1Color::Grenadine => PantoneFhiNamedColor::Grenadine.to_color(),
    WorldWar1Color::BrightWhite => PantoneFhiNamedColor::BrightWhite.to_color(),
};
