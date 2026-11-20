//! Interactive demo for smearor-commons-drawing.
//!
//! Demonstrates all major rendering capabilities of the library:
//! - `fill_background` — solid color fill
//! - `draw_icon` — icon rendering by name
//! - `draw_icon_placeholder` — fallback placeholder
//! - `draw_codepoint` — icon rendering by codepoint
//! - `draw_icon_grid` — multi-icon grid layout
//! - `draw_progress_bar` — progress bar rendering
//! - `draw_label` — bottom label text
//! - `draw_text_centered` — centered text at arbitrary position
//!
//! Run with:
//! ```sh
//! cargo run --example interactive_demo -p smearor-commons-drawing
//! ```

use miette::IntoDiagnostic;
use miette::Result;
use nerd_fonts_rs::NerdFonts;
use nerd_fonts_rs::fonts::label_font;
use nerd_fonts_rs::fonts::nerd_font;
use nerd_fonts_rs::init;
use smearor_commons_buffer::PixelBuffer;
use smearor_commons_color::Color;
use smearor_commons_drawing::RenderContext;
use smearor_commons_drawing::prelude::*;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Grid;
use smearor_commons_geometry::Pixels;
use smearor_commons_geometry::Size;
use smearor_commons_geometry::Vec2;

use gtk4::Align;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Box;
use gtk4::Frame;
use gtk4::Image;
use gtk4::Label;
use gtk4::Orientation;
use gtk4::Scale;
use gtk4::glib;
use gtk4::prelude::*;

const APP_ID: &str = "io.smearor.commons_drawing.interactive_demo";

/// A curated selection of Nerd Font icons to display.
const GALLERY_ICONS: &[&str] = &[
    "nf-fa-gamepad",
    "nf-md-cube",
    "nf-linux-tux",
    "nf-fa-star",
    "nf-md-cog",
    "nf-fa-heart",
    "nf-md-battery",
    "nf-fa-music",
];

fn main() -> Result<glib::ExitCode> {
    gtk4::init().into_diagnostic()?;
    init().into_diagnostic()?;

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    Ok(app.run())
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("smearor-commons-drawing Interactive Demo")
        .default_width(900)
        .default_height(900)
        .build();

    window.connect_close_request({
        let app = app.clone();
        move |_| {
            app.quit();
            glib::Propagation::Proceed
        }
    });

    let main_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let title = Label::builder()
        .label("smearor-commons-drawing Interactive Demo")
        .css_classes(["title-1"])
        .halign(Align::Center)
        .build();
    main_box.append(&title);

    let hint = Label::builder()
        .label("Software rendering of icons, text, and progress bars onto raw RGBA pixel buffers.")
        .halign(Align::Center)
        .css_classes(["dim-label"])
        .build();
    main_box.append(&hint);

    main_box.append(&build_fill_background_section());
    main_box.append(&build_icon_rendering_section());
    main_box.append(&build_codepoint_section());
    main_box.append(&build_icon_grid_section());
    main_box.append(&build_progress_bar_section());
    main_box.append(&build_text_section());
    main_box.append(&build_render_context_section());

    let scrolled = gtk4::ScrolledWindow::builder().child(&main_box).hexpand(true).vexpand(true).build();

    window.set_child(Some(&scrolled));
    window.present();
}

// ---------------------------------------------------------------------------
// Helper: resolve icon name to codepoint
// ---------------------------------------------------------------------------

fn resolve_codepoint(name: &str) -> Option<char> {
    NerdFonts::codepoint(name).map(|c| c.as_char())
}

// ---------------------------------------------------------------------------
// Helper: convert pixel buffer to GTK texture
// ---------------------------------------------------------------------------

fn pixels_to_texture(buf: &PixelBuffer) -> gtk4::gdk::MemoryTexture {
    let size = buf.size();
    let bytes = glib::Bytes::from(buf.pixels());
    let rowstride = size.x * 4;
    gtk4::gdk::MemoryTexture::new(size.x as i32, size.y as i32, gtk4::gdk::MemoryFormat::R8g8b8a8, &bytes, rowstride as usize)
}

