use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Edwardian Affairs" palette - 1900s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum EdwardianAffairsColor {
        /// PANTONE 12-0000 TCX `White Swan`.
        WhiteSwan => "white-swan",
        /// PANTONE 14-4106 TCX `Gray Dawn`.
        GrayDawn => "gray-dawn",
        /// PANTONE 14-0935 TCX `Jojoba`.
        Jojoba => "jojoba",
        /// PANTONE 16-1707 TCX `Deauville Mauve`.
        DeauvilleMauve => "deauville-mauve",
        /// PANTONE 16-1715 TCX `Wild Rose`.
        WildRose => "wild-rose",
        /// PANTONE 16-6116 TCX `Shale Green`.
        ShaleGreen => "shale-green",
        /// PANTONE 19-2014 TCX `Prune`.
        Prune => "prune",
        /// PANTONE 18-1629 TCX `Faded Rose`.
        FadedRose => "faded-rose"
    }
}

/// The sRGB values of the [`EdwardianAffairsColor`] slots.
pub static EDWARDIAN_AFFAIRS: StaticPalette<EdwardianAffairsColor, u8> = static_palette! {
    EdwardianAffairsColor::WhiteSwan => PantoneFhiNamedColor::WhiteSwan.to_color(),
    EdwardianAffairsColor::GrayDawn => PantoneFhiNamedColor::GrayDawn.to_color(),
    EdwardianAffairsColor::Jojoba => PantoneFhiNamedColor::Jojoba.to_color(),
    EdwardianAffairsColor::DeauvilleMauve => PantoneFhiNamedColor::DeauvilleMauve.to_color(),
    EdwardianAffairsColor::WildRose => PantoneFhiNamedColor::WildRose.to_color(),
    EdwardianAffairsColor::ShaleGreen => PantoneFhiNamedColor::ShaleGreen.to_color(),
    EdwardianAffairsColor::Prune => PantoneFhiNamedColor::Prune.to_color(),
    EdwardianAffairsColor::FadedRose => PantoneFhiNamedColor::FadedRose.to_color(),
};
