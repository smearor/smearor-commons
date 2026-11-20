//! The 1900s chapter palettes of "Pantone: The Twentieth
//! Century in Color" (Eiseman & Recker, 2011).
//!
//! The slot names and order match the book's swatch rows. TCX
//! swatches resolve to the official PANTONE Fashion, Home +
//! Interiors (FHI) color via [`crate::PantoneFhiNamedColor`];
//! swatches numbered in the PANTONE solid-coated system resolve to
//! [`crate::PantoneNamedColor`]. Premium-metallic and pastel chips
//! outside those catalogs carry the sRGB value sampled from the
//! book page and say so in their doc comments.

mod arts_and_crafts;
mod charm_of_iridescence;
mod edwardian_affairs;
mod fauves;
mod firsts_for_women;
mod jewel_tones;
mod poiret_revolution;
mod signature_hues;

pub use arts_and_crafts::ARTS_AND_CRAFTS;
pub use arts_and_crafts::ArtsAndCraftsColor;
pub use charm_of_iridescence::CHARM_OF_IRIDESCENCE;
pub use charm_of_iridescence::CharmOfIridescenceColor;
pub use edwardian_affairs::EDWARDIAN_AFFAIRS;
pub use edwardian_affairs::EdwardianAffairsColor;
pub use fauves::FAUVES;
pub use fauves::FauvesColor;
pub use firsts_for_women::FIRSTS_FOR_WOMEN;
pub use firsts_for_women::FirstsForWomenColor;
pub use jewel_tones::JEWEL_TONES;
pub use jewel_tones::JewelTonesColor;
pub use poiret_revolution::POIRET_REVOLUTION;
pub use poiret_revolution::PoiretRevolutionColor;
pub use signature_hues::SIGNATURE_HUES_1900;
pub use signature_hues::SignatureHues1900Color;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Color;
    use crate::PaletteDefinition;

    #[test]
    fn palettes_are_complete() {
        assert!(EDWARDIAN_AFFAIRS.is_complete());
        assert!(ARTS_AND_CRAFTS.is_complete());
        assert!(JEWEL_TONES.is_complete());
        assert!(CHARM_OF_IRIDESCENCE.is_complete());
        assert!(FIRSTS_FOR_WOMEN.is_complete());
        assert!(POIRET_REVOLUTION.is_complete());
        assert!(FAUVES.is_complete());
    }

    #[test]
    fn palettes_hold_the_book_chip_counts() {
        assert_eq!(EDWARDIAN_AFFAIRS.len(), 8);
        assert_eq!(ARTS_AND_CRAFTS.len(), 8);
        assert_eq!(JEWEL_TONES.len(), 6);
        assert_eq!(CHARM_OF_IRIDESCENCE.len(), 8);
        assert_eq!(FIRSTS_FOR_WOMEN.len(), 8);
        assert_eq!(POIRET_REVOLUTION.len(), 7);
        assert_eq!(FAUVES.len(), 8);
    }

    #[test]
    fn signature_hues_are_complete() {
        assert!(SIGNATURE_HUES_1900.is_complete());
    }

    #[test]
    fn signature_hues_hold_chapter_leads_in_order() {
        assert_eq!(SIGNATURE_HUES_1900.len(), 7);
        assert_eq!(SignatureHues1900Color::slots()[0], SignatureHues1900Color::EdwardianAffairs);
    }

    #[test]
    fn first_signature_hue_matches_the_book() {
        assert_eq!(SIGNATURE_HUES_1900.get(&SignatureHues1900Color::EdwardianAffairs), Some(&Color::<u8>::from_rgb_u32(0xE4D7C5)));
    }
}
