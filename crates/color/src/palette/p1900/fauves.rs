use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "The Fauves" palette - 1900s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum FauvesColor {
        /// PANTONE 16-1454 TCX `Jaffa Orange`.
        JaffaOrange => "jaffa-orange",
        /// PANTONE 16-1543 TCX `Fusion Coral`.
        FusionCoral => "fusion-coral",
        /// PANTONE 14-3207 TCX `Pink Lavender`. Caption spells `Pink Lavendar`.
        PinkLavender => "pink-lavender",
        /// PANTONE 16-1723 TCX `Confetti`.
        Confetti => "confetti",
        /// PANTONE 18-4051 TCX `Strong Blue`.
        StrongBlue => "strong-blue",
        /// PANTONE 18-1658 TCX `Pompeian Red`.
        PompeianRed => "pompeian-red",
        /// PANTONE 17-0133 TCX `Fluorite Green`.
        FluoriteGreen => "fluorite-green",
        /// PANTONE 19-5917 TCX `Sycamore`.
        Sycamore => "sycamore"
    }
}

/// The sRGB values of the [`FauvesColor`] slots.
pub static FAUVES: StaticPalette<FauvesColor, u8> = static_palette! {
    FauvesColor::JaffaOrange => PantoneFhiNamedColor::JaffaOrange.to_color(),
    FauvesColor::FusionCoral => PantoneFhiNamedColor::FusionCoral.to_color(),
    FauvesColor::PinkLavender => PantoneFhiNamedColor::PinkLavender.to_color(),
    FauvesColor::Confetti => PantoneFhiNamedColor::Confetti.to_color(),
    FauvesColor::StrongBlue => PantoneFhiNamedColor::StrongBlue.to_color(),
    FauvesColor::PompeianRed => PantoneFhiNamedColor::PompeianRed.to_color(),
    FauvesColor::FluoriteGreen => PantoneFhiNamedColor::FluoriteGreen.to_color(),
    FauvesColor::Sycamore => PantoneFhiNamedColor::Sycamore.to_color(),
};
