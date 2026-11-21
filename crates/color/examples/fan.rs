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

mod common;

use common::info_bar;
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
use gtk4::glib;
use gtk4::prelude::*;
use smearor_commons_color::Alpha;
use smearor_commons_color::Color;
use smearor_commons_color::Hsv;
use smearor_commons_color::Hue;
use smearor_commons_color::Luminance;
use smearor_commons_color::PantoneNamedColor;
use smearor_commons_color::PantoneSuffix;
use smearor_commons_color::Saturation;
use smearor_commons_drawing::CairoTarget;
use smearor_commons_drawing::DrawPrimitiveOps;
use smearor_commons_drawing::DrawTextOps;
use smearor_commons_drawing::FillCircle;
use smearor_commons_drawing::FillSector;
use smearor_commons_geometry::Annulus;
use smearor_commons_geometry::Arc;
use smearor_commons_geometry::Circle;
use smearor_commons_geometry::Degrees;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Polygon;
use smearor_commons_geometry::Radians;
use smearor_commons_geometry::RotationRad;
use smearor_commons_geometry::Size;
use smearor_commons_geometry::Vec2;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::f64::consts::FRAC_PI_2;
use std::rc::Rc;

const APP_ID: &str = "io.smearor.commons_color.fan";

/// The explanation shown at the bottom of the window.
const ABOUT: &str = "The PANTONE Matching System was created in 1963 by Lawrence Herbert, a printer's employee, \
    to end the guessing game of describing color over the phone: each number names one standard printing ink. \
    The fan book (Formula Guide) is the physical interface to that vocabulary - designers pick a chip, printers mix it. \
    The C/U suffixes encode the same ink on coated vs. uncoated paper, a reminder that Pantone specifies \
    production recipes, not appearances: the swatch is the contract.";

/// Chips stacked radially on a single fan blade.
const CHIPS_PER_BLADE: usize = 12;
/// Total opening angle of the fan.
const FAN_SWEEP: Degrees = Degrees::new(140.0);
/// The radius the fan pivot ring keeps free of chips, relative to the
/// outer radius.
const HUB_FRACTION: Fraction = Fraction::new(0.16);
/// Gap left between neighbouring blades, as a fraction of the blade
/// angle.
const BLADE_GAP: Fraction = Fraction::new(0.06);
/// Gap left between stacked chips, as a fraction of the chip depth.
const CHIP_GAP: Fraction = Fraction::new(0.05);

/// One chip of the fan: a coated PANTONE color plus its position in
/// the catalog.
struct FanChip {
    /// The catalog position inside [`PantoneNamedColor::ALL`].
    catalog_index: usize,
    /// The named color.
    named: PantoneNamedColor,
    /// The sRGB value.
    rgb: Color<u8>,
    /// The HSV hue (`0 deg` for achromatic chips).
    hue: Hue,
    /// The HSV saturation.
    saturation: Saturation,
    /// The perceived lightness (Rec. 709), used to order achromatic
    /// chips.
    lightness: Luminance,
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
    /// The chip under the pointer, if any - takes precedence for
    /// the detail card.
    hovered: Option<usize>,
    /// The chip chosen by click - the detail card falls back to it
    /// when the pointer leaves the fan.
    selected: Option<usize>,
}

/// The geometry of the fan inside the drawing area.
#[derive(Clone, Copy)]
struct FanLayout {
    /// The pivot point of the fan.
    pivot: Vec2<Pixels>,
    /// Radius of the first chip ring.
    inner_radius: Pixels,
    /// Depth of a single chip (outer minus inner radius per ring).
    chip_depth: Pixels,
    /// Angular width of one blade.
    blade_angle: Radians,
    /// Number of fan blades.
    blades: usize,
}

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

