//! GTK example: renders the PANTONE solid-coated formula guide as a
//! fan deck - the ~1.300 coated chips fanned out like the physical
//! fan book.
//!
//! Hovering (or clicking) a chip selects it; the side panel shows the
//! name, sRGB and CMYK values and the uncoated counterpart for the
//! same formula. The dropdown switches the chip order between hue,
//! catalog number and name.
//!
//! Run with `cargo run -p smearor-commons-color --features gtk4 --example fan`.

use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Box;
use gtk4::DrawingArea;
use gtk4::DropDown;
use gtk4::EventControllerMotion;
use gtk4::GestureClick;
use gtk4::Label;
use gtk4::Orientation;
use gtk4::StringList;
use gtk4::cairo::Context;
use gtk4::cairo::FontSlant;
use gtk4::cairo::FontWeight;
use gtk4::glib;
use gtk4::prelude::*;
use smearor_commons_color::Color;
use smearor_commons_color::PantoneNamedColor;
use smearor_commons_color::PantoneSuffix;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;
use std::str::FromStr;

const APP_ID: &str = "io.smearor.commons_color.fan";

/// Chips stacked radially on a single fan blade.
const CHIPS_PER_BLADE: usize = 12;
/// Total opening angle of the fan.
const FAN_SWEEP: f64 = std::f64::consts::TAU * 140.0 / 360.0;
/// The radius the fan pivot ring keeps free of chips, relative to the
/// outer radius.
const HUB_FRACTION: f64 = 0.16;
/// Gap left between neighbouring blades, as a fraction of the blade
/// angle.
const BLADE_GAP: f64 = 0.06;
/// Gap left between stacked chips, as a fraction of the chip depth.
const CHIP_GAP: f64 = 0.05;

/// One chip of the fan: a coated PANTONE color plus its position in
/// the catalog.
struct FanChip {
    /// The catalog position inside [`PantoneNamedColor::ALL`].
    catalog_index: usize,
    /// The named color.
    named: PantoneNamedColor,
    /// The sRGB value.
    rgb: Color<u8>,
    /// The HSV hue in degrees (`0.0` for achromatic chips).
    hue: f64,
    /// The HSV saturation.
    saturation: f64,
    /// The perceived lightness, used to order achromatic chips.
    lightness: f64,
}

/// The sort orders offered by the dropdown.
#[derive(Clone, Copy)]
enum FanOrder {
    /// Chromatic chips by hue; achromatic chips last, by lightness.
    Hue,
    /// The catalog order of [`PantoneNamedColor::ALL`].
    Number,
    /// Alphabetic by the `Pantone <name> C` string.
    Name,
}

/// Shared example state.
struct FanState {
    /// All coated chips in the currently selected order.
    chips: Vec<FanChip>,
    /// The chip under the pointer, if any.
    hovered: Option<usize>,
}

/// The geometry of the fan inside the drawing area.
#[derive(Clone, Copy)]
struct FanLayout {
    /// X coordinate of the fan pivot.
    pivot_x: f64,
    /// Y coordinate of the fan pivot.
    pivot_y: f64,
    /// Radius of the first chip ring.
    inner_radius: f64,
    /// Depth of a single chip (outer minus inner radius per ring).
    chip_depth: f64,
    /// Angular width of one blade.
    blade_angle: f64,
    /// Number of fan blades.
    blades: usize,
}

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

/// Computes hue, saturation and lightness of a chip for sorting.
fn fan_chip(catalog_index: usize, named: PantoneNamedColor) -> FanChip {
    let rgb = named.to_color();
    let (r, g, b) = (f64::from(rgb.r) / 255.0, f64::from(rgb.g) / 255.0, f64::from(rgb.b) / 255.0);
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;
    let saturation = if max > 0.0 { delta / max } else { 0.0 };
    let hue = if delta > 0.0 {
        let sector = if max == r {
            (g - b) / delta
        } else if max == g {
            2.0 + (b - r) / delta
        } else {
            4.0 + (r - g) / delta
        };
        (sector * 60.0).rem_euclid(360.0)
    } else {
        0.0
    };
    let lightness = 0.2126 * r + 0.7152 * g + 0.0722 * b;
    FanChip {
        catalog_index,
        named,
        rgb,
        hue,
        saturation,
        lightness,
    }
}

