use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Coast-to-Coast Woolens" palette - 1950s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum CoastToCoastWoolensColor {
        /// PANTONE 19-1759 TCX `American Beauty`.
        AmericanBeauty => "american-beauty",
        /// PANTONE 17-1444 TCX `Ginger`.
        Ginger => "ginger",
        /// PANTONE 18-3324 TCX `Dahlia`.
        Dahlia => "dahlia",
        /// PANTONE 18-3840 TCX `Purple Opulence`.
        PurpleOpulence => "purple-opulence",
        /// PANTONE 17-0324 TCX `Epsom`. Caption spells `Epsome`.
        Epsom => "epsom",
        /// PANTONE 18-0601 TCX `Charcoal Gray`.
        CharcoalGray => "charcoal-gray",
        /// PANTONE 17-4023 TCX `Blue Heaven`.
        BlueHeaven => "blue-heaven",
        /// PANTONE 14-1118 TCX `Beige`.
        Beige => "beige"
    }
}

/// The sRGB values of the [`CoastToCoastWoolensColor`] slots.
pub static COAST_TO_COAST_WOOLENS: StaticPalette<CoastToCoastWoolensColor, u8> = static_palette! {
    CoastToCoastWoolensColor::AmericanBeauty => PantoneFhiNamedColor::AmericanBeauty.to_color(),
    CoastToCoastWoolensColor::Ginger => PantoneFhiNamedColor::Ginger.to_color(),
    CoastToCoastWoolensColor::Dahlia => PantoneFhiNamedColor::Dahlia.to_color(),
    CoastToCoastWoolensColor::PurpleOpulence => PantoneFhiNamedColor::PurpleOpulence.to_color(),
    CoastToCoastWoolensColor::Epsom => PantoneFhiNamedColor::Epsom.to_color(),
    CoastToCoastWoolensColor::CharcoalGray => PantoneFhiNamedColor::CharcoalGray.to_color(),
    CoastToCoastWoolensColor::BlueHeaven => PantoneFhiNamedColor::BlueHeaven.to_color(),
    CoastToCoastWoolensColor::Beige => PantoneFhiNamedColor::Beige.to_color(),
};
