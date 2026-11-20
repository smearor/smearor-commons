use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Jewel Tones" palette - 1900s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum JewelTonesColor {
        /// PANTONE 16-0836 TCX `Rich Gold`.
        RichGold => "rich-gold",
        /// PANTONE 18-4148 TCX `Victoria Blue`.
        VictoriaBlue => "victoria-blue",
        /// PANTONE 17-5734 TCX `Viridis`.
        Viridis => "viridis",
        /// PANTONE 18-4440 TCX `Cloisonne`. Labeled `Cloisonné`.
        Cloisonne => "cloisonne",
        /// PANTONE 18-3418 TCX `Chinese Violet`.
        ChineseViolet => "chinese-violet",
        /// PANTONE 17-2120 TCX `Chateau Rose`.
        ChateauRose => "chateau-rose"
    }
}

/// The sRGB values of the [`JewelTonesColor`] slots.
pub static JEWEL_TONES: StaticPalette<JewelTonesColor, u8> = static_palette! {
    JewelTonesColor::RichGold => PantoneFhiNamedColor::RichGold.to_color(),
    JewelTonesColor::VictoriaBlue => PantoneFhiNamedColor::VictoriaBlue.to_color(),
    JewelTonesColor::Viridis => PantoneFhiNamedColor::Viridis.to_color(),
    JewelTonesColor::Cloisonne => PantoneFhiNamedColor::Cloisonne.to_color(),
    JewelTonesColor::ChineseViolet => PantoneFhiNamedColor::ChineseViolet.to_color(),
    JewelTonesColor::ChateauRose => PantoneFhiNamedColor::ChateauRose.to_color(),
};
