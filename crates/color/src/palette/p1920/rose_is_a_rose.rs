use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "A Rose Is a Rose" palette - 1920s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum RoseIsARoseColor {
        /// PANTONE 18-1629 TCX `Faded Rose`.
        FadedRose => "faded-rose",
        /// PANTONE 13-1015 TCX `Honey Peach`.
        HoneyPeach => "honey-peach",
        /// PANTONE 18-1314 TCX `Acorn`.
        Acorn => "acorn",
        /// PANTONE 18-3415 TCX `Grape Jam`.
        GrapeJam => "grape-jam",
        /// PANTONE 16-1107 TCX `Aluminum`.
        Aluminum => "aluminum",
        /// PANTONE 19-4318 TCX `Mallard Blue`.
        MallardBlue => "mallard-blue"
    }
}

/// The sRGB values of the [`RoseIsARoseColor`] slots.
pub static ROSE_IS_A_ROSE: StaticPalette<RoseIsARoseColor, u8> = static_palette! {
    RoseIsARoseColor::FadedRose => PantoneFhiNamedColor::FadedRose.to_color(),
    RoseIsARoseColor::HoneyPeach => PantoneFhiNamedColor::HoneyPeach.to_color(),
    RoseIsARoseColor::Acorn => PantoneFhiNamedColor::Acorn.to_color(),
    RoseIsARoseColor::GrapeJam => PantoneFhiNamedColor::GrapeJam.to_color(),
    RoseIsARoseColor::Aluminum => PantoneFhiNamedColor::Aluminum.to_color(),
    RoseIsARoseColor::MallardBlue => PantoneFhiNamedColor::MallardBlue.to_color(),
};