/// Decomposes a catalog color for sorting: HSV hue and saturation
/// plus the Rec. 709 luminance.
fn fan_chip(catalog_index: usize, named: PantoneNamedColor) -> FanChip {
    let rgb = named.to_color();
    let hsv = Hsv::from(rgb);
    FanChip {
        catalog_index,
        named,
        rgb,
        hue: hsv.h,
        saturation: hsv.s,
        lightness: rgb.luminance_709(),
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
            let achromatic = Saturation::new(0.08);
            match (a.saturation < achromatic, b.saturation < achromatic) {
                (true, false) => Ordering::Greater,
                (false, true) => Ordering::Less,
                (true, true) => b.lightness.as_f64().total_cmp(&a.lightness.as_f64()),
                (false, false) => a
                    .hue
                    .as_f64()
                    .total_cmp(&b.hue.as_f64())
                    .then(a.lightness.as_f64().total_cmp(&b.lightness.as_f64())),
            }
        }),
        FanOrder::Number => chips.sort_by_key(|chip| chip.catalog_index),
        FanOrder::Name => chips.sort_by(|a, b| a.named.as_str().cmp(b.named.as_str())),
    }
}

impl FanLayout {
    /// The fan geometry for a drawing area of `size` pixels.
    fn new(size: Size<i32>, blades: usize) -> Self {
        let margin = 24.0;
        let half = FAN_SWEEP.to_radians().as_f64() / 2.0;
        let (width, height) = (size.x as f64, size.y as f64);
        let outer = ((width - 2.0 * margin) / (2.0 * half.sin())).min(height - 3.0 * margin).max(1.0);
        let inner = outer * HUB_FRACTION.as_f64();
        Self {
            pivot: Vec2::<Pixels>::new(width / 2.0, (height + outer) / 2.0),
            inner_radius: Pixels::new(inner),
            chip_depth: Pixels::new((outer - inner) / CHIPS_PER_BLADE as f64),
            blade_angle: FAN_SWEEP.to_radians() / blades.max(1) as f64,
            blades,
        }
    }

    /// The angular center of blade `index`.
    fn blade_center(&self, index: usize) -> Radians {
        -FAN_SWEEP.to_radians() / 2.0 + self.blade_angle * (index as f64 + 0.5)
    }

    /// A point of the fan: `angle` from the upward vertical, `radius`
    /// from the pivot.
    fn point(&self, angle: Radians, radius: Pixels) -> Vec2<Pixels> {
        let (sin, cos) = angle.sin_cos();
        self.pivot + Vec2::<Pixels>::new(radius.as_f64() * sin, -(radius.as_f64() * cos))
    }

    /// The annulus sector of chip `index`: ring radii and angular
    /// interval, as `Arc` directions (the fan convention "angle from
    /// upward vertical" becomes the `atan2` direction `a - PI/2`).
    fn chip_geometry(&self, index: usize) -> (Annulus<Pixels>, Arc) {
        let blade = index / CHIPS_PER_BLADE;
        let slot = index % CHIPS_PER_BLADE;
        let center = self.blade_center(blade);
        let half = self.blade_angle * (Fraction::ONE - BLADE_GAP) / 2.0;
        let (a0, a1) = (center - half, center + half);
        let r0 = self.inner_radius + self.chip_depth * (slot as f64 + CHIP_GAP.as_f64() / 2.0);
        let r1 = self.inner_radius + self.chip_depth * (slot as f64 + 1.0 - CHIP_GAP.as_f64() / 2.0);
        let arc = Arc::spanning(RotationRad::new(a0.as_f64() - FRAC_PI_2), RotationRad::new(a1.as_f64() - FRAC_PI_2));
        (Annulus::new(self.pivot, r0.as_f64(), r1.as_f64()), arc)
    }

    /// The closed outline of chip `index` for the hover stroke.
    fn chip_outline(&self, index: usize) -> Polygon<Pixels> {
        let blade = index / CHIPS_PER_BLADE;
        let slot = index % CHIPS_PER_BLADE;
        let center = self.blade_center(blade);
        let half = self.blade_angle * (Fraction::ONE - BLADE_GAP) / 2.0;
        let (a0, a1) = (center - half, center + half);
        let r0 = self.inner_radius + self.chip_depth * (slot as f64 + CHIP_GAP.as_f64() / 2.0);
        let r1 = self.inner_radius + self.chip_depth * (slot as f64 + 1.0 - CHIP_GAP.as_f64() / 2.0);
        const STEPS: usize = 6;
        let outer = (0..=STEPS).map(|step| self.point(a0 + (a1 - a0) * step as f64 / STEPS as f64, r1));
        let inner = (0..=STEPS).rev().map(|step| self.point(a0 + (a1 - a0) * step as f64 / STEPS as f64, r0));
        Polygon::new(outer.chain(inner).collect())
    }

