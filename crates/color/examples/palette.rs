//! Palette viewer for `smearor-commons-color`.
//!
//! Shows the curated decade palettes and signature hues (`p19NN`
//! modules) as selectable swatches. Picking a swatch shows a large
//! color area and every representation the crate can produce:
//! `Color<u8>`, `Color<f64>`, [`HexColor`] and the exact CSS/PMS/FHI
//! named-color match when the sRGB value exists in that vocabulary.
//!
//! Run with:
//! ```sh
//! cargo run --example palette -p smearor-commons-color --features gtk4
//! ```

use std::cell::RefCell;
use std::rc::Rc;

use gtk4::Align;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Box;
use gtk4::Button;
use gtk4::CssProvider;
use gtk4::DrawingArea;
use gtk4::DropDown;
use gtk4::FlowBox;
use gtk4::Grid;
use gtk4::Label;
use gtk4::Orientation;
use gtk4::ScrolledWindow;
use gtk4::StringList;
use gtk4::gdk;
use gtk4::glib;
use gtk4::prelude::*;
use smearor_commons_color::ABSTRACT_EXPRESSIONISTS;
use smearor_commons_color::AMERICAN_DREAM;
use smearor_commons_color::ANIME;
use smearor_commons_color::ART_DECO;
use smearor_commons_color::ARTS_AND_CRAFTS;
use smearor_commons_color::AVOCADO_AND_HARVEST_GOLD;
use smearor_commons_color::BAUHAUS;
use smearor_commons_color::BLACK_IS_BEAUTIFUL;
use smearor_commons_color::CAREFREE_AND_CASUAL;
use smearor_commons_color::CHARM_OF_IRIDESCENCE;
use smearor_commons_color::CHIC_OVER_GEEK;
use smearor_commons_color::COAST_TO_COAST_WOOLENS;
use smearor_commons_color::COCKTAILS_AND_LAUGHTER;
use smearor_commons_color::COLORS_AND_COORDINATES;
use smearor_commons_color::COMING_HOME;
use smearor_commons_color::CONSPICUOUS_CONSUMPTION;
use smearor_commons_color::COSMETIC_SUPERSTARS;
use smearor_commons_color::CUBISM;
use smearor_commons_color::Color;
use smearor_commons_color::CssNamedColor;
use smearor_commons_color::DAY_THE_WORLD_TURNED_DAY_GLO;
use smearor_commons_color::DECO_ARCHITECTURE;
use smearor_commons_color::DESTINATIONS;
use smearor_commons_color::DIFFERENT_SPACE;
use smearor_commons_color::DIVERSIONS;
use smearor_commons_color::EDWARD_HOPPER;
use smearor_commons_color::EDWARDIAN_AFFAIRS;
use smearor_commons_color::FANTASIA;
use smearor_commons_color::FANTASTIC_PLASTIC;
use smearor_commons_color::FANTASYLAND;
use smearor_commons_color::FAUVES;
use smearor_commons_color::FEATHERS_AND_LEATHERS;
use smearor_commons_color::FILM_NOIR;
use smearor_commons_color::FIRSTS_FOR_WOMEN;
use smearor_commons_color::FUTURE_FORECASTS;
use smearor_commons_color::GRUNGE_AND_GRAFFITI;
use smearor_commons_color::HAPPY_AT_HOME;
use smearor_commons_color::HIT_PARADE;
use smearor_commons_color::HOTEL_CALIFORNIA;
use smearor_commons_color::HexColor;
use smearor_commons_color::ILLUSIONS;
use smearor_commons_color::ITS_A_GOOD_THING;
use smearor_commons_color::JAPONAIS;
use smearor_commons_color::JEWEL_TONES;
use smearor_commons_color::KENSINGTON_AND_CARNABY;
use smearor_commons_color::LA_MODE;
use smearor_commons_color::LAND_ART;
use smearor_commons_color::LATIN_FLAVORS;
use smearor_commons_color::LEYENDECKER_MAN;
use smearor_commons_color::MAJORELLE_AND_MOROCCO;
use smearor_commons_color::MEMPHIS_MICHAEL_AND_PHILIPPE;
use smearor_commons_color::MIAMI_VICE;
use smearor_commons_color::MID_CENTURY_MODERNISTS;
use smearor_commons_color::MODERN_PLEASURES;
use smearor_commons_color::MOVIE_GODDESSES;
use smearor_commons_color::NATURE_OF_ZEN;
use smearor_commons_color::NIGHT_LIFE;
use smearor_commons_color::OUT_OF_AFRICA;
use smearor_commons_color::PANTONE;
use smearor_commons_color::PARKS_AND_RECREATION;
use smearor_commons_color::PARRISH_BLUES;
use smearor_commons_color::PASSAGE_TO_INDIA;
use smearor_commons_color::PERSONAL_COLORS;
use smearor_commons_color::POIRET_REVOLUTION;
use smearor_commons_color::PROVENCE;
use smearor_commons_color::PSYCHEDELIA;
use smearor_commons_color::PaletteDefinition;
use smearor_commons_color::PantoneFhiNamedColor;
use smearor_commons_color::PantoneNamedColor;
use smearor_commons_color::ROSE_IS_A_ROSE;
use smearor_commons_color::ROSEVILLE;
use smearor_commons_color::SANTA_FE;
use smearor_commons_color::SESAME_STREET;
use smearor_commons_color::SIGNATURE_HUES_1900;
use smearor_commons_color::SIGNATURE_HUES_1910;
use smearor_commons_color::SIGNATURE_HUES_1920;
use smearor_commons_color::SIGNATURE_HUES_1930;
use smearor_commons_color::SIGNATURE_HUES_1940;
use smearor_commons_color::SIGNATURE_HUES_1950;
use smearor_commons_color::SIGNATURE_HUES_1960;
use smearor_commons_color::SIGNATURE_HUES_1970;
use smearor_commons_color::SIGNATURE_HUES_1980;
use smearor_commons_color::SIGNATURE_HUES_1990;
use smearor_commons_color::SIGNS_AND_SYMBOLS;
use smearor_commons_color::StaticPalette;
use smearor_commons_color::TEEN_ANGELS;
use smearor_commons_color::THEATRICS;
use smearor_commons_color::TO_THE_MANOR_BORN;
use smearor_commons_color::TUTMANIA;
use smearor_commons_color::URBAN_COWBOYS;
use smearor_commons_color::WARHOL;
use smearor_commons_color::WIENER_WERKSTAETTE;
use smearor_commons_color::WIZARD_OF_OZ;
use smearor_commons_color::WORLD_OF_TOMORROW;
use smearor_commons_color::WORLD_WAR_1;
use smearor_commons_color::WORLD_WAR_2;
use smearor_commons_color::YOUTHFUL_PASTIMES;

