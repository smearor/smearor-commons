//! Renders the noise book's SVG figures from the crate's real output:
//! every plot shows actual field samples, so a change to the recipes
//! shows up in the documentation pictures.
//!
//! `cargo run -p smearor-commons-noise --example render_book_svg [dir]`
//! (default output dir: `book/src/noise/img`).

use smearor_commons_geometry::Vec2;
use smearor_commons_noise::Fbm;
use smearor_commons_noise::Field;
use smearor_commons_noise::Gain;
use smearor_commons_noise::Hash;
use smearor_commons_noise::Lacunarity;
use smearor_commons_noise::Octaves;
use smearor_commons_noise::Seed;
use smearor_commons_noise::Simplex2D;
use std::fmt::Write as _;
use std::fs;
use std::io;
use std::path::Path;

const BG: &str = "#0a0e16";
const PANEL: &str = "#10151f";
const STROKE: &str = "#1c2433";
const ACCENT: &str = "#00a1e4";
const TEXT: &str = "#8b98ad";
const FONT: &str = "'Source Code Pro',ui-monospace,monospace";

fn svg_document(width: u32, height: u32, body: &str) -> String {
    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {width} {height}\" font-family=\"{FONT}\">\n\
         <rect width=\"{width}\" height=\"{height}\" rx=\"14\" fill=\"{BG}\" stroke=\"{STROKE}\"/>\n{body}</svg>\n"
    )
}

/// Two-stop value ramp: dark panel -> accent -> light cyan.
fn ramp(v: f64) -> String {
    let (r, g, b) = if v < 0.65 {
        let t = v / 0.65;
        (16.0 - 16.0 * t, 21.0 + 140.0 * t, 31.0 + 197.0 * t)
    } else {
        let t = (v - 0.65) / 0.35;
        (140.0 * t, 161.0 + 59.0 * t, 228.0 + 27.0 * t)
    };
    format!("#{:02x}{:02x}{:02x}", r as u8, g as u8, b as u8)
}

/// Fills a `cols`x`rows` rect grid with `sample(u, v)` mapped through `ramp`.
fn heatmap(body: &mut String, x: u32, y: u32, cols: u32, rows: u32, cell: u32, sample: impl Fn(f64, f64) -> f64) {
    for row in 0..rows {
        for col in 0..cols {
            let v = sample(col as f64, row as f64).clamp(0.0, 1.0);
            let _ = writeln!(
                body,
                "<rect x=\"{}\" y=\"{}\" width=\"{cell}\" height=\"{cell}\" fill=\"{}\"/>",
                x + col * cell,
                y + row * cell,
                ramp(v)
            );
        }
    }
}

/// `hash11(x)` over `[0, 20)` - the deterministic jitter curve.
fn hash_curve() -> String {
    let (width, height, left, top, plot_w, plot_h) = (640, 260, 40, 50, 560, 160);
    let mut body = String::new();
    let _ = writeln!(
        body,
        "<rect x=\"{left}\" y=\"{top}\" width=\"{plot_w}\" height=\"{plot_h}\" fill=\"{PANEL}\" stroke=\"{STROKE}\"/>"
    );
    let mut points = String::from("points=\"");
    for i in 0..400 {
        let x = i as f64 / 400.0 * 20.0;
        let v: f64 = x.hash();
        let _ = write!(points, "{:.1},{:.1} ", left as f64 + x / 20.0 * plot_w as f64, top as f64 + (1.0 - v) * plot_h as f64);
    }
    let _ = writeln!(body, "<polyline {points}\" fill=\"none\" stroke=\"{ACCENT}\" stroke-width=\"1.5\"/>");
    let _ = writeln!(
        body,
        "<text x=\"320\" y=\"30\" fill=\"{TEXT}\" font-size=\"11\" letter-spacing=\"2\" text-anchor=\"middle\">HASH11(x) - DETERMINISTIC, NO TRIG</text>"
    );
    let _ = writeln!(body, "<text x=\"{left}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\">x = 0</text>", top + plot_h + 20);
    let _ = writeln!(
        body,
        "<text x=\"{}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"end\">x = 20</text>",
        left + plot_w,
        top + plot_h + 20
    );
    let _ = writeln!(body, "<text x=\"{left}\" y=\"{}\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"start\">1.0</text>", top + 8);
    svg_document(width, height, &body)
}

