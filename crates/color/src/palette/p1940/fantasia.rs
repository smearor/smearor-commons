use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Fantasia" palette - 1940s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum FantasiaColor {
        /// PANTONE 17-1663 TCX `Bittersweet`.
        Bittersweet => "bittersweet",
        /// PANTONE 13-0939 TCX `Golden Cream`.
        GoldenCream => "golden-cream",
        /// PANTONE 18-0312 TCX `Deep Lichen Green`.
        DeepLichenGreen => "deep-lichen-green",
        /// PANTONE 17-5111 TCX `Oil Blue`.
        OilBlue => "oil-blue",
        /// PANTONE 15-4225 TCX `Alaskan Blue`.
        AlaskanBlue => "alaskan-blue",
        /// PANTONE 19-4053 TCX `Turkish Sea`.
        TurkishSea => "turkish-sea",
        /// PANTONE 17-1929 TCX `Rapture Rose`.
        RaptureRose => "rapture-rose",
        /// PANTONE 19-2432 TCX `Raspberry Radiance`.
        RaspberryRadiance => "raspberry-radiance",
        /// PANTONE 19-4010 TCX `Total Eclipse`. Caption spells `Total Elclipse`.
        TotalEclipse => "total-eclipse"
    }
}

/// The sRGB values of the [`FantasiaColor`] slots.
pub static FANTASIA: StaticPalette<FantasiaColor, u8> = static_palette! {
    FantasiaColor::Bittersweet => PantoneFhiNamedColor::Bittersweet.to_color(),
    FantasiaColor::GoldenCream => PantoneFhiNamedColor::GoldenCream.to_color(),
    FantasiaColor::DeepLichenGreen => PantoneFhiNamedColor::DeepLichenGreen.to_color(),
    FantasiaColor::OilBlue => PantoneFhiNamedColor::OilBlue.to_color(),
    FantasiaColor::AlaskanBlue => PantoneFhiNamedColor::AlaskanBlue.to_color(),
    FantasiaColor::TurkishSea => PantoneFhiNamedColor::TurkishSea.to_color(),
    FantasiaColor::RaptureRose => PantoneFhiNamedColor::RaptureRose.to_color(),
    FantasiaColor::RaspberryRadiance => PantoneFhiNamedColor::RaspberryRadiance.to_color(),
    FantasiaColor::TotalEclipse => PantoneFhiNamedColor::TotalEclipse.to_color(),
};