const APP_ID: &str = "io.smearor.commons_color.palette";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

/// A named palette flattened to `(slot name, color)` pairs.
struct NamedPalette {
    /// The chapter title shown in the palette switcher.
    title: &'static str,
    /// The `(display name, sRGB)` pairs in declaration order.
    colors: Vec<(String, Color<u8>)>,
}

/// A decade chapter with its palettes, `Signature Hues` last.
struct NamedDecade {
    /// The decade label shown in the decade switcher.
    title: &'static str,
    /// The decade's palettes; `Signature Hues` comes last.
    palettes: Vec<NamedPalette>,
}

/// The `(title-cased name, color)` pairs of a `const` palette.
fn palette_entries<N: PaletteDefinition>(palette: &'static StaticPalette<N, u8>) -> Vec<(String, Color<u8>)> {
    palette.iter().map(|(slot, color)| (title_case(slot.name()), *color)).collect()
}

/// `"jet-black"` → `"Jet Black"`.
fn title_case(kebab: &str) -> String {
    kebab
        .split('-')
        .map(|word| format!("{}{}", word[..1].to_uppercase(), &word[1..]))
        .collect::<Vec<_>>()
        .join(" ")
}

/// The CSS named color of this exact sRGB value, if one exists.
fn css_name(color: &Color<u8>) -> Option<&'static str> {
    CssNamedColor::ALL.iter().find(|named| named.to_color() == *color).map(CssNamedColor::as_str)
}