    /// The chip index at `point`, if any.
    fn chip_at(&self, chips: &[FanChip], point: Vec2<Pixels>) -> Option<usize> {
        let ring = Annulus::new(
            self.pivot,
            self.inner_radius.as_f64(),
            (self.inner_radius + self.chip_depth * CHIPS_PER_BLADE as f64).as_f64(),
        );
        if !ring.contains(&point) {
            return None;
        }
        let angle = Radians::new((point.x - self.pivot.x).atan2(-(point.y - self.pivot.y)));
        let half = FAN_SWEEP.to_radians() / 2.0;
        if !(-half..=half).contains(&angle) {
            return None;
        }
        let blade_pos = (angle + half) / self.blade_angle;
        let slot_pos = (self.pivot.distance_f64(&point) - self.inner_radius.as_f64()) / self.chip_depth.as_f64();
        let blade = blade_pos as usize;
        let slot = slot_pos as usize;
        if blade >= self.blades || slot >= CHIPS_PER_BLADE {
            return None;
        }
        // A position inside a chip gap hits nothing: the gap
        // fraction on both axes is a dead zone between chips.
        let in_gap = |pos: f64, gap: Fraction| {
            let f = pos - pos.floor();
            f < gap.as_f64() / 2.0 || f > 1.0 - gap.as_f64() / 2.0
        };
        if in_gap(blade_pos, BLADE_GAP) || in_gap(slot_pos, CHIP_GAP) {
            return None;
        }
        let index = blade * CHIPS_PER_BLADE + slot;
        (index < chips.len()).then_some(index)
    }
}

