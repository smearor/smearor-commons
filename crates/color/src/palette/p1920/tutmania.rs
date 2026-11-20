use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Tutmania" palette - 1920s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum TutmaniaColor {
        /// PANTONE 16-1836 TCX `Rich Gold`. The caption reads 16-1836; the official code of `Rich Gold` is 16-0836.
        RichGold => "rich-gold",
        /// PANTONE 19-1540 TCX `Burnt Henna`.
        BurntHenna => "burnt-henna",
        /// PANTONE 19-4245 TCX `Imperial Blue`.
        ImperialBlue => "imperial-blue",
        /// PANTONE 15-5209 TCX `Aqua Haze`.
        AquaHaze => "aqua-haze",
        /// PANTONE 14-0121 TCX `Nile Green`.
        NileGreen => "nile-green",
        /// PANTONE 14-0936 TCX `Sahara Sun`.
        SaharaSun => "sahara-sun",
        /// PANTONE 16-0110 TCX `Desert Sage`.
        DesertSage => "desert-sage"
    }
}

/// The sRGB values of the [`TutmaniaColor`] slots.
pub static TUTMANIA: StaticPalette<TutmaniaColor, u8> = static_palette! {
    TutmaniaColor::RichGold => PantoneFhiNamedColor::RichGold.to_color(),
    TutmaniaColor::BurntHenna => PantoneFhiNamedColor::BurntHenna.to_color(),
    TutmaniaColor::ImperialBlue => PantoneFhiNamedColor::ImperialBlue.to_color(),
    TutmaniaColor::AquaHaze => PantoneFhiNamedColor::AquaHaze.to_color(),
    TutmaniaColor::NileGreen => PantoneFhiNamedColor::NileGreen.to_color(),
    TutmaniaColor::SaharaSun => PantoneFhiNamedColor::SaharaSun.to_color(),
    TutmaniaColor::DesertSage => PantoneFhiNamedColor::DesertSage.to_color(),
};