/// The Pantone Matching System name of this exact sRGB value, if one exists.
fn pms_name(color: &Color<u8>) -> Option<&'static str> {
    PantoneNamedColor::ALL
        .iter()
        .find(|named| named.to_color() == *color)
        .map(PantoneNamedColor::as_str)
}

/// The Pantone FHI name of this exact sRGB value, if one exists.
fn fhi_name(color: &Color<u8>) -> Option<&'static str> {
    PantoneFhiNamedColor::ALL
        .iter()
        .find(|named| named.to_color() == *color)
        .map(PantoneFhiNamedColor::as_str)
}

/// The `(title, palette)` pair of a `const` palette.
fn named_palette<N: PaletteDefinition>(title: &'static str, palette: &'static StaticPalette<N, u8>) -> NamedPalette {
    NamedPalette {
        title,
        colors: palette_entries(palette),
    }
}

/// The decade chapters; each palette list ends with `Signature
/// Hues`.
fn decades() -> Vec<NamedDecade> {
    vec![
        NamedDecade {
            title: "1900s",
            palettes: vec![
                named_palette("Edwardian Affairs", &EDWARDIAN_AFFAIRS),
                named_palette("Arts and Crafts", &ARTS_AND_CRAFTS),
                named_palette("Jewel Tones", &JEWEL_TONES),
                named_palette("The Charm of Iridescence", &CHARM_OF_IRIDESCENCE),
                named_palette("Firsts for Women", &FIRSTS_FOR_WOMEN),
                named_palette("Poiret Revolution", &POIRET_REVOLUTION),
                named_palette("The Fauves", &FAUVES),
                named_palette("Signature Hues", &SIGNATURE_HUES_1900),
            ],
        },
        NamedDecade {
            title: "1910s",
            palettes: vec![
                named_palette("Theatrics", &THEATRICS),
                named_palette("Parrish Blues", &PARRISH_BLUES),
                named_palette("Wiener Werkstätte", &WIENER_WERKSTAETTE),
                named_palette("Youthful Pastimes", &YOUTHFUL_PASTIMES),
                named_palette("Cubism", &CUBISM),
                named_palette("World War I", &WORLD_WAR_1),
                named_palette("Coming Home", &COMING_HOME),
                named_palette("Signature Hues", &SIGNATURE_HUES_1910),
            ],
        },
        NamedDecade {
            title: "1920s",
            palettes: vec![
                named_palette("Art Deco", &ART_DECO),
                named_palette("Tutmania", &TUTMANIA),
                named_palette("Cocktails and Laughter", &COCKTAILS_AND_LAUGHTER),
                named_palette("Destinations", &DESTINATIONS),
                named_palette("The Leyendecker Man", &LEYENDECKER_MAN),
                named_palette("Bauhaus", &BAUHAUS),
                named_palette("Modern Pleasures", &MODERN_PLEASURES),
                named_palette("A Rose Is a Rose", &ROSE_IS_A_ROSE),
                named_palette("Signature Hues", &SIGNATURE_HUES_1920),
            ],
        },
        NamedDecade {
            title: "1930s",
            palettes: vec![
                named_palette("Deco Architecture", &DECO_ARCHITECTURE),
                named_palette("Illusions", &ILLUSIONS),
                named_palette("Fantastic Plastic", &FANTASTIC_PLASTIC),
                named_palette("Diversions", &DIVERSIONS),
                named_palette("Parks and Recreation", &PARKS_AND_RECREATION),
                named_palette("Roseville", &ROSEVILLE),
                named_palette("The Wizard of Oz", &WIZARD_OF_OZ),
                named_palette("The World of Tomorrow", &WORLD_OF_TOMORROW),
                named_palette("Signature Hues", &SIGNATURE_HUES_1930),
            ],
        },
        NamedDecade {
            title: "1940s",
            palettes: vec![
                named_palette("Fantasia", &FANTASIA),
                named_palette("Edward Hopper", &EDWARD_HOPPER),
                named_palette("World War II", &WORLD_WAR_2),
                named_palette("Carefree and Casual", &CAREFREE_AND_CASUAL),
                named_palette("The American Dream", &AMERICAN_DREAM),
                named_palette("Hit Parade", &HIT_PARADE),
                named_palette("Film Noir", &FILM_NOIR),
                named_palette("La Mode", &LA_MODE),
                named_palette("Signature Hues", &SIGNATURE_HUES_1940),
            ],
        },
        NamedDecade {
            title: "1950s",
            palettes: vec![
                named_palette("Happy at Home", &HAPPY_AT_HOME),
                named_palette("Teen Angels", &TEEN_ANGELS),
                named_palette("Mid-Century Modernists", &MID_CENTURY_MODERNISTS),
                named_palette("Movie Goddesses", &MOVIE_GODDESSES),
                named_palette("Cosmetic Superstars", &COSMETIC_SUPERSTARS),
                named_palette("Coast-to-Coast Woolens", &COAST_TO_COAST_WOOLENS),
                named_palette("Fantasyland", &FANTASYLAND),
                named_palette("Abstract Expressionists", &ABSTRACT_EXPRESSIONISTS),
                named_palette("Signature Hues", &SIGNATURE_HUES_1950),
            ],
        },
        NamedDecade {
            title: "1960s",
            palettes: vec![
                named_palette("Passage to India", &PASSAGE_TO_INDIA),
                named_palette("A Different Space", &DIFFERENT_SPACE),
                named_palette("Kensington and Carnaby", &KENSINGTON_AND_CARNABY),
                named_palette("Black Is Beautiful", &BLACK_IS_BEAUTIFUL),
                named_palette("Psychedelia", &PSYCHEDELIA),
                named_palette("Sesame Street", &SESAME_STREET),
                named_palette("Warhol", &WARHOL),
                named_palette("PANTONE", &PANTONE),
                named_palette("Signature Hues", &SIGNATURE_HUES_1960),
            ],
        },
        NamedDecade {
            title: "1970s",
            palettes: vec![
                named_palette("Colors and Coordinates", &COLORS_AND_COORDINATES),
                named_palette("Avocado and Harvest Gold", &AVOCADO_AND_HARVEST_GOLD),
                named_palette("Feathers and Leathers", &FEATHERS_AND_LEATHERS),
                named_palette("Provence", &PROVENCE),
                named_palette("Land Art", &LAND_ART),
                named_palette("The Day the World Turned Day-Glo", &DAY_THE_WORLD_TURNED_DAY_GLO),
                named_palette("Night Life", &NIGHT_LIFE),
                named_palette("Hotel California", &HOTEL_CALIFORNIA),
                named_palette("Signature Hues", &SIGNATURE_HUES_1970),
            ],
        },
        NamedDecade {
            title: "1980s",
            palettes: vec![
                named_palette("Memphis, Michael, and Philippe", &MEMPHIS_MICHAEL_AND_PHILIPPE),
                named_palette("To the Manor Born", &TO_THE_MANOR_BORN),
                named_palette("Urban Cowboys", &URBAN_COWBOYS),
                named_palette("Signs and Symbols", &SIGNS_AND_SYMBOLS),
                named_palette("Miami Vice", &MIAMI_VICE),
                named_palette("Majorelle and Morocco", &MAJORELLE_AND_MOROCCO),
                named_palette("Santa Fe", &SANTA_FE),
                named_palette("Personal Colors", &PERSONAL_COLORS),
                named_palette("Japonais", &JAPONAIS),
                named_palette("Signature Hues", &SIGNATURE_HUES_1980),
            ],
        },
        NamedDecade {
            title: "1990s",
            palettes: vec![
                named_palette("Grunge and Graffiti", &GRUNGE_AND_GRAFFITI),
                named_palette("It's a Good Thing", &ITS_A_GOOD_THING),
                named_palette("The Nature of Zen", &NATURE_OF_ZEN),
                named_palette("Out of Africa", &OUT_OF_AFRICA),
                named_palette("Latin Flavors", &LATIN_FLAVORS),
                named_palette("Chic over Geek", &CHIC_OVER_GEEK),
                named_palette("Anime", &ANIME),
                named_palette("Conspicuous Consumption", &CONSPICUOUS_CONSUMPTION),
                named_palette("Future Forecasts", &FUTURE_FORECASTS),
                named_palette("Signature Hues", &SIGNATURE_HUES_1990),
            ],
        },
    ]
}

