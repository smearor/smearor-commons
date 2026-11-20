use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Poiret Revolution" palette - 1900s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum PoiretRevolutionColor {
        /// PANTONE 16-1454 TCX `Jaffa Orange`.
        JaffaOrange => "jaffa-orange",
        /// PANTONE 14-0846 TCX `Yolk Yellow`.
        YolkYellow => "yolk-yellow",
        /// PANTONE 14-1025 TCX `Cocoon`.
        Cocoon => "cocoon",
        /// PANTONE 18-5424 TCX `Cadmium Green`.
        CadmiumGreen => "cadmium-green",
        /// PANTONE 18-3418 TCX `Chinese Violet`.
        ChineseViolet => "chinese-violet",
        /// PANTONE 17-3615 TCX `Chalk Violet`.
        ChalkViolet => "chalk-violet",
        /// PANTONE 16-3115 TCX `Crocus`.
        Crocus => "crocus"
    }
}

/// The sRGB values of the [`PoiretRevolutionColor`] slots.
pub static POIRET_REVOLUTION: StaticPalette<PoiretRevolutionColor, u8> = static_palette! {
    PoiretRevolutionColor::JaffaOrange => PantoneFhiNamedColor::JaffaOrange.to_color(),
    PoiretRevolutionColor::YolkYellow => PantoneFhiNamedColor::YolkYellow.to_color(),
    PoiretRevolutionColor::Cocoon => PantoneFhiNamedColor::Cocoon.to_color(),
    PoiretRevolutionColor::CadmiumGreen => PantoneFhiNamedColor::CadmiumGreen.to_color(),
    PoiretRevolutionColor::ChineseViolet => PantoneFhiNamedColor::ChineseViolet.to_color(),
    PoiretRevolutionColor::ChalkViolet => PantoneFhiNamedColor::ChalkViolet.to_color(),
    PoiretRevolutionColor::Crocus => PantoneFhiNamedColor::Crocus.to_color(),
};