/// All coated chips of the catalog.
fn coated_chips() -> Vec<FanChip> {
    PantoneNamedColor::ALL
        .iter()
        .enumerate()
        .filter(|(_, named)| named.suffix() == PantoneSuffix::Coated)
        .map(|(index, named)| fan_chip(index, *named))
        .collect()
}

/// Applies a sort order to the chip list.
fn sort_chips(chips: &mut [FanChip], order: FanOrder) {
    match order {
        FanOrder::Hue => chips.sort_by(|a, b| {
            let a_achromatic = a.saturation < 0.08;
            let b_achromatic = b.saturation < 0.08;
            match (a_achromatic, b_achromatic) {
                (true, false) => Ordering::Greater,
                (false, true) => Ordering::Less,
                (true, true) => b.lightness.total_cmp(&a.lightness),
                (false, false) => a.hue.total_cmp(&b.hue).then(a.lightness.total_cmp(&b.lightness)),
            }
        }),
        FanOrder::Number => chips.sort_by_key(|chip| chip.catalog_index),
        FanOrder::Name => chips.sort_by(|a, b| a.named.as_str().cmp(b.named.as_str())),
    }
}

/// The fan geometry for a drawing area of the given size.
fn fan_layout(width: f64, height: f64, blades: usize) -> FanLayout {
    let margin = 24.0;
    let half = FAN_SWEEP / 2.0;
    let outer = ((width - 2.0 * margin) / (2.0 * half.sin())).min(height - 3.0 * margin).max(1.0);
    let inner = outer * HUB_FRACTION;
    FanLayout {
        pivot_x: width / 2.0,
        pivot_y: height - margin,
        inner_radius: inner,
        chip_depth: (outer - inner) / CHIPS_PER_BLADE as f64,
        blade_angle: FAN_SWEEP / blades.max(1) as f64,
        blades,
    }
}

/// The angular center of blade `index`.
fn blade_center(layout: FanLayout, index: usize) -> f64 {
    -FAN_SWEEP / 2.0 + (index as f64 + 0.5) * layout.blade_angle
}

/// A point of the fan: `angle` from the upward vertical, `radius` from
/// the pivot.
fn fan_point(layout: FanLayout, angle: f64, radius: f64) -> (f64, f64) {
    (layout.pivot_x + radius * angle.sin(), layout.pivot_y - radius * angle.cos())
}

/// Traces the arc segment of chip `index` into the context path.
fn chip_path(context: &Context, layout: FanLayout, index: usize) {
    let blade = index / CHIPS_PER_BLADE;
    let slot = index % CHIPS_PER_BLADE;
    let center = blade_center(layout, blade);
    let half = layout.blade_angle * (1.0 - BLADE_GAP) / 2.0;
    let (a0, a1) = (center - half, center + half);
    let r0 = layout.inner_radius + slot as f64 * layout.chip_depth + layout.chip_depth * CHIP_GAP / 2.0;
    let r1 = layout.inner_radius + (slot + 1) as f64 * layout.chip_depth - layout.chip_depth * CHIP_GAP / 2.0;
    const STEPS: usize = 6;
    for step in 0..=STEPS {
        let angle = a0 + (a1 - a0) * step as f64 / STEPS as f64;
        let (x, y) = fan_point(layout, angle, r1);
        if step == 0 { context.move_to(x, y) } else { context.line_to(x, y) }
    }
    for step in (0..=STEPS).rev() {
        let angle = a0 + (a1 - a0) * step as f64 / STEPS as f64;
        let (x, y) = fan_point(layout, angle, r0);
        context.line_to(x, y);
    }
    context.close_path();
}

/// The chip index at widget coordinates `(x, y)`, if any.
fn chip_at(layout: FanLayout, chips: &[FanChip], x: f64, y: f64) -> Option<usize> {
    let dx = x - layout.pivot_x;
    let dy = y - layout.pivot_y;
    let radius = dx.hypot(dy);
    let angle = dx.atan2(-dy);
    let half = FAN_SWEEP / 2.0;
    if !(-half..=half).contains(&angle) {
        return None;
    }
    let blade = ((angle + half) / layout.blade_angle) as usize;
    let slot = ((radius - layout.inner_radius) / layout.chip_depth) as usize;
    if blade >= layout.blades || slot >= CHIPS_PER_BLADE || radius < layout.inner_radius {
        return None;
    }
    let index = blade * CHIPS_PER_BLADE + slot;
    (index < chips.len()).then_some(index)
}

