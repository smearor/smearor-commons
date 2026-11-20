use crate::PantoneFhiNamedColor;
use crate::PantoneNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The signature hues of the 1920s - the lead swatch of each
    /// chapter palette of "Pantone: The Twentieth Century in Color"
    /// (Eiseman & Recker, 2011), in chapter order.
    pub enum SignatureHues1920Color {
        /// The lead swatch of the "Art Deco" palette - PANTONE 14-5002 TCX `Silver`.
        ArtDeco => "art-deco",
        /// The lead swatch of the "Tutmania" palette - PANTONE 16-1836 TCX `Rich Gold`. The caption reads 16-1836; the official code of `Rich Gold` is 16-0836.
        Tutmania => "tutmania",
        /// The lead swatch of the "Cocktails and Laughter" palette - PANTONE 19-2118 TCX `Winetasting`.
        CocktailsAndLaughter => "cocktails-and-laughter",
        /// The lead swatch of the "Destinations" palette - Pantone 7410 C.
        Destinations => "destinations",
        /// The lead swatch of the "The Leyendecker Man" palette - PANTONE 16-1331 TCX `Toast`.
        LeyendeckerMan => "leyendecker-man",
        /// The lead swatch of the "Bauhaus" palette - PANTONE 19-4203 TCX `Moonless Night`.
        Bauhaus => "bauhaus",
        /// The lead swatch of the "Modern Pleasures" palette - PANTONE 14-0244 TCX `Bright Lime Green`.
        ModernPleasures => "modern-pleasures",
        /// The lead swatch of the "A Rose Is a Rose" palette - PANTONE 18-1629 TCX `Faded Rose`.
        RoseIsARose => "rose-is-a-rose"
    }
}

/// The sRGB values of the [`SignatureHues1920Color`] slots.
pub static SIGNATURE_HUES_1920: StaticPalette<SignatureHues1920Color, u8> = static_palette! {
    SignatureHues1920Color::ArtDeco => PantoneFhiNamedColor::Silver.to_color(),
    SignatureHues1920Color::Tutmania => PantoneFhiNamedColor::RichGold.to_color(),
    SignatureHues1920Color::CocktailsAndLaughter => PantoneFhiNamedColor::Winetasting.to_color(),
    SignatureHues1920Color::Destinations => PantoneNamedColor::Pms7410C.to_color(),
    SignatureHues1920Color::LeyendeckerMan => PantoneFhiNamedColor::Toast.to_color(),
    SignatureHues1920Color::Bauhaus => PantoneFhiNamedColor::MoonlessNight.to_color(),
    SignatureHues1920Color::ModernPleasures => PantoneFhiNamedColor::BrightLimeGreen.to_color(),
    SignatureHues1920Color::RoseIsARose => PantoneFhiNamedColor::FadedRose.to_color(),
};
