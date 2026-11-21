use crate::Color;
use crate::PaletteDefinition;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::collections::hash_map;
use std::hash::Hash;

mod builder;
mod p1900;
mod p1910;
mod p1920;
mod p1930;
mod p1940;
mod p1950;
mod p1960;
mod p1970;
mod p1980;
mod p1990;

pub use builder::PaletteBuildError;
pub use builder::PaletteBuilder;
pub use p1900::ARTS_AND_CRAFTS;
pub use p1900::ArtsAndCraftsColor;
pub use p1900::CHARM_OF_IRIDESCENCE;
pub use p1900::CharmOfIridescenceColor;
pub use p1900::EDWARDIAN_AFFAIRS;
pub use p1900::EdwardianAffairsColor;
pub use p1900::FAUVES;
pub use p1900::FIRSTS_FOR_WOMEN;
pub use p1900::FauvesColor;
pub use p1900::FirstsForWomenColor;
pub use p1900::JEWEL_TONES;
pub use p1900::JewelTonesColor;
pub use p1900::POIRET_REVOLUTION;
pub use p1900::PoiretRevolutionColor;
pub use p1900::SIGNATURE_HUES_1900;
pub use p1900::SignatureHues1900Color;
pub use p1910::COMING_HOME;
pub use p1910::CUBISM;
pub use p1910::ComingHomeColor;
pub use p1910::CubismColor;
pub use p1910::PARRISH_BLUES;
pub use p1910::ParrishBluesColor;
pub use p1910::SIGNATURE_HUES_1910;
pub use p1910::SignatureHues1910Color;
pub use p1910::THEATRICS;
pub use p1910::TheatricsColor;
pub use p1910::WIENER_WERKSTAETTE;
pub use p1910::WORLD_WAR_1;
pub use p1910::WienerWerkstaetteColor;
pub use p1910::WorldWar1Color;
pub use p1910::YOUTHFUL_PASTIMES;
pub use p1910::YouthfulPastimesColor;
pub use p1920::ART_DECO;
pub use p1920::ArtDecoColor;
pub use p1920::BAUHAUS;
pub use p1920::BauhausColor;
pub use p1920::COCKTAILS_AND_LAUGHTER;
pub use p1920::CocktailsAndLaughterColor;
pub use p1920::DESTINATIONS;
pub use p1920::DestinationsColor;
pub use p1920::LEYENDECKER_MAN;
pub use p1920::LeyendeckerManColor;
pub use p1920::MODERN_PLEASURES;
pub use p1920::ModernPleasuresColor;
pub use p1920::ROSE_IS_A_ROSE;
pub use p1920::RoseIsARoseColor;
pub use p1920::SIGNATURE_HUES_1920;
pub use p1920::SignatureHues1920Color;
pub use p1920::TUTMANIA;
pub use p1920::TutmaniaColor;
pub use p1930::DECO_ARCHITECTURE;
pub use p1930::DIVERSIONS;
pub use p1930::DecoArchitectureColor;
pub use p1930::DiversionsColor;
pub use p1930::FANTASTIC_PLASTIC;
pub use p1930::FantasticPlasticColor;
pub use p1930::ILLUSIONS;
pub use p1930::IllusionsColor;
pub use p1930::PARKS_AND_RECREATION;
pub use p1930::ParksAndRecreationColor;
pub use p1930::ROSEVILLE;
pub use p1930::RosevilleColor;
pub use p1930::SIGNATURE_HUES_1930;
pub use p1930::SignatureHues1930Color;
pub use p1930::WIZARD_OF_OZ;
pub use p1930::WORLD_OF_TOMORROW;
pub use p1930::WizardOfOzColor;
pub use p1930::WorldOfTomorrowColor;
pub use p1940::AMERICAN_DREAM;
pub use p1940::AmericanDreamColor;
pub use p1940::CAREFREE_AND_CASUAL;
pub use p1940::CarefreeAndCasualColor;
pub use p1940::EDWARD_HOPPER;
pub use p1940::EdwardHopperColor;
pub use p1940::FANTASIA;
pub use p1940::FILM_NOIR;
pub use p1940::FantasiaColor;
pub use p1940::FilmNoirColor;
pub use p1940::HIT_PARADE;
pub use p1940::HitParadeColor;
pub use p1940::LA_MODE;
pub use p1940::LaModeColor;
pub use p1940::SIGNATURE_HUES_1940;
pub use p1940::SignatureHues1940Color;
pub use p1940::WORLD_WAR_2;
pub use p1940::WorldWar2Color;
pub use p1950::ABSTRACT_EXPRESSIONISTS;
pub use p1950::AbstractExpressionistsColor;
pub use p1950::COAST_TO_COAST_WOOLENS;
pub use p1950::COSMETIC_SUPERSTARS;
pub use p1950::CoastToCoastWoolensColor;
pub use p1950::CosmeticSuperstarsColor;
pub use p1950::FANTASYLAND;
pub use p1950::FantasylandColor;
pub use p1950::HAPPY_AT_HOME;
pub use p1950::HappyAtHomeColor;
pub use p1950::MID_CENTURY_MODERNISTS;
pub use p1950::MOVIE_GODDESSES;
pub use p1950::MidCenturyModernistsColor;
pub use p1950::MovieGoddessesColor;
pub use p1950::SIGNATURE_HUES_1950;
pub use p1950::SignatureHues1950Color;
pub use p1950::TEEN_ANGELS;
pub use p1950::TeenAngelsColor;
pub use p1960::BLACK_IS_BEAUTIFUL;
pub use p1960::BlackIsBeautifulColor;
pub use p1960::DIFFERENT_SPACE;
pub use p1960::DifferentSpaceColor;
pub use p1960::KENSINGTON_AND_CARNABY;
pub use p1960::KensingtonAndCarnabyColor;
pub use p1960::PANTONE;
pub use p1960::PASSAGE_TO_INDIA;
pub use p1960::PSYCHEDELIA;
pub use p1960::PantoneColor;
pub use p1960::PassageToIndiaColor;
pub use p1960::PsychedeliaColor;
pub use p1960::SESAME_STREET;
pub use p1960::SIGNATURE_HUES_1960;
pub use p1960::SesameStreetColor;
pub use p1960::SignatureHues1960Color;
pub use p1960::WARHOL;
pub use p1960::WarholColor;
pub use p1970::AVOCADO_AND_HARVEST_GOLD;
pub use p1970::AvocadoAndHarvestGoldColor;
pub use p1970::COLORS_AND_COORDINATES;
pub use p1970::ColorsAndCoordinatesColor;
pub use p1970::DAY_THE_WORLD_TURNED_DAY_GLO;
pub use p1970::DayTheWorldTurnedDayGloColor;
pub use p1970::FEATHERS_AND_LEATHERS;
pub use p1970::FeathersAndLeathersColor;
pub use p1970::HOTEL_CALIFORNIA;
pub use p1970::HotelCaliforniaColor;
pub use p1970::LAND_ART;
pub use p1970::LandArtColor;
pub use p1970::NIGHT_LIFE;
pub use p1970::NightLifeColor;
pub use p1970::PROVENCE;
pub use p1970::ProvenceColor;
pub use p1970::SIGNATURE_HUES_1970;
pub use p1970::SignatureHues1970Color;
pub use p1980::JAPONAIS;
pub use p1980::JaponaisColor;
pub use p1980::MAJORELLE_AND_MOROCCO;
pub use p1980::MEMPHIS_MICHAEL_AND_PHILIPPE;
pub use p1980::MIAMI_VICE;
pub use p1980::MajorelleAndMoroccoColor;
pub use p1980::MemphisMichaelAndPhilippeColor;
pub use p1980::MiamiViceColor;
pub use p1980::PERSONAL_COLORS;
pub use p1980::PersonalColorsColor;
pub use p1980::SANTA_FE;
pub use p1980::SIGNATURE_HUES_1980;
pub use p1980::SIGNS_AND_SYMBOLS;
pub use p1980::SantaFeColor;
pub use p1980::SignatureHues1980Color;
pub use p1980::SignsAndSymbolsColor;
pub use p1980::TO_THE_MANOR_BORN;
pub use p1980::ToTheManorBornColor;
pub use p1980::URBAN_COWBOYS;
pub use p1980::UrbanCowboysColor;
pub use p1990::ANIME;
pub use p1990::AnimeColor;
pub use p1990::CHIC_OVER_GEEK;
pub use p1990::CONSPICUOUS_CONSUMPTION;
pub use p1990::ChicOverGeekColor;
pub use p1990::ConspicuousConsumptionColor;
pub use p1990::FUTURE_FORECASTS;
pub use p1990::FutureForecastsColor;
pub use p1990::GRUNGE_AND_GRAFFITI;
pub use p1990::GrungeAndGraffitiColor;
pub use p1990::ITS_A_GOOD_THING;
pub use p1990::ItsAGoodThingColor;
pub use p1990::LATIN_FLAVORS;
pub use p1990::LatinFlavorsColor;
pub use p1990::NATURE_OF_ZEN;
pub use p1990::NatureOfZenColor;
pub use p1990::OUT_OF_AFRICA;
pub use p1990::OutOfAfricaColor;
pub use p1990::SIGNATURE_HUES_1990;
pub use p1990::SignatureHues1990Color;

