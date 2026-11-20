//! The 1990s chapter palettes of "Pantone: The Twentieth
//! Century in Color" (Eiseman & Recker, 2011).
//!
//! The slot names and order match the book's swatch rows. TCX
//! swatches resolve to the official PANTONE Fashion, Home +
//! Interiors (FHI) color via [`crate::PantoneFhiNamedColor`];
//! swatches numbered in the PANTONE solid-coated system resolve to
//! [`crate::PantoneNamedColor`]. Premium-metallic and pastel chips
//! outside those catalogs carry the sRGB value sampled from the
//! book page and say so in their doc comments.

mod anime;
mod chic_over_geek;
mod conspicuous_consumption;
mod future_forecasts;
mod grunge_and_graffiti;
mod its_a_good_thing;
mod latin_flavors;
mod nature_of_zen;
mod out_of_africa;
mod signature_hues;

pub use anime::ANIME;
pub use anime::AnimeColor;
pub use chic_over_geek::CHIC_OVER_GEEK;
pub use chic_over_geek::ChicOverGeekColor;
pub use conspicuous_consumption::CONSPICUOUS_CONSUMPTION;
pub use conspicuous_consumption::ConspicuousConsumptionColor;
pub use future_forecasts::FUTURE_FORECASTS;
pub use future_forecasts::FutureForecastsColor;
pub use grunge_and_graffiti::GRUNGE_AND_GRAFFITI;
pub use grunge_and_graffiti::GrungeAndGraffitiColor;
pub use its_a_good_thing::ITS_A_GOOD_THING;
pub use its_a_good_thing::ItsAGoodThingColor;
pub use latin_flavors::LATIN_FLAVORS;
pub use latin_flavors::LatinFlavorsColor;
pub use nature_of_zen::NATURE_OF_ZEN;
pub use nature_of_zen::NatureOfZenColor;
pub use out_of_africa::OUT_OF_AFRICA;
pub use out_of_africa::OutOfAfricaColor;
pub use signature_hues::SIGNATURE_HUES_1990;
pub use signature_hues::SignatureHues1990Color;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Color;
    use crate::PaletteDefinition;

    #[test]
    fn palettes_are_complete() {
        assert!(GRUNGE_AND_GRAFFITI.is_complete());
        assert!(ITS_A_GOOD_THING.is_complete());
        assert!(NATURE_OF_ZEN.is_complete());
        assert!(OUT_OF_AFRICA.is_complete());
        assert!(LATIN_FLAVORS.is_complete());
        assert!(CHIC_OVER_GEEK.is_complete());
        assert!(ANIME.is_complete());
        assert!(CONSPICUOUS_CONSUMPTION.is_complete());
        assert!(FUTURE_FORECASTS.is_complete());
    }

    #[test]
    fn palettes_hold_the_book_chip_counts() {
        assert_eq!(GRUNGE_AND_GRAFFITI.len(), 6);
        assert_eq!(ITS_A_GOOD_THING.len(), 8);
        assert_eq!(NATURE_OF_ZEN.len(), 6);
        assert_eq!(OUT_OF_AFRICA.len(), 8);
        assert_eq!(LATIN_FLAVORS.len(), 6);
        assert_eq!(CHIC_OVER_GEEK.len(), 6);
        assert_eq!(ANIME.len(), 9);
        assert_eq!(CONSPICUOUS_CONSUMPTION.len(), 6);
        assert_eq!(FUTURE_FORECASTS.len(), 12);
    }

    #[test]
    fn signature_hues_are_complete() {
        assert!(SIGNATURE_HUES_1990.is_complete());
    }

    #[test]
    fn signature_hues_hold_chapter_leads_in_order() {
        assert_eq!(SIGNATURE_HUES_1990.len(), 9);
        assert_eq!(SignatureHues1990Color::slots()[0], SignatureHues1990Color::GrungeAndGraffiti);
    }

    #[test]
    fn first_signature_hue_matches_the_book() {
        assert_eq!(
            SIGNATURE_HUES_1990.get(&SignatureHues1990Color::GrungeAndGraffiti),
            Some(&Color::<u8>::from_rgb_u32(0x807396))
        );
    }
}
