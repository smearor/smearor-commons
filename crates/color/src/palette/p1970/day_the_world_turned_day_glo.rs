use crate::PantoneNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "The Day the World Turned Day-Glo" palette - 1970s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum DayTheWorldTurnedDayGloColor {
        /// Pantone Process Blue C.
        ProcessBlue => "process-blue",
        /// Pantone 7488 C.
        Pms7488 => "pms-7488",
        /// Pantone Yellow 012 C.
        Yellow012 => "yellow-012",
        /// Pantone 1575 C.
        Pms1575 => "pms-1575",
        /// Pantone Orange 021 C.
        Orange021 => "orange-021",
        /// Pantone 213 C.
        Pms213 => "pms-213",
        /// Pantone Rhodamine Red C.
        RhodamineRed => "rhodamine-red",
        /// Pantone Black 6 C.
        Black6 => "black-6"
    }
}

/// The sRGB values of the [`DayTheWorldTurnedDayGloColor`] slots.
pub static DAY_THE_WORLD_TURNED_DAY_GLO: StaticPalette<DayTheWorldTurnedDayGloColor, u8> = static_palette! {
    DayTheWorldTurnedDayGloColor::ProcessBlue => PantoneNamedColor::ProcessBlueC.to_color(),
    DayTheWorldTurnedDayGloColor::Pms7488 => PantoneNamedColor::Pms7488C.to_color(),
    DayTheWorldTurnedDayGloColor::Yellow012 => PantoneNamedColor::Yellow012C.to_color(),
    DayTheWorldTurnedDayGloColor::Pms1575 => PantoneNamedColor::Pms1575C.to_color(),
    DayTheWorldTurnedDayGloColor::Orange021 => PantoneNamedColor::Orange021C.to_color(),
    DayTheWorldTurnedDayGloColor::Pms213 => PantoneNamedColor::Pms213C.to_color(),
    DayTheWorldTurnedDayGloColor::RhodamineRed => PantoneNamedColor::RhodamineRedC.to_color(),
    DayTheWorldTurnedDayGloColor::Black6 => PantoneNamedColor::Black6C.to_color(),
};
