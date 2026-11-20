use crate::PantoneFhiNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The signature hues of the 1900s - the lead swatch of each
    /// chapter palette of "Pantone: The Twentieth Century in Color"
    /// (Eiseman & Recker, 2011), in chapter order.
    pub enum SignatureHues1900Color {
        /// The lead swatch of the "Edwardian Affairs" palette - PANTONE 12-0000 TCX `White Swan`.
        EdwardianAffairs => "edwardian-affairs",
        /// The lead swatch of the "Arts and Crafts" palette - PANTONE 17-1410 TCX `Pine Bark`.
        ArtsAndCrafts => "arts-and-crafts",
        /// The lead swatch of the "Jewel Tones" palette - PANTONE 16-0836 TCX `Rich Gold`.
        JewelTones => "jewel-tones",
        /// The lead swatch of the "The Charm of Iridescence" palette - PANTONE 15-1927 TCX `Pale Gold`. The caption reads 15-1927; the official code of `Pale Gold` is 15-0927.
        CharmOfIridescence => "charm-of-iridescence",
        /// The lead swatch of the "Firsts for Women" palette - PANTONE 19-4007 TCX `Anthracite`.
        FirstsForWomen => "firsts-for-women",
        /// The lead swatch of the "Poiret Revolution" palette - PANTONE 16-1454 TCX `Jaffa Orange`.
        PoiretRevolution => "poiret-revolution",
        /// The lead swatch of the "The Fauves" palette - PANTONE 16-1454 TCX `Jaffa Orange`.
        Fauves => "fauves"
    }
}

/// The sRGB values of the [`SignatureHues1900Color`] slots.
pub static SIGNATURE_HUES_1900: StaticPalette<SignatureHues1900Color, u8> = static_palette! {
    SignatureHues1900Color::EdwardianAffairs => PantoneFhiNamedColor::WhiteSwan.to_color(),
    SignatureHues1900Color::ArtsAndCrafts => PantoneFhiNamedColor::PineBark.to_color(),
    SignatureHues1900Color::JewelTones => PantoneFhiNamedColor::RichGold.to_color(),
    SignatureHues1900Color::CharmOfIridescence => PantoneFhiNamedColor::PaleGold.to_color(),
    SignatureHues1900Color::FirstsForWomen => PantoneFhiNamedColor::Anthracite.to_color(),
    SignatureHues1900Color::PoiretRevolution => PantoneFhiNamedColor::JaffaOrange.to_color(),
    SignatureHues1900Color::Fauves => PantoneFhiNamedColor::JaffaOrange.to_color(),
};
