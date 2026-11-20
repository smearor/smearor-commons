use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Feathers and Leathers" palette - 1970s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum FeathersAndLeathersColor {
        /// PANTONE 19-4329 TCX `Corsair`.
        Corsair => "corsair",
        /// PANTONE 16-0518 TCX `Gray Green`.
        GrayGreen => "gray-green",
        /// PANTONE 16-1334 TCX `Tan`.
        Tan => "tan",
        /// PANTONE 19-1217 TCX `Mustang`.
        Mustang => "mustang",
        /// PANTONE 18-1540 TCX `Cinnabar`.
        Cinnabar => "cinnabar",
        /// PANTONE 16-1332 TCX `Pheasant`.
        Pheasant => "pheasant"
    }
}

/// The sRGB values of the [`FeathersAndLeathersColor`] slots.
pub static FEATHERS_AND_LEATHERS: StaticPalette<FeathersAndLeathersColor, u8> = static_palette! {
    FeathersAndLeathersColor::Corsair => PantoneFhiNamedColor::Corsair.to_color(),
    FeathersAndLeathersColor::GrayGreen => PantoneFhiNamedColor::GrayGreen.to_color(),
    FeathersAndLeathersColor::Tan => PantoneFhiNamedColor::Tan.to_color(),
    FeathersAndLeathersColor::Mustang => PantoneFhiNamedColor::Mustang.to_color(),
    FeathersAndLeathersColor::Cinnabar => PantoneFhiNamedColor::Cinnabar.to_color(),
    FeathersAndLeathersColor::Pheasant => PantoneFhiNamedColor::Pheasant.to_color(),
};