/// A named-color palette: semantic names mapped to colors and
/// looked up by hash.
///
/// The key type is usually a fieldless enum generated by
/// [`crate::palette_definition!`], but any key works - strings or
/// [`crate::NamedColor`] stay usable, just without the completeness
/// checks [`Palette::missing`] and [`Palette::is_complete`] offer
/// for [`PaletteDefinition`] keys.
///
/// For `const`-constructible palettes see
/// [`crate::StaticPalette`], which upgrades into this type via
/// [`Palette::from`].
#[derive(Debug, Clone, PartialEq)]
pub struct Palette<N: Eq + Hash + 'static, T: 'static> {
    /// Name/color pairs.
    colors: HashMap<N, Color<T>>,
}

impl<N: Eq + Hash + 'static, T: 'static> Palette<N, T> {
    /// Creates a palette from a name/color map.
    pub fn new(colors: HashMap<N, Color<T>>) -> Self {
        Self { colors }
    }

    /// The number of colors.
    pub fn len(&self) -> usize {
        self.colors.len()
    }

    /// `true` when the palette holds no colors.
    pub fn is_empty(&self) -> bool {
        self.colors.is_empty()
    }

    /// Iterates `(name, color)` pairs in unspecified order.
    pub fn iter(&self) -> hash_map::Iter<'_, N, Color<T>> {
        self.colors.iter()
    }

    /// Returns a palette with every color transformed by `f` (e.g.
    /// channel conversion `Palette<N, f64>` -> `Palette<N, f32>`).
    pub fn map_colors<U>(&self, f: impl Fn(Color<T>) -> Color<U>) -> Palette<N, U>
    where
        N: Clone,
        T: Copy,
        U: 'static,
    {
        Palette::new(self.colors.iter().map(|(name, color)| (name.clone(), f(*color))).collect())
    }

    /// A fluent builder for assembling a palette name by name - see
    /// [`PaletteBuilder`].
    pub fn builder() -> PaletteBuilder<N, T> {
        PaletteBuilder::new()
    }
}

