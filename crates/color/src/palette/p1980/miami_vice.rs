use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Miami Vice" palette - 1980s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum MiamiViceColor {
        /// PANTONE 13-2805 TCX `Pink Mist`.
        PinkMist => "pink-mist",
        /// PANTONE 15-3620 TCX `Lavendula`.
        Lavendula => "lavendula",
        /// PANTONE 18-3224 TCX `Radiant Orchid`.
        RadiantOrchid => "radiant-orchid",
        /// PANTONE 16-1624 TCX `Lantana`. Caption spells `Lavanta`.
        Lantana => "lantana",
        /// PANTONE 14-4103 TCX `Gray Violet`.
        GrayViolet => "gray-violet",
        /// PANTONE 19-3950 TCX `Deep Ultramarine`.
        DeepUltramarine => "deep-ultramarine",
        /// PANTONE 19-4203 TCX `Moonless Night`.
        MoonlessNight => "moonless-night"
    }
}

/// The sRGB values of the [`MiamiViceColor`] slots.
pub static MIAMI_VICE: StaticPalette<MiamiViceColor, u8> = static_palette! {
    MiamiViceColor::PinkMist => PantoneFhiNamedColor::PinkMist.to_color(),
    MiamiViceColor::Lavendula => PantoneFhiNamedColor::Lavendula.to_color(),
    MiamiViceColor::RadiantOrchid => PantoneFhiNamedColor::RadiantOrchid.to_color(),
    MiamiViceColor::Lantana => PantoneFhiNamedColor::Lantana.to_color(),
    MiamiViceColor::GrayViolet => PantoneFhiNamedColor::GrayViolet.to_color(),
    MiamiViceColor::DeepUltramarine => PantoneFhiNamedColor::DeepUltramarine.to_color(),
    MiamiViceColor::MoonlessNight => PantoneFhiNamedColor::MoonlessNight.to_color(),
};