/// Shared view state updated whenever a swatch is selected.
struct Detail {
    /// The currently shown color.
    selected: Rc<RefCell<Color<u8>>>,
    /// The heading above the color area.
    heading: Label,
    /// The large color area.
    area: DrawingArea,
    /// The representation value labels, in row order.
    values: Vec<Label>,
}

impl Detail {
    /// The representation texts of `color`, in row order.
    fn representations(color: &Color<u8>) -> Vec<String> {
        let normalized = Color::<f64>::from(*color);
        vec![
            format!("({}, {}, {}, {})", color.r, color.g, color.b, color.a),
            format!("({:.3}, {:.3}, {:.3}, {:.3})", normalized.r, normalized.g, normalized.b, normalized.a),
            HexColor::from(*color).to_string(),
            css_name(color).unwrap_or("—").to_string(),
            pms_name(color).unwrap_or("—").to_string(),
            fhi_name(color).unwrap_or("—").to_string(),
        ]
    }

    /// Shows `color` and `name` in the color area and the
    /// representation rows.
    fn show(&self, name: &str, color: Color<u8>) {
        *self.selected.borrow_mut() = color;
        self.heading.set_label(name);
        self.area.queue_draw();
        for (value, text) in self.values.iter().zip(Self::representations(&color)) {
            value.set_label(&text);
        }
    }
}

