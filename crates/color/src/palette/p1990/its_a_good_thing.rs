use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "It's a Good Thing" palette - 1990s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum ItsAGoodThingColor {
        /// PANTONE 19-2924 TCX `Hollyhock`. Caption spells `Holyhock`.
        Hollyhock => "hollyhock",
        /// PANTONE 17-3020 TCX `Spring Crocus`.
        SpringCrocus => "spring-crocus",
        /// PANTONE 16-2124 TCX `Pink Carnation`.
        PinkCarnation => "pink-carnation",
        /// PANTONE 18-2333 TCX `Raspberry Rose`.
        RaspberryRose => "raspberry-rose",
        /// PANTONE 17-3930 TCX `Jacaranda`.
        Jacaranda => "jacaranda",
        /// PANTONE 18-6018 TCX `Foliage Green`.
        FoliageGreen => "foliage-green",
        /// PANTONE 15-0533 TCX `Linden Green`.
        LindenGreen => "linden-green",
        /// PANTONE 11-0602 TCX `Snow White`.
        SnowWhite => "snow-white"
    }
}

/// The sRGB values of the [`ItsAGoodThingColor`] slots.
pub static ITS_A_GOOD_THING: StaticPalette<ItsAGoodThingColor, u8> = static_palette! {
    ItsAGoodThingColor::Hollyhock => PantoneFhiNamedColor::Hollyhock.to_color(),
    ItsAGoodThingColor::SpringCrocus => PantoneFhiNamedColor::SpringCrocus.to_color(),
    ItsAGoodThingColor::PinkCarnation => PantoneFhiNamedColor::PinkCarnation.to_color(),
    ItsAGoodThingColor::RaspberryRose => PantoneFhiNamedColor::RaspberryRose.to_color(),
    ItsAGoodThingColor::Jacaranda => PantoneFhiNamedColor::Jacaranda.to_color(),
    ItsAGoodThingColor::FoliageGreen => PantoneFhiNamedColor::FoliageGreen.to_color(),
    ItsAGoodThingColor::LindenGreen => PantoneFhiNamedColor::LindenGreen.to_color(),
    ItsAGoodThingColor::SnowWhite => PantoneFhiNamedColor::SnowWhite.to_color(),
};
