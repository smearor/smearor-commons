use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Sesame Street" palette - 1960s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum SesameStreetColor {
        /// PANTONE 16-0439 TCX `Spinach Green`.
        SpinachGreen => "spinach-green",
        /// PANTONE 13-0858 TCX `Vibrant Yellow`.
        VibrantYellow => "vibrant-yellow",
        /// PANTONE 17-1563 TCX `Cherry Tomato`.
        CherryTomato => "cherry-tomato",
        /// PANTONE 16-1257 TCX `Sun Orange`.
        SunOrange => "sun-orange",
        /// PANTONE 18-3949 TCX `Dazzling Blue`.
        DazzlingBlue => "dazzling-blue",
        /// PANTONE 17-4336 TCX `Blithe`.
        Blithe => "blithe",
        /// PANTONE 17-2127 TCX `Shocking Pink`.
        ShockingPink => "shocking-pink"
    }
}

/// The sRGB values of the [`SesameStreetColor`] slots.
pub static SESAME_STREET: StaticPalette<SesameStreetColor, u8> = static_palette! {
    SesameStreetColor::SpinachGreen => PantoneFhiNamedColor::SpinachGreen.to_color(),
    SesameStreetColor::VibrantYellow => PantoneFhiNamedColor::VibrantYellow.to_color(),
    SesameStreetColor::CherryTomato => PantoneFhiNamedColor::CherryTomato.to_color(),
    SesameStreetColor::SunOrange => PantoneFhiNamedColor::SunOrange.to_color(),
    SesameStreetColor::DazzlingBlue => PantoneFhiNamedColor::DazzlingBlue.to_color(),
    SesameStreetColor::Blithe => PantoneFhiNamedColor::Blithe.to_color(),
    SesameStreetColor::ShockingPink => PantoneFhiNamedColor::ShockingPink.to_color(),
};
