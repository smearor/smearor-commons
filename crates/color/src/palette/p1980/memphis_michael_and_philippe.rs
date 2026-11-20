use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Memphis, Michael, and Philippe" palette - 1980s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum MemphisMichaelAndPhilippeColor {
        /// PANTONE 19-3847 TCX `Deep Blue`.
        DeepBlue => "deep-blue",
        /// PANTONE 19-1663 TCX `Ribbon Red`.
        RibbonRed => "ribbon-red",
        /// PANTONE 17-0235 TCX `Piquant Green`.
        PiquantGreen => "piquant-green",
        /// PANTONE 19-4007 TCX `Anthracite`.
        Anthracite => "anthracite",
        /// PANTONE 13-0935 TCX `Flax`.
        Flax => "flax",
        /// PANTONE 16-1235 TCX `Sandstorm`.
        Sandstorm => "sandstorm",
        /// PANTONE 16-4021 TCX `Allure`.
        Allure => "allure"
    }
}

/// The sRGB values of the [`MemphisMichaelAndPhilippeColor`] slots.
pub static MEMPHIS_MICHAEL_AND_PHILIPPE: StaticPalette<MemphisMichaelAndPhilippeColor, u8> = static_palette! {
    MemphisMichaelAndPhilippeColor::DeepBlue => PantoneFhiNamedColor::DeepBlue.to_color(),
    MemphisMichaelAndPhilippeColor::RibbonRed => PantoneFhiNamedColor::RibbonRed.to_color(),
    MemphisMichaelAndPhilippeColor::PiquantGreen => PantoneFhiNamedColor::PiquantGreen.to_color(),
    MemphisMichaelAndPhilippeColor::Anthracite => PantoneFhiNamedColor::Anthracite.to_color(),
    MemphisMichaelAndPhilippeColor::Flax => PantoneFhiNamedColor::Flax.to_color(),
    MemphisMichaelAndPhilippeColor::Sandstorm => PantoneFhiNamedColor::Sandstorm.to_color(),
    MemphisMichaelAndPhilippeColor::Allure => PantoneFhiNamedColor::Allure.to_color(),
};
