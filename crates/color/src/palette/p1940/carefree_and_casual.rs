use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Carefree and Casual" palette - 1940s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum CarefreeAndCasualColor {
        /// PANTONE 18-5621 TCX `Fir`.
        Fir => "fir",
        /// PANTONE 16-6127 TCX `Greenbriar`.
        Greenbriar => "greenbriar",
        /// PANTONE 15-4319 TCX `Air Blue`.
        AirBlue => "air-blue",
        /// PANTONE 17-3919 TCX `Purple Impression`.
        PurpleImpression => "purple-impression",
        /// PANTONE 11-4300 TCX `Marshmallow`.
        Marshmallow => "marshmallow",
        /// PANTONE 13-0746 TCX `Maize`.
        Maize => "maize",
        /// PANTONE 18-1555 TCX `Molten Lava`.
        MoltenLava => "molten-lava"
    }
}

/// The sRGB values of the [`CarefreeAndCasualColor`] slots.
pub static CAREFREE_AND_CASUAL: StaticPalette<CarefreeAndCasualColor, u8> = static_palette! {
    CarefreeAndCasualColor::Fir => PantoneFhiNamedColor::Fir.to_color(),
    CarefreeAndCasualColor::Greenbriar => PantoneFhiNamedColor::Greenbriar.to_color(),
    CarefreeAndCasualColor::AirBlue => PantoneFhiNamedColor::AirBlue.to_color(),
    CarefreeAndCasualColor::PurpleImpression => PantoneFhiNamedColor::PurpleImpression.to_color(),
    CarefreeAndCasualColor::Marshmallow => PantoneFhiNamedColor::Marshmallow.to_color(),
    CarefreeAndCasualColor::Maize => PantoneFhiNamedColor::Maize.to_color(),
    CarefreeAndCasualColor::MoltenLava => PantoneFhiNamedColor::MoltenLava.to_color(),
};
