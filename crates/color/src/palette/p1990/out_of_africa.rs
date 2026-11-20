use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Out of Africa" palette - 1990s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum OutOfAfricaColor {
        /// PANTONE 18-1454 TCX `Red Clay`.
        RedClay => "red-clay",
        /// PANTONE 18-0130 TCX `Cactus`.
        Cactus => "cactus",
        /// PANTONE 18-3949 TCX `Dazzling Blue`.
        DazzlingBlue => "dazzling-blue",
        /// PANTONE 11-0103 TCX `Egret`.
        Egret => "egret",
        /// PANTONE 17-1436 TCX `Raw Sienna`.
        RawSienna => "raw-sienna",
        /// PANTONE 12-0921 TCX `Golden Straw`.
        GoldenStraw => "golden-straw",
        /// PANTONE 18-1016 TCX `Cub`.
        Cub => "cub",
        /// PANTONE 19-1020 TCX `Dark Earth`.
        DarkEarth => "dark-earth"
    }
}

/// The sRGB values of the [`OutOfAfricaColor`] slots.
pub static OUT_OF_AFRICA: StaticPalette<OutOfAfricaColor, u8> = static_palette! {
    OutOfAfricaColor::RedClay => PantoneFhiNamedColor::RedClay.to_color(),
    OutOfAfricaColor::Cactus => PantoneFhiNamedColor::Cactus.to_color(),
    OutOfAfricaColor::DazzlingBlue => PantoneFhiNamedColor::DazzlingBlue.to_color(),
    OutOfAfricaColor::Egret => PantoneFhiNamedColor::Egret.to_color(),
    OutOfAfricaColor::RawSienna => PantoneFhiNamedColor::RawSienna.to_color(),
    OutOfAfricaColor::GoldenStraw => PantoneFhiNamedColor::GoldenStraw.to_color(),
    OutOfAfricaColor::Cub => PantoneFhiNamedColor::Cub.to_color(),
    OutOfAfricaColor::DarkEarth => PantoneFhiNamedColor::DarkEarth.to_color(),
};