impl<N: Eq + Hash + 'static, T: 'static> Palette<N, T> {
    /// The color registered under `name`.
    pub fn get<Q>(&self, name: &Q) -> Option<&Color<T>>
    where
        N: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.colors.get(name)
    }

    /// `true` when a color is registered under `name`.
    pub fn contains<Q>(&self, name: &Q) -> bool
    where
        N: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.colors.contains_key(name)
    }

    /// The color registered under `name`, or `fallback` when
    /// absent.
    pub fn get_or<Q>(&self, name: &Q, fallback: Color<T>) -> Color<T>
    where
        N: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
        T: Clone,
    {
        self.get(name).cloned().unwrap_or(fallback)
    }

    /// The underlying name/color map.
    pub fn colors(&self) -> &HashMap<N, Color<T>> {
        &self.colors
    }

    /// Registers `color` under `name`, returning the replaced
    /// color. Names go through `Into`, so enum variants can be used
    /// directly and [`crate::NamedColor`] keys can be written as
    /// plain `&str`. For fluent assembly see
    /// [`Palette::builder`].
    pub fn insert(&mut self, name: impl Into<N>, color: Color<T>) -> Option<Color<T>> {
        self.colors.insert(name.into(), color)
    }

    /// Removes the color registered under `name`, returning it.
    pub fn remove<Q>(&mut self, name: &Q) -> Option<Color<T>>
    where
        N: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.colors.remove(name)
    }
}

/// Methods available when the key type enumerates its vocabulary.
impl<N: PaletteDefinition, T: 'static> Palette<N, T> {
    /// The declared slots that have no color registered, in
    /// declaration order.
    pub fn missing(&self) -> Vec<N> {
        N::slots().iter().filter(|slot| !self.contains(*slot)).copied().collect()
    }

    /// `true` when every declared slot carries a color.
    pub fn is_complete(&self) -> bool {
        N::slots().iter().all(|slot| self.contains(slot))
    }

    /// Iterates all declared slots in declaration order, paired
    /// with their color (`None` for unfilled slots).
    pub fn ordered(&self) -> impl Iterator<Item = (N, Option<&Color<T>>)> {
        N::slots().iter().copied().map(|slot| (slot, self.get(&slot)))
    }
}

