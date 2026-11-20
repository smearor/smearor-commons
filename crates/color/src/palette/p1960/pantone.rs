use crate::PantoneNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "PANTONE" palette - 1960s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum PantoneColor {
        /// Pantone Rubine Red C.
        RubineRed => "rubine-red",
        /// Pantone Rhodamine Red C.
        RhodamineRed => "rhodamine-red",
        /// Pantone Purple C.
        Purple => "purple",
        /// Pantone Reflex Blue C.
        ReflexBlue => "reflex-blue",
        /// Pantone Yellow C.
        Yellow => "yellow",
        /// Pantone Warm Red C.
        WarmRed => "warm-red",
        /// Pantone Process Blue C.
        ProcessBlue => "process-blue",
        /// Pantone Green C.
        Green => "green"
    }
}

/// The sRGB values of the [`PantoneColor`] slots.
pub static PANTONE: StaticPalette<PantoneColor, u8> = static_palette! {
    PantoneColor::RubineRed => PantoneNamedColor::RubineRedC.to_color(),
    PantoneColor::RhodamineRed => PantoneNamedColor::RhodamineRedC.to_color(),
    PantoneColor::Purple => PantoneNamedColor::PurpleC.to_color(),
    PantoneColor::ReflexBlue => PantoneNamedColor::ReflexBlueC.to_color(),
    PantoneColor::Yellow => PantoneNamedColor::YellowC.to_color(),
    PantoneColor::WarmRed => PantoneNamedColor::WarmRedC.to_color(),
    PantoneColor::ProcessBlue => PantoneNamedColor::ProcessBlueC.to_color(),
    PantoneColor::Green => PantoneNamedColor::GreenC.to_color(),
};
