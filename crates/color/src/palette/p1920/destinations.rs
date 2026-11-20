use crate::PantoneNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Destinations" palette - 1920s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum DestinationsColor {
        /// Pantone 7410 C.
        Pms7410 => "pms-7410",
        /// Pantone 7407 C.
        Pms7407 => "pms-7407",
        /// Pantone 7566 C.
        Pms7566 => "pms-7566",
        /// Pantone 7499 C.
        Pms7499 => "pms-7499",
        /// Pantone 7580 C.
        Pms7580 => "pms-7580",
        /// Pantone 5783 C.
        Pms5783 => "pms-5783",
        /// Pantone 5763 C.
        Pms5763 => "pms-5763",
        /// Pantone 476 C.
        Pms476 => "pms-476"
    }
}

/// The sRGB values of the [`DestinationsColor`] slots.
pub static DESTINATIONS: StaticPalette<DestinationsColor, u8> = static_palette! {
    DestinationsColor::Pms7410 => PantoneNamedColor::Pms7410C.to_color(),
    DestinationsColor::Pms7407 => PantoneNamedColor::Pms7407C.to_color(),
    DestinationsColor::Pms7566 => PantoneNamedColor::Pms7566C.to_color(),
    DestinationsColor::Pms7499 => PantoneNamedColor::Pms7499C.to_color(),
    DestinationsColor::Pms7580 => PantoneNamedColor::Pms7580C.to_color(),
    DestinationsColor::Pms5783 => PantoneNamedColor::Pms5783C.to_color(),
    DestinationsColor::Pms5763 => PantoneNamedColor::Pms5763C.to_color(),
    DestinationsColor::Pms476 => PantoneNamedColor::Pms476C.to_color(),
};
