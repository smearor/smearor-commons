use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Roseville" palette - 1930s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum RosevilleColor {
        /// PANTONE 17-0215 TCX `Aspen Green`.
        AspenGreen => "aspen-green",
        /// PANTONE 18-3937 TCX `Blue Yonder`.
        BlueYonder => "blue-yonder",
        /// PANTONE 16-1336 TCX `Biscuit`.
        Biscuit => "biscuit",
        /// PANTONE 14-1935 TCX `Jojoba`. The caption reads 14-1935; the official code of `Jojoba` is 14-0935.
        Jojoba => "jojoba",
        /// PANTONE 14-1316 TCX `Dusty Pink`.
        DustyPink => "dusty-pink",
        /// PANTONE 17-1608 TCX `Heather Rose`.
        HeatherRose => "heather-rose",
        /// PANTONE 16-3521 TCX `Lupine`.
        Lupine => "lupine",
        /// PANTONE 18-3418 TCX `Chinese Violet`.
        ChineseViolet => "chinese-violet"
    }
}

/// The sRGB values of the [`RosevilleColor`] slots.
pub static ROSEVILLE: StaticPalette<RosevilleColor, u8> = static_palette! {
    RosevilleColor::AspenGreen => PantoneFhiNamedColor::AspenGreen.to_color(),
    RosevilleColor::BlueYonder => PantoneFhiNamedColor::BlueYonder.to_color(),
    RosevilleColor::Biscuit => PantoneFhiNamedColor::Biscuit.to_color(),
    RosevilleColor::Jojoba => PantoneFhiNamedColor::Jojoba.to_color(),
    RosevilleColor::DustyPink => PantoneFhiNamedColor::DustyPink.to_color(),
    RosevilleColor::HeatherRose => PantoneFhiNamedColor::HeatherRose.to_color(),
    RosevilleColor::Lupine => PantoneFhiNamedColor::Lupine.to_color(),
    RosevilleColor::ChineseViolet => PantoneFhiNamedColor::ChineseViolet.to_color(),
};
