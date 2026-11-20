use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Urban Cowboys" palette - 1980s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum UrbanCowboysColor {
        /// PANTONE 19-4118 TCX `Dark Denim`.
        DarkDenim => "dark-denim",
        /// PANTONE 17-4021 TCX `Faded Denim`.
        FadedDenim => "faded-denim",
        /// PANTONE 16-3803 TCX `Gull Gray`.
        GullGray => "gull-gray",
        /// PANTONE 18-3930 TCX `Bleached Denim`.
        BleachedDenim => "bleached-denim",
        /// PANTONE 18-1142 TCX `Leather Brown`.
        LeatherBrown => "leather-brown",
        /// PANTONE 18-1027 TCX `Bison`.
        Bison => "bison"
    }
}

/// The sRGB values of the [`UrbanCowboysColor`] slots.
pub static URBAN_COWBOYS: StaticPalette<UrbanCowboysColor, u8> = static_palette! {
    UrbanCowboysColor::DarkDenim => PantoneFhiNamedColor::DarkDenim.to_color(),
    UrbanCowboysColor::FadedDenim => PantoneFhiNamedColor::FadedDenim.to_color(),
    UrbanCowboysColor::GullGray => PantoneFhiNamedColor::GullGray.to_color(),
    UrbanCowboysColor::BleachedDenim => PantoneFhiNamedColor::BleachedDenim.to_color(),
    UrbanCowboysColor::LeatherBrown => PantoneFhiNamedColor::LeatherBrown.to_color(),
    UrbanCowboysColor::Bison => PantoneFhiNamedColor::Bison.to_color(),
};
