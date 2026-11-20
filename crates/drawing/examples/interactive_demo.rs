//! Interactive demo for smearor-commons-drawing.
//!
//! Demonstrates all major rendering capabilities of the library, rendered
//! by **both backends** side by side when built with `--features cairo`:
//! the raster `PixelBuffer` backend and the `CairoTarget` vector backend
//! (`ImageSurface` → readback → unpremultiply → `MemoryTexture`).
//!
//! - `fill_background` — solid color fill
//! - `draw_icon` — icon rendering by name
//! - `draw_icon_placeholder` — fallback placeholder
//! - `draw_codepoint` — icon rendering by codepoint
//! - `draw_icon_grid` — multi-icon grid layout
//! - `draw_progress_bar` — progress bar rendering
//! - `draw_label` — bottom label text
//! - `draw_text_centered` — centered text at arbitrary position
//!
//! Each scene is a generic `fn scene_*<T: DrawOps>(t: &mut T)` — a single
//! source of truth exercised through both targets.
//!
//! Run with:
//! ```sh
//! cargo run --example interactive_demo -p smearor-commons-drawing
//! cargo run --example interactive_demo -p smearor-commons-drawing --features cairo
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

#[cfg(feature = "cairo")]
use smearor_commons_buffer::BgraPremultiplied;
#[cfg(feature = "cairo")]
use smearor_commons_buffer::PixelFormat;
#[cfg(feature = "cairo")]
use smearor_commons_buffer::Rgba;
#[cfg(feature = "cairo")]
use smearor_commons_drawing::CairoTarget;
#[cfg(feature = "cairo")]
use smearor_commons_drawing::pixel_buffer_from_surface;

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
        .label(if cfg!(feature = "cairo") {
            "Same scene, two backends — left: raster PixelBuffer, right: CairoTarget (ImageSurface → readback → unpremultiply)."
        } else {
            "Software rendering onto RGBA pixel buffers. Build with --features cairo for a side-by-side backend comparison."
        })
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
// Backend helpers: render the same scene through both targets
// ---------------------------------------------------------------------------

/// Render `draw` onto a fresh `PixelBuffer` and upload it as a texture.
/// `E` is `Infallible` for the raster backend.
fn raster_texture<E: std::fmt::Debug>(size: Size<u32>, draw: impl FnOnce(&mut PixelBuffer) -> Result<(), E>) -> gtk4::gdk::MemoryTexture {
    let mut buf = PixelBuffer::new(size);
    draw(&mut buf).unwrap();
    pixels_to_texture(&buf)
}

/// The same scene through the cairo backend: `CairoTarget` on an
/// `ImageSurface`, read back via `pixel_buffer_from_surface`, then
/// unpremultiplied BGRA → straight RGBA through the `PixelFormat`
/// channel interface — exercising the `ViewSurface` path end to end.
#[cfg(feature = "cairo")]
fn cairo_texture<E: std::fmt::Debug>(size: Size<u32>, draw: impl FnOnce(&mut CairoTarget) -> Result<(), E>) -> gtk4::gdk::MemoryTexture {
    let mut surface = cairo::ImageSurface::create(cairo::Format::ARgb32, size.x as i32, size.y as i32).expect("image surface");
    {
        let ctx = cairo::Context::new(&surface).expect("context");
        let mut target = CairoTarget::new(&ctx, size);
        draw(&mut target).unwrap();
    }
    let buf = pixel_buffer_from_surface(&mut surface).expect("surface readback");
    pixels_to_texture(&unpremultiply(&buf))
}

/// Convert a premultiplied BGRA buffer to straight-alpha RGBA via the
/// `PixelFormat` read/write channel interface.
#[cfg(feature = "cairo")]
fn unpremultiply(src: &PixelBuffer<BgraPremultiplied>) -> PixelBuffer<Rgba> {
    let mut dst = PixelBuffer::<Rgba>::new(src.size());
    for (s, d) in src.pixels().as_chunks::<4>().0.iter().zip(dst.pixels_mut().as_chunks_mut::<4>().0.iter_mut()) {
        let [r, g, b, a] = BgraPremultiplied::read_pixel(s);
        Rgba::write_pixel(d, r, g, b, a);
    }
    dst
}

/// One backend column: a labeled image.
fn backend_column(label: &str, texture: &gtk4::gdk::MemoryTexture, size: u32) -> Box {
    let column = Box::builder().orientation(Orientation::Vertical).spacing(4).halign(Align::Center).build();
    column.append(&make_image(texture, size));
    column.append(&Label::new(Some(label)));
    column
}

