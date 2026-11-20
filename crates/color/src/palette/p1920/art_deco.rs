use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The "Art Deco" palette - 1920s chapter of "Pantone: The
    /// Twentieth Century in Color" (Eiseman & Recker, 2011).
    pub enum ArtDecoColor {
        /// PANTONE 14-5002 TCX `Silver`.
        Silver => "silver",
        /// PANTONE 19-0303 TCX `Jet Black`.
        JetBlack => "jet-black",
        /// PANTONE 16-1435 TCX `Carnelian`.
        Carnelian => "carnelian",
        /// PANTONE 14-1012 TCX `Gilded Beige`.
        GildedBeige => "gilded-beige",
        /// PANTONE 12-5202 TCX `Turtledove`.
        Turtledove => "turtledove",
        /// PANTONE 17-3924 TCX `Lavender Violet`.
        LavenderViolet => "lavender-violet"
    }
}

/// The sRGB values of the [`ArtDecoColor`] slots.
pub static ART_DECO: StaticPalette<ArtDecoColor, u8> = static_palette! {
    ArtDecoColor::Silver => PantoneFhiNamedColor::Silver.to_color(),
    ArtDecoColor::JetBlack => PantoneFhiNamedColor::JetBlack.to_color(),
    ArtDecoColor::Carnelian => PantoneFhiNamedColor::Carnelian.to_color(),
    ArtDecoColor::GildedBeige => PantoneFhiNamedColor::GildedBeige.to_color(),
    ArtDecoColor::Turtledove => PantoneFhiNamedColor::Turtledove.to_color(),
    ArtDecoColor::LavenderViolet => PantoneFhiNamedColor::LavenderViolet.to_color(),
};
