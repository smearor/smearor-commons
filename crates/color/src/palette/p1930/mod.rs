//! The 1930s chapter palettes of "Pantone: The Twentieth
//! Century in Color" (Eiseman & Recker, 2011).
//!
//! The slot names and order match the book's swatch rows. TCX
//! swatches resolve to the official PANTONE Fashion, Home +
//! Interiors (FHI) color via [`crate::PantoneFhiNamedColor`];
//! swatches numbered in the PANTONE solid-coated system resolve to
//! [`crate::PantoneNamedColor`]. Premium-metallic and pastel chips
//! outside those catalogs carry the sRGB value sampled from the
//! book page and say so in their doc comments.

mod deco_architecture;
mod diversions;
mod fantastic_plastic;
mod illusions;
mod parks_and_recreation;
mod roseville;
mod signature_hues;
mod wizard_of_oz;
mod world_of_tomorrow;

pub use deco_architecture::DECO_ARCHITECTURE;
pub use deco_architecture::DecoArchitectureColor;
pub use diversions::DIVERSIONS;
pub use diversions::DiversionsColor;
pub use fantastic_plastic::FANTASTIC_PLASTIC;
pub use fantastic_plastic::FantasticPlasticColor;
pub use illusions::ILLUSIONS;
pub use illusions::IllusionsColor;
pub use parks_and_recreation::PARKS_AND_RECREATION;
pub use parks_and_recreation::ParksAndRecreationColor;
pub use roseville::ROSEVILLE;
pub use roseville::RosevilleColor;
pub use signature_hues::SIGNATURE_HUES_1930;
pub use signature_hues::SignatureHues1930Color;
pub use wizard_of_oz::WIZARD_OF_OZ;
pub use wizard_of_oz::WizardOfOzColor;
pub use world_of_tomorrow::WORLD_OF_TOMORROW;
pub use world_of_tomorrow::WorldOfTomorrowColor;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Color;
    use crate::PaletteDefinition;

    #[test]
    fn palettes_are_complete() {
        assert!(DECO_ARCHITECTURE.is_complete());
        assert!(ILLUSIONS.is_complete());
        assert!(FANTASTIC_PLASTIC.is_complete());
        assert!(DIVERSIONS.is_complete());
        assert!(PARKS_AND_RECREATION.is_complete());
        assert!(ROSEVILLE.is_complete());
        assert!(WIZARD_OF_OZ.is_complete());
        assert!(WORLD_OF_TOMORROW.is_complete());
    }

    #[test]
    fn palettes_hold_the_book_chip_counts() {
        assert_eq!(DECO_ARCHITECTURE.len(), 5);
        assert_eq!(ILLUSIONS.len(), 6);
        assert_eq!(FANTASTIC_PLASTIC.len(), 8);
        assert_eq!(DIVERSIONS.len(), 7);
        assert_eq!(PARKS_AND_RECREATION.len(), 7);
        assert_eq!(ROSEVILLE.len(), 8);
        assert_eq!(WIZARD_OF_OZ.len(), 7);
        assert_eq!(WORLD_OF_TOMORROW.len(), 6);
    }

    #[test]
    fn signature_hues_are_complete() {
        assert!(SIGNATURE_HUES_1930.is_complete());
    }

    #[test]
    fn signature_hues_hold_chapter_leads_in_order() {
        assert_eq!(SIGNATURE_HUES_1930.len(), 8);
        assert_eq!(SignatureHues1930Color::slots()[0], SignatureHues1930Color::DecoArchitecture);
    }

    #[test]
    fn first_signature_hue_matches_the_book() {
        assert_eq!(SIGNATURE_HUES_1930.get(&SignatureHues1930Color::DecoArchitecture), Some(&Color::<u8>::from_rgb_u32(0xABAFB0)));
    }
}
