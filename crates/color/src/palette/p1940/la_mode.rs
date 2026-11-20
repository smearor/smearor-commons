use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "La Mode" palette - 1940s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum LaModeColor {
        /// PANTONE 19-4006 TCX `Caviar`.
        Caviar => "caviar",
        /// PANTONE 18-3921 TCX `Bijou Blue`.
        BijouBlue => "bijou-blue",
        /// PANTONE 17-3910 TCX `Lavender Gray`.
        LavenderGray => "lavender-gray",
        /// PANTONE 13-2804 TCX `Parfait Pink`.
        ParfaitPink => "parfait-pink",
        /// PANTONE 15-1309 TCX `Moonlight`.
        Moonlight => "moonlight",
        /// PANTONE 11-4201 TCX `Cloud Dancer`.
        CloudDancer => "cloud-dancer",
        /// PANTONE 12-0812 TCX `Alabaster Gleam`.
        AlabasterGleam => "alabaster-gleam"
    }
}

/// The sRGB values of the [`LaModeColor`] slots.
pub static LA_MODE: StaticPalette<LaModeColor, u8> = static_palette! {
    LaModeColor::Caviar => PantoneFhiNamedColor::Caviar.to_color(),
    LaModeColor::BijouBlue => PantoneFhiNamedColor::BijouBlue.to_color(),
    LaModeColor::LavenderGray => PantoneFhiNamedColor::LavenderGray.to_color(),
    LaModeColor::ParfaitPink => PantoneFhiNamedColor::ParfaitPink.to_color(),
    LaModeColor::Moonlight => PantoneFhiNamedColor::Moonlight.to_color(),
    LaModeColor::CloudDancer => PantoneFhiNamedColor::CloudDancer.to_color(),
    LaModeColor::AlabasterGleam => PantoneFhiNamedColor::AlabasterGleam.to_color(),
};