fn build_ui(app: &Application) {
    let state = Rc::new(RefCell::new(FanState {
        chips: coated_chips(),
        hovered: None,
        selected: None,
    }));
    sort_chips(&mut state.borrow_mut().chips, FanOrder::Hue);

    let detail_name = Label::new(Some("Pantone fan"));
    detail_name.add_css_class("title-2");
    detail_name.set_selectable(true);
    detail_name.set_wrap(true);
    detail_name.set_max_width_chars(24);
    let detail_hex = Label::new(Some("#------"));
    detail_hex.set_selectable(true);
    let detail_rgb = Label::new(Some("RGB ---/---/---"));
    detail_rgb.set_selectable(true);
    let detail_cmyk = Label::new(Some("CMYK -/-/-/-"));
    detail_cmyk.set_selectable(true);
    let uncoated_preview = DrawingArea::builder().content_height(48).build();
    let uncoated_name = Label::new(Some("Uncoated counterpart: -"));
    uncoated_name.set_wrap(true);
    uncoated_name.set_max_width_chars(24);
    uncoated_name.set_selectable(true);

    let preview = DrawingArea::builder().content_height(140).build();
    {
        let state = Rc::clone(&state);
        preview.set_draw_func(move |_, context, width, height| {
            let state = state.borrow();
            if let Some(chip) = state.hovered.or(state.selected).and_then(|index| state.chips.get(index)) {
                let mut target = CairoTarget::new(context, Size::new(width.max(0) as u32, height.max(0) as u32));
                let _ = target.fill_background(chip.rgb);
            }
        });
    }
    {
        let state = Rc::clone(&state);
        uncoated_preview.set_draw_func(move |_, context, width, height| {
            let uncoated = state
                .borrow()
                .hovered
                .or(state.borrow().selected)
                .and_then(|index| state.borrow().chips.get(index).map(|chip| chip.named))
                .and_then(|named| named.counterpart());
            if let Some(named) = uncoated {
                let mut target = CairoTarget::new(context, Size::new(width.max(0) as u32, height.max(0) as u32));
                let _ = target.fill_background(named.to_color());
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
            let (chips_len, hovered, selected) = {
                let state = state.borrow();
                (state.chips.len(), state.hovered, state.selected)
            };
            let blades = chips_len.div_ceil(CHIPS_PER_BLADE);
            let layout = FanLayout::new(Size::new(width, height), blades);
            let mut target = CairoTarget::new(context, Size::new(width.max(0) as u32, height.max(0) as u32));
            {
                let state = state.borrow();
                for (index, chip) in state.chips.iter().enumerate() {
                    let (annulus, arc) = layout.chip_geometry(index);
                    let _ = target.fill_sector(FillSector::new(annulus, arc, chip.rgb, Alpha::OPAQUE));
                }
            }
            if let Some(index) = selected.filter(|index| *index < chips_len && Some(*index) != hovered) {
                common::stroke_polygon(context, &layout.chip_outline(index), 2.0, &Color::<u8>::new(255, 255, 255));
            }
            if let Some(index) = hovered.filter(|index| *index < chips_len) {
                common::stroke_polygon(context, &layout.chip_outline(index), 2.0, &Color::<u8>::new(0, 0, 0));
            }
            let _ = target.fill_circle(FillCircle::new(
                Circle::new(layout.pivot, layout.inner_radius.as_f64() * 0.55),
                Color::<f64>::new(0.93, 0.93, 0.93),
                Alpha::OPAQUE,
            ));
            let state = state.borrow();
            if let Some(name) = hovered.or(selected).and_then(|index| state.chips.get(index)).map(|chip| chip.named.as_str()) {
                let _ = target.draw_text(None, Vec2::<Pixels, f32>::new(16.0, 28.0), name, Pixels::new(16.0), Color::<u8>::new(51, 51, 51));
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
            let Some(chip) = state.hovered.or(state.selected).and_then(|index| state.chips.get(index)) else {
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
            match chip.named.counterpart() {
                Some(named) => uncoated_name.set_text(&format!("Uncoated: {}", named.as_str())),
                None => uncoated_name.set_text("Uncoated counterpart: -"),
            }
            preview.queue_draw();
            uncoated_preview.queue_draw();
        }
    };

    let chip_at = {
        let state = Rc::clone(&state);
        let canvas = canvas.clone();
        move |x: f64, y: f64| {
            let state = state.borrow();
            let layout = FanLayout::new(Size::new(canvas.width(), canvas.height()), state.chips.len().div_ceil(CHIPS_PER_BLADE));
            layout.chip_at(&state.chips, Vec2::<Pixels>::new(x, y))
        }
    };
    let hover = {
        let state = Rc::clone(&state);
        let canvas = canvas.clone();
        let update_detail = update_detail.clone();
        move |index: Option<usize>| {
            let changed = {
                let mut state = state.borrow_mut();
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
        let chip_at = chip_at.clone();
        let hover_motion = hover.clone();
        motion.connect_motion(move |_, x, y| hover_motion(chip_at(x, y)));
        motion.connect_leave(move |_| hover(None));
        canvas.add_controller(motion);
    }
    {
        let click = GestureClick::new();
        let state = Rc::clone(&state);
        let canvas_click = canvas.clone();
        click.connect_released(move |_, _, x, y| {
            if let Some(index) = chip_at(x, y) {
                state.borrow_mut().selected = Some(index);
                canvas_click.queue_draw();
                update_detail();
            }
        });
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
            state.selected = None;
            sort_chips(&mut state.chips, order);
            canvas_clone.queue_draw();
        });
    }

    let top_spacer = Box::new(Orientation::Vertical, 0);
    top_spacer.set_vexpand(true);
    let bottom_spacer = Box::new(Orientation::Vertical, 0);
    bottom_spacer.set_vexpand(true);
    let sidebar = Box::new(Orientation::Vertical, 8);
    sidebar.append(&sort_switcher);
    sidebar.append(&top_spacer);
    sidebar.append(&detail);
    sidebar.append(&bottom_spacer);

    let content = Box::new(Orientation::Horizontal, 0);
    content.append(&canvas);
    content.append(&sidebar);

    let outer = Box::new(Orientation::Vertical, 0);
    outer.append(&content);
    outer.append(&info_bar(ABOUT));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("PANTONE Solid Coated fan")
        .default_width(1100)
        .default_height(880)
        .child(&outer)
        .build();
    window.present();
}