/// `Simplex2D` over a coarse domain - continuous smooth field.
fn simplex_field() -> String {
    let (width, height, left, top, cell, cols, rows) = (500, 340, 20, 46, 6, 76, 46);
    let mut body = String::new();
    let _ = writeln!(
        body,
        "<rect x=\"{left}\" y=\"{top}\" width=\"{}\" height=\"{}\" fill=\"{PANEL}\" stroke=\"{STROKE}\"/>",
        cols * cell,
        rows * cell
    );
    heatmap(&mut body, left, top, cols, rows, cell, |i, j| Simplex2D.at(Vec2::<()>::new(i * 0.11, j * 0.11)));
    let _ = writeln!(
        body,
        "<text x=\"250\" y=\"30\" fill=\"{TEXT}\" font-size=\"11\" letter-spacing=\"2\" text-anchor=\"middle\">SIMPLEX2D FIELD</text>"
    );
    svg_document(width, height, &body)
}

/// Cumulative fBm octaves 1..4 of `Simplex2D` - detail adds per octave.
fn fbm_octaves() -> String {
    let (width, height, panel_w, panel_h, cell) = (660, 190, 140, 100, 4);
    let mut body = String::new();
    for octave in 1..=4u32 {
        let fbm = Fbm {
            octaves: Octaves(octave),
            gain: Gain(0.5),
            lacunarity: Lacunarity(2.0),
        };
        let norm = (0..octave).map(|k| 0.5f64.powi(k as i32)).sum::<f64>();
        let x = 20 + (octave - 1) * (panel_w + 20);
        let _ = writeln!(
            body,
            "<rect x=\"{x}\" y=\"50\" width=\"{panel_w}\" height=\"{panel_h}\" fill=\"{PANEL}\" stroke=\"{STROKE}\"/>"
        );
        heatmap(&mut body, x, 50, panel_w / cell, panel_h / cell, cell, |i, j| {
            fbm.at(&Simplex2D, Vec2::<()>::new(i * 0.16, j * 0.16)) / norm
        });
        let _ = writeln!(
            body,
            "<text x=\"{}\" y=\"170\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"middle\">{octave} octave{}</text>",
            x + panel_w / 2,
            if octave == 1 { "" } else { "s" }
        );
    }
    let _ = writeln!(
        body,
        "<text x=\"330\" y=\"30\" fill=\"{TEXT}\" font-size=\"11\" letter-spacing=\"2\" text-anchor=\"middle\">FBM - CUMULATIVE OCTAVES (GAIN 0.5, LACUNARITY 2)</text>"
    );
    svg_document(width, height, &body)
}

/// Same field, two fixed seeds - `Seed::offset_2d` decorrelates.
fn seed_fields() -> String {
    let (width, height, panel_w, panel_h, cell) = (560, 250, 240, 150, 5);
    let mut body = String::new();
    for (index, seed) in [Seed(7), Seed(8)].iter().enumerate() {
        let x = 20 + index as u32 * (panel_w + 40);
        let offset = seed.offset_2d();
        let _ = writeln!(
            body,
            "<rect x=\"{x}\" y=\"50\" width=\"{panel_w}\" height=\"{panel_h}\" fill=\"{PANEL}\" stroke=\"{STROKE}\"/>"
        );
        heatmap(&mut body, x, 50, panel_w / cell, panel_h / cell, cell, |i, j| {
            Simplex2D.at(Vec2::<()>::new(i * 0.12, j * 0.12) + offset)
        });
        let _ = writeln!(
            body,
            "<text x=\"{}\" y=\"225\" fill=\"{TEXT}\" font-size=\"10\" text-anchor=\"middle\">Seed({})</text>",
            x + panel_w / 2,
            seed.0
        );
    }
    let _ = writeln!(
        body,
        "<text x=\"280\" y=\"30\" fill=\"{TEXT}\" font-size=\"11\" letter-spacing=\"2\" text-anchor=\"middle\">SEED OFFSETS DECORRELATE THE FIELD</text>"
    );
    svg_document(width, height, &body)
}

fn main() -> io::Result<()> {
    let dir = std::env::args().nth(1).unwrap_or_else(|| "book/src/noise/img".to_string());
    fs::create_dir_all(&dir)?;
    for (name, svg) in [
        ("hash_curve.svg", hash_curve()),
        ("simplex_field.svg", simplex_field()),
        ("fbm_octaves.svg", fbm_octaves()),
        ("seed_fields.svg", seed_fields()),
    ] {
        fs::write(Path::new(&dir).join(name), svg)?;
        println!("wrote {dir}/{name}");
    }
    Ok(())
}
