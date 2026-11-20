use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Cubism" palette - 1910s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum CubismColor {
        /// PANTONE 19-3903 TCX `Shale`.
        Shale => "shale",
        /// PANTONE 18-3710 TCX `Gray Ridge`.
        GrayRidge => "gray-ridge",
        /// PANTONE 19-1118 TCX `Chestnut`.
        Chestnut => "chestnut",
        /// PANTONE 18-0940 TCX `Golden Brown`. The caption reads 18-1940; the chip label and the official code of `Golden Brown` are 18-0940.
        GoldenBrown => "golden-brown",
        /// PANTONE 16-1334 TCX `Tan`.
        Tan => "tan",
        /// PANTONE 18-1550 TCX `Aurora Red`.
        AuroraRed => "aurora-red",
        /// PANTONE 19-4026 TCX `Ensign Blue`.
        EnsignBlue => "ensign-blue",
        /// PANTONE 19-5513 TCX `Dark Green`.
        DarkGreen => "dark-green"
    }
}

/// The sRGB values of the [`CubismColor`] slots.
pub static CUBISM: StaticPalette<CubismColor, u8> = static_palette! {
    CubismColor::Shale => PantoneFhiNamedColor::Shale.to_color(),
    CubismColor::GrayRidge => PantoneFhiNamedColor::GrayRidge.to_color(),
    CubismColor::Chestnut => PantoneFhiNamedColor::Chestnut.to_color(),
    CubismColor::GoldenBrown => PantoneFhiNamedColor::GoldenBrown.to_color(),
    CubismColor::Tan => PantoneFhiNamedColor::Tan.to_color(),
    CubismColor::AuroraRed => PantoneFhiNamedColor::AuroraRed.to_color(),
    CubismColor::EnsignBlue => PantoneFhiNamedColor::EnsignBlue.to_color(),
    CubismColor::DarkGreen => PantoneFhiNamedColor::DarkGreen.to_color(),
};