/// The CSS class naming a swatch color - the packed RGB is part of
/// the class name, so identical colors always share their rule.
fn swatch_class(color: &Color<u8>) -> String {
    format!("swatch-{:02x}{:02x}{:02x}", color.r, color.g, color.b)
}

/// The CSS rule for a swatch color: colored background, text color
/// picked by BT.709 luminance.
fn swatch_rule(color: &Color<u8>) -> String {
    let foreground = if color.luminance_709().as_f64() > 0.5 { "#000000" } else { "#ffffff" };
    format!(".{} {{ background: {}; color: {foreground}; }}", swatch_class(color), HexColor::from(*color))
}

/// Registers the swatch rules of every palette color on the display.
fn install_swatch_css(decades: &[NamedDecade]) {
    let rules = decades
        .iter()
        .flat_map(|decade| {
            decade
                .palettes
                .iter()
                .flat_map(|palette| palette.colors.iter().map(|(_, color)| swatch_rule(color)))
        })
        .collect::<Vec<_>>()
        .join("\n");
    let provider = CssProvider::new();
    provider.load_from_string(&rules);
    if let Some(display) = gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(&display, &provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);
    }
}

/// A swatch button with the color as its background.
fn swatch_button(name: &str, color: &Color<u8>) -> Button {
    let button = Button::builder().label(name).height_request(36).build();
    button.add_css_class(&swatch_class(color));
    button
}

/// The representation grid: label column + value column.
fn representation_grid() -> (Grid, Vec<Label>) {
    let rows = ["Color<u8>", "Color<f64>", "HexColor", "CSS name", "Pantone PMS", "Pantone FHI (TCX)"];
    let grid = Grid::builder().row_spacing(6).column_spacing(24).build();
    let mut values = Vec::new();
    for (row, key) in rows.iter().enumerate() {
        let key_label = Label::builder().label(*key).halign(Align::End).css_classes(["dim-label"]).build();
        let value_label = Label::builder().halign(Align::Start).selectable(true).build();
        grid.attach(&key_label, 0, row as i32, 1, 1);
        grid.attach(&value_label, 1, row as i32, 1, 1);
        values.push(value_label);
    }
    (grid, values)
}