/// The two backend columns side by side; without the `cairo` feature
/// only the raster column is shown.
fn backend_row(raster: &gtk4::gdk::MemoryTexture, size: u32) -> Box {
    let row = Box::builder().orientation(Orientation::Horizontal).spacing(24).halign(Align::Center).build();
    row.append(&backend_column("raster", raster, size));
    row
}

/// With `cairo`: append the second column.
#[cfg(feature = "cairo")]
fn backend_row_with_cairo(raster: &gtk4::gdk::MemoryTexture, cairo: &gtk4::gdk::MemoryTexture, size: u32) -> Box {
    let row = backend_row(raster, size);
    row.append(&backend_column("cairo", cairo, size));
    row
}

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

/// Shared section scaffold: vertical box + header + description + Frame.
fn section_frame(title: &str, header: &str, desc: &str) -> (Box, Frame) {
    let section = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(8)
        .margin_top(8)
        .margin_bottom(8)
        .margin_start(8)
        .margin_end(8)
        .build();

    let header_label = Label::builder().label(header).css_classes(["title-3"]).halign(Align::Start).build();
    section.append(&header_label);

    let desc_label = Label::builder().label(desc).halign(Align::Start).wrap(true).build();
    section.append(&desc_label);

    let frame = Frame::builder().label(title).child(&section).build();
    (section, frame)
}

// ---------------------------------------------------------------------------
// Scenes — generic over the drawing target, exercised through both backends
// ---------------------------------------------------------------------------

fn scene_fill_background<T: DrawOps>(t: &mut T) -> Result<(), T::Error> {
    t.fill_background(Color::<f64>::new(0.2, 0.5, 0.8))?;
    Ok(())
}

fn scene_icon<T: DrawOps>(t: &mut T, icon_name: &str) -> Result<(), T::Error> {
    t.fill_background(Color::<f64>::new(30.0 / 255.0, 30.0 / 255.0, 30.0 / 255.0))?;
    t.draw_icon(Icon::new(nerd_font(), icon_name, true, &resolve_codepoint, Some(Color::<f64>::new(0.4, 0.7, 1.0))))?;
    t.draw_label(label_font().or_else(nerd_font), &icon_name.replace("nf-", ""), false, None)?;
    Ok(())
}

fn scene_placeholder<T: DrawOps>(t: &mut T) -> Result<(), T::Error> {
    t.fill_background(Color::<f64>::new(30.0 / 255.0, 30.0 / 255.0, 30.0 / 255.0))?;
    t.draw_icon_placeholder(false, None)?;
    Ok(())
}

fn scene_codepoint<T: DrawOps>(t: &mut T, size: Size<u32>) -> Result<(), T::Error> {
    t.fill_background(Color::<f64>::new(25.0 / 255.0, 25.0 / 255.0, 35.0 / 255.0))?;
    if let Some(font) = nerd_font() {
        t.draw_codepoint(Codepoint::new(
            font,
            '\u{F11B}',
            Vec2::<Pixels, f32>::new(size.x as f32 / 2.0, size.y as f32 / 2.0),
            Pixels::new(48.0),
            Color::<u8>::new(100, 180, 255),
        ))?;
    }
    Ok(())
}

fn scene_icon_grid<T: DrawOps>(t: &mut T) -> Result<(), T::Error> {
    t.fill_background(Color::<f64>::new(20.0 / 255.0, 20.0 / 255.0, 25.0 / 255.0))?;
    if let Some(font) = nerd_font() {
        t.draw_icon_grid(IconGrid::new(font, GALLERY_ICONS, Grid::for_items(GALLERY_ICONS.len() as u32, 2), true, &resolve_codepoint))?;
    }
    Ok(())
}

fn scene_progress<T: DrawOps>(t: &mut T, value: Fraction) -> Result<(), T::Error> {
    t.fill_background(Color::<f64>::new(20.0 / 255.0, 20.0 / 255.0, 20.0 / 255.0))?;
    t.draw_progress_bar(ProgressBar::new(value, Color::<f64>::new(0.4, 0.7, 1.0), Pixels::new(4.0)))?;
    Ok(())
}

fn scene_label<T: DrawOps>(t: &mut T) -> Result<(), T::Error> {
    t.fill_background(Color::<f64>::new(30.0 / 255.0, 30.0 / 255.0, 30.0 / 255.0))?;
    t.draw_icon(Icon::new(nerd_font(), "nf-fa-star", true, &resolve_codepoint, Some(Color::<f64>::new(1.0, 0.8, 0.2))))?;
    t.draw_label(label_font().or_else(nerd_font), "star", true, None)?;
    Ok(())
}

