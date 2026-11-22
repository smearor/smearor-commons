//! Renders every trajectory type to an individual SVG card in
//! `book/src/animation/img/` - run with
//! `cargo run -p smearor-commons-animation --example diagrams`.

use smearor_commons_animation::BezierPath;
use smearor_commons_animation::BouncePath;
use smearor_commons_animation::CatmullRom;
use smearor_commons_animation::ConicalHelix;
use smearor_commons_animation::Delay;
use smearor_commons_animation::Eased;
use smearor_commons_animation::EasingFunction;
use smearor_commons_animation::Ellipse;
use smearor_commons_animation::Figure8;
use smearor_commons_animation::Helix;
use smearor_commons_animation::Keyframes;
use smearor_commons_animation::Lemniskate;
use smearor_commons_animation::Lissajous;
use smearor_commons_animation::Lorenz;
use smearor_commons_animation::Mirror;
use smearor_commons_animation::Mobius;
use smearor_commons_animation::Orbit;
use smearor_commons_animation::PolygonPath;
use smearor_commons_animation::Repeat;
use smearor_commons_animation::Rose;
use smearor_commons_animation::Span;
use smearor_commons_animation::SphereLissajous;
use smearor_commons_animation::SphericalOrbit;
use smearor_commons_animation::Spiral;
use smearor_commons_animation::Spiro;
use smearor_commons_animation::StarPath;
use smearor_commons_animation::Steps;
use smearor_commons_animation::Superellipse;
use smearor_commons_animation::ThreePhaseSpan;
use smearor_commons_animation::TorusKnot;
use smearor_commons_animation::Trajectory;
use smearor_commons_animation::Viviani;
use smearor_commons_animation::WaveTrail;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Vec2;
use smearor_commons_geometry::Vec3;
use std::fmt::Write as _;
use std::fs;
use std::io;

const SAMPLES: usize = 256;

/// The SVG chrome shared by every card - same dark style as the
/// hand-drawn overview diagrams.
fn card(name: &str, body: &str) -> String {
    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 220 190\" font-family=\"'Source Code Pro',ui-monospace,monospace\">\n  <rect width=\"220\" height=\"190\" rx=\"14\" fill=\"#0a0e16\" stroke=\"#1c2433\"/>\n  <rect x=\"10\" y=\"10\" width=\"200\" height=\"140\" rx=\"8\" fill=\"#10151f\" stroke=\"#1c2433\"/>\n{body}  <text x=\"110\" y=\"174\" fill=\"#8b98ad\" font-size=\"11\" text-anchor=\"middle\">{name}</text>\n</svg>\n"
    )
}

/// Fits `points` into the card area (20..200 x, 20..140 y) with a
/// uniform scale - returns svg-space coordinates.
fn fit(points: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let (mut lo_x, mut hi_x) = (f64::INFINITY, f64::NEG_INFINITY);
    let (mut lo_y, mut hi_y) = (f64::INFINITY, f64::NEG_INFINITY);
    for &(x, y) in points {
        lo_x = lo_x.min(x);
        hi_x = hi_x.max(x);
        lo_y = lo_y.min(y);
        hi_y = hi_y.max(y);
    }
    let scale = (180.0 / (hi_x - lo_x).max(1e-6)).min(120.0 / (hi_y - lo_y).max(1e-6));
    let (mid_x, mid_y) = ((lo_x + hi_x) / 2.0, (lo_y + hi_y) / 2.0);
    points.iter().map(|&(x, y)| (110.0 + (x - mid_x) * scale, 80.0 - (y - mid_y) * scale)).collect()
}

fn polyline(points: &[(f64, f64)], color: &str, width: f64, dash: &str) -> String {
    let mut d = String::new();
    for (i, &(x, y)) in points.iter().enumerate() {
        let _ = write!(d, "{}{x:.1},{y:.1} ", if i == 0 { "M " } else { "L " });
    }
    let dash_attr = if dash.is_empty() {
        String::new()
    } else {
        format!(" stroke-dasharray=\"{dash}\"")
    };
    format!("    <path d=\"{d}\" fill=\"none\" stroke=\"{color}\" stroke-width=\"{width}\"{dash_attr}/>\n")
}

fn dots(points: &[(f64, f64)], radius: f64, color: &str) -> String {
    points
        .iter()
        .map(|&(x, y)| format!("    <circle cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"{radius}\" fill=\"{color}\"/>\n"))
        .collect()
}

