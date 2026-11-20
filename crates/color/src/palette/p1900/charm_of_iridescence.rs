use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "The Charm of Iridescence" palette - 1900s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum CharmOfIridescenceColor {
        /// PANTONE 15-1927 TCX `Pale Gold`. The caption reads 15-1927; the official code of `Pale Gold` is 15-0927.
        PaleGold => "pale-gold",
        /// PANTONE 16-0730 TCX `Antique Gold`.
        AntiqueGold => "antique-gold",
        /// PANTONE 18-6330 TCX `Juniper`.
        Juniper => "juniper",
        /// PANTONE 18-1928 TCX `Sepia`. The caption reads 18-1928; the official code of `Sepia` is 18-0928.
        Sepia => "sepia",
        /// PANTONE 19-4914 TCX `Deep Teal`.
        DeepTeal => "deep-teal",
        /// PANTONE 15-3817 TCX `Lavender`.
        Lavender => "lavender",
        /// PANTONE 14-1311 TCX `Evening Sand`.
        EveningSand => "evening-sand",
        /// PANTONE 12-2103 TCX `Almost Mauve`.
        AlmostMauve => "almost-mauve"
    }
}

/// The sRGB values of the [`CharmOfIridescenceColor`] slots.
pub static CHARM_OF_IRIDESCENCE: StaticPalette<CharmOfIridescenceColor, u8> = static_palette! {
    CharmOfIridescenceColor::PaleGold => PantoneFhiNamedColor::PaleGold.to_color(),
    CharmOfIridescenceColor::AntiqueGold => PantoneFhiNamedColor::AntiqueGold.to_color(),
    CharmOfIridescenceColor::Juniper => PantoneFhiNamedColor::Juniper.to_color(),
    CharmOfIridescenceColor::Sepia => PantoneFhiNamedColor::Sepia.to_color(),
    CharmOfIridescenceColor::DeepTeal => PantoneFhiNamedColor::DeepTeal.to_color(),
    CharmOfIridescenceColor::Lavender => PantoneFhiNamedColor::Lavender.to_color(),
    CharmOfIridescenceColor::EveningSand => PantoneFhiNamedColor::EveningSand.to_color(),
    CharmOfIridescenceColor::AlmostMauve => PantoneFhiNamedColor::AlmostMauve.to_color(),
};
