use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Bauhaus" palette - 1920s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum BauhausColor {
        /// PANTONE 19-4203 TCX `Moonless Night`.
        MoonlessNight => "moonless-night",
        /// PANTONE 18-3944 TCX `Violet Storm`.
        VioletStorm => "violet-storm",
        /// PANTONE 15-3817 TCX `Lavender`.
        Lavender => "lavender",
        /// PANTONE 19-4039 TCX `Delft`.
        Delft => "delft",
        /// PANTONE 19-1524 TCX `Oxblood Red`.
        OxbloodRed => "oxblood-red",
        /// PANTONE 18-1354 TCX `Burnt Ochre`.
        BurntOchre => "burnt-ochre",
        /// PANTONE 14-0846 TCX `Yolk Yellow`.
        YolkYellow => "yolk-yellow",
        /// PANTONE 16-1054 TCX `Sunflower`.
        Sunflower => "sunflower"
    }
}

/// The sRGB values of the [`BauhausColor`] slots.
pub static BAUHAUS: StaticPalette<BauhausColor, u8> = static_palette! {
    BauhausColor::MoonlessNight => PantoneFhiNamedColor::MoonlessNight.to_color(),
    BauhausColor::VioletStorm => PantoneFhiNamedColor::VioletStorm.to_color(),
    BauhausColor::Lavender => PantoneFhiNamedColor::Lavender.to_color(),
    BauhausColor::Delft => PantoneFhiNamedColor::Delft.to_color(),
    BauhausColor::OxbloodRed => PantoneFhiNamedColor::OxbloodRed.to_color(),
    BauhausColor::BurntOchre => PantoneFhiNamedColor::BurntOchre.to_color(),
    BauhausColor::YolkYellow => PantoneFhiNamedColor::YolkYellow.to_color(),
    BauhausColor::Sunflower => PantoneFhiNamedColor::Sunflower.to_color(),
};
