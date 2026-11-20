use crate::PantoneFhiNamedColor;
use crate::PantoneNamedColor;
use crate::StaticPalette;
use crate::palette_definition;
use crate::static_palette;

palette_definition! {
    /// The signature hues of the 1980s - the lead swatch of each
    /// chapter palette of "Pantone: The Twentieth Century in Color"
    /// (Eiseman & Recker, 2011), in chapter order.
    pub enum SignatureHues1980Color {
        /// The lead swatch of the "Memphis, Michael, and Philippe" palette - PANTONE 19-3847 TCX `Deep Blue`.
        MemphisMichaelAndPhilippe => "memphis-michael-and-philippe",
        /// The lead swatch of the "To the Manor Born" palette - PANTONE 19-6110 TCX `Deep Forest`. Caption spells `Deep Forrest`.
        ToTheManorBorn => "to-the-manor-born",
        /// The lead swatch of the "Urban Cowboys" palette - PANTONE 19-4118 TCX `Dark Denim`.
        UrbanCowboys => "urban-cowboys",
        /// The lead swatch of the "Signs and Symbols" palette - Pantone 583 C.
        SignsAndSymbols => "signs-and-symbols",
        /// The lead swatch of the "Miami Vice" palette - PANTONE 13-2805 TCX `Pink Mist`.
        MiamiVice => "miami-vice",
        /// The lead swatch of the "Majorelle and Morocco" palette - PANTONE 18-4043 TCX `Palace Blue`.
        MajorelleAndMorocco => "majorelle-and-morocco",
        /// The lead swatch of the "Santa Fe" palette - Pantone 4735 C.
        SantaFe => "santa-fe",
        /// The lead swatch of the "Personal Colors" palette - PANTONE 19-1220 TCX `Cappuccino`.
        PersonalColors => "personal-colors",
        /// The lead swatch of the "Japonais" palette - PANTONE 19-4005 TCX `Stretch Limo`.
        Japonais => "japonais"
    }
}

/// The sRGB values of the [`SignatureHues1980Color`] slots.
pub static SIGNATURE_HUES_1980: StaticPalette<SignatureHues1980Color, u8> = static_palette! {
    SignatureHues1980Color::MemphisMichaelAndPhilippe => PantoneFhiNamedColor::DeepBlue.to_color(),
    SignatureHues1980Color::ToTheManorBorn => PantoneFhiNamedColor::DeepForest.to_color(),
    SignatureHues1980Color::UrbanCowboys => PantoneFhiNamedColor::DarkDenim.to_color(),
    SignatureHues1980Color::SignsAndSymbols => PantoneNamedColor::Pms583C.to_color(),
    SignatureHues1980Color::MiamiVice => PantoneFhiNamedColor::PinkMist.to_color(),
    SignatureHues1980Color::MajorelleAndMorocco => PantoneFhiNamedColor::PalaceBlue.to_color(),
    SignatureHues1980Color::SantaFe => PantoneNamedColor::Pms4735C.to_color(),
    SignatureHues1980Color::PersonalColors => PantoneFhiNamedColor::Cappuccino.to_color(),
    SignatureHues1980Color::Japonais => PantoneFhiNamedColor::StretchLimo.to_color(),
};