/// A `t -> value` graph card for scalar trajectories.
fn graph(name: &str, trajectory: &dyn Trajectory<Value = f64>, file: &str) -> io::Result<()> {
    let raw: Vec<(f64, f64)> = (0..=SAMPLES)
        .map(|i| {
            let t = i as f64 / SAMPLES as f64;
            (t, trajectory.value_at(Fraction::new(t)))
        })
        .collect();
    // graphs fit both axes independently - value range matters
    let points = fit(&raw);
    let mut body = String::new();
    // the 0..1 guides
    let zero = fit(&[(0.0, 0.0), (1.0, 0.0)]);
    let one = fit(&[(0.0, 1.0), (1.0, 1.0)]);
    body.push_str(&polyline(&zero, "#3a4560", 0.8, "3 3"));
    body.push_str(&polyline(&one, "#3a4560", 0.8, "3 3"));
    body.push_str(&polyline(&points, "#04e762", 2.2, ""));
    body.push_str(&dots(&[points[0]], 3.0, "#04e762"));
    body.push_str(&dots(&[points[points.len() - 1]], 3.0, "#8b98ad"));
    fs::write(file, card(name, &body))
}

/// Oblique projection for spatial paths: `z` shears up-right.
fn project(v: Vec3<()>) -> (f64, f64) {
    (v.x - 0.45 * v.z, v.y - 0.30 * v.z)
}

/// A path-trace card for planar and spatial trajectories -
/// `skeleton` draws dashed guide segments (control polygons),
/// `is3d` adds the z=0 footprint shadow.
fn path(name: &str, raw: &[(f64, f64)], shadow: Option<&[(f64, f64)]>, skeleton: Option<&[(f64, f64)]>, file: &str) -> io::Result<()> {
    let points = fit(raw);
    let mut body = String::new();
    if let Some(shadow) = shadow {
        body.push_str(&polyline(&fit(shadow), "#3a4560", 0.9, "3 3"));
    }
    if let Some(skeleton) = skeleton {
        let skeleton = fit(skeleton);
        body.push_str(&polyline(&skeleton, "#3a4560", 0.9, "3 3"));
        body.push_str(&dots(&skeleton, 2.5, "#8b98ad"));
    }
    body.push_str(&polyline(&points, "#04e762", 2.2, ""));
    body.push_str(&dots(&[points[0]], 3.0, "#04e762"));
    body.push_str(&dots(&[points[points.len() - 1]], 3.0, "#8b98ad"));
    fs::write(file, card(name, &body))
}

fn trace2(trajectory: &dyn Trajectory<Value = Vec2<()>>) -> Trace {
    (0..=SAMPLES)
        .map(|i| trajectory.value_at(Fraction::new(i as f64 / SAMPLES as f64)))
        .map(|v| (v.x, v.y))
        .collect()
}

/// A sampled 2D point list - `(svg-x, svg-y)` model pairs.
type Trace = Vec<(f64, f64)>;

fn trace3(trajectory: &dyn Trajectory<Value = Vec3<()>>) -> (Trace, Trace) {
    let raw: Vec<Vec3<()>> = (0..=SAMPLES).map(|i| trajectory.value_at(Fraction::new(i as f64 / SAMPLES as f64))).collect();
    (raw.iter().map(|&v| project(v)).collect(), raw.iter().map(|v| (v.x, v.y)).collect())
}

