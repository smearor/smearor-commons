use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Passage to India" palette - 1960s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum PassageToIndiaColor {
        /// PANTONE 15-2216 TCX `Sachet Pink`.
        SachetPink => "sachet-pink",
        /// PANTONE 15-1435 TCX `Desert Flower`.
        DesertFlower => "desert-flower",
        /// PANTONE 18-5841 TCX `Pepper Green`.
        PepperGreen => "pepper-green",
        /// PANTONE 15-1062 TCX `Gold Fusion`. Chip labeled `Amber Yellow`.
        GoldFusion => "gold-fusion",
        /// PANTONE 11-0601 TCX `Bright White`.
        BrightWhite => "bright-white",
        /// PANTONE 16-0836 TCX `Rich Gold`.
        RichGold => "rich-gold"
    }
}

/// The sRGB values of the [`PassageToIndiaColor`] slots.
pub static PASSAGE_TO_INDIA: StaticPalette<PassageToIndiaColor, u8> = static_palette! {
    PassageToIndiaColor::SachetPink => PantoneFhiNamedColor::SachetPink.to_color(),
    PassageToIndiaColor::DesertFlower => PantoneFhiNamedColor::DesertFlower.to_color(),
    PassageToIndiaColor::PepperGreen => PantoneFhiNamedColor::PepperGreen.to_color(),
    PassageToIndiaColor::GoldFusion => PantoneFhiNamedColor::GoldFusion.to_color(),
    PassageToIndiaColor::BrightWhite => PantoneFhiNamedColor::BrightWhite.to_color(),
    PassageToIndiaColor::RichGold => PantoneFhiNamedColor::RichGold.to_color(),
};
