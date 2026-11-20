use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Black Is Beautiful" palette - 1960s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum BlackIsBeautifulColor {
        /// PANTONE 19-1118 TCX `Chestnut`.
        Chestnut => "chestnut",
        /// PANTONE 19-5708 TCX `Jet Set`.
        JetSet => "jet-set",
        /// PANTONE 15-4502 TCX `Silver Cloud`.
        SilverCloud => "silver-cloud",
        /// PANTONE 11-4201 TCX `Cloud Dancer`.
        CloudDancer => "cloud-dancer",
        /// PANTONE 17-1564 TCX `Fiesta`.
        Fiesta => "fiesta",
        /// PANTONE 17-0836 TCX `Ecru Olive`.
        EcruOlive => "ecru-olive"
    }
}

/// The sRGB values of the [`BlackIsBeautifulColor`] slots.
pub static BLACK_IS_BEAUTIFUL: StaticPalette<BlackIsBeautifulColor, u8> = static_palette! {
    BlackIsBeautifulColor::Chestnut => PantoneFhiNamedColor::Chestnut.to_color(),
    BlackIsBeautifulColor::JetSet => PantoneFhiNamedColor::JetSet.to_color(),
    BlackIsBeautifulColor::SilverCloud => PantoneFhiNamedColor::SilverCloud.to_color(),
    BlackIsBeautifulColor::CloudDancer => PantoneFhiNamedColor::CloudDancer.to_color(),
    BlackIsBeautifulColor::Fiesta => PantoneFhiNamedColor::Fiesta.to_color(),
    BlackIsBeautifulColor::EcruOlive => PantoneFhiNamedColor::EcruOlive.to_color(),
};
