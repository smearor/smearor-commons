use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Future Forecasts" palette - 1990s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum FutureForecastsColor {
        /// PANTONE 19-1655 TCX `Garnet`.
        Garnet => "garnet",
        /// PANTONE 19-3903 TCX `Shale`.
        Shale => "shale",
        /// PANTONE 19-1623 TCX `Vineyard Wine`. Caption spells `Vinyard Wine`.
        VineyardWine => "vineyard-wine",
        /// PANTONE 18-1248 TCX `Rust`.
        Rust => "rust",
        /// PANTONE 18-1415 TCX `Marron`.
        Marron => "marron",
        /// PANTONE 19-4110 TCX `Midnight Navy`.
        MidnightNavy => "midnight-navy",
        /// PANTONE 18-1449 TCX `Ketchup`.
        Ketchup => "ketchup",
        /// PANTONE 19-4340 TCX `Lyons Blue`.
        LyonsBlue => "lyons-blue",
        /// PANTONE 19-3842 TCX `Deep Wisteria`.
        DeepWisteria => "deep-wisteria",
        /// PANTONE 16-0540 TCX `Oasis`.
        Oasis => "oasis",
        /// PANTONE 16-4411 TCX `Tourmaline`.
        Tourmaline => "tourmaline",
        /// PANTONE 16-1324 TCX `Lark`.
        Lark => "lark"
    }
}

/// The sRGB values of the [`FutureForecastsColor`] slots.
pub static FUTURE_FORECASTS: StaticPalette<FutureForecastsColor, u8> = static_palette! {
    FutureForecastsColor::Garnet => PantoneFhiNamedColor::Garnet.to_color(),
    FutureForecastsColor::Shale => PantoneFhiNamedColor::Shale.to_color(),
    FutureForecastsColor::VineyardWine => PantoneFhiNamedColor::VineyardWine.to_color(),
    FutureForecastsColor::Rust => PantoneFhiNamedColor::Rust.to_color(),
    FutureForecastsColor::Marron => PantoneFhiNamedColor::Marron.to_color(),
    FutureForecastsColor::MidnightNavy => PantoneFhiNamedColor::MidnightNavy.to_color(),
    FutureForecastsColor::Ketchup => PantoneFhiNamedColor::Ketchup.to_color(),
    FutureForecastsColor::LyonsBlue => PantoneFhiNamedColor::LyonsBlue.to_color(),
    FutureForecastsColor::DeepWisteria => PantoneFhiNamedColor::DeepWisteria.to_color(),
    FutureForecastsColor::Oasis => PantoneFhiNamedColor::Oasis.to_color(),
    FutureForecastsColor::Tourmaline => PantoneFhiNamedColor::Tourmaline.to_color(),
    FutureForecastsColor::Lark => PantoneFhiNamedColor::Lark.to_color(),
};