/// The uncoated counterpart (`Pantone 485 U`) of a coated chip, if the
/// catalog contains it.
fn uncoated_counterpart(named: PantoneNamedColor) -> Option<PantoneNamedColor> {
    let name = named.as_str().strip_suffix(" C")?;
    PantoneNamedColor::from_str(&format!("{name} U")).ok()
}

fn build_ui(app: &Application) {
    let state = Rc::new(RefCell::new(FanState {
        chips: coated_chips(),
        hovered: None,
    }));
    sort_chips(&mut state.borrow_mut().chips, FanOrder::Hue);

    let detail_name = Label::new(Some("Pantone fan"));
    detail_name.add_css_class("title-2");
    detail_name.set_selectable(true);
    detail_name.set_wrap(true);
    let detail_hex = Label::new(Some("#------"));
    detail_hex.set_selectable(true);
    let detail_rgb = Label::new(Some("RGB ---/---/---"));
    detail_rgb.set_selectable(true);
    let detail_cmyk = Label::new(Some("CMYK -/-/-/-"));
    detail_cmyk.set_selectable(true);
    let uncoated_preview = DrawingArea::builder().content_height(48).build();
    let uncoated_name = Label::new(Some("Uncoated counterpart: -"));
    uncoated_name.set_wrap(true);
    uncoated_name.set_selectable(true);

    let preview = DrawingArea::builder().content_height(140).build();
    {
        let state = Rc::clone(&state);
        preview.set_draw_func(move |_, context, width, height| {
            let state = state.borrow();
            if let Some(chip) = state.hovered.and_then(|index| state.chips.get(index)) {
                let color = &chip.rgb;
                context.set_source_rgb(color.r as f64 / 255.0, color.g as f64 / 255.0, color.b as f64 / 255.0);
                context.rectangle(0.0, 0.0, width as f64, height as f64);
                let _ = context.fill();
            }
        });
    }
    {
        let state = Rc::clone(&state);
        uncoated_preview.set_draw_func(move |_, context, width, height| {
            let uncoated = state
                .borrow()
                .hovered
                .and_then(|index| state.borrow().chips.get(index).map(|chip| chip.named))
                .and_then(uncoated_counterpart);
            if let Some(named) = uncoated {
                let color = named.to_color();
                context.set_source_rgb(color.r as f64 / 255.0, color.g as f64 / 255.0, color.b as f64 / 255.0);
                context.rectangle(0.0, 0.0, width as f64, height as f64);
                let _ = context.fill();
            }
        });
    }

    let detail = Box::new(Orientation::Vertical, 6);
    detail.set_margin_top(12);
    detail.set_margin_bottom(12);
    detail.set_margin_start(12);
    detail.set_margin_end(12);
    detail.set_width_request(220);
    detail.append(&preview);
    detail.append(&detail_name);
    detail.append(&detail_hex);
    detail.append(&detail_rgb);
    detail.append(&detail_cmyk);
    detail.append(&uncoated_name);
    detail.append(&uncoated_preview);

    let canvas = DrawingArea::builder().hexpand(true).vexpand(true).build();
    {
        let state = Rc::clone(&state);
        canvas.set_draw_func(move |_, context, width, height| {
            let (chips_len, hovered) = {
                let state = state.borrow();
                (state.chips.len(), state.hovered)
            };
            let blades = chips_len.div_ceil(CHIPS_PER_BLADE);
            let layout = fan_layout(width as f64, height as f64, blades);
            {
                let state = state.borrow();
                for (index, chip) in state.chips.iter().enumerate() {
                    chip_path(context, layout, index);
                    let color = &chip.rgb;
                    context.set_source_rgb(color.r as f64 / 255.0, color.g as f64 / 255.0, color.b as f64 / 255.0);
                    let _ = context.fill();
                }
            }
            if let Some(index) = hovered.filter(|index| *index < chips_len) {
                chip_path(context, layout, index);
                context.set_source_rgba(0.0, 0.0, 0.0, 0.85);
                context.set_line_width(2.0);
                let _ = context.stroke();
            }
            context.arc(layout.pivot_x, layout.pivot_y, layout.inner_radius * 0.55, 0.0, std::f64::consts::TAU);
            context.set_source_rgb(0.93, 0.93, 0.93);
            let _ = context.fill_preserve();
            context.set_source_rgb(0.3, 0.3, 0.3);
            context.set_line_width(1.5);
            let _ = context.stroke();
            let state = state.borrow();
            if let Some(name) = hovered.and_then(|index| state.chips.get(index)).map(|chip| chip.named.as_str()) {
                context.select_font_face("Sans", FontSlant::Normal, FontWeight::Bold);
                context.set_font_size(16.0);
                context.set_source_rgb(0.2, 0.2, 0.2);
                context.move_to(16.0, 28.0);
                let _ = context.show_text(name);
            }
        });
    }

    let update_detail = {
        let detail_name = detail_name.clone();
        let detail_hex = detail_hex.clone();
        let detail_rgb = detail_rgb.clone();
        let detail_cmyk = detail_cmyk.clone();
        let uncoated_name = uncoated_name.clone();
        let preview = preview.clone();
        let uncoated_preview = uncoated_preview.clone();
        let state = Rc::clone(&state);
        move || {
            let state = state.borrow();
            let Some(chip) = state.hovered.and_then(|index| state.chips.get(index)) else {
                return;
            };
            let color = &chip.rgb;
            detail_name.set_text(chip.named.as_str());
            detail_hex.set_text(&format!("#{:02X}{:02X}{:02X}", color.r, color.g, color.b));
            detail_rgb.set_text(&format!("RGB {}/{}/{}", color.r, color.g, color.b));
            match chip.named.to_cmyk() {
                Some(cmyk) => detail_cmyk.set_text(&format!("CMYK {}% {}% {}% {}%", cmyk.c, cmyk.m, cmyk.y, cmyk.k)),
                None => detail_cmyk.set_text("CMYK -/-/-/-"),
            }
            match uncoated_counterpart(chip.named) {
                Some(named) => uncoated_name.set_text(&format!("Uncoated: {}", named.as_str())),
                None => uncoated_name.set_text("Uncoated counterpart: -"),
            }
            preview.queue_draw();
            uncoated_preview.queue_draw();
        }
    };

    let pick = {
        let state = Rc::clone(&state);
        let canvas = canvas.clone();
        move |x: f64, y: f64| {
            let changed = {
                let mut state = state.borrow_mut();
                let layout = fan_layout(canvas.width() as f64, canvas.height() as f64, state.chips.len().div_ceil(CHIPS_PER_BLADE));
                let index = chip_at(layout, &state.chips, x, y);
                let changed = state.hovered != index;
                state.hovered = index;
                changed
            };
            if changed {
                canvas.queue_draw();
                update_detail();
            }
        }
    };
    {
        let motion = EventControllerMotion::new();
        let pick = pick.clone();
        motion.connect_motion(move |_, x, y| pick(x, y));
        canvas.add_controller(motion);
    }
    {
        let click = GestureClick::new();
        click.connect_released(move |_, _, x, y| pick(x, y));
        canvas.add_controller(click);
    }

    let sort_model = StringList::new(&["Hue", "Number", "Name"]);
    let sort_switcher = DropDown::builder().model(&sort_model).selected(0).build();
    {
        let state = Rc::clone(&state);
        let canvas_clone = canvas.clone();
        sort_switcher.connect_selected_notify(move |dropdown| {
            let order = match dropdown.selected() {
                1 => FanOrder::Number,
                2 => FanOrder::Name,
                _ => FanOrder::Hue,
            };
            let mut state = state.borrow_mut();
            state.hovered = None;
            sort_chips(&mut state.chips, order);
            canvas_clone.queue_draw();
        });
    }

    let sidebar = Box::new(Orientation::Vertical, 8);
    sidebar.append(&sort_switcher);
    sidebar.append(&detail);

    let content = Box::new(Orientation::Horizontal, 0);
    content.append(&canvas);
    content.append(&sidebar);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("PANTONE Solid Coated fan")
        .default_width(1100)
        .default_height(800)
        .child(&content)
        .build();
    window.present();
}
