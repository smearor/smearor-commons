//! The 1980s chapter palettes of "Pantone: The Twentieth
//! Century in Color" (Eiseman & Recker, 2011).
//!
//! The slot names and order match the book's swatch rows. TCX
//! swatches resolve to the official PANTONE Fashion, Home +
//! Interiors (FHI) color via [`crate::PantoneFhiNamedColor`];
//! swatches numbered in the PANTONE solid-coated system resolve to
//! [`crate::PantoneNamedColor`]. Premium-metallic and pastel chips
//! outside those catalogs carry the sRGB value sampled from the
//! book page and say so in their doc comments.

mod japonais;
mod majorelle_and_morocco;
mod memphis_michael_and_philippe;
mod miami_vice;
mod personal_colors;
mod santa_fe;
mod signature_hues;
mod signs_and_symbols;
mod to_the_manor_born;
mod urban_cowboys;

pub use japonais::JAPONAIS;
pub use japonais::JaponaisColor;
pub use majorelle_and_morocco::MAJORELLE_AND_MOROCCO;
pub use majorelle_and_morocco::MajorelleAndMoroccoColor;
pub use memphis_michael_and_philippe::MEMPHIS_MICHAEL_AND_PHILIPPE;
pub use memphis_michael_and_philippe::MemphisMichaelAndPhilippeColor;
pub use miami_vice::MIAMI_VICE;
pub use miami_vice::MiamiViceColor;
pub use personal_colors::PERSONAL_COLORS;
pub use personal_colors::PersonalColorsColor;
pub use santa_fe::SANTA_FE;
pub use santa_fe::SantaFeColor;
pub use signature_hues::SIGNATURE_HUES_1980;
pub use signature_hues::SignatureHues1980Color;
pub use signs_and_symbols::SIGNS_AND_SYMBOLS;
pub use signs_and_symbols::SignsAndSymbolsColor;
pub use to_the_manor_born::TO_THE_MANOR_BORN;
pub use to_the_manor_born::ToTheManorBornColor;
pub use urban_cowboys::URBAN_COWBOYS;
pub use urban_cowboys::UrbanCowboysColor;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Color;
    use crate::PaletteDefinition;

    #[test]
    fn palettes_are_complete() {
        assert!(MEMPHIS_MICHAEL_AND_PHILIPPE.is_complete());
        assert!(TO_THE_MANOR_BORN.is_complete());
        assert!(URBAN_COWBOYS.is_complete());
        assert!(SIGNS_AND_SYMBOLS.is_complete());
        assert!(MIAMI_VICE.is_complete());
        assert!(MAJORELLE_AND_MOROCCO.is_complete());
        assert!(SANTA_FE.is_complete());
        assert!(PERSONAL_COLORS.is_complete());
        assert!(JAPONAIS.is_complete());
    }

    #[test]
    fn palettes_hold_the_book_chip_counts() {
        assert_eq!(MEMPHIS_MICHAEL_AND_PHILIPPE.len(), 7);
        assert_eq!(TO_THE_MANOR_BORN.len(), 5);
        assert_eq!(URBAN_COWBOYS.len(), 6);
        assert_eq!(SIGNS_AND_SYMBOLS.len(), 7);
        assert_eq!(MIAMI_VICE.len(), 7);
        assert_eq!(MAJORELLE_AND_MOROCCO.len(), 7);
        assert_eq!(SANTA_FE.len(), 7);
        assert_eq!(PERSONAL_COLORS.len(), 8);
        assert_eq!(JAPONAIS.len(), 6);
    }

    #[test]
    fn signature_hues_are_complete() {
        assert!(SIGNATURE_HUES_1980.is_complete());
    }

    #[test]
    fn signature_hues_hold_chapter_leads_in_order() {
        assert_eq!(SIGNATURE_HUES_1980.len(), 9);
        assert_eq!(SignatureHues1980Color::slots()[0], SignatureHues1980Color::MemphisMichaelAndPhilippe);
    }

    #[test]
    fn first_signature_hue_matches_the_book() {
        assert_eq!(
            SIGNATURE_HUES_1980.get(&SignatureHues1980Color::MemphisMichaelAndPhilippe),
            Some(&Color::<u8>::from_rgb_u32(0x44377D))
        );
    }
}
