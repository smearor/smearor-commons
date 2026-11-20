use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Illusions" palette - 1930s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum IllusionsColor {
        /// PANTONE 18-0000 TCX `Smoked Pearl`.
        SmokedPearl => "smoked-pearl",
        /// PANTONE 11-0907 TCX `Pearled Ivory`.
        PearledIvory => "pearled-ivory",
        /// PANTONE 18-1015 TCX `Shitake`.
        Shitake => "shitake",
        /// PANTONE 17-0929 TCX `Fennel Seed`.
        FennelSeed => "fennel-seed",
        /// PANTONE 14-0935 TCX `Jojoba`.
        Jojoba => "jojoba",
        /// PANTONE 16-5806 TCX `Green Milieu`.
        GreenMilieu => "green-milieu"
    }
}

/// The sRGB values of the [`IllusionsColor`] slots.
pub static ILLUSIONS: StaticPalette<IllusionsColor, u8> = static_palette! {
    IllusionsColor::SmokedPearl => PantoneFhiNamedColor::SmokedPearl.to_color(),
    IllusionsColor::PearledIvory => PantoneFhiNamedColor::PearledIvory.to_color(),
    IllusionsColor::Shitake => PantoneFhiNamedColor::Shitake.to_color(),
    IllusionsColor::FennelSeed => PantoneFhiNamedColor::FennelSeed.to_color(),
    IllusionsColor::Jojoba => PantoneFhiNamedColor::Jojoba.to_color(),
    IllusionsColor::GreenMilieu => PantoneFhiNamedColor::GreenMilieu.to_color(),
};
