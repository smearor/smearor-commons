use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Grunge and Graffiti" palette - 1990s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum GrungeAndGraffitiColor {
        /// PANTONE 18-3718 TCX `Purple Haze`.
        PurpleHaze => "purple-haze",
        /// PANTONE 19-0915 TCX `Coffee Bean`.
        CoffeeBean => "coffee-bean",
        /// PANTONE 17-4021 TCX `Faded Denim`.
        FadedDenim => "faded-denim",
        /// PANTONE 18-1631 TCX `Earth Red`.
        EarthRed => "earth-red",
        /// PANTONE 17-3802 TCX `Gull`.
        Gull => "gull",
        /// PANTONE 19-3906 TCX `Dark Shadow`.
        DarkShadow => "dark-shadow"
    }
}

/// The sRGB values of the [`GrungeAndGraffitiColor`] slots.
pub static GRUNGE_AND_GRAFFITI: StaticPalette<GrungeAndGraffitiColor, u8> = static_palette! {
    GrungeAndGraffitiColor::PurpleHaze => PantoneFhiNamedColor::PurpleHaze.to_color(),
    GrungeAndGraffitiColor::CoffeeBean => PantoneFhiNamedColor::CoffeeBean.to_color(),
    GrungeAndGraffitiColor::FadedDenim => PantoneFhiNamedColor::FadedDenim.to_color(),
    GrungeAndGraffitiColor::EarthRed => PantoneFhiNamedColor::EarthRed.to_color(),
    GrungeAndGraffitiColor::Gull => PantoneFhiNamedColor::Gull.to_color(),
    GrungeAndGraffitiColor::DarkShadow => PantoneFhiNamedColor::DarkShadow.to_color(),
};
