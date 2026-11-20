use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "To the Manor Born" palette - 1980s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum ToTheManorBornColor {
        /// PANTONE 19-6110 TCX `Deep Forest`. Caption spells `Deep Forrest`.
        DeepForest => "deep-forest",
        /// PANTONE 19-5420 TCX `Evergreen`.
        Evergreen => "evergreen",
        /// PANTONE 19-1034 TCX `Breen`.
        Breen => "breen",
        /// PANTONE 19-1762 TCX `Crimson`.
        Crimson => "crimson",
        /// PANTONE 16-0836 TCX `Rich Gold`.
        RichGold => "rich-gold"
    }
}

/// The sRGB values of the [`ToTheManorBornColor`] slots.
pub static TO_THE_MANOR_BORN: StaticPalette<ToTheManorBornColor, u8> = static_palette! {
    ToTheManorBornColor::DeepForest => PantoneFhiNamedColor::DeepForest.to_color(),
    ToTheManorBornColor::Evergreen => PantoneFhiNamedColor::Evergreen.to_color(),
    ToTheManorBornColor::Breen => PantoneFhiNamedColor::Breen.to_color(),
    ToTheManorBornColor::Crimson => PantoneFhiNamedColor::Crimson.to_color(),
    ToTheManorBornColor::RichGold => PantoneFhiNamedColor::RichGold.to_color(),
};
