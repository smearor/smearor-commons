use crate::PantoneNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Youthful Pastimes" palette - 1910s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum YouthfulPastimesColor {
        /// Pantone 7556 C.
        Pms7556 => "pms-7556",
        /// Pantone 7621 C.
        Pms7621 => "pms-7621",
        /// Pantone 462 C.
        Pms462 => "pms-462",
        /// Pantone 7527 C.
        Pms7527 => "pms-7527",
        /// Pantone 7410 C.
        Pms7410 => "pms-7410"
    }
}

/// The sRGB values of the [`YouthfulPastimesColor`] slots.
pub static YOUTHFUL_PASTIMES: StaticPalette<YouthfulPastimesColor, u8> = static_palette! {
    YouthfulPastimesColor::Pms7556 => PantoneNamedColor::Pms7556C.to_color(),
    YouthfulPastimesColor::Pms7621 => PantoneNamedColor::Pms7621C.to_color(),
    YouthfulPastimesColor::Pms462 => PantoneNamedColor::Pms462C.to_color(),
    YouthfulPastimesColor::Pms7527 => PantoneNamedColor::Pms7527C.to_color(),
    YouthfulPastimesColor::Pms7410 => PantoneNamedColor::Pms7410C.to_color(),
};
