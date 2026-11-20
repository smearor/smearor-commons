use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Modern Pleasures" palette - 1920s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum ModernPleasuresColor {
        /// PANTONE 14-0244 TCX `Bright Lime Green`.
        BrightLimeGreen => "bright-lime-green",
        /// PANTONE 12-0752 TCX `Buttercup`.
        Buttercup => "buttercup",
        /// PANTONE 16-1361 TCX `Carrot`.
        Carrot => "carrot",
        /// PANTONE 15-2216 TCX `Sachet Pink`.
        SachetPink => "sachet-pink",
        /// PANTONE 17-3934 TCX `Persian Jewel`.
        PersianJewel => "persian-jewel",
        /// PANTONE 16-3520 TCX `African Violet`.
        AfricanViolet => "african-violet",
        /// PANTONE 17-3628 TCX `Amethyst Orchid`.
        AmethystOrchid => "amethyst-orchid"
    }
}

/// The sRGB values of the [`ModernPleasuresColor`] slots.
pub static MODERN_PLEASURES: StaticPalette<ModernPleasuresColor, u8> = static_palette! {
    ModernPleasuresColor::BrightLimeGreen => PantoneFhiNamedColor::BrightLimeGreen.to_color(),
    ModernPleasuresColor::Buttercup => PantoneFhiNamedColor::Buttercup.to_color(),
    ModernPleasuresColor::Carrot => PantoneFhiNamedColor::Carrot.to_color(),
    ModernPleasuresColor::SachetPink => PantoneFhiNamedColor::SachetPink.to_color(),
    ModernPleasuresColor::PersianJewel => PantoneFhiNamedColor::PersianJewel.to_color(),
    ModernPleasuresColor::AfricanViolet => PantoneFhiNamedColor::AfricanViolet.to_color(),
    ModernPleasuresColor::AmethystOrchid => PantoneFhiNamedColor::AmethystOrchid.to_color(),
};
