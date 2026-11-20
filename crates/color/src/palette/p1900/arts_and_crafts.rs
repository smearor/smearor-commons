use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Arts and Crafts" palette - 1900s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum ArtsAndCraftsColor {
        /// PANTONE 17-1410 TCX `Pine Bark`.
        PineBark => "pine-bark",
        /// PANTONE 19-1618 TCX `Beech`. The caption reads 19-1618; the official code of `Beech` is 19-0618.
        Beech => "beech",
        /// PANTONE 11-0105 TCX `Antique White`.
        AntiqueWhite => "antique-white",
        /// PANTONE 13-1108 TCX `Cream Tan`.
        CreamTan => "cream-tan",
        /// PANTONE 18-1142 TCX `Leather Brown`.
        LeatherBrown => "leather-brown",
        /// PANTONE 17-1347 TCX `Autumn Leaf`.
        AutumnLeaf => "autumn-leaf",
        /// PANTONE 18-5610 TCX `Brittany Blue`.
        BrittanyBlue => "brittany-blue",
        /// PANTONE 18-0422 TCX `Loden Green`.
        LodenGreen => "loden-green"
    }
}

/// The sRGB values of the [`ArtsAndCraftsColor`] slots.
pub static ARTS_AND_CRAFTS: StaticPalette<ArtsAndCraftsColor, u8> = static_palette! {
    ArtsAndCraftsColor::PineBark => PantoneFhiNamedColor::PineBark.to_color(),
    ArtsAndCraftsColor::Beech => PantoneFhiNamedColor::Beech.to_color(),
    ArtsAndCraftsColor::AntiqueWhite => PantoneFhiNamedColor::AntiqueWhite.to_color(),
    ArtsAndCraftsColor::CreamTan => PantoneFhiNamedColor::CreamTan.to_color(),
    ArtsAndCraftsColor::LeatherBrown => PantoneFhiNamedColor::LeatherBrown.to_color(),
    ArtsAndCraftsColor::AutumnLeaf => PantoneFhiNamedColor::AutumnLeaf.to_color(),
    ArtsAndCraftsColor::BrittanyBlue => PantoneFhiNamedColor::BrittanyBlue.to_color(),
    ArtsAndCraftsColor::LodenGreen => PantoneFhiNamedColor::LodenGreen.to_color(),
};