fn main() -> io::Result<()> {
    let dir = "book/src/animation/img";
    fs::create_dir_all(dir)?;
    let f = |name: &str| format!("{dir}/traj-{}.svg", name.to_lowercase());

    // --- scalar: value-over-progress graphs ----------------------
    graph("Span", &Span::new(0.0, 1.0), &f("span"))?;
    graph(
        "ThreePhaseSpan",
        &ThreePhaseSpan::new(0.0, 1.2, 1.0, Fraction::new(0.3), Fraction::new(0.7)),
        &f("threephasespan"),
    )?;
    graph(
        "Keyframes",
        &Keyframes::new(
            (Fraction::ZERO, 0.0),
            [
                (Fraction::new(0.25), 0.8),
                (Fraction::HALF, 0.4),
                (Fraction::new(0.75), 1.0),
                (Fraction::ONE, 0.6),
            ],
        ),
        &f("keyframes"),
    )?;
    graph("Repeat", &Repeat::new(Span::new(0.0, 1.0), 3), &f("repeat"))?;
    graph("Mirror", &Mirror::new(Span::new(0.0, 1.0)), &f("mirror"))?;
    graph("Delay", &Delay::new(Span::new(0.0, 1.0), Fraction::new(0.35)), &f("delay"))?;
    graph("Steps", &Steps::new(0.0, [0.6, 0.2, 1.0]), &f("steps"))?;
    graph("Eased", &Eased::new(Span::new(0.0, 1.0), EasingFunction::Bounce), &f("eased"))?;

    // --- planar: path traces -------------------------------------
    let c = Vec2::<()>::new(0.0, 0.0);
    let bezier_points = [
        Vec2::<()>::new(-1.0, -0.6),
        Vec2::<()>::new(-0.4, 1.0),
        Vec2::<()>::new(0.4, -1.0),
        Vec2::<()>::new(1.0, 0.6),
    ];
    path(
        "BezierPath",
        &trace2(&BezierPath::new(bezier_points[0], bezier_points[1..].iter().copied())),
        None,
        Some(&bezier_points.iter().map(|p| (p.x, p.y)).collect::<Vec<_>>()),
        &f("bezierpath"),
    )?;
    path("Orbit", &trace2(&Orbit::new(c, 1.0)), None, None, &f("orbit"))?;
    path("Lissajous", &trace2(&Lissajous::new(c, 1.0, 3.0, 2.0)), None, None, &f("lissajous"))?;
    path("Spiral", &trace2(&Spiral::new(c, 1.0).with_turns(3.0)), None, None, &f("spiral"))?;
    let polygon = StarPath::<Vec2<()>>::regular(c, 1.0, 5).points;
    path(
        "PolygonPath",
        &trace2(&PolygonPath::new(polygon.iter().copied())),
        None,
        Some(&polygon.iter().map(|p| (p.x, p.y)).collect::<Vec<_>>()),
        &f("polygonpath"),
    )?;
    let spline_points = [
        Vec2::<()>::new(-1.0, -0.5),
        Vec2::<()>::new(-0.5, 0.8),
        Vec2::<()>::new(0.0, -0.6),
        Vec2::<()>::new(0.6, 0.9),
        Vec2::<()>::new(1.0, -0.2),
    ];
    path(
        "CatmullRom",
        &trace2(&CatmullRom::new(spline_points)),
        None,
        Some(&spline_points.iter().map(|p| (p.x, p.y)).collect::<Vec<_>>()),
        &f("catmullrom"),
    )?;
    path("Lemniskate", &trace2(&Lemniskate::new(c, 1.0)), None, None, &f("lemniskate"))?;
    path("Ellipse", &trace2(&Ellipse::new(c, 1.0, 0.55).with_rotation(0.5)), None, None, &f("ellipse"))?;
    path("Rose", &trace2(&Rose::new(c, 1.0, 5.0)), None, None, &f("rose"))?;
    path("Spiro", &trace2(&Spiro::new(c, 1.0, 0.25, 0.25).with_turns(4.0)), None, None, &f("spiro"))?;
    path("Superellipse", &trace2(&Superellipse::new(c, 1.0, 0.8, 4.0)), None, None, &f("superellipse"))?;
    path(
        "StarPath",
        &trace2(&StarPath::regular(c, 1.0, 5).with_step(2)),
        None,
        Some(&polygon.iter().map(|p| (p.x, p.y)).collect::<Vec<_>>()),
        &f("starpath"),
    )?;
    path(
        "WaveTrail",
        &trace2(&WaveTrail::new(Vec2::<()>::new(-1.0, 0.0), Vec2::<()>::new(1.0, 0.0), 0.3, 3.0)),
        None,
        None,
        &f("wavetrail"),
    )?;
    path(
        "BouncePath",
        &trace2(&BouncePath::new(Vec2::<()>::new(-1.0, -0.5), Vec2::<()>::new(1.0, -0.5), 0.6, 4.0, 0.3)),
        None,
        None,
        &f("bouncepath"),
    )?;

    // --- spatial: oblique projection + z=0 footprint -------------
    let spatial = |name: &str, trajectory: &dyn Trajectory<Value = Vec3<()>>| -> io::Result<()> {
        let (curve, footprint) = trace3(trajectory);
        path(name, &curve, Some(&footprint), None, &f(name))
    };
    spatial("Helix", &Helix::new(c, 1.0, -1.0, 1.0).with_turns(3.0))?;
    spatial("SphericalOrbit", &SphericalOrbit::new(c, 1.0, 1.0))?;
    spatial("TorusKnot", &TorusKnot::new(c, 1.0, 2, 3))?;
    spatial("Figure8", &Figure8::new(c, 1.0, 0.6))?;
    spatial("ConicalHelix", &ConicalHelix::new(c, 0.3, 1.0, -1.0, 1.0).with_turns(3.0))?;
    spatial("Viviani", &Viviani::new(c, 1.0))?;
    spatial("Mobius", &Mobius::new(c, 1.0, 0.35))?;
    spatial("SphereLissajous", &SphereLissajous::new(c, 1.0, 6.0, 5.0))?;
    spatial("Lorenz", &Lorenz::new(c, 1.0))?;
    println!("wrote 30 svg cards to {dir}/");
    Ok(())
}