fn make_image(texture: &gtk4::gdk::MemoryTexture, size: u32) -> Image {
    let image = Image::from_paintable(Some(texture));
    image.set_halign(Align::Center);
    image.set_pixel_size(size as i32);
    image.set_width_request(size as i32);
    image.set_height_request(size as i32);
    image
}

// ---------------------------------------------------------------------------
// Section 1: fill_background
// ---------------------------------------------------------------------------

fn build_fill_background_section() -> Frame {
    let section = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(8)
        .margin_top(8)
        .margin_bottom(8)
        .margin_start(8)
        .margin_end(8)
        .build();

    let header = Label::builder()
        .label("1. fill_background")
        .css_classes(["title-3"])
        .halign(Align::Start)
        .build();
    section.append(&header);

    let desc = Label::builder()
        .label("Fills an RGBA pixel buffer with a solid color.")
        .halign(Align::Start)
        .wrap(true)
        .build();
    section.append(&desc);

    let size = Size::new(64, 64);
    let mut buf = PixelBuffer::new(size);
    buf.fill_background(Color::<f64>::new(0.2, 0.5, 0.8)).unwrap();

    let texture = pixels_to_texture(&buf);
    let image = make_image(&texture, size.x);
    section.append(&image);

    Frame::builder().label("fill_background").child(&section).build()
}

// ---------------------------------------------------------------------------
// Section 2: draw_icon + draw_icon_placeholder
// ---------------------------------------------------------------------------

fn build_icon_rendering_section() -> Frame {
    let section = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(8)
        .margin_top(8)
        .margin_bottom(8)
        .margin_start(8)
        .margin_end(8)
        .build();

    let header = Label::builder()
        .label("2. draw_icon & draw_icon_placeholder")
        .css_classes(["title-3"])
        .halign(Align::Start)
        .build();
    section.append(&header);

    let desc = Label::builder()
        .label("Renders font icons by name onto a pixel buffer. Unknown icons fall back to a circular placeholder.")
        .halign(Align::Start)
        .wrap(true)
        .build();
    section.append(&desc);

    let grid = gtk4::Grid::builder()
        .column_spacing(12)
        .row_spacing(12)
        .halign(Align::Center)
        .margin_top(8)
        .margin_bottom(8)
        .build();

    let cols = 4u32;
    let icon_size = Size::new(96, 96);

    for (i, icon_name) in GALLERY_ICONS.iter().enumerate() {
        let mut buf = PixelBuffer::new(icon_size);
        buf.fill_background(Color::<f64>::new(30.0 / 255.0, 30.0 / 255.0, 30.0 / 255.0)).unwrap();
        buf.draw_icon(Icon::new(nerd_font(), icon_name, true, &resolve_codepoint, Some(Color::<f64>::new(0.4, 0.7, 1.0))));
        buf.draw_label(label_font().or_else(nerd_font), &icon_name.replace("nf-", ""), false, None);

        let texture = pixels_to_texture(&buf);
        let image = make_image(&texture, icon_size.x);

        let col = (i as u32) % cols;
        let row = (i as u32) / cols;
        grid.attach(&image, col as i32, row as i32, 1, 1);
    }

    // Draw a placeholder for an unknown icon
    let mut placeholder_buf = PixelBuffer::new(icon_size);
    placeholder_buf
        .fill_background(Color::<f64>::new(30.0 / 255.0, 30.0 / 255.0, 30.0 / 255.0))
        .unwrap();
    placeholder_buf.draw_icon_placeholder(false, None).unwrap();

    let placeholder_texture = pixels_to_texture(&placeholder_buf);
    let placeholder_image = make_image(&placeholder_texture, icon_size.x);
    grid.attach(&placeholder_image, cols as i32, 0, 1, 1);

    section.append(&grid);

    Frame::builder().label("Icon Rendering").child(&section).build()
}

// ---------------------------------------------------------------------------
// Section 3: draw_codepoint
// ---------------------------------------------------------------------------

