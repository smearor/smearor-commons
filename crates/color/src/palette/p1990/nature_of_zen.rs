use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "The Nature of Zen" palette - 1990s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum NatureOfZenColor {
        /// PANTONE 14-0627 TCX `Shadow Green`.
        ShadowGreen => "shadow-green",
        /// PANTONE 16-6116 TCX `Shale Green`.
        ShaleGreen => "shale-green",
        /// PANTONE 15-6410 TCX `Moss Gray`.
        MossGray => "moss-gray",
        /// PANTONE 14-4506 TCX `Ether`.
        Ether => "ether",
        /// PANTONE 15-5812 TCX `Lichen`.
        Lichen => "lichen",
        /// PANTONE 11-0304 TCX `Water Lily`.
        WaterLily => "water-lily"
    }
}

/// The sRGB values of the [`NatureOfZenColor`] slots.
pub static NATURE_OF_ZEN: StaticPalette<NatureOfZenColor, u8> = static_palette! {
    NatureOfZenColor::ShadowGreen => PantoneFhiNamedColor::ShadowGreen.to_color(),
    NatureOfZenColor::ShaleGreen => PantoneFhiNamedColor::ShaleGreen.to_color(),
    NatureOfZenColor::MossGray => PantoneFhiNamedColor::MossGray.to_color(),
    NatureOfZenColor::Ether => PantoneFhiNamedColor::Ether.to_color(),
    NatureOfZenColor::Lichen => PantoneFhiNamedColor::Lichen.to_color(),
    NatureOfZenColor::WaterLily => PantoneFhiNamedColor::WaterLily.to_color(),
};
