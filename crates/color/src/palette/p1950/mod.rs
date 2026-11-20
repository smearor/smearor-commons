//! The 1950s chapter palettes of "Pantone: The Twentieth
//! Century in Color" (Eiseman & Recker, 2011).
//!
//! The slot names and order match the book's swatch rows. TCX
//! swatches resolve to the official PANTONE Fashion, Home +
//! Interiors (FHI) color via [`crate::PantoneFhiNamedColor`];
//! swatches numbered in the PANTONE solid-coated system resolve to
//! [`crate::PantoneNamedColor`]. Premium-metallic and pastel chips
//! outside those catalogs carry the sRGB value sampled from the
//! book page and say so in their doc comments.

mod abstract_expressionists;
mod coast_to_coast_woolens;
mod cosmetic_superstars;
mod fantasyland;
mod happy_at_home;
mod mid_century_modernists;
mod movie_goddesses;
mod signature_hues;
mod teen_angels;

pub use abstract_expressionists::ABSTRACT_EXPRESSIONISTS;
pub use abstract_expressionists::AbstractExpressionistsColor;
pub use coast_to_coast_woolens::COAST_TO_COAST_WOOLENS;
pub use coast_to_coast_woolens::CoastToCoastWoolensColor;
pub use cosmetic_superstars::COSMETIC_SUPERSTARS;
pub use cosmetic_superstars::CosmeticSuperstarsColor;
pub use fantasyland::FANTASYLAND;
pub use fantasyland::FantasylandColor;
pub use happy_at_home::HAPPY_AT_HOME;
pub use happy_at_home::HappyAtHomeColor;
pub use mid_century_modernists::MID_CENTURY_MODERNISTS;
pub use mid_century_modernists::MidCenturyModernistsColor;
pub use movie_goddesses::MOVIE_GODDESSES;
pub use movie_goddesses::MovieGoddessesColor;
pub use signature_hues::SIGNATURE_HUES_1950;
pub use signature_hues::SignatureHues1950Color;
pub use teen_angels::TEEN_ANGELS;
pub use teen_angels::TeenAngelsColor;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Color;
    use crate::PaletteDefinition;

    #[test]
    fn palettes_are_complete() {
        assert!(HAPPY_AT_HOME.is_complete());
        assert!(TEEN_ANGELS.is_complete());
        assert!(MID_CENTURY_MODERNISTS.is_complete());
        assert!(MOVIE_GODDESSES.is_complete());
        assert!(COSMETIC_SUPERSTARS.is_complete());
        assert!(COAST_TO_COAST_WOOLENS.is_complete());
        assert!(FANTASYLAND.is_complete());
        assert!(ABSTRACT_EXPRESSIONISTS.is_complete());
    }

    #[test]
    fn palettes_hold_the_book_chip_counts() {
        assert_eq!(HAPPY_AT_HOME.len(), 7);
        assert_eq!(TEEN_ANGELS.len(), 8);
        assert_eq!(MID_CENTURY_MODERNISTS.len(), 7);
        assert_eq!(MOVIE_GODDESSES.len(), 6);
        assert_eq!(COSMETIC_SUPERSTARS.len(), 6);
        assert_eq!(COAST_TO_COAST_WOOLENS.len(), 8);
        assert_eq!(FANTASYLAND.len(), 6);
        assert_eq!(ABSTRACT_EXPRESSIONISTS.len(), 7);
    }

    #[test]
    fn signature_hues_are_complete() {
        assert!(SIGNATURE_HUES_1950.is_complete());
    }

    #[test]
    fn signature_hues_hold_chapter_leads_in_order() {
        assert_eq!(SIGNATURE_HUES_1950.len(), 8);
        assert_eq!(SignatureHues1950Color::slots()[0], SignatureHues1950Color::HappyAtHome);
    }

    #[test]
    fn first_signature_hue_matches_the_book() {
        assert_eq!(SIGNATURE_HUES_1950.get(&SignatureHues1950Color::HappyAtHome), Some(&Color::<u8>::from_rgb_u32(0xFDD878)));
    }
}
