//! The 1960s chapter palettes of "Pantone: The Twentieth
//! Century in Color" (Eiseman & Recker, 2011).
//!
//! The slot names and order match the book's swatch rows. TCX
//! swatches resolve to the official PANTONE Fashion, Home +
//! Interiors (FHI) color via [`crate::PantoneFhiNamedColor`];
//! swatches numbered in the PANTONE solid-coated system resolve to
//! [`crate::PantoneNamedColor`]. Premium-metallic and pastel chips
//! outside those catalogs carry the sRGB value sampled from the
//! book page and say so in their doc comments.

mod black_is_beautiful;
mod different_space;
mod kensington_and_carnaby;
mod pantone;
mod passage_to_india;
mod psychedelia;
mod sesame_street;
mod signature_hues;
mod warhol;

pub use black_is_beautiful::BLACK_IS_BEAUTIFUL;
pub use black_is_beautiful::BlackIsBeautifulColor;
pub use different_space::DIFFERENT_SPACE;
pub use different_space::DifferentSpaceColor;
pub use kensington_and_carnaby::KENSINGTON_AND_CARNABY;
pub use kensington_and_carnaby::KensingtonAndCarnabyColor;
pub use pantone::PANTONE;
pub use pantone::PantoneColor;
pub use passage_to_india::PASSAGE_TO_INDIA;
pub use passage_to_india::PassageToIndiaColor;
pub use psychedelia::PSYCHEDELIA;
pub use psychedelia::PsychedeliaColor;
pub use sesame_street::SESAME_STREET;
pub use sesame_street::SesameStreetColor;
pub use signature_hues::SIGNATURE_HUES_1960;
pub use signature_hues::SignatureHues1960Color;
pub use warhol::WARHOL;
pub use warhol::WarholColor;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Color;
    use crate::PaletteDefinition;

    #[test]
    fn palettes_are_complete() {
        assert!(PASSAGE_TO_INDIA.is_complete());
        assert!(DIFFERENT_SPACE.is_complete());
        assert!(KENSINGTON_AND_CARNABY.is_complete());
        assert!(BLACK_IS_BEAUTIFUL.is_complete());
        assert!(PSYCHEDELIA.is_complete());
        assert!(SESAME_STREET.is_complete());
        assert!(WARHOL.is_complete());
        assert!(PANTONE.is_complete());
    }

    #[test]
    fn palettes_hold_the_book_chip_counts() {
        assert_eq!(PASSAGE_TO_INDIA.len(), 6);
        assert_eq!(DIFFERENT_SPACE.len(), 7);
        assert_eq!(KENSINGTON_AND_CARNABY.len(), 8);
        assert_eq!(BLACK_IS_BEAUTIFUL.len(), 6);
        assert_eq!(PSYCHEDELIA.len(), 8);
        assert_eq!(SESAME_STREET.len(), 7);
        assert_eq!(WARHOL.len(), 9);
        assert_eq!(PANTONE.len(), 8);
    }

    #[test]
    fn signature_hues_are_complete() {
        assert!(SIGNATURE_HUES_1960.is_complete());
    }

    #[test]
    fn signature_hues_hold_chapter_leads_in_order() {
        assert_eq!(SIGNATURE_HUES_1960.len(), 8);
        assert_eq!(SignatureHues1960Color::slots()[0], SignatureHues1960Color::PassageToIndia);
    }

    #[test]
    fn first_signature_hue_matches_the_book() {
        assert_eq!(SIGNATURE_HUES_1960.get(&SignatureHues1960Color::PassageToIndia), Some(&Color::<u8>::from_rgb_u32(0xF18AAD)));
    }
}
