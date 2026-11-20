use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "The Wizard of Oz" palette - 1930s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum WizardOfOzColor {
        /// PANTONE 14-4121 TCX `Blue Bell`.
        BlueBell => "blue-bell",
        /// PANTONE 17-1664 TCX `Poppy Red`.
        PoppyRed => "poppy-red",
        /// PANTONE 14-5002 TCX `Silver`.
        Silver => "silver",
        /// PANTONE 13-0922 TCX `Straw`.
        Straw => "straw",
        /// PANTONE 17-1330 TCX `Lion`.
        Lion => "lion",
        /// PANTONE 16-6339 TCX `Vibrant Green`.
        VibrantGreen => "vibrant-green",
        /// PANTONE 14-0957 TCX `Spectra Yellow`.
        SpectraYellow => "spectra-yellow"
    }
}

/// The sRGB values of the [`WizardOfOzColor`] slots.
pub static WIZARD_OF_OZ: StaticPalette<WizardOfOzColor, u8> = static_palette! {
    WizardOfOzColor::BlueBell => PantoneFhiNamedColor::BlueBell.to_color(),
    WizardOfOzColor::PoppyRed => PantoneFhiNamedColor::PoppyRed.to_color(),
    WizardOfOzColor::Silver => PantoneFhiNamedColor::Silver.to_color(),
    WizardOfOzColor::Straw => PantoneFhiNamedColor::Straw.to_color(),
    WizardOfOzColor::Lion => PantoneFhiNamedColor::Lion.to_color(),
    WizardOfOzColor::VibrantGreen => PantoneFhiNamedColor::VibrantGreen.to_color(),
    WizardOfOzColor::SpectraYellow => PantoneFhiNamedColor::SpectraYellow.to_color(),
};