/// An empty palette.
impl<N: Eq + Hash + 'static, T: 'static> Default for Palette<N, T> {
    fn default() -> Self {
        Self::new(HashMap::new())
    }
}

impl<N: Eq + Hash + 'static, T: 'static> From<HashMap<N, Color<T>>> for Palette<N, T> {
    fn from(colors: HashMap<N, Color<T>>) -> Self {
        Self::new(colors)
    }
}

impl<N: Eq + Hash + 'static, T: 'static> FromIterator<(N, Color<T>)> for Palette<N, T> {
    fn from_iter<I: IntoIterator<Item = (N, Color<T>)>>(iter: I) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

impl<'a, N: Eq + Hash + 'static, T: 'static> IntoIterator for &'a Palette<N, T> {
    type Item = (&'a N, &'a Color<T>);
    type IntoIter = hash_map::Iter<'a, N, Color<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.colors.iter()
    }
}

/// A map `{ "name": "#rrggbb(aa)" }` - the same wire format as
/// [`crate::StaticPalette`].
impl<N: Eq + Hash + 'static, T: 'static> Serialize for Palette<N, T>
where
    N: Serialize,
    Color<T>: Serialize,
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.colors.serialize(serializer)
    }
}

impl<'de, N: Eq + Hash + 'static, T: 'static> Deserialize<'de> for Palette<N, T>
where
    N: Deserialize<'de>,
    Color<T>: Deserialize<'de>,
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        HashMap::<N, Color<T>>::deserialize(deserializer).map(Self::new)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NamedColor;
    use crate::palette_definition;

    const LIGHT: NamedColor = NamedColor::new("light");

    palette_definition! {
        /// Colors of a chess board theme.
        pub enum BoardSlot {
            /// Light squares.
            Light => "light",
            /// Dark squares.
            Dark => "dark",
            /// Frame around the board.
            Frame => "frame"
        }
    }

    fn board_palette() -> Palette<NamedColor, f64> {
        Palette::builder()
            .color(LIGHT.clone(), Color::<f64>::new(0.93, 0.83, 0.62))
            .color("dark", Color::<f64>::new(0.45, 0.30, 0.18))
            .build()
    }

    fn slot_palette() -> Palette<BoardSlot, f64> {
        Palette::builder()
            .color(BoardSlot::Light, Color::<f64>::new(0.93, 0.83, 0.62))
            .color(BoardSlot::Dark, Color::<f64>::new(0.45, 0.30, 0.18))
            .build()
    }

    #[test]
    fn insert_and_get_by_name() {
        let palette = board_palette();
        assert_eq!(palette.get("light"), Some(&Color::<f64>::new(0.93, 0.83, 0.62)));
        assert_eq!(palette.get(&LIGHT), Some(&Color::<f64>::new(0.93, 0.83, 0.62)));
        assert_eq!(palette.len(), 2);
    }

    #[test]
    fn insert_replaces_and_returns_previous() {
        let mut palette = board_palette();
        let previous = palette.insert("light", Color::<f64>::WHITE);
        assert_eq!(previous, Some(Color::<f64>::new(0.93, 0.83, 0.62)));
        assert_eq!(palette.get("light"), Some(&Color::<f64>::WHITE));
    }

    #[test]
    fn remove_returns_the_color() {
        let mut palette = board_palette();
        assert_eq!(palette.remove("light"), Some(Color::<f64>::new(0.93, 0.83, 0.62)));
        assert_eq!(palette.remove("light"), None);
    }

    #[test]
    fn map_colors_converts_channel_type() {
        let converted: Palette<BoardSlot, f32> = slot_palette().map_colors(|color| color.into());
        assert_eq!(converted.get(&BoardSlot::Dark), Some(&Color::<f32>::new(0.45, 0.30, 0.18)));
    }

    #[test]
    fn ordered_iterates_declaration_order() {
        let palette = slot_palette();
        let slots: Vec<BoardSlot> = palette.ordered().map(|(slot, _)| slot).collect();
        assert_eq!(slots, vec![BoardSlot::Light, BoardSlot::Dark, BoardSlot::Frame]);
        let light = palette.ordered().next().map(|(_, color)| color);
        assert_eq!(light, Some(Some(&Color::<f64>::new(0.93, 0.83, 0.62))));
    }

    #[test]
    fn missing_reports_unfilled_slots_in_declaration_order() {
        assert_eq!(slot_palette().missing(), vec![BoardSlot::Frame]);
        assert!(!slot_palette().is_complete());
    }
}