fn build_codepoint_section() -> Frame {
    let section = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(8)
        .margin_top(8)
        .margin_bottom(8)
        .margin_start(8)
        .margin_end(8)
        .build();

    let header = Label::builder()
        .label("3. draw_codepoint")
        .css_classes(["title-3"])
        .halign(Align::Start)
        .build();
    section.append(&header);

    let desc = Label::builder()
        .label("Renders a font icon by its Unicode codepoint at a custom position and size.")
        .halign(Align::Start)
        .wrap(true)
        .build();
    section.append(&desc);

    let size = Size::new(96, 96);
    let mut buf = PixelBuffer::new(size);
    buf.fill_background(Color::<f64>::new(25.0 / 255.0, 25.0 / 255.0, 35.0 / 255.0)).unwrap();
    if let Some(font) = nerd_font() {
        buf.draw_codepoint(Codepoint::new(
            font,
            '\u{F11B}',
            Vec2::<Pixels, f32>::new(size.x as f32 / 2.0, size.y as f32 / 2.0),
            Pixels::new(48.0),
            Color::<u8>::new(100, 180, 255),
        ))
        .unwrap();
    }

    let texture = pixels_to_texture(&buf);
    let image = make_image(&texture, size.x);
    section.append(&image);

    Frame::builder().label("Codepoint Rendering").child(&section).build()
}

// ---------------------------------------------------------------------------
// Section 4: draw_icon_grid
// ---------------------------------------------------------------------------

fn build_icon_grid_section() -> Frame {
    let section = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(8)
        .margin_top(8)
        .margin_bottom(8)
        .margin_start(8)
        .margin_end(8)
        .build();

    let header = Label::builder()
        .label("4. draw_icon_grid")
        .css_classes(["title-3"])
        .halign(Align::Start)
        .build();
    section.append(&header);

    let desc = Label::builder()
        .label("Renders a grid of icons within a single pixel buffer.")
        .halign(Align::Start)
        .wrap(true)
        .build();
    section.append(&desc);

    let size = Size::new(192, 192);
    let mut buf = PixelBuffer::new(size);
    buf.fill_background(Color::<f64>::new(20.0 / 255.0, 20.0 / 255.0, 25.0 / 255.0)).unwrap();
    if let Some(font) = nerd_font() {
        buf.draw_icon_grid(IconGrid::new(font, GALLERY_ICONS, Grid::for_items(GALLERY_ICONS.len() as u32, 2), true, &resolve_codepoint));
    }

    let texture = pixels_to_texture(&buf);
    let image = make_image(&texture, size.x);
    section.append(&image);

    Frame::builder().label("Icon Grid").child(&section).build()
}

// ---------------------------------------------------------------------------
// Section 5: draw_progress_bar (interactive)
// ---------------------------------------------------------------------------

fn build_progress_bar_section() -> Frame {
    let section = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(8)
        .margin_top(8)
        .margin_bottom(8)
        .margin_start(8)
        .margin_end(8)
        .build();

    let header = Label::builder()
        .label("5. draw_progress_bar")
        .css_classes(["title-3"])
        .halign(Align::Start)
        .build();
    section.append(&header);

    let desc = Label::builder()
        .label("Renders a horizontal progress bar. Drag the slider to update.")
        .halign(Align::Start)
        .wrap(true)
        .build();
    section.append(&desc);

    let progress_size = Size::new(256, 32);

    let render_progress = move |value: Fraction| -> gtk4::gdk::MemoryTexture {
        let mut buf = PixelBuffer::new(progress_size);
        buf.fill_background(Color::<f64>::new(20.0 / 255.0, 20.0 / 255.0, 20.0 / 255.0)).unwrap();
        buf.draw_progress_bar(ProgressBar::new(value, Color::<f64>::new(0.4, 0.7, 1.0), Pixels::new(4.0)))
            .unwrap();
        pixels_to_texture(&buf)
    };

    let progress_texture = render_progress(Fraction::new(0.65));
    let progress_image = Image::from_paintable(Some(&progress_texture));
    progress_image.set_halign(Align::Center);
    progress_image.set_pixel_size(progress_size.x as i32);
    progress_image.set_width_request(progress_size.x as i32);
    progress_image.set_height_request(progress_size.y as i32);
    section.append(&progress_image);

    let progress_slider = Scale::with_range(Orientation::Horizontal, 0.0, 1.0, 0.01);
    progress_slider.set_value(0.65);
    progress_slider.set_halign(Align::Center);
    progress_slider.set_width_request(256);
    section.append(&progress_slider);

    progress_slider.connect_value_changed(glib::clone!(
        #[weak]
        progress_image,
        move |slider| {
            let texture = render_progress(Fraction::new(slider.value()));
            progress_image.set_paintable(Some(&texture));
        }
    ));

    Frame::builder().label("Progress Bar").child(&section).build()
}

