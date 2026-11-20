use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Mid-Century Modernists" palette - 1950s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum MidCenturyModernistsColor {
        /// PANTONE 14-0848 TCX `Mimosa`.
        Mimosa => "mimosa",
        /// PANTONE 15-1242 TCX `Muskmelon`.
        Muskmelon => "muskmelon",
        /// PANTONE 16-0435 TCX `Dark Citron`. Caption spells `Dark Cirton`.
        DarkCitron => "dark-citron",
        /// PANTONE 11-0701 TCX `Whisper White`.
        WhisperWhite => "whisper-white",
        /// PANTONE 18-4005 TCX `Steel Gray`.
        SteelGray => "steel-gray",
        /// PANTONE 19-0414 TCX `Forest Night`.
        ForestNight => "forest-night",
        /// PANTONE 15-5209 TCX `Aqua Haze`.
        AquaHaze => "aqua-haze"
    }
}

/// The sRGB values of the [`MidCenturyModernistsColor`] slots.
pub static MID_CENTURY_MODERNISTS: StaticPalette<MidCenturyModernistsColor, u8> = static_palette! {
    MidCenturyModernistsColor::Mimosa => PantoneFhiNamedColor::Mimosa.to_color(),
    MidCenturyModernistsColor::Muskmelon => PantoneFhiNamedColor::Muskmelon.to_color(),
    MidCenturyModernistsColor::DarkCitron => PantoneFhiNamedColor::DarkCitron.to_color(),
    MidCenturyModernistsColor::WhisperWhite => PantoneFhiNamedColor::WhisperWhite.to_color(),
    MidCenturyModernistsColor::SteelGray => PantoneFhiNamedColor::SteelGray.to_color(),
    MidCenturyModernistsColor::ForestNight => PantoneFhiNamedColor::ForestNight.to_color(),
    MidCenturyModernistsColor::AquaHaze => PantoneFhiNamedColor::AquaHaze.to_color(),
};
