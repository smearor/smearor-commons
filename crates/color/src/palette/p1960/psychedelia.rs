use crate::PantoneNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Psychedelia" palette - 1960s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum PsychedeliaColor {
        /// Pantone 7649 C.
        Pms7649 => "pms-7649",
        /// Pantone 265 C.
        Pms265 => "pms-265",
        /// Pantone 246 C.
        Pms246 => "pms-246",
        /// Pantone 383 C.
        Pms383 => "pms-383",
        /// Pantone 7548 C.
        Pms7548 => "pms-7548",
        /// Pantone 2735 C.
        Pms2735 => "pms-2735",
        /// Pantone 1505 C.
        Pms1505 => "pms-1505",
        /// Pantone 2995 C.
        Pms2995 => "pms-2995"
    }
}

/// The sRGB values of the [`PsychedeliaColor`] slots.
pub static PSYCHEDELIA: StaticPalette<PsychedeliaColor, u8> = static_palette! {
    PsychedeliaColor::Pms7649 => PantoneNamedColor::Pms7649C.to_color(),
    PsychedeliaColor::Pms265 => PantoneNamedColor::Pms265C.to_color(),
    PsychedeliaColor::Pms246 => PantoneNamedColor::Pms246C.to_color(),
    PsychedeliaColor::Pms383 => PantoneNamedColor::Pms383C.to_color(),
    PsychedeliaColor::Pms7548 => PantoneNamedColor::Pms7548C.to_color(),
    PsychedeliaColor::Pms2735 => PantoneNamedColor::Pms2735C.to_color(),
    PsychedeliaColor::Pms1505 => PantoneNamedColor::Pms1505C.to_color(),
    PsychedeliaColor::Pms2995 => PantoneNamedColor::Pms2995C.to_color(),
};