fn scene_centered_text<T: DrawOps>(t: &mut T, size: Size<u32>) -> Result<(), T::Error> {
    t.fill_background(Color::<f64>::new(25.0 / 255.0, 25.0 / 255.0, 35.0 / 255.0))?;
    t.draw_text_centered(
        label_font().or_else(nerd_font),
        "Centered",
        Pixels::new(size.y as f32 / 2.0),
        Pixels::new(20.0),
        Color::<u8>::new(255, 255, 255),
    )?;
    t.draw_text_centered(
        label_font().or_else(nerd_font),
        "Text",
        Pixels::new(size.y as f32 / 2.0 + 24.0),
        Pixels::new(16.0),
        Color::<u8>::new(100, 180, 255),
    )?;
    Ok(())
}

fn scene_render_context<T: DrawTarget>(t: &mut T) -> Result<(), T::Error> {
    let mut ctx = RenderContext::new(t)
        .icon_font(nerd_font())
        .label_font(label_font())
        .background(Color::<f64>::new(30.0 / 255.0, 30.0 / 255.0, 30.0 / 255.0))
        .icon_color(Color::<f64>::new(0.9, 0.3, 0.4))
        .active(true);
    ctx.fill_background()?;
    ctx.draw_icon_by_name("nf-fa-heart", resolve_codepoint)?;
    ctx.draw_label("heart")?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Section 1: fill_background
// ---------------------------------------------------------------------------

fn build_fill_background_section() -> Frame {
    let (section, frame) = section_frame("fill_background", "1. fill_background", "Fills a pixel buffer with a solid color.");

    let size = Size::new(64, 64);
    let raster = raster_texture(size, scene_fill_background);
    #[cfg(feature = "cairo")]
    let row = backend_row_with_cairo(&raster, &cairo_texture(size, scene_fill_background), size.x);
    #[cfg(not(feature = "cairo"))]
    let row = backend_row(&raster, size.x);
    section.append(&row);

    frame
}

// ---------------------------------------------------------------------------
// Section 2: draw_icon + draw_icon_placeholder
// ---------------------------------------------------------------------------

fn build_icon_rendering_section() -> Frame {
    let (section, frame) = section_frame(
        "Icon Rendering",
        "2. draw_icon & draw_icon_placeholder",
        "Renders font icons by name. Unknown icons fall back to a circular placeholder.",
    );

    let grid = gtk4::Grid::builder()
        .column_spacing(12)
        .row_spacing(12)
        .halign(Align::Center)
        .margin_top(8)
        .margin_bottom(8)
        .build();

    let cols = 4u32;
    let icon_size = Size::new(48, 48);

    for (i, icon_name) in GALLERY_ICONS.iter().enumerate() {
        let raster = raster_texture(icon_size, |t| scene_icon(t, icon_name));
        #[cfg(feature = "cairo")]
        let pair = backend_row_with_cairo(&raster, &cairo_texture(icon_size, |t| scene_icon(t, icon_name)), icon_size.x);
        #[cfg(not(feature = "cairo"))]
        let pair = backend_row(&raster, icon_size.x);

        let col = (i as u32) % cols;
        let row = (i as u32) / cols;
        grid.attach(&pair, col as i32, row as i32, 1, 1);
    }

    let raster = raster_texture(icon_size, scene_placeholder);
    #[cfg(feature = "cairo")]
    let pair = backend_row_with_cairo(&raster, &cairo_texture(icon_size, scene_placeholder), icon_size.x);
    #[cfg(not(feature = "cairo"))]
    let pair = backend_row(&raster, icon_size.x);
    grid.attach(&pair, cols as i32, 0, 1, 1);

    section.append(&grid);
    frame
}

// ---------------------------------------------------------------------------
// Section 3: draw_codepoint
// ---------------------------------------------------------------------------

fn build_codepoint_section() -> Frame {
    let (section, frame) = section_frame(
        "Codepoint Rendering",
        "3. draw_codepoint",
        "Renders a font icon by its Unicode codepoint at a custom position and size.",
    );

    let size = Size::new(96, 96);
    let raster = raster_texture(size, |t| scene_codepoint(t, size));
    #[cfg(feature = "cairo")]
    let row = backend_row_with_cairo(&raster, &cairo_texture(size, |t| scene_codepoint(t, size)), size.x);
    #[cfg(not(feature = "cairo"))]
    let row = backend_row(&raster, size.x);
    section.append(&row);

    frame
}

// ---------------------------------------------------------------------------
// Section 4: draw_icon_grid
// ---------------------------------------------------------------------------

fn build_icon_grid_section() -> Frame {
    let (section, frame) = section_frame("Icon Grid", "4. draw_icon_grid", "Renders a grid of icons within a single pixel buffer.");

    let size = Size::new(192, 192);
    let raster = raster_texture(size, scene_icon_grid);
    #[cfg(feature = "cairo")]
    let row = backend_row_with_cairo(&raster, &cairo_texture(size, scene_icon_grid), size.x);
    #[cfg(not(feature = "cairo"))]
    let row = backend_row(&raster, size.x);
    section.append(&row);

    frame
}

// ---------------------------------------------------------------------------
// Section 5: draw_progress_bar (interactive)
// ---------------------------------------------------------------------------

fn build_progress_bar_section() -> Frame {
    let (section, frame) = section_frame("Progress Bar", "5. draw_progress_bar", "Renders a horizontal progress bar. Drag the slider to update.");

    let progress_size = Size::new(256, 32);

    let render_raster = move |value: Fraction| raster_texture(progress_size, |t| scene_progress(t, value));
    #[cfg(feature = "cairo")]
    let render_cairo = move |value: Fraction| cairo_texture(progress_size, |t| scene_progress(t, value));

    let raster_image = Image::from_paintable(Some(&render_raster(Fraction::new(0.65))));
    let raster_col = Box::builder().orientation(Orientation::Vertical).spacing(4).build();
    raster_col.append(&raster_image);
    raster_col.append(&Label::new(Some("raster")));

    let row = Box::builder().orientation(Orientation::Horizontal).spacing(24).halign(Align::Center).build();
    row.append(&raster_col);

    #[cfg(feature = "cairo")]
    let cairo_image = {
        let image = Image::from_paintable(Some(&render_cairo(Fraction::new(0.65))));
        let col = Box::builder().orientation(Orientation::Vertical).spacing(4).build();
        col.append(&image);
        col.append(&Label::new(Some("cairo")));
        row.append(&col);
        image
    };

    section.append(&row);

    let progress_slider = Scale::with_range(Orientation::Horizontal, 0.0, 1.0, 0.01);
    progress_slider.set_value(0.65);
    progress_slider.set_halign(Align::Center);
    progress_slider.set_width_request(256);
    section.append(&progress_slider);

    progress_slider.connect_value_changed(move |slider| {
        let value = Fraction::new(slider.value());
        raster_image.set_paintable(Some(&render_raster(value)));
        #[cfg(feature = "cairo")]
        cairo_image.set_paintable(Some(&render_cairo(value)));
    });

    frame
}

// ---------------------------------------------------------------------------
// Section 6: draw_label & draw_text_centered
// ---------------------------------------------------------------------------

fn build_text_section() -> Frame {
    let (section, frame) = section_frame(
        "Text Rendering",
        "6. draw_label & draw_text_centered",
        "Renders text labels at the bottom of an image and centered text at arbitrary positions.",
    );

    let row = Box::builder().orientation(Orientation::Horizontal).spacing(24).halign(Align::Center).build();

    let label_size = Size::new(96, 96);
    let raster = raster_texture(label_size, scene_label);
    #[cfg(feature = "cairo")]
    row.append(&backend_row_with_cairo(&raster, &cairo_texture(label_size, scene_label), label_size.x));
    #[cfg(not(feature = "cairo"))]
    row.append(&backend_row(&raster, label_size.x));

    let text_size = Size::new(128, 128);
    let raster = raster_texture(text_size, |t| scene_centered_text(t, text_size));
    #[cfg(feature = "cairo")]
    row.append(&backend_row_with_cairo(
        &raster,
        &cairo_texture(text_size, |t| scene_centered_text(t, text_size)),
        text_size.x,
    ));
    #[cfg(not(feature = "cairo"))]
    row.append(&backend_row(&raster, text_size.x));

    section.append(&row);
    frame
}

// ---------------------------------------------------------------------------
// Section 7: RenderContext (builder pattern)
// ---------------------------------------------------------------------------

fn build_render_context_section() -> Frame {
    let (section, frame) = section_frame(
        "RenderContext",
        "7. RenderContext (builder pattern)",
        "Combines fill_background, draw_icon, and draw_label into a single builder-pattern call.",
    );

    let size = Size::new(96, 96);
    let raster = raster_texture(size, scene_render_context);
    #[cfg(feature = "cairo")]
    let row = backend_row_with_cairo(&raster, &cairo_texture(size, scene_render_context), size.x);
    #[cfg(not(feature = "cairo"))]
    let row = backend_row(&raster, size.x);
    section.append(&row);

    frame
}
