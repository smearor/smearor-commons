use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Kensington and Carnaby" palette - 1960s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum KensingtonAndCarnabyColor {
        /// PANTONE 18-3937 TCX `Blue Yonder`.
        BlueYonder => "blue-yonder",
        /// PANTONE 19-1016 TCX `Java`.
        Java => "java",
        /// PANTONE 19-1726 TCX `Cordovan`.
        Cordovan => "cordovan",
        /// PANTONE 17-1537 TCX `Mineral Red`.
        MineralRed => "mineral-red",
        /// PANTONE 19-3438 TCX `Bright Violet`.
        BrightViolet => "bright-violet",
        /// PANTONE 16-1454 TCX `Jaffa Orange`.
        JaffaOrange => "jaffa-orange",
        /// PANTONE 14-1122 TCX `Sheepskin`.
        Sheepskin => "sheepskin",
        /// PANTONE 15-0927 TCX `Pale Gold`.
        PaleGold => "pale-gold"
    }
}

/// The sRGB values of the [`KensingtonAndCarnabyColor`] slots.
pub static KENSINGTON_AND_CARNABY: StaticPalette<KensingtonAndCarnabyColor, u8> = static_palette! {
    KensingtonAndCarnabyColor::BlueYonder => PantoneFhiNamedColor::BlueYonder.to_color(),
    KensingtonAndCarnabyColor::Java => PantoneFhiNamedColor::Java.to_color(),
    KensingtonAndCarnabyColor::Cordovan => PantoneFhiNamedColor::Cordovan.to_color(),
    KensingtonAndCarnabyColor::MineralRed => PantoneFhiNamedColor::MineralRed.to_color(),
    KensingtonAndCarnabyColor::BrightViolet => PantoneFhiNamedColor::BrightViolet.to_color(),
    KensingtonAndCarnabyColor::JaffaOrange => PantoneFhiNamedColor::JaffaOrange.to_color(),
    KensingtonAndCarnabyColor::Sheepskin => PantoneFhiNamedColor::Sheepskin.to_color(),
    KensingtonAndCarnabyColor::PaleGold => PantoneFhiNamedColor::PaleGold.to_color(),
};
