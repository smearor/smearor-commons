use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Firsts for Women" palette - 1900s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum FirstsForWomenColor {
        /// PANTONE 19-4007 TCX `Anthracite`.
        Anthracite => "anthracite",
        /// PANTONE 19-1235 TCX `Brunette`.
        Brunette => "brunette",
        /// PANTONE 15-1315 TCX `Rugby Tan`.
        RugbyTan => "rugby-tan",
        /// PANTONE 15-1214 TCX `Warm Sand`.
        WarmSand => "warm-sand",
        /// PANTONE 11-0606 TCX `Pristine`.
        Pristine => "pristine",
        /// PANTONE 15-4703 TCX `Mirage Gray`.
        MirageGray => "mirage-gray",
        /// PANTONE 14-1511 TCX `Powder Pink`.
        PowderPink => "powder-pink",
        /// PANTONE 19-1759 TCX `American Beauty`.
        AmericanBeauty => "american-beauty"
    }
}

/// The sRGB values of the [`FirstsForWomenColor`] slots.
pub static FIRSTS_FOR_WOMEN: StaticPalette<FirstsForWomenColor, u8> = static_palette! {
    FirstsForWomenColor::Anthracite => PantoneFhiNamedColor::Anthracite.to_color(),
    FirstsForWomenColor::Brunette => PantoneFhiNamedColor::Brunette.to_color(),
    FirstsForWomenColor::RugbyTan => PantoneFhiNamedColor::RugbyTan.to_color(),
    FirstsForWomenColor::WarmSand => PantoneFhiNamedColor::WarmSand.to_color(),
    FirstsForWomenColor::Pristine => PantoneFhiNamedColor::Pristine.to_color(),
    FirstsForWomenColor::MirageGray => PantoneFhiNamedColor::MirageGray.to_color(),
    FirstsForWomenColor::PowderPink => PantoneFhiNamedColor::PowderPink.to_color(),
    FirstsForWomenColor::AmericanBeauty => PantoneFhiNamedColor::AmericanBeauty.to_color(),
};
