use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Majorelle and Morocco" palette - 1980s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum MajorelleAndMoroccoColor {
        /// PANTONE 18-4043 TCX `Palace Blue`.
        PalaceBlue => "palace-blue",
        /// PANTONE 15-1318 TCX `Pink Sand`.
        PinkSand => "pink-sand",
        /// PANTONE 17-1524 TCX `Desert Sand`.
        DesertSand => "desert-sand",
        /// PANTONE 14-1064 TCX `Saffron`.
        Saffron => "saffron",
        /// PANTONE 16-1441 TCX `Arabesque`.
        Arabesque => "arabesque",
        /// PANTONE 17-2624 TCX `Rose Violet`.
        RoseViolet => "rose-violet",
        /// PANTONE 19-3138 TCX `Byzantium`.
        Byzantium => "byzantium"
    }
}

/// The sRGB values of the [`MajorelleAndMoroccoColor`] slots.
pub static MAJORELLE_AND_MOROCCO: StaticPalette<MajorelleAndMoroccoColor, u8> = static_palette! {
    MajorelleAndMoroccoColor::PalaceBlue => PantoneFhiNamedColor::PalaceBlue.to_color(),
    MajorelleAndMoroccoColor::PinkSand => PantoneFhiNamedColor::PinkSand.to_color(),
    MajorelleAndMoroccoColor::DesertSand => PantoneFhiNamedColor::DesertSand.to_color(),
    MajorelleAndMoroccoColor::Saffron => PantoneFhiNamedColor::Saffron.to_color(),
    MajorelleAndMoroccoColor::Arabesque => PantoneFhiNamedColor::Arabesque.to_color(),
    MajorelleAndMoroccoColor::RoseViolet => PantoneFhiNamedColor::RoseViolet.to_color(),
    MajorelleAndMoroccoColor::Byzantium => PantoneFhiNamedColor::Byzantium.to_color(),
};
