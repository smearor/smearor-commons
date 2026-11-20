use crate::PantoneFhiNamedColor;
use crate::PantoneNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The signature hues of the 1960s - the lead swatch of each
    /// chapter palette of "Pantone: The Twentieth Century in Color"
    /// (Eiseman & Recker, 2011), in chapter order.
    pub enum SignatureHues1960Color {
        /// The lead swatch of the "Passage to India" palette - PANTONE 15-2216 TCX `Sachet Pink`.
        PassageToIndia => "passage-to-india",
        /// The lead swatch of the "A Different Space" palette - Pantone Black C.
        DifferentSpace => "different-space",
        /// The lead swatch of the "Kensington and Carnaby" palette - PANTONE 18-3937 TCX `Blue Yonder`.
        KensingtonAndCarnaby => "kensington-and-carnaby",
        /// The lead swatch of the "Black Is Beautiful" palette - PANTONE 19-1118 TCX `Chestnut`.
        BlackIsBeautiful => "black-is-beautiful",
        /// The lead swatch of the "Psychedelia" palette - Pantone 7649 C.
        Psychedelia => "psychedelia",
        /// The lead swatch of the "Sesame Street" palette - PANTONE 16-0439 TCX `Spinach Green`.
        SesameStreet => "sesame-street",
        /// The lead swatch of the "Warhol" palette - Pantone 7621 C.
        Warhol => "warhol",
        /// The lead swatch of the "PANTONE" palette - Pantone Rubine Red C.
        Pantone => "pantone"
    }
}

/// The sRGB values of the [`SignatureHues1960Color`] slots.
pub static SIGNATURE_HUES_1960: StaticPalette<SignatureHues1960Color, u8> = static_palette! {
    SignatureHues1960Color::PassageToIndia => PantoneFhiNamedColor::SachetPink.to_color(),
    SignatureHues1960Color::DifferentSpace => PantoneNamedColor::BlackC.to_color(),
    SignatureHues1960Color::KensingtonAndCarnaby => PantoneFhiNamedColor::BlueYonder.to_color(),
    SignatureHues1960Color::BlackIsBeautiful => PantoneFhiNamedColor::Chestnut.to_color(),
    SignatureHues1960Color::Psychedelia => PantoneNamedColor::Pms7649C.to_color(),
    SignatureHues1960Color::SesameStreet => PantoneFhiNamedColor::SpinachGreen.to_color(),
    SignatureHues1960Color::Warhol => PantoneNamedColor::Pms7621C.to_color(),
    SignatureHues1960Color::Pantone => PantoneNamedColor::RubineRedC.to_color(),
};