// ---------------------------------------------------------------------------
// Section 6: draw_label & draw_text_centered
// ---------------------------------------------------------------------------

fn build_text_section() -> Frame {
    let section = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(8)
        .margin_top(8)
        .margin_bottom(8)
        .margin_start(8)
        .margin_end(8)
        .build();

    let header = Label::builder()
        .label("6. draw_label & draw_text_centered")
        .css_classes(["title-3"])
        .halign(Align::Start)
        .build();
    section.append(&header);

    let desc = Label::builder()
        .label("Renders text labels at the bottom of an image and centered text at arbitrary positions.")
        .halign(Align::Start)
        .wrap(true)
        .build();
    section.append(&desc);

    let row = Box::builder().orientation(Orientation::Horizontal).spacing(12).halign(Align::Center).build();

    // draw_label
    let label_size = Size::new(96, 96);
    let mut label_buf = PixelBuffer::new(label_size);
    label_buf.fill_background(Color::<f64>::new(30.0 / 255.0, 30.0 / 255.0, 30.0 / 255.0)).unwrap();
    label_buf.draw_icon(Icon::new(nerd_font(), "nf-fa-star", true, &resolve_codepoint, Some(Color::<f64>::new(1.0, 0.8, 0.2))));
    label_buf.draw_label(label_font().or_else(nerd_font), "star", true, None);

    let label_texture = pixels_to_texture(&label_buf);
    let label_image = make_image(&label_texture, label_size.x);
    row.append(&label_image);

    // draw_text_centered
    let text_size = Size::new(128, 128);
    let mut text_buf = PixelBuffer::new(text_size);
    text_buf.fill_background(Color::<f64>::new(25.0 / 255.0, 25.0 / 255.0, 35.0 / 255.0)).unwrap();
    text_buf
        .draw_text_centered(
            label_font().or_else(nerd_font),
            "Centered",
            Pixels::new(text_size.y as f32 / 2.0),
            Pixels::new(20.0),
            Color::<u8>::new(255, 255, 255),
        )
        .unwrap();
    text_buf
        .draw_text_centered(
            label_font().or_else(nerd_font),
            "Text",
            Pixels::new(text_size.y as f32 / 2.0 + 24.0),
            Pixels::new(16.0),
            Color::<u8>::new(100, 180, 255),
        )
        .unwrap();

    let text_texture = pixels_to_texture(&text_buf);
    let text_image = make_image(&text_texture, text_size.x);
    row.append(&text_image);

    section.append(&row);

    Frame::builder().label("Text Rendering").child(&section).build()
}

// ---------------------------------------------------------------------------
// Section 7: RenderContext (builder pattern)
// ---------------------------------------------------------------------------

fn build_render_context_section() -> Frame {
    let section = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(8)
        .margin_top(8)
        .margin_bottom(8)
        .margin_start(8)
        .margin_end(8)
        .build();

    let header = Label::builder()
        .label("7. RenderContext (builder pattern)")
        .css_classes(["title-3"])
        .halign(Align::Start)
        .build();
    section.append(&header);

    let desc = Label::builder()
        .label("Combines fill_background, draw_icon, and draw_label into a single builder-pattern call.")
        .halign(Align::Start)
        .wrap(true)
        .build();
    section.append(&desc);

    let size = Size::new(96, 96);
    let mut buf = PixelBuffer::new(size);

    let mut ctx = RenderContext::new(&mut buf)
        .icon_font(nerd_font())
        .label_font(label_font())
        .background(Color::<f64>::new(30.0 / 255.0, 30.0 / 255.0, 30.0 / 255.0))
        .icon_color(Color::<f64>::new(0.9, 0.3, 0.4))
        .active(true);
    ctx.fill_background().unwrap();
    ctx.draw_icon_by_name("nf-fa-heart", resolve_codepoint).unwrap();
    ctx.draw_label("heart").unwrap();

    let texture = pixels_to_texture(&buf);
    let image = make_image(&texture, size.x);
    section.append(&image);

    Frame::builder().label("RenderContext").child(&section).build()
}
