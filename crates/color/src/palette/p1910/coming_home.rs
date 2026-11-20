use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Coming Home" palette - 1910s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum ComingHomeColor {
        /// PANTONE 14-1025 TCX `Cocoon`.
        Cocoon => "cocoon",
        /// PANTONE 13-0939 TCX `Golden Cream`.
        GoldenCream => "golden-cream",
        /// PANTONE 17-1137 TCX `Cashew`.
        Cashew => "cashew",
        /// PANTONE 16-3920 TCX `Lavender Lustre`.
        LavenderLustre => "lavender-lustre",
        /// PANTONE 19-4057 TCX `True Blue`.
        TrueBlue => "true-blue",
        /// PANTONE 18-0318 TCX `Deep Lichen Green`.
        DeepLichenGreen => "deep-lichen-green",
        /// PANTONE 19-4203 TCX `Moonless Night`.
        MoonlessNight => "moonless-night"
    }
}

/// The sRGB values of the [`ComingHomeColor`] slots.
pub static COMING_HOME: StaticPalette<ComingHomeColor, u8> = static_palette! {
    ComingHomeColor::Cocoon => PantoneFhiNamedColor::Cocoon.to_color(),
    ComingHomeColor::GoldenCream => PantoneFhiNamedColor::GoldenCream.to_color(),
    ComingHomeColor::Cashew => PantoneFhiNamedColor::Cashew.to_color(),
    ComingHomeColor::LavenderLustre => PantoneFhiNamedColor::LavenderLustre.to_color(),
    ComingHomeColor::TrueBlue => PantoneFhiNamedColor::TrueBlue.to_color(),
    ComingHomeColor::DeepLichenGreen => PantoneFhiNamedColor::DeepLichenGreen.to_color(),
    ComingHomeColor::MoonlessNight => PantoneFhiNamedColor::MoonlessNight.to_color(),
};
