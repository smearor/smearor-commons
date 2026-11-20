use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Parrish Blues" palette - 1910s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum ParrishBluesColor {
        /// PANTONE 17-4041 TCX `Marina`.
        Marina => "marina",
        /// PANTONE 18-4530 TCX `Celestial`.
        Celestial => "celestial",
        /// PANTONE 19-4053 TCX `Turkish Sea`.
        TurkishSea => "turkish-sea",
        /// PANTONE 17-3014 TCX `Mulberry`.
        Mulberry => "mulberry",
        /// PANTONE 17-2520 TCX `Ibis Rose`.
        IbisRose => "ibis-rose",
        /// PANTONE 17-0230 TCX `Forest Green`.
        ForestGreen => "forest-green",
        /// PANTONE 12-0811 TCX `Dawn`.
        Dawn => "dawn"
    }
}

/// The sRGB values of the [`ParrishBluesColor`] slots.
pub static PARRISH_BLUES: StaticPalette<ParrishBluesColor, u8> = static_palette! {
    ParrishBluesColor::Marina => PantoneFhiNamedColor::Marina.to_color(),
    ParrishBluesColor::Celestial => PantoneFhiNamedColor::Celestial.to_color(),
    ParrishBluesColor::TurkishSea => PantoneFhiNamedColor::TurkishSea.to_color(),
    ParrishBluesColor::Mulberry => PantoneFhiNamedColor::Mulberry.to_color(),
    ParrishBluesColor::IbisRose => PantoneFhiNamedColor::IbisRose.to_color(),
    ParrishBluesColor::ForestGreen => PantoneFhiNamedColor::ForestGreen.to_color(),
    ParrishBluesColor::Dawn => PantoneFhiNamedColor::Dawn.to_color(),
};
