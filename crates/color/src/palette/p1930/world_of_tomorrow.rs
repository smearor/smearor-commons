use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "The World of Tomorrow" palette - 1930s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum WorldOfTomorrowColor {
        /// PANTONE 19-3847 TCX `Deep Blue`.
        DeepBlue => "deep-blue",
        /// PANTONE 19-4057 TCX `True Blue`.
        TrueBlue => "true-blue",
        /// PANTONE 11-0601 TCX `Bright White`.
        BrightWhite => "bright-white",
        /// PANTONE 13-0082 TCX `Sunlight`.
        Sunlight => "sunlight",
        /// PANTONE 19-4205 TCX `Phantom`.
        Phantom => "phantom",
        /// PANTONE 14-5002 TCX `Silver`.
        Silver => "silver"
    }
}

/// The sRGB values of the [`WorldOfTomorrowColor`] slots.
pub static WORLD_OF_TOMORROW: StaticPalette<WorldOfTomorrowColor, u8> = static_palette! {
    WorldOfTomorrowColor::DeepBlue => PantoneFhiNamedColor::DeepBlue.to_color(),
    WorldOfTomorrowColor::TrueBlue => PantoneFhiNamedColor::TrueBlue.to_color(),
    WorldOfTomorrowColor::BrightWhite => PantoneFhiNamedColor::BrightWhite.to_color(),
    WorldOfTomorrowColor::Sunlight => PantoneFhiNamedColor::Sunlight.to_color(),
    WorldOfTomorrowColor::Phantom => PantoneFhiNamedColor::Phantom.to_color(),
    WorldOfTomorrowColor::Silver => PantoneFhiNamedColor::Silver.to_color(),
};