fn build_ui(app: &Application) {
    let decades = Rc::new(decades());
    install_swatch_css(&decades);
    let window = ApplicationWindow::builder()
        .application(app)
        .title("smearor-commons-color palettes")
        .default_width(820)
        .default_height(560)
        .build();

    // Left: decade + palette switchers and the swatch list. The
    // palette model is rebuilt whenever the decade changes and
    // preselects the decade's `Signature Hues`.
    let decade_switcher = DropDown::from_strings(&decades.iter().map(|decade| decade.title).collect::<Vec<_>>());
    let palette_switcher = DropDown::default();
    let swatches = FlowBox::builder()
        .column_spacing(4)
        .row_spacing(4)
        .selection_mode(gtk4::SelectionMode::None)
        .build();
    let scrolled = ScrolledWindow::builder()
        .child(&swatches)
        .min_content_width(230)
        .vexpand(true)
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .build();
    let sidebar = Box::builder().orientation(Orientation::Vertical).spacing(12).build();
    sidebar.append(&decade_switcher);
    sidebar.append(&palette_switcher);
    sidebar.append(&scrolled);

    // Right: heading, large color area, representations.
    let heading = Label::builder().css_classes(["title-2"]).halign(Align::Start).build();
    let selected = Rc::new(RefCell::new(decades[0].palettes[0].colors[0].1));
    let area = DrawingArea::builder().vexpand(true).hexpand(true).build();
    area.set_draw_func({
        let selected = Rc::clone(&selected);
        move |_, cr, width, height| {
            let rgba = gdk::RGBA::from(*selected.borrow());
            cr.set_source_rgba(f64::from(rgba.red()), f64::from(rgba.green()), f64::from(rgba.blue()), f64::from(rgba.alpha()));
            cr.rectangle(0.0, 0.0, f64::from(width), f64::from(height));
            let _ = cr.fill();
        }
    });
    let (grid, values) = representation_grid();
    let detail = Rc::new(Detail {
        selected,
        heading: heading.clone(),
        area: area.clone(),
        values,
    });
    let content = Box::builder().orientation(Orientation::Vertical).spacing(12).hexpand(true).build();
    content.append(&heading);
    content.append(&area);
    content.append(&grid);

    // Rebuilds the swatch list of `palette` within `decade` and shows
    // its first color.
    let show_palette = {
        let detail = Rc::clone(&detail);
        let decades = Rc::clone(&decades);
        move |decade: u32, palette: u32| {
            let palette = &decades[decade as usize].palettes[palette as usize];
            while let Some(child) = swatches.first_child() {
                swatches.remove(&child);
            }
            for (name, color) in &palette.colors {
                let button = swatch_button(name, color);
                button.connect_clicked({
                    let detail = Rc::clone(&detail);
                    let name = name.clone();
                    let color = *color;
                    move |_| detail.show(&name, color)
                });
                swatches.append(&button);
            }
            if let Some((name, color)) = palette.colors.first() {
                detail.show(name, *color);
            }
        }
    };

    // Rebuilds the palette model of decade `index` and selects its
    // `Signature Hues` entry - the palette switcher's notify signal
    // then triggers `show_palette`.
    let select_decade = {
        let palette_switcher = palette_switcher.clone();
        let show_palette = show_palette.clone();
        move |index: u32| {
            let decade = &decades[index as usize];
            let titles = decade.palettes.iter().map(|palette| palette.title).collect::<Vec<_>>();
            palette_switcher.set_model(Some(&StringList::new(&titles)));
            let last = decade.palettes.len() as u32 - 1;
            palette_switcher.set_selected(last);
            // Explicit call: `set_selected` may not emit `notify`
            // when the index stays 0 (single-palette decades).
            show_palette(index, last);
        }
    };

    decade_switcher.connect_selected_notify({
        let select_decade = select_decade.clone();
        move |switcher| select_decade(switcher.selected())
    });
    palette_switcher.connect_selected_notify({
        let decade_switcher = decade_switcher.clone();
        let show_palette = show_palette.clone();
        move |switcher| show_palette(decade_switcher.selected(), switcher.selected())
    });
    select_decade(0);

    let root = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(12)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    root.append(&sidebar);
    root.append(&content);
    window.set_child(Some(&root));
    window.present();
}
