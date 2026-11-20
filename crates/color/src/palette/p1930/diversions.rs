use crate::PantoneNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Diversions" palette - 1930s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum DiversionsColor {
        /// Pantone 5473 C.
        Pms5473 => "pms-5473",
        /// Pantone 185 C.
        Pms185 => "pms-185",
        /// Pantone 7549 C.
        Pms7549 => "pms-7549",
        /// Pantone 1505 C.
        Pms1505 => "pms-1505",
        /// Pantone 661 C.
        Pms661 => "pms-661",
        /// Pantone 1565 C.
        Pms1565 => "pms-1565",
        /// Pantone 658 C.
        Pms658 => "pms-658"
    }
}

/// The sRGB values of the [`DiversionsColor`] slots.
pub static DIVERSIONS: StaticPalette<DiversionsColor, u8> = static_palette! {
    DiversionsColor::Pms5473 => PantoneNamedColor::Pms5473C.to_color(),
    DiversionsColor::Pms185 => PantoneNamedColor::Pms185C.to_color(),
    DiversionsColor::Pms7549 => PantoneNamedColor::Pms7549C.to_color(),
    DiversionsColor::Pms1505 => PantoneNamedColor::Pms1505C.to_color(),
    DiversionsColor::Pms661 => PantoneNamedColor::Pms661C.to_color(),
    DiversionsColor::Pms1565 => PantoneNamedColor::Pms1565C.to_color(),
    DiversionsColor::Pms658 => PantoneNamedColor::Pms658C.to_color(),
};
