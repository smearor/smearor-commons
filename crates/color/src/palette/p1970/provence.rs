use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Provence" palette - 1970s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum ProvenceColor {
        /// PANTONE 16-4032 TCX `Provence`.
        Provence => "provence",
        /// PANTONE 17-4432 TCX `Vivid Blue`.
        VividBlue => "vivid-blue",
        /// PANTONE 18-0125 TCX `Artichoke Green`.
        ArtichokeGreen => "artichoke-green",
        /// PANTONE 19-1840 TCX `Deep Claret`.
        DeepClaret => "deep-claret",
        /// PANTONE 17-2627 TCX `Phlox Pink`. Caption spells `Plox Pink`.
        PhloxPink => "phlox-pink",
        /// PANTONE 18-3633 TCX `Deep Lavender`.
        DeepLavender => "deep-lavender",
        /// PANTONE 15-3817 TCX `Lavender`.
        Lavender => "lavender"
    }
}

/// The sRGB values of the [`ProvenceColor`] slots.
pub static PROVENCE: StaticPalette<ProvenceColor, u8> = static_palette! {
    ProvenceColor::Provence => PantoneFhiNamedColor::Provence.to_color(),
    ProvenceColor::VividBlue => PantoneFhiNamedColor::VividBlue.to_color(),
    ProvenceColor::ArtichokeGreen => PantoneFhiNamedColor::ArtichokeGreen.to_color(),
    ProvenceColor::DeepClaret => PantoneFhiNamedColor::DeepClaret.to_color(),
    ProvenceColor::PhloxPink => PantoneFhiNamedColor::PhloxPink.to_color(),
    ProvenceColor::DeepLavender => PantoneFhiNamedColor::DeepLavender.to_color(),
    ProvenceColor::Lavender => PantoneFhiNamedColor::Lavender.to_color(),
};
