use crate::PantoneNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Edward Hopper" palette - 1940s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum EdwardHopperColor {
        /// Pantone 7477 C.
        Pms7477 => "pms-7477",
        /// Pantone 7728 C.
        Pms7728 => "pms-7728",
        /// Pantone 7597 C.
        Pms7597 => "pms-7597",
        /// Pantone 131 C.
        Pms131 => "pms-131",
        /// Pantone 724 C.
        Pms724 => "pms-724"
    }
}

/// The sRGB values of the [`EdwardHopperColor`] slots.
pub static EDWARD_HOPPER: StaticPalette<EdwardHopperColor, u8> = static_palette! {
    EdwardHopperColor::Pms7477 => PantoneNamedColor::Pms7477C.to_color(),
    EdwardHopperColor::Pms7728 => PantoneNamedColor::Pms7728C.to_color(),
    EdwardHopperColor::Pms7597 => PantoneNamedColor::Pms7597C.to_color(),
    EdwardHopperColor::Pms131 => PantoneNamedColor::Pms131C.to_color(),
    EdwardHopperColor::Pms724 => PantoneNamedColor::Pms724C.to_color(),
};
