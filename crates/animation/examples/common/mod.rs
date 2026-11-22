//! Shared infrastructure for the animation examples: trajectory and
//! easing builders, the per-component control column widgets and the
//! frame-clock playback group.
//!
//! Each example picks its own set of components (value axes, geometry
//! aspects, color channels, palette index) and reuses the same panel
//! and timing machinery.

// not every example uses every helper
#![allow(dead_code)]

use gtk4::Box as GtkBox;
use gtk4::DrawingArea;
use gtk4::DropDown;
use gtk4::Label;
use gtk4::Orientation;
use gtk4::Scale;
use gtk4::StringList;
use gtk4::glib;
use gtk4::prelude::*;
use serde::Deserialize;
use serde::Serialize;
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
use smearor_commons_animation::HoldMode;
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
use smearor_commons_animation::StepsJump;
use smearor_commons_animation::Superellipse;
use smearor_commons_animation::ThreePhaseSpan;
use smearor_commons_animation::TorusKnot;
use smearor_commons_animation::Trajectory;
use smearor_commons_animation::Viviani;
use smearor_commons_animation::WaveTrail;
use smearor_commons_geometry::Fraction;
use smearor_commons_geometry::Vec2;
use smearor_commons_geometry::Vec3;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

/// The scalar trajectories - they run in every dimension via the
/// `LiftF64`/`LiftF64To3` wrappers.
pub const SCALAR_TRAJECTORIES: &[&str] = &["Span", "ThreePhaseSpan", "Keyframes", "Repeat", "Mirror", "Delay", "Steps"];

/// The 2D list adds the planar natives - bezier path and orbit.
pub const TRAJECTORIES_2D: &[&str] = &[
    "Span",
    "ThreePhaseSpan",
    "Keyframes",
    "Repeat",
    "Mirror",
    "Delay",
    "Steps",
    "BezierPath",
    "Orbit",
    "Lissajous",
    "Spiral",
    "PolygonPath",
    "CatmullRom",
    "Lemniskate",
    "Ellipse",
    "Rose",
    "Spiro",
    "Superellipse",
    "StarPath",
    "WaveTrail",
    "BouncePath",
];

/// The 3D list adds the spatial natives - volumetric bezier and
/// helix.
pub const TRAJECTORIES_3D: &[&str] = &[
    "Span",
    "ThreePhaseSpan",
    "Keyframes",
    "Repeat",
    "Mirror",
    "Delay",
    "Steps",
    "BezierPath",
    "Helix",
    "SphericalOrbit",
    "TorusKnot",
    "Figure8",
    "ConicalHelix",
    "Viviani",
    "Mobius",
    "SphereLissajous",
    "Lorenz",
];

/// How the value axes may be split into independently driven
/// components - a "2D + 1D" composition runs one planar trajectory
/// on `x`/`y` and a scalar one on `z`.
pub fn compositions(dimension: u32) -> &'static [&'static str] {
    match dimension {
        1 => &["2D", "1D + 1D"],
        2 => &["3D", "1D + 1D + 1D", "2D + 1D", "1D + 2D"],
        _ => &["1D"],
    }
}

/// The axis group each component slot drives in `(dimension,
/// composition)` - `"xy"` takes a planar trajectory, `"xyz"` a
/// spatial one, single letters scalars.
pub fn component_roles(dimension: u32, composition: u32) -> &'static [&'static str] {
    match (dimension, composition) {
        (1, 1) => &["x", "y"],
        (2, 1) => &["x", "y", "z"],
        (2, 2) => &["xy", "z"],
        (2, 3) => &["x", "yz"],
        (1, _) => &["xy"],
        (2, _) => &["xyz"],
        _ => &["x"],
    }
}

/// The trajectory list a role's dropdown offers - group width is
/// the role's dimension.
pub fn role_choices(role: &str) -> &'static [&'static str] {
    match role.len() {
        2 => TRAJECTORIES_2D,
        3 => TRAJECTORIES_3D,
        _ => SCALAR_TRAJECTORIES,
    }
}

/// The easing functions the examples offer - parameterized variants
/// read their parameters from the sliders at build time.
pub const EASINGS: &[&str] = &[
    "Linear",
    "EaseIn",
    "EaseOut",
    "EaseInOut",
    "EaseInCubic",
    "EaseOutCubic",
    "EaseInOutCubic",
    "EaseInQuart",
    "EaseOutQuart",
    "EaseInOutQuart",
    "EaseInQuint",
    "EaseOutQuint",
    "EaseInOutQuint",
    "EaseInSine",
    "EaseOutSine",
    "EaseInOutSine",
    "EaseInExpo",
    "EaseOutExpo",
    "EaseInOutExpo",
    "EaseInCirc",
    "EaseOutCirc",
    "EaseInOutCirc",
    "Overshoot",
    "BackIn",
    "BackOut",
    "Elastic",
    "Bounce",
    "CubicBezier",
    "Steps",
];

/// The parameter-free variants offered for the `ThreePhaseSpan`
/// phase easings - parameterized curves get their values from the
/// easing group's sliders instead.
pub const PHASE_EASINGS: &[&str] = &[
    "Linear",
    "EaseIn",
    "EaseOut",
    "EaseInOut",
    "EaseInCubic",
    "EaseOutCubic",
    "EaseInOutCubic",
    "EaseInQuart",
    "EaseOutQuart",
    "EaseInOutQuart",
    "EaseInQuint",
    "EaseOutQuint",
    "EaseInOutQuint",
    "EaseInSine",
    "EaseOutSine",
    "EaseInOutSine",
    "EaseInExpo",
    "EaseOutExpo",
    "EaseInOutExpo",
    "EaseInCirc",
    "EaseOutCirc",
    "EaseInOutCirc",
    "Bounce",
];

/// Which easing selection reads which sliders.
#[derive(Clone, Copy)]
pub enum Params {
    None,
    Overshoot,
    Elastic,
    CubicBezier,
    Steps,
}

/// Which segment the "phase" progress bar subdivides for a given
/// trajectory.
pub enum PhaseShape {
    /// One phase - the bar mirrors total progress.
    Identity,
    /// `ThreePhaseSpan` lead / hold / tail boundaries.
    ThreePhase { lead: f64, trail: f64 },
    /// `Repeat` - one bar run per cycle.
    Cycles(u32),
    /// `Mirror` - out, then back.
    Mirror,
    /// `Delay` - idle run-up, then the inner span.
    Delayed(f64),
    /// `Steps` - the bar jumps with the quantized value.
    Steps(u32),
}

impl PhaseShape {
    /// The current segment as `(index, count)`, 1-based.
    pub fn phase_at(&self, p: f64) -> (u32, u32) {
        match *self {
            Self::Identity => (1, 1),
            Self::ThreePhase { lead, trail } => {
                if p < lead {
                    (1, 3)
                } else if p <= trail {
                    (2, 3)
                } else {
                    (3, 3)
                }
            }
            Self::Cycles(n) => (((p * n as f64) as u32 + 1).min(n), n),
            Self::Mirror => {
                if p <= 0.5 {
                    (1, 2)
                } else {
                    (2, 2)
                }
            }
            Self::Delayed(delay) => {
                if p < delay {
                    (1, 2)
                } else {
                    (2, 2)
                }
            }
            Self::Steps(n) => (((p * n as f64) as u32 + 1).min(n), n),
        }
    }

    /// Progress within the current segment, `0..=1`.
    pub fn fraction(&self, p: f64) -> f64 {
        match *self {
            Self::Identity => p,
            Self::ThreePhase { lead, trail } => {
                if p < lead {
                    p / lead.max(f64::EPSILON)
                } else if p <= trail {
                    (p - lead) / (trail - lead).max(f64::EPSILON)
                } else {
                    (p - trail) / (1.0 - trail).max(f64::EPSILON)
                }
            }
            Self::Cycles(n) => {
                if p >= 1.0 {
                    1.0
                } else {
                    (p * n as f64).fract()
                }
            }
            Self::Mirror => {
                if p <= 0.5 {
                    2.0 * p
                } else {
                    2.0 * (1.0 - p)
                }
            }
            Self::Delayed(delay) => ((p - delay) / (1.0 - delay).max(f64::EPSILON)).clamp(0.0, 1.0),
            Self::Steps(n) => {
                if p >= 1.0 {
                    1.0
                } else {
                    ((p * n as f64).floor() + 1.0) / n as f64
                }
            }
        }
    }
}

/// Lifts a scalar trajectory onto the 2D diagonal.
pub struct LiftF64<S>(pub S);

impl<S: Trajectory<Value = f64>> Trajectory for LiftF64<S> {
    type Value = Vec2<()>;

    fn value_at(&self, t: Fraction) -> Self::Value {
        v2(self.0.value_at(t))
    }
}

/// Lifts a scalar trajectory into 3D.
pub struct LiftF64To3<S>(pub S);

impl<S: Trajectory<Value = f64>> Trajectory for LiftF64To3<S> {
    type Value = Vec3<()>;

    fn value_at(&self, t: Fraction) -> Self::Value {
        v3(self.0.value_at(t))
    }
}

/// Lifts a planar trajectory into 3D at mid depth - 2D positions
/// project identically, the wrapper unifies position aspects that
/// may also carry a real `z`.
pub struct LiftVec2<S>(pub S);

impl<S: Trajectory<Value = Vec2<()>>> Trajectory for LiftVec2<S> {
    type Value = Vec3<()>;

    fn value_at(&self, t: Fraction) -> Self::Value {
        let v = self.0.value_at(t);
        Vec3::new(v.x, v.y, 0.5)
    }
}

/// Composes two scalar trajectories into a planar value - each
/// drives its own axis.
pub struct Compose2<X, Y> {
    pub x: X,
    pub y: Y,
}

impl<X: Trajectory<Value = f64>, Y: Trajectory<Value = f64>> Trajectory for Compose2<X, Y> {
    type Value = Vec2<()>;

    fn value_at(&self, t: Fraction) -> Vec2<()> {
        Vec2::new(self.x.value_at(t), self.y.value_at(t))
    }
}

/// A planar trajectory on `x`/`y` plus a scalar on `z`.
pub struct ComposeXY<XY, Z> {
    pub xy: XY,
    pub z: Z,
}

impl<XY: Trajectory<Value = Vec2<()>>, Z: Trajectory<Value = f64>> Trajectory for ComposeXY<XY, Z> {
    type Value = Vec3<()>;

    fn value_at(&self, t: Fraction) -> Vec3<()> {
        let xy = self.xy.value_at(t);
        Vec3::new(xy.x, xy.y, self.z.value_at(t))
    }
}

/// A scalar on `x` plus a planar trajectory on `y`/`z`.
pub struct ComposeYZ<X, YZ> {
    pub x: X,
    pub yz: YZ,
}

impl<X: Trajectory<Value = f64>, YZ: Trajectory<Value = Vec2<()>>> Trajectory for ComposeYZ<X, YZ> {
    type Value = Vec3<()>;

    fn value_at(&self, t: Fraction) -> Vec3<()> {
        let yz = self.yz.value_at(t);
        Vec3::new(self.x.value_at(t), yz.x, yz.y)
    }
}

/// The default value range of a component - the full `0..1` span.
fn unit_range() -> [f64; 2] {
    [0.0, 1.0]
}

/// The per-component parameters - trajectory choice, easing
/// choice and both parameter sets. Each split component owns one.
/// Serde-capable so effects round-trip through JSON.
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ComponentKnobs {
    pub trajectory: u32,
    pub easing: u32,
    pub overshoot: f64,
    pub amplitude: f64,
    pub period: f64,
    pub easing_steps: u32,
    pub trajectory_steps: u32,
    pub jump_start: bool,
    pub bezier: [f64; 4],
    pub repeat: u32,
    pub cycle_easing: u32,
    pub delay: f64,
    pub radius: f64,
    pub turns: f64,
    pub y_turns: f64,
    pub tilt: f64,
    pub windings: f64,
    pub depth: f64,
    pub center: [f64; 2],
    pub phase: f64,
    pub climb: [f64; 2],
    /// The `from`/`to` endpoints of the inner scalar span.
    #[serde(default = "unit_range")]
    pub range: [f64; 2],
    pub radii: [f64; 2],
    pub spiro: [f64; 3],
    pub exponent: f64,
    pub step: f64,
    pub waves: f64,
    pub height: f64,
    pub decay: f64,
    pub verts: f64,
    pub lead_easing: u32,
    pub trail_easing: u32,
    pub hold_easing: u32,
    pub hold_mode: u32,
    pub hold_to: f64,
    pub key_count: u32,
    pub key_points: Vec<(f64, f64)>,
    pub points: Vec<Vec3<()>>,
    pub lead: f64,
    pub trail: f64,
}

/// Builds a component's easing function by name.
pub fn easing_by_name(name: &str, knobs: &ComponentKnobs) -> (EasingFunction, Params) {
    match name {
        "Linear" => (EasingFunction::Linear, Params::None),
        "EaseIn" => (EasingFunction::EaseIn, Params::None),
        "EaseOut" => (EasingFunction::EaseOut, Params::None),
        "EaseInOut" => (EasingFunction::EaseInOut, Params::None),
        "EaseInCubic" => (EasingFunction::EaseInCubic, Params::None),
        "EaseOutCubic" => (EasingFunction::EaseOutCubic, Params::None),
        "EaseInOutCubic" => (EasingFunction::EaseInOutCubic, Params::None),
        "EaseInQuart" => (EasingFunction::EaseInQuart, Params::None),
        "EaseOutQuart" => (EasingFunction::EaseOutQuart, Params::None),
        "EaseInOutQuart" => (EasingFunction::EaseInOutQuart, Params::None),
        "EaseInQuint" => (EasingFunction::EaseInQuint, Params::None),
        "EaseOutQuint" => (EasingFunction::EaseOutQuint, Params::None),
        "EaseInOutQuint" => (EasingFunction::EaseInOutQuint, Params::None),
        "EaseInSine" => (EasingFunction::EaseInSine, Params::None),
        "EaseOutSine" => (EasingFunction::EaseOutSine, Params::None),
        "EaseInOutSine" => (EasingFunction::EaseInOutSine, Params::None),
        "EaseInExpo" => (EasingFunction::EaseInExpo, Params::None),
        "EaseOutExpo" => (EasingFunction::EaseOutExpo, Params::None),
        "EaseInOutExpo" => (EasingFunction::EaseInOutExpo, Params::None),
        "EaseInCirc" => (EasingFunction::EaseInCirc, Params::None),
        "EaseOutCirc" => (EasingFunction::EaseOutCirc, Params::None),
        "EaseInOutCirc" => (EasingFunction::EaseInOutCirc, Params::None),
        "Overshoot" => (
            EasingFunction::Overshoot {
                overshoot_amount: knobs.overshoot,
            },
            Params::Overshoot,
        ),
        "BackIn" => (
            EasingFunction::BackIn {
                overshoot_amount: knobs.overshoot,
            },
            Params::Overshoot,
        ),
        "BackOut" => (
            EasingFunction::BackOut {
                overshoot_amount: knobs.overshoot,
            },
            Params::Overshoot,
        ),
        "Elastic" => (
            EasingFunction::Elastic {
                amplitude: knobs.amplitude,
                period: knobs.period,
            },
            Params::Elastic,
        ),
        "Bounce" => (EasingFunction::Bounce, Params::None),
        "CubicBezier" => (
            EasingFunction::CubicBezier {
                x1: knobs.bezier[0],
                y1: knobs.bezier[1],
                x2: knobs.bezier[2],
                y2: knobs.bezier[3],
            },
            Params::CubicBezier,
        ),
        "Steps" => (
            EasingFunction::Steps {
                steps: knobs.easing_steps,
                jump: if knobs.jump_start { StepsJump::Start } else { StepsJump::End },
            },
            Params::Steps,
        ),
        _ => (EasingFunction::Linear, Params::None),
    }
}

/// Maps the scalar demo value onto the 2D diagonal - affine, so
/// lifting a trajectory and building it on `Vec2` are identical.
pub fn v2(v: f64) -> Vec2<()> {
    Vec2::new(v, 0.85 - 0.7 * v)
}

/// Maps the scalar demo value into 3D - depth grows with `v`, so
/// the logo scales up along the run.
pub fn v3(v: f64) -> Vec3<()> {
    Vec3::new(v, 0.85 - 0.7 * v, 0.15 + 0.7 * v)
}

/// The examples' bezier control polygon - `z` only matters in 3D.
pub fn bezier_points() -> Vec<Vec3<()>> {
    vec![
        Vec3::new(0.05, 0.8, 0.0),
        Vec3::new(0.3, -0.2, 0.3),
        Vec3::new(0.7, 1.2, 0.7),
        Vec3::new(0.95, 0.2, 1.0),
    ]
}

/// A bezier path over a control polygon - two points a line, three
/// quadratic, four cubic, beyond that de Casteljau of higher order.
pub fn bezier_path(points: &[Vec3<()>]) -> BezierPath {
    let rest: Vec<Vec2<()>> = points.iter().skip(1).map(|p| Vec2::<()>::new(p.x, p.y)).collect();
    let first = points.first().copied().unwrap_or_default();
    BezierPath::new(Vec2::<()>::new(first.x, first.y), rest)
}

/// The spatial bezier - a true volumetric curve through the 3D
/// control points.
pub fn bezier3_path(points: &[Vec3<()>]) -> BezierPath<Vec3<()>> {
    let rest: Vec<Vec3<()>> = points.iter().skip(1).copied().collect();
    BezierPath::new(points.first().copied().unwrap_or_default(), rest)
}

/// Builds the scalar trajectory + phase shape from the controls -
/// 2D and 3D demos lift its values onto the diagonal.
pub fn build_scalar_trajectory(name: &str, knobs: &ComponentKnobs) -> (Box<dyn Trajectory<Value = f64>>, PhaseShape) {
    let span = || Span::new(knobs.range[0], knobs.range[1]);
    match name {
        "ThreePhaseSpan" => (
            Box::new({
                let span = ThreePhaseSpan::new(
                    knobs.range[0],
                    (knobs.range[0] + knobs.range[1]) / 2.0,
                    knobs.range[1],
                    Fraction::new(knobs.lead),
                    Fraction::new(knobs.trail),
                )
                .with_easing(
                    easing_by_name(PHASE_EASINGS[knobs.lead_easing as usize], knobs).0,
                    easing_by_name(PHASE_EASINGS[knobs.trail_easing as usize], knobs).0,
                );
                // a hold easing only matters with a hold target
                if knobs.hold_easing > 0 || knobs.hold_mode > 0 || (knobs.hold_to - 0.5).abs() > f64::EPSILON {
                    let mode = if knobs.hold_mode == 1 { HoldMode::Pulse } else { HoldMode::Drift };
                    span.with_hold_mode(knobs.hold_to, easing_by_name(PHASE_EASINGS[knobs.hold_easing as usize], knobs).0, mode)
                } else {
                    span
                }
            }),
            PhaseShape::ThreePhase {
                lead: knobs.lead,
                trail: knobs.trail,
            },
        ),
        "Keyframes" => {
            let keys: Vec<(Fraction, f64)> = knobs
                .key_points
                .iter()
                .take(knobs.key_count.max(2) as usize)
                .map(|(t, v)| (Fraction::new(*t), *v))
                .collect();
            let (first, rest) = keys.split_first().map(|(f, r)| (*f, r.to_vec())).unwrap_or(((Fraction::ZERO, 0.0), vec![]));
            (Box::new(Keyframes::new(first, rest)), PhaseShape::Identity)
        }
        "Repeat" => (
            Box::new(Repeat::new(span(), knobs.repeat).with_easing(easing_by_name(PHASE_EASINGS[knobs.cycle_easing as usize], knobs).0)),
            PhaseShape::Cycles(knobs.repeat),
        ),
        "Mirror" => (Box::new(Mirror::new(span())), PhaseShape::Mirror),
        "Delay" => (Box::new(Delay::new(span(), Fraction::new(knobs.delay))), PhaseShape::Delayed(knobs.delay)),
        "Steps" => {
            let n = knobs.trajectory_steps.max(2);
            let ladder = (1..n).map(|i| i as f64 / (n - 1) as f64);
            let ladder: Vec<f64> = ladder.map(|u| knobs.range[0] + u * (knobs.range[1] - knobs.range[0])).collect();
            (Box::new(Steps::new(knobs.range[0], ladder)), PhaseShape::Steps(n))
        }
        _ => (Box::new(span()), PhaseShape::Identity),
    }
}

/// The shared center knob of every orbital trajectory.
pub fn center(knobs: &ComponentKnobs) -> Vec2<()> {
    Vec2::<()>::new(knobs.center[0], knobs.center[1])
}

/// The planar trajectory + phase shape for a 2D component - the
/// native paths, else the scalar lifted onto the diagonal.
pub fn planar_trajectory(name: &str, knobs: &ComponentKnobs) -> (Box<dyn Trajectory<Value = Vec2<()>>>, PhaseShape) {
    let laps = || PhaseShape::Cycles(knobs.turns.round().max(1.0) as u32);
    match name {
        "Orbit" => (Box::new(Orbit::new(center(knobs), knobs.radius).with_turns(knobs.turns)), laps()),
        "Lissajous" => (
            Box::new(Lissajous::new(center(knobs), knobs.radius, knobs.turns, knobs.y_turns).with_phase(knobs.phase * std::f64::consts::TAU)),
            PhaseShape::Identity,
        ),
        "Spiral" => (Box::new(Spiral::new(center(knobs), knobs.radius).with_turns(knobs.turns)), PhaseShape::Identity),
        "PolygonPath" => (Box::new(PolygonPath::new(knobs.points.iter().map(|p| Vec2::<()>::new(p.x, p.y)))), PhaseShape::Identity),
        "CatmullRom" => (Box::new(CatmullRom::new(knobs.points.iter().map(|p| Vec2::<()>::new(p.x, p.y)))), PhaseShape::Identity),
        "Lemniskate" => (Box::new(Lemniskate::new(center(knobs), knobs.radius).with_turns(knobs.turns)), laps()),
        "Ellipse" => (
            Box::new(
                Ellipse::new(center(knobs), knobs.radii[0], knobs.radii[1])
                    .with_rotation(knobs.phase * std::f64::consts::TAU)
                    .with_turns(knobs.turns),
            ),
            laps(),
        ),
        "Rose" => (
            Box::new(
                Rose::new(center(knobs), knobs.radius, knobs.windings)
                    .with_phase(knobs.phase * std::f64::consts::TAU)
                    .with_turns(knobs.turns),
            ),
            laps(),
        ),
        "Spiro" => (
            Box::new(Spiro::new(center(knobs), knobs.spiro[0], knobs.spiro[1], knobs.spiro[2]).with_turns(knobs.turns)),
            PhaseShape::Identity,
        ),
        "Superellipse" => (
            Box::new(Superellipse::new(center(knobs), knobs.radii[0], knobs.radii[1], knobs.exponent).with_turns(knobs.turns)),
            laps(),
        ),
        "StarPath" => (
            Box::new(
                StarPath::regular(center(knobs), knobs.radius, knobs.verts as usize)
                    .with_step((knobs.step as usize).min((knobs.verts as usize).saturating_sub(1).max(1))),
            ),
            PhaseShape::Identity,
        ),
        "WaveTrail" => (
            Box::new(WaveTrail::new(
                Vec2::<()>::new(knobs.points[0].x, knobs.points[0].y),
                Vec2::<()>::new(knobs.points[1].x, knobs.points[1].y),
                knobs.height,
                knobs.waves,
            )),
            PhaseShape::Identity,
        ),
        "BouncePath" => (
            Box::new(BouncePath::new(
                Vec2::<()>::new(knobs.points[0].x, knobs.points[0].y),
                Vec2::<()>::new(knobs.points[1].x, knobs.points[1].y),
                knobs.height,
                knobs.turns,
                knobs.decay,
            )),
            PhaseShape::Identity,
        ),
        "BezierPath" => (Box::new(bezier_path(&knobs.points)), PhaseShape::Identity),
        _ => {
            let (trajectory, phase) = build_scalar_trajectory(name, knobs);
            (Box::new(LiftF64(trajectory)), phase)
        }
    }
}

/// The spatial trajectory + phase shape for a 3D component - the
/// native paths, else the scalar lifted with depth.
pub fn spatial_trajectory(name: &str, knobs: &ComponentKnobs) -> (Box<dyn Trajectory<Value = Vec3<()>>>, PhaseShape) {
    let laps = || PhaseShape::Cycles(knobs.turns.round().max(1.0) as u32);
    match name {
        "Helix" => (
            Box::new(Helix::new(center(knobs), knobs.radius, knobs.climb[0], knobs.climb[1]).with_turns(knobs.turns)),
            laps(),
        ),
        "SphericalOrbit" => (
            Box::new(SphericalOrbit::new(center(knobs), knobs.radius, knobs.tilt * std::f64::consts::FRAC_PI_2).with_turns(knobs.turns)),
            laps(),
        ),
        "TorusKnot" => (
            Box::new(TorusKnot::new(
                center(knobs),
                knobs.radius,
                knobs.turns.round().max(1.0) as u32,
                knobs.windings.round().max(1.0) as u32,
            )),
            PhaseShape::Identity,
        ),
        "Figure8" => (Box::new(Figure8::new(center(knobs), knobs.radius, knobs.depth).with_turns(knobs.turns)), laps()),
        "ConicalHelix" => (
            Box::new(ConicalHelix::new(center(knobs), knobs.radii[0], knobs.radii[1], knobs.climb[0], knobs.climb[1]).with_turns(knobs.turns)),
            laps(),
        ),
        "Viviani" => (Box::new(Viviani::new(center(knobs), knobs.radius).with_turns(knobs.turns)), laps()),
        "Mobius" => (Box::new(Mobius::new(center(knobs), knobs.radius, knobs.depth)), PhaseShape::Identity),
        "SphereLissajous" => (Box::new(SphereLissajous::new(center(knobs), knobs.radius, knobs.turns, knobs.y_turns)), laps()),
        "Lorenz" => (Box::new(Lorenz::new(center(knobs), knobs.radius)), PhaseShape::Identity),
        "BezierPath" => (Box::new(bezier3_path(&knobs.points)), PhaseShape::Identity),
        _ => {
            let (trajectory, phase) = build_scalar_trajectory(name, knobs);
            (Box::new(LiftF64To3(trajectory)), phase)
        }
    }
}

/// Wraps a trajectory in `Eased` - the component's own easing shapes
/// the progress it sees, so the shared clock can run linear.
pub fn eased<T: Trajectory + 'static>(trajectory: T, knobs: &ComponentKnobs) -> Box<dyn Trajectory<Value = T::Value>> {
    let (easing, _) = easing_by_name(EASINGS[knobs.easing as usize], knobs);
    Box::new(Eased::new(trajectory, easing))
}

/// A labeled slider row for the sidebar - returns the row too, so
/// parameter groups can hide it entirely.
pub fn slider(parent: &GtkBox, label: &str, min: f64, max: f64, value: f64, digits: i32) -> (GtkBox, Scale) {
    let row = GtkBox::new(Orientation::Horizontal, 8);
    let text = Label::new(Some(label));
    text.set_width_chars(11);
    text.set_halign(gtk4::Align::Start);
    let scale = Scale::with_range(Orientation::Horizontal, min, max, 0.01);
    scale.set_hexpand(true);
    scale.set_digits(digits);
    scale.set_value(value);
    row.append(&text);
    row.append(&scale);
    parent.append(&row);
    (row, scale)
}

/// Shared change-listener list of a `RangeSlider`.
pub type ChangeListeners = Rc<RefCell<Vec<Box<dyn Fn()>>>>;

/// A two-grip range slider for `from`/`to` style parameters. Drawn
/// on a `DrawingArea`; dragging grabs the nearest grip. Whether the
/// grips may pass each other is a constructor flag - with crossing
/// `lower` carries the `from` value and `upper` the `to` value.
pub struct RangeSlider {
    /// The `from` grip value.
    pub lower: Rc<Cell<f64>>,
    /// The `to` grip value.
    pub upper: Rc<Cell<f64>>,
    /// Change listeners run on every grip move.
    pub listeners: ChangeListeners,
}

impl RangeSlider {
    /// Registers a change listener - the `connect_changed` pattern
    /// of the builtin widgets.
    pub fn connect_changed(&self, listener: impl Fn() + 'static) {
        self.listeners.borrow_mut().push(Box::new(listener));
    }
}

/// Trajectories with a `center` knob - the center is drawn on the
/// canvas as a draggable marker.
pub const ORBITAL: &[&str] = &[
    "Orbit",
    "Helix",
    "Lissajous",
    "Spiral",
    "Lemniskate",
    "SphericalOrbit",
    "TorusKnot",
    "Figure8",
    "Ellipse",
    "Rose",
    "Spiro",
    "Superellipse",
    "ConicalHelix",
    "Viviani",
    "Mobius",
    "SphereLissajous",
    "Lorenz",
    "StarPath",
];

/// Trajectories driven by the P1-P4 point rows.
pub const POINT_PATHS: &[&str] = &["BezierPath", "PolygonPath", "CatmullRom", "WaveTrail", "BouncePath"];

/// A labeled two-grip range slider row for the sidebar - the
/// grips carry `min..=max` values and `marks` draws the scale. With
/// `allow_crossing` the grips may pass each other - `lower` is then
/// the `from` grip and `upper` the `to` grip, so the span can run
/// backwards (e.g. `0.9 -> 0.2`).
#[allow(clippy::too_many_arguments)]
pub fn range_slider(parent: &GtkBox, label: &str, min: f64, max: f64, lower: f64, upper: f64, marks: &[f64], allow_crossing: bool) -> (GtkBox, RangeSlider) {
    let row = GtkBox::new(Orientation::Horizontal, 8);
    let text = Label::new(Some(label));
    text.set_width_chars(11);
    text.set_halign(gtk4::Align::Start);
    let area = DrawingArea::builder().hexpand(true).height_request(64).build();
    row.append(&text);
    row.append(&area);
    parent.append(&row);
    let marks: Rc<Vec<f64>> = Rc::new(marks.to_vec());

    const PAD: f64 = 14.0;
    let lower_cell = Rc::new(Cell::new(lower));
    let upper_cell = Rc::new(Cell::new(upper));
    let listeners: ChangeListeners = Rc::new(RefCell::new(Vec::new()));
    // 0 = no grip, 1 = lower, 2 = upper
    let grabbed = Rc::new(Cell::new(0_i8));

    area.set_draw_func({
        let (lower, upper) = (Rc::clone(&lower_cell), Rc::clone(&upper_cell));
        move |_, context, width, height| {
            let (w, h) = (f64::from(width), f64::from(height));
            let (track_w, mid) = (w - 2.0 * PAD, h * 0.35);
            let position = |v: f64| (v - min) / (max - min);
            let lx = PAD + track_w * position(lower.get());
            let ux = PAD + track_w * position(upper.get());
            context.set_source_rgb(0.55, 0.55, 0.55);
            context.move_to(PAD, mid);
            context.line_to(PAD + track_w, mid);
            context.set_line_width(4.0);
            let _ = context.stroke();
            // a warm band marks a reversed from > to span
            if lower.get() <= upper.get() {
                context.set_source_rgb(0.2, 0.45, 0.8);
            } else {
                context.set_source_rgb(0.85, 0.5, 0.15);
            }
            context.move_to(lx, mid);
            context.line_to(ux, mid);
            let _ = context.stroke();
            for gx in [lx, ux] {
                context.arc(gx, mid, 8.0, 0.0, std::f64::consts::TAU);
                let _ = context.fill();
            }
            // the value scale below the track
            context.select_font_face("monospace", gtk4::cairo::FontSlant::Normal, gtk4::cairo::FontWeight::Normal);
            context.set_font_size(9.0);
            context.set_source_rgb(0.5, 0.5, 0.5);
            for &v in marks.iter() {
                let tx = PAD + track_w * position(v);
                context.move_to(tx, mid + 10.0);
                context.line_to(tx, mid + 15.0);
                context.set_line_width(1.0);
                let _ = context.stroke();
                // integers get short labels, fractions two decimals
                let label = if (v - v.round()).abs() < 1e-9 { format!("{v:.0}") } else { format!("{v:.2}") };
                if let Ok(extents) = context.text_extents(&label) {
                    context.move_to(tx - extents.width() / 2.0, mid + 26.0);
                    let _ = context.show_text(&label);
                }
            }
            // the grip values above the grips - nudged apart when
            // the grips sit close together
            context.set_source_rgb(0.2, 0.45, 0.8);
            context.set_font_size(10.0);
            let close = ux - lx < 44.0;
            for (gx, value, side) in [(lx, lower.get(), -1.0_f64), (ux, upper.get(), 1.0)] {
                let label = format!("{value:.2}");
                if let Ok(extents) = context.text_extents(&label) {
                    let shift = if close { side * 14.0 } else { 0.0 };
                    context.move_to(gx - extents.width() / 2.0 + shift, mid - 13.0);
                    let _ = context.show_text(&label);
                }
            }
        }
    });

    let notify = {
        let listeners = Rc::clone(&listeners);
        let area = area.clone();
        move || {
            area.queue_draw();
            for listener in listeners.borrow().iter() {
                listener();
            }
        }
    };
    let drag = gtk4::GestureDrag::new();
    {
        let (lower, upper, grabbed) = (Rc::clone(&lower_cell), Rc::clone(&upper_cell), Rc::clone(&grabbed));
        drag.connect_drag_begin(move |drag, x, _| {
            let w = f64::from(drag.widget().map(|w| w.width()).unwrap_or(0)).max(2.0 * PAD + 1.0);
            let span = max - min;
            let lx = PAD + (w - 2.0 * PAD) * (lower.get() - min) / span;
            let ux = PAD + (w - 2.0 * PAD) * (upper.get() - min) / span;
            grabbed.set(if (x - lx).abs() <= (x - ux).abs() { 1 } else { 2 });
        });
    }
    {
        let (lower, upper, grabbed) = (Rc::clone(&lower_cell), Rc::clone(&upper_cell), Rc::clone(&grabbed));
        let notify = Rc::new(notify);
        drag.connect_drag_update(move |drag, dx, _| {
            let Some((start_x, _)) = drag.start_point() else { return };
            let w = f64::from(drag.widget().map(|w| w.width()).unwrap_or(0)).max(2.0 * PAD + 1.0);
            let value = (min + (start_x + dx - PAD) / (w - 2.0 * PAD) * (max - min)).clamp(min, max);
            match grabbed.get() {
                1 if allow_crossing => lower.set(value),
                2 if allow_crossing => upper.set(value),
                1 => lower.set(value.min(upper.get())),
                2 => upper.set(value.max(lower.get())),
                _ => {}
            }
            notify();
        });
    }
    {
        let grabbed = Rc::clone(&grabbed);
        drag.connect_drag_end(move |_, _, _| grabbed.set(0));
    }
    area.add_controller(drag);
    (
        row,
        RangeSlider {
            lower: lower_cell,
            upper: upper_cell,
            listeners,
        },
    )
}

/// A labeled dropdown row for the sidebar - same shape as `slider`.
pub fn dropdown_row(parent: &GtkBox, label: &str, items: &[&str]) -> (GtkBox, DropDown) {
    let row = GtkBox::new(Orientation::Horizontal, 8);
    let text = Label::new(Some(label));
    text.set_width_chars(11);
    text.set_halign(gtk4::Align::Start);
    let dropdown = DropDown::new(Some(StringList::new(items)), gtk4::Expression::NONE);
    dropdown.set_hexpand(true);
    row.append(&text);
    row.append(&dropdown);
    parent.append(&row);
    (row, dropdown)
}

/// A row of labeled spin buttons behind a row label - one field per
/// `(axis, min, max, value)` spec. Used for bezier control points
/// (`x`/`y`/`z`) and keyframes (`t`/`v`); fields can be hidden per
/// context by the caller.
pub fn spin_row(parent: &GtkBox, label: &str, specs: &[(&str, f64, f64, f64)]) -> (GtkBox, Vec<gtk4::SpinButton>) {
    let row = GtkBox::new(Orientation::Horizontal, 4);
    let text = Label::new(Some(label));
    text.set_width_chars(4);
    text.set_halign(gtk4::Align::Start);
    row.append(&text);
    let spins = specs
        .iter()
        .map(|(axis, min, max, value)| {
            let axis_label = Label::new(Some(axis));
            axis_label.set_margin_start(6);
            let spin = gtk4::SpinButton::with_range(*min, *max, 0.05);
            spin.set_digits(2);
            spin.set_value(*value);
            spin.set_width_chars(6);
            row.append(&axis_label);
            row.append(&spin);
            spin
        })
        .collect();
    parent.append(&row);
    (row, spins)
}

/// A bezier control-point row: `x`/`y`/`z` spin buttons behind the
/// point label.
pub fn point_row(parent: &GtkBox, label: &str, default: Vec3<()>) -> (GtkBox, Vec<gtk4::SpinButton>) {
    spin_row(parent, label, &[("x", -0.5, 1.5, default.x), ("y", -0.5, 1.5, default.y), ("z", -1.5, 1.5, default.z)])
}

/// The default keyframes - a wavy curve the spin rows start
/// from. The "Keys" count picks the first `n`.
pub const DEFAULT_KEYS: [(f64, f64); 8] = [
    (0.0, 0.0),
    (0.15, 0.4),
    (0.3, 0.8),
    (0.45, 0.45),
    (0.6, 0.2),
    (0.75, 0.5),
    (0.9, 0.8),
    (1.0, 1.0),
];

/// A framed section with a bold, larger title and padded content.
pub fn group_frame(title: &str, child: &GtkBox) -> gtk4::Frame {
    let frame = gtk4::Frame::new(None);
    let label = Label::new(None);
    label.set_markup(&format!("<span size=\"large\" weight=\"bold\">{title}</span>"));
    frame.set_label_widget(Some(&label));
    child.set_margin_top(8);
    child.set_margin_bottom(8);
    child.set_margin_start(8);
    child.set_margin_end(8);
    frame.set_child(Some(child));
    // groups share the full column height
    frame.set_vexpand(true);
    frame
}

/// One split component's controls: a frame titled by the axes it
/// drives, holding the trajectory dropdown + parameters and the
/// easing dropdown + parameters - a self-contained column.
pub struct ComponentPanel {
    /// The column wrapping the whole panel.
    pub column: GtkBox,
    /// The axes heading on top (`"Axis x"`, `"Axis yz"`, ...).
    pub title: Label,
    /// The trajectory dropdown - model swapped per role.
    pub trajectory: DropDown,
    /// The easing dropdown.
    pub easing: DropDown,
    // --- trajectory parameter rows ---
    repeat_row: GtkBox,
    repeat: Scale,
    cycle_row: GtkBox,
    cycle_easing: DropDown,
    delay_row: GtkBox,
    delay: Scale,
    steps_row: GtkBox,
    steps: Scale,
    keys_row: GtkBox,
    keys_count: Scale,
    key_rows: Rc<Vec<(GtkBox, Vec<gtk4::SpinButton>)>>,
    lead_trail_row: GtkBox,
    lead_trail: RangeSlider,
    lead_row: GtkBox,
    lead_easing: DropDown,
    hold_row: GtkBox,
    hold_easing: DropDown,
    hold_mode_row: GtkBox,
    hold_mode: DropDown,
    hold_to_row: GtkBox,
    hold_to: Scale,
    trail_row: GtkBox,
    trail_easing: DropDown,
    radius_row: GtkBox,
    radius: Scale,
    turns_row: GtkBox,
    turns: Scale,
    y_turns_row: GtkBox,
    y_turns: Scale,
    tilt_row: GtkBox,
    tilt: Scale,
    windings_row: GtkBox,
    windings: Scale,
    depth_row: GtkBox,
    depth: Scale,
    /// The `center` x/y spin buttons - canvas markers write back.
    pub center_row: GtkBox,
    pub center: Vec<gtk4::SpinButton>,
    phase_row: GtkBox,
    phase: Scale,
    range_row: GtkBox,
    range: RangeSlider,
    climb_row: GtkBox,
    climb: RangeSlider,
    radii_row: GtkBox,
    radii: Vec<gtk4::SpinButton>,
    spiro_row: GtkBox,
    spiro: Vec<gtk4::SpinButton>,
    exponent_row: GtkBox,
    exponent: Scale,
    step_row: GtkBox,
    step: Scale,
    waves_row: GtkBox,
    waves: Scale,
    height_row: GtkBox,
    height: Scale,
    decay_row: GtkBox,
    decay: Scale,
    verts_row: GtkBox,
    verts: Scale,
    point_rows: Rc<Vec<(GtkBox, Vec<gtk4::SpinButton>)>>,
    // --- easing parameter rows ---
    overshoot_row: GtkBox,
    overshoot: Scale,
    amplitude_row: GtkBox,
    amplitude: Scale,
    period_row: GtkBox,
    period: Scale,
    easing_steps_row: GtkBox,
    easing_steps: Scale,
    jump_row: GtkBox,
    jump: DropDown,
    bx1_row: GtkBox,
    bx1: Scale,
    by1_row: GtkBox,
    by1: Scale,
    bx2_row: GtkBox,
    bx2: Scale,
    by2_row: GtkBox,
    by2: Scale,
}

/// Builds a component panel into `parent` - one frame per split
/// component, every parameter row created up front and toggled by
/// the update pass.
pub fn component_panel(parent: &GtkBox) -> ComponentPanel {
    // the column: axes heading on top, one group frame per section
    let column = GtkBox::new(Orientation::Vertical, 8);
    column.set_width_request(280);
    column.set_margin_top(4);
    column.set_margin_start(4);
    column.set_margin_end(4);
    let title = Label::new(None);
    title.set_markup("<span size=\"large\" weight=\"bold\">Axis x</span>");
    column.append(&title);

    let trajectory_box = GtkBox::new(Orientation::Vertical, 6);
    let trajectory = DropDown::new(Some(StringList::new(SCALAR_TRAJECTORIES)), gtk4::Expression::NONE);
    trajectory_box.append(&trajectory);

    // --- trajectory parameters ---
    let (repeat_row, repeat) = slider(&trajectory_box, "Repeat", 1.0, 8.0, 3.0, 0);
    repeat.set_round_digits(0);
    for mark in 1..=8 {
        repeat.add_mark(f64::from(mark), gtk4::PositionType::Bottom, Some(&mark.to_string()));
    }
    let (cycle_row, cycle_easing) = dropdown_row(&trajectory_box, "Cycle", PHASE_EASINGS);
    let (delay_row, delay) = slider(&trajectory_box, "Delay", 0.0, 0.9, 0.25, 2);
    for mark in [0.0, 0.3, 0.6, 0.9] {
        delay.add_mark(mark, gtk4::PositionType::Bottom, Some(&format!("{mark:.1}")));
    }
    for mark in [0.15, 0.45, 0.75] {
        delay.add_mark(mark, gtk4::PositionType::Bottom, None);
    }
    let (steps_row, steps) = slider(&trajectory_box, "Steps", 2.0, 12.0, 5.0, 0);
    steps.set_round_digits(0);
    for mark in 2..=12 {
        steps.add_mark(f64::from(mark), gtk4::PositionType::Bottom, (mark % 2 == 0).then(|| mark.to_string()).as_deref());
    }
    let (range_row, range) = range_slider(&trajectory_box, "Range", -1.0, 2.0, 0.0, 1.0, &[-1.0, 0.0, 1.0, 2.0], true);
    let (keys_row, keys_count) = slider(&trajectory_box, "Keys", 2.0, 8.0, 4.0, 0);
    keys_count.set_round_digits(0);
    for mark in 2..=8 {
        keys_count.add_mark(f64::from(mark), gtk4::PositionType::Bottom, Some(&mark.to_string()));
    }
    let key_rows: Rc<Vec<(GtkBox, Vec<gtk4::SpinButton>)>> = Rc::new(
        DEFAULT_KEYS
            .iter()
            .enumerate()
            .map(|(i, (t, v))| spin_row(&trajectory_box, &format!("K{}", i + 1), &[("t", 0.0, 1.0, *t), ("v", -0.5, 1.5, *v)]))
            .collect(),
    );
    let (lead_trail_row, lead_trail) = range_slider(&trajectory_box, "Lead/Trail", 0.0, 1.0, 0.3, 0.7, &[0.0, 0.25, 0.5, 0.75, 1.0], false);
    let (lead_row, lead_easing) = dropdown_row(&trajectory_box, "Phase 1", PHASE_EASINGS);
    lead_easing.set_selected(2); // EaseOut
    let (hold_row, hold_easing) = dropdown_row(&trajectory_box, "Phase 2", PHASE_EASINGS);
    let (hold_mode_row, hold_mode) = dropdown_row(&trajectory_box, "Mode", &["Drift", "Pulse"]);
    let (hold_to_row, hold_to) = slider(&trajectory_box, "Hold to", 0.0, 1.0, 0.5, 2);
    hold_to.add_mark(0.5, gtk4::PositionType::Bottom, Some("mid"));
    {
        // snap to the mid mark - the classic constant hold
        let snap = hold_to.clone();
        snap.connect_value_changed(|scale| {
            const MID: f64 = 0.5;
            const SNAP: f64 = 0.05;
            let value = scale.value();
            if (value - MID).abs() < SNAP && (value - MID).abs() > f64::EPSILON {
                scale.set_value(MID);
            }
        });
    }
    let (trail_row, trail_easing) = dropdown_row(&trajectory_box, "Phase 3", PHASE_EASINGS);
    trail_easing.set_selected(1); // EaseIn
    let (radius_row, radius) = slider(&trajectory_box, "Radius", 0.05, 0.5, 0.3, 2);
    let (turns_row, turns) = slider(&trajectory_box, "Turns", 0.5, 4.0, 1.0, 1);
    turns.set_round_digits(1);
    for mark in 1..=4 {
        turns.add_mark(f64::from(mark), gtk4::PositionType::Bottom, Some(&mark.to_string()));
    }
    for mark in [0.5, 1.5, 2.5, 3.5] {
        turns.add_mark(mark, gtk4::PositionType::Bottom, None);
    }
    let (y_turns_row, y_turns) = slider(&trajectory_box, "Freq y", 1.0, 6.0, 2.0, 0);
    y_turns.set_round_digits(0);
    for mark in 1..=6 {
        y_turns.add_mark(f64::from(mark), gtk4::PositionType::Bottom, Some(&mark.to_string()));
    }
    let (tilt_row, tilt) = slider(&trajectory_box, "Tilt", 0.0, 1.0, 0.5, 2);
    let (windings_row, windings) = slider(&trajectory_box, "Windings", 1.0, 9.0, 3.0, 0);
    windings.set_round_digits(0);
    for mark in 1..=9 {
        windings.add_mark(f64::from(mark), gtk4::PositionType::Bottom, Some(&mark.to_string()));
    }
    let (depth_row, depth) = slider(&trajectory_box, "Depth", -1.0, 1.0, 0.5, 2);
    let (center_row, center) = spin_row(&trajectory_box, "Center", &[("x", -0.5, 1.5, 0.5), ("y", -0.5, 1.5, 0.5)]);
    let (phase_row, phase) = slider(&trajectory_box, "Phase", 0.0, 1.0, 0.25, 2);
    let (climb_row, climb) = range_slider(&trajectory_box, "Climb", -2.0, 2.0, -1.0, 1.0, &[-2.0, -1.0, 0.0, 1.0, 2.0], true);
    let (radii_row, radii) = spin_row(&trajectory_box, "Radii", &[("r0", 0.05, 0.8, 0.35), ("r1", 0.05, 0.8, 0.2)]);
    let (spiro_row, spiro) = spin_row(&trajectory_box, "Spiro", &[("R", 0.1, 0.6, 0.4), ("r", 0.02, 0.3, 0.1), ("d", 0.0, 0.6, 0.1)]);
    let (exponent_row, exponent) = slider(&trajectory_box, "Exponent", 0.3, 8.0, 2.0, 2);
    for mark in [1.0, 2.0, 4.0, 8.0] {
        exponent.add_mark(mark, gtk4::PositionType::Bottom, Some(&format!("{mark:.0}")));
    }
    let (step_row, step) = slider(&trajectory_box, "Step", 1.0, 6.0, 2.0, 0);
    step.set_round_digits(0);
    for mark in 1..=6 {
        step.add_mark(f64::from(mark), gtk4::PositionType::Bottom, Some(&mark.to_string()));
    }
    let (waves_row, waves) = slider(&trajectory_box, "Waves", 0.5, 8.0, 3.0, 1);
    for mark in 1..=8 {
        waves.add_mark(f64::from(mark), gtk4::PositionType::Bottom, Some(&mark.to_string()));
    }
    let (height_row, height) = slider(&trajectory_box, "Height", -0.8, 0.8, 0.3, 2);
    let (decay_row, decay) = slider(&trajectory_box, "Decay", 0.05, 1.0, 0.5, 2);
    let (verts_row, verts) = slider(&trajectory_box, "Verts", 3.0, 10.0, 5.0, 0);
    verts.set_round_digits(0);
    for mark in 3..=10 {
        verts.add_mark(f64::from(mark), gtk4::PositionType::Bottom, Some(&mark.to_string()));
    }
    let point_rows: Rc<Vec<(GtkBox, Vec<gtk4::SpinButton>)>> = Rc::new(
        bezier_points()
            .iter()
            .enumerate()
            .map(|(i, point)| point_row(&trajectory_box, &format!("P{}", i + 1), *point))
            .collect(),
    );

    column.append(&group_frame("Trajectory", &trajectory_box));

    // --- easing + parameters ---
    let easing_box = GtkBox::new(Orientation::Vertical, 6);
    let easing = DropDown::new(Some(StringList::new(EASINGS)), gtk4::Expression::NONE);
    easing.set_selected(5); // EaseOutCubic
    easing_box.append(&easing);
    let (overshoot_row, overshoot) = slider(&easing_box, "Overshoot", 0.0, 4.0, 1.7, 2);
    let (amplitude_row, amplitude) = slider(&easing_box, "Amplitude", 0.2, 3.0, 1.0, 2);
    let (period_row, period) = slider(&easing_box, "Period", 0.05, 1.0, 0.3, 2);
    let (easing_steps_row, easing_steps) = slider(&easing_box, "Steps", 2.0, 12.0, 5.0, 0);
    easing_steps.set_round_digits(0);
    for mark in 2..=12 {
        easing_steps.add_mark(f64::from(mark), gtk4::PositionType::Bottom, (mark % 2 == 0).then(|| mark.to_string()).as_deref());
    }
    let jump = DropDown::new(Some(StringList::new(&["End", "Start"])), gtk4::Expression::NONE);
    let jump_row = GtkBox::new(Orientation::Horizontal, 8);
    let jump_label = Label::new(Some("Jump"));
    jump_label.set_width_chars(11);
    jump_label.set_halign(gtk4::Align::Start);
    jump_row.append(&jump_label);
    jump_row.append(&jump);
    easing_box.append(&jump_row);
    let (bx1_row, bx1) = slider(&easing_box, "Bezier x1", 0.0, 1.0, 0.25, 2);
    let (by1_row, by1) = slider(&easing_box, "Bezier y1", -1.0, 2.0, 0.1, 2);
    let (bx2_row, bx2) = slider(&easing_box, "Bezier x2", 0.0, 1.0, 0.25, 2);
    let (by2_row, by2) = slider(&easing_box, "Bezier y2", -1.0, 2.0, 1.0, 2);

    column.append(&group_frame("Easing", &easing_box));
    parent.append(&column);

    ComponentPanel {
        column,
        title,
        trajectory,
        easing,
        repeat_row,
        repeat,
        cycle_row,
        cycle_easing,
        delay_row,
        delay,
        steps_row,
        steps,
        keys_row,
        keys_count,
        key_rows,
        lead_trail_row,
        lead_trail,
        lead_row,
        lead_easing,
        hold_row,
        hold_easing,
        hold_mode_row,
        hold_mode,
        hold_to_row,
        hold_to,
        trail_row,
        trail_easing,
        radius_row,
        radius,
        turns_row,
        turns,
        y_turns_row,
        y_turns,
        tilt_row,
        tilt,
        windings_row,
        windings,
        depth_row,
        depth,
        center_row,
        center,
        phase_row,
        phase,
        climb_row,
        climb,
        range_row,
        range,
        radii_row,
        radii,
        spiro_row,
        spiro,
        exponent_row,
        exponent,
        step_row,
        step,
        waves_row,
        waves,
        height_row,
        height,
        decay_row,
        decay,
        verts_row,
        verts,
        point_rows,
        overshoot_row,
        overshoot,
        amplitude_row,
        amplitude,
        period_row,
        period,
        easing_steps_row,
        easing_steps,
        jump_row,
        jump,
        bx1_row,
        bx1,
        by1_row,
        by1,
        bx2_row,
        bx2,
        by2_row,
        by2,
    }
}

/// Reads a component panel's widgets into its knob set.
pub fn component_knobs(panel: &ComponentPanel) -> ComponentKnobs {
    ComponentKnobs {
        trajectory: panel.trajectory.selected(),
        easing: panel.easing.selected(),
        overshoot: panel.overshoot.value(),
        amplitude: panel.amplitude.value(),
        period: panel.period.value(),
        easing_steps: panel.easing_steps.value() as u32,
        trajectory_steps: panel.steps.value() as u32,
        jump_start: panel.jump.selected() == 1,
        bezier: [panel.bx1.value(), panel.by1.value(), panel.bx2.value(), panel.by2.value()],
        repeat: panel.repeat.value() as u32,
        cycle_easing: panel.cycle_easing.selected(),
        delay: panel.delay.value(),
        radius: panel.radius.value(),
        turns: panel.turns.value(),
        y_turns: panel.y_turns.value(),
        tilt: panel.tilt.value(),
        windings: panel.windings.value(),
        depth: panel.depth.value(),
        center: [panel.center[0].value(), panel.center[1].value()],
        phase: panel.phase.value(),
        climb: [panel.climb.lower.get(), panel.climb.upper.get()],
        range: [panel.range.lower.get(), panel.range.upper.get()],
        radii: [panel.radii[0].value(), panel.radii[1].value()],
        spiro: [panel.spiro[0].value(), panel.spiro[1].value(), panel.spiro[2].value()],
        exponent: panel.exponent.value(),
        step: panel.step.value(),
        waves: panel.waves.value(),
        height: panel.height.value(),
        decay: panel.decay.value(),
        verts: panel.verts.value(),
        lead_easing: panel.lead_easing.selected(),
        trail_easing: panel.trail_easing.selected(),
        hold_easing: panel.hold_easing.selected(),
        hold_mode: panel.hold_mode.selected(),
        hold_to: panel.hold_to.value(),
        key_count: panel.keys_count.value() as u32,
        key_points: panel.key_rows.iter().map(|(_, spins)| (spins[0].value(), spins[1].value())).collect(),
        points: panel
            .point_rows
            .iter()
            .map(|(_, spins)| Vec3::<()>::new(spins[0].value(), spins[1].value(), spins[2].value()))
            .collect(),
        lead: panel.lead_trail.lower.get(),
        trail: panel.lead_trail.upper.get(),
    }
}

/// Wires a restart callback into every interactive widget of a
/// component panel.
pub fn connect_panel(panel: &ComponentPanel, restart: &Rc<dyn Fn()>) {
    for scale in [
        &panel.repeat,
        &panel.delay,
        &panel.steps,
        &panel.keys_count,
        &panel.hold_to,
        &panel.radius,
        &panel.turns,
        &panel.y_turns,
        &panel.tilt,
        &panel.windings,
        &panel.depth,
        &panel.phase,
        &panel.exponent,
        &panel.step,
        &panel.waves,
        &panel.height,
        &panel.decay,
        &panel.verts,
        &panel.overshoot,
        &panel.amplitude,
        &panel.period,
        &panel.easing_steps,
        &panel.bx1,
        &panel.by1,
        &panel.bx2,
        &panel.by2,
    ] {
        let restart = Rc::clone(restart);
        scale.connect_value_changed(move |_| restart());
    }
    for dropdown in [
        &panel.trajectory,
        &panel.easing,
        &panel.jump,
        &panel.cycle_easing,
        &panel.lead_easing,
        &panel.trail_easing,
        &panel.hold_easing,
        &panel.hold_mode,
    ] {
        let restart = Rc::clone(restart);
        dropdown.connect_selected_notify(move |_| restart());
    }
    {
        let restart = Rc::clone(restart);
        panel.lead_trail.connect_changed(move || restart());
    }
    for (_, spins) in panel.key_rows.iter().chain(panel.point_rows.iter()) {
        for spin in spins {
            let restart = Rc::clone(restart);
            spin.connect_value_changed(move |_| restart());
        }
    }
    for spin in panel.center.iter().chain(panel.radii.iter()).chain(panel.spiro.iter()) {
        let restart = Rc::clone(restart);
        spin.connect_value_changed(move |_| restart());
    }
    panel.climb.connect_changed({
        let restart = Rc::clone(restart);
        move || restart()
    });
    panel.range.connect_changed({
        let restart = Rc::clone(restart);
        move || restart()
    });
}

/// Shows only the parameter rows a component's selection actually
/// reads - `name` is the trajectory, `params` its easing family,
/// `spatial` whether the role drives 3 axes.
pub fn update_panel(panel: &ComponentPanel, name: &str, params: Params, spatial: bool) {
    panel.repeat_row.set_visible(name == "Repeat");
    panel.cycle_row.set_visible(name == "Repeat");
    panel.delay_row.set_visible(name == "Delay");
    panel.steps_row.set_visible(name == "Steps");
    panel
        .range_row
        .set_visible(matches!(name, "Span" | "Repeat" | "Mirror" | "Delay" | "Steps" | "ThreePhaseSpan"));
    panel.keys_row.set_visible(name == "Keyframes");
    let key_count = panel.keys_count.value() as usize;
    for (i, (row, _)) in panel.key_rows.iter().enumerate() {
        row.set_visible(name == "Keyframes" && i < key_count);
    }
    for (row, spins) in panel.point_rows.iter() {
        row.set_visible(POINT_PATHS.contains(&name));
        spins[2].set_visible(spatial);
    }
    panel.radius_row.set_visible(matches!(
        name,
        "Orbit"
            | "Helix"
            | "Lissajous"
            | "Spiral"
            | "Lemniskate"
            | "SphericalOrbit"
            | "TorusKnot"
            | "Figure8"
            | "Rose"
            | "Viviani"
            | "Mobius"
            | "SphereLissajous"
            | "Lorenz"
            | "StarPath"
    ));
    panel.turns_row.set_visible(matches!(
        name,
        "Orbit"
            | "Helix"
            | "Lissajous"
            | "Spiral"
            | "Lemniskate"
            | "SphericalOrbit"
            | "TorusKnot"
            | "Figure8"
            | "Ellipse"
            | "Rose"
            | "Spiro"
            | "Superellipse"
            | "ConicalHelix"
            | "Viviani"
            | "SphereLissajous"
            | "BouncePath"
    ));
    panel.y_turns_row.set_visible(matches!(name, "Lissajous" | "SphereLissajous"));
    panel.phase_row.set_visible(matches!(name, "Lissajous" | "Ellipse" | "Rose"));
    panel.tilt_row.set_visible(name == "SphericalOrbit");
    panel.windings_row.set_visible(matches!(name, "TorusKnot" | "Rose"));
    panel.depth_row.set_visible(matches!(name, "Figure8" | "Mobius"));
    panel.climb_row.set_visible(matches!(name, "Helix" | "ConicalHelix"));
    panel.center_row.set_visible(ORBITAL.contains(&name));
    panel.radii_row.set_visible(matches!(name, "Ellipse" | "Superellipse" | "ConicalHelix"));
    panel.spiro_row.set_visible(name == "Spiro");
    panel.exponent_row.set_visible(name == "Superellipse");
    panel.step_row.set_visible(name == "StarPath");
    panel.verts_row.set_visible(name == "StarPath");
    if name == "StarPath" {
        // {n/k} degenerates at k >= n - the step range tracks the
        // vertex count, e.g. the pentagram needs verts 5, step 2
        let last = (panel.verts.value() - 1.0).max(1.0);
        panel.step.set_range(1.0, last);
        panel.step.clear_marks();
        for mark in 1..=last as i32 {
            panel.step.add_mark(f64::from(mark), gtk4::PositionType::Bottom, Some(&mark.to_string()));
        }
    }
    panel.waves_row.set_visible(name == "WaveTrail");
    panel.height_row.set_visible(matches!(name, "WaveTrail" | "BouncePath"));
    panel.decay_row.set_visible(name == "BouncePath");
    for row in [
        &panel.lead_row,
        &panel.trail_row,
        &panel.hold_row,
        &panel.hold_mode_row,
        &panel.hold_to_row,
        &panel.lead_trail_row,
    ] {
        row.set_visible(name == "ThreePhaseSpan");
    }
    let on_elastic = matches!(params, Params::Elastic);
    let on_bezier = matches!(params, Params::CubicBezier);
    let on_steps = matches!(params, Params::Steps);
    panel.overshoot_row.set_visible(matches!(params, Params::Overshoot));
    panel.amplitude_row.set_visible(on_elastic);
    panel.period_row.set_visible(on_elastic);
    panel.easing_steps_row.set_visible(on_steps);
    panel.jump_row.set_visible(on_steps);
    for row in [&panel.bx1_row, &panel.by1_row, &panel.bx2_row, &panel.by2_row] {
        row.set_visible(on_bezier);
    }
}

/// The resizable `canvas | controls` split every example shares -
/// a `Paned` with a wide draggable handle. The canvas absorbs
/// window resizes, the scrolled controls keep the width the user
/// dragged them to.
pub fn paned_view(canvas: &DrawingArea, controls: &GtkBox) -> gtk4::Paned {
    let view = gtk4::Paned::new(Orientation::Horizontal);
    view.set_wide_handle(true);
    view.set_start_child(Some(canvas));
    let scroller = gtk4::ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Automatic)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .child(controls)
        .build();
    view.set_end_child(Some(&scroller));
    view.set_resize_start_child(true);
    view.set_resize_end_child(false);
    view.set_shrink_start_child(true);
    view.set_shrink_end_child(true);
    view
}

/// The timing widgets every example shares: duration and pause
/// sliders, the loop toggle, a replay button and the two progress
/// bars plus the phase counter.
pub struct Playback {
    pub duration: Scale,
    pub pause: Scale,
    pub looping: gtk4::CheckButton,
    pub replay: gtk4::Button,
    /// Resumes a paused animation.
    pub play: gtk4::Button,
    /// Freezes the animation at its current progress.
    pub pause_button: gtk4::Button,
    /// The run progress - dragging it seeks the animation.
    pub total: Scale,
    pub phase_bar: gtk4::ProgressBar,
    pub phase_number: Label,
}

/// Appends the framed Playback group to `controls`.
pub fn playback_group(controls: &GtkBox) -> Playback {
    let playback_box = GtkBox::new(Orientation::Vertical, 6);
    let (_duration_row, duration) = slider(&playback_box, "Duration", 0.25, 6.0, 1.6, 2);
    let (_pause_row, pause) = slider(&playback_box, "Pause", 0.0, 3.0, 0.5, 2);
    let looping = gtk4::CheckButton::with_label("Loop");
    looping.set_active(true);
    playback_box.append(&looping);
    let buttons = GtkBox::new(Orientation::Horizontal, 8);
    buttons.set_homogeneous(true);
    let play = gtk4::Button::with_label("Play");
    let pause_button = gtk4::Button::with_label("Pause");
    let replay = gtk4::Button::with_label("Replay");
    buttons.append(&play);
    buttons.append(&pause_button);
    buttons.append(&replay);
    playback_box.append(&buttons);
    let total_label = Label::new(Some("Total"));
    total_label.set_halign(gtk4::Align::Start);
    playback_box.append(&total_label);
    let total = Scale::with_range(Orientation::Horizontal, 0.0, 1.0, 0.005);
    total.add_mark(0.0, gtk4::PositionType::Bottom, Some("0"));
    total.add_mark(1.0, gtk4::PositionType::Bottom, Some("1"));
    playback_box.append(&total);
    let phase_label = Label::new(Some("Phase"));
    phase_label.set_halign(gtk4::Align::Start);
    playback_box.append(&phase_label);
    let phase_bar = gtk4::ProgressBar::new();
    playback_box.append(&phase_bar);
    let phase_number = Label::new(Some("Phase 1 / 1"));
    phase_number.set_halign(gtk4::Align::Center);
    playback_box.append(&phase_number);
    controls.append(&group_frame("Playback", &playback_box));
    Playback {
        duration,
        pause,
        looping,
        replay,
        play,
        pause_button,
        total,
        phase_bar,
        phase_number,
    }
}

/// What the frame clock needs from an example's animation - the
/// `TimedAnimation` plus the phase layout the bars subdivide by.
pub trait Animatable {
    /// Whether the animation ran through.
    fn is_complete(&self) -> bool;
    /// Restarts the clock.
    fn start(&mut self);
    /// Pins the clock at `progress` - seek for scrubbing; called
    /// every frame it is what pausing looks like.
    fn seek(&mut self, progress: Fraction);
    /// The linear run progress, `None` before the first start.
    fn progress(&self) -> Option<Fraction>;
    /// The first component's easing - the phase bar tracks what
    /// that component's trajectory sees.
    fn first_easing(&self) -> Option<EasingFunction>;
    /// The current trajectory segment as `(index, count)`.
    fn phase_at(&self, progress: f64) -> (u32, u32);
    /// Progress within the current trajectory segment.
    fn phase_fraction(&self, progress: f64) -> f64;
}

/// Installs the shared frame clock on the canvas: drives the demo,
/// handles the loop-with-pause restarts and feeds both progress
/// bars. `on_sample` runs after each update - store the latest
/// value for the draw callback there.
pub fn install_tick<A: Animatable + 'static>(
    canvas: &DrawingArea,
    demo: Rc<RefCell<A>>,
    playback: &Playback,
    hold: Rc<Cell<Option<std::time::Instant>>>,
    on_sample: impl Fn(&A) + 'static,
) {
    let (pause, looping) = (playback.pause.clone(), playback.looping.clone());
    let (total, phase_bar, phase_number) = (playback.total.clone(), playback.phase_bar.clone(), playback.phase_number.clone());

    // paused holds the pinned progress - `None` means running
    let paused = Rc::new(Cell::new(None::<f64>));
    // while the user drags the Total scale the tick does not
    // write the thumb back
    let scrubbing = Rc::new(Cell::new(false));

    playback.play.connect_clicked({
        let paused = Rc::clone(&paused);
        move |_| paused.set(None)
    });
    playback.pause_button.connect_clicked({
        let paused = Rc::clone(&paused);
        let demo = Rc::clone(&demo);
        move |_| {
            let u = demo.borrow().progress().map(|p| p.as_f64()).unwrap_or(0.0).min(1.0);
            paused.set(Some(u));
        }
    });
    let drag = gtk4::GestureDrag::new();
    drag.connect_drag_begin({
        let scrubbing = Rc::clone(&scrubbing);
        move |_, _, _| scrubbing.set(true)
    });
    drag.connect_drag_end({
        let scrubbing = Rc::clone(&scrubbing);
        move |_, _, _| scrubbing.set(false)
    });
    total.add_controller(drag);
    total.connect_change_value({
        let paused = Rc::clone(&paused);
        let demo = Rc::clone(&demo);
        move |_, _, value| {
            let u = value.clamp(0.0, 1.0);
            demo.borrow_mut().seek(Fraction::new(u));
            if paused.get().is_some() {
                paused.set(Some(u));
            }
            glib::Propagation::Proceed
        }
    });

    let last_phase = Rc::new(Cell::new((u32::MAX, u32::MAX)));
    canvas.add_tick_callback(move |canvas, _clock| {
        {
            let mut animation = demo.borrow_mut();
            if let Some(u) = paused.get() {
                // pinned clock - the animation stands still here
                animation.seek(Fraction::new(u));
                hold.set(None);
            } else if animation.is_complete() && looping.is_active() {
                // a breather between the end and the restart
                match hold.get() {
                    None => hold.set(Some(std::time::Instant::now())),
                    Some(at) if at.elapsed() >= Duration::from_secs_f64(pause.value()) => {
                        animation.start();
                        hold.set(None);
                    }
                    _ => {}
                }
            }
            on_sample(&animation);
            let progress = animation.progress().map(|p| p.as_f64()).unwrap_or(0.0).clamp(0.0, 1.0);
            if !scrubbing.get() {
                total.set_value(progress);
            }
            // the phase bar tracks what component 0's trajectory
            // sees - its own eased progress
            let eased0 = animation.first_easing().map(|e| e.apply(Fraction::new(progress)).as_f64()).unwrap_or(progress);
            if hold.get().is_some() {
                phase_bar.pulse();
            } else {
                phase_bar.set_fraction(animation.phase_fraction(eased0).clamp(0.0, 1.0));
            }
            let phase = animation.phase_at(eased0);
            if phase != last_phase.get() {
                last_phase.set(phase);
                phase_number.set_text(&format!("Phase {} / {}", phase.0, phase.1));
            }
        }
        canvas.queue_draw();
        glib::ControlFlow::Continue
    });
}

// ------------------------------------------------------------------
// Effects - a serializable snapshot of everything an example is
// configured with. Saved to JSON, loaded back onto the widgets and
// emitted as Rust code.
// ------------------------------------------------------------------

/// One column of an effect: what drives it (`role`), which
/// trajectory and easing are selected and the full knob snapshot.
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ComponentSpec {
    /// The column title, e.g. `Center x`, `H`, `Index`.
    pub title: String,
    /// The trajectory role - `"x"` scalar, `"xy"` planar,
    /// `"xyz"` spatial.
    pub role: String,
    /// The selected trajectory name, e.g. `Orbit`.
    pub trajectory: String,
    /// The selected easing name, e.g. `Elastic`.
    pub easing: String,
    /// All trajectory and easing parameters.
    pub knobs: ComponentKnobs,
}

/// A complete, serializable animation effect: the object it applies
/// to plus every component column's configuration.
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Effect {
    /// Which example created it: `geometry`, `color`, `palette`.
    pub demo: String,
    /// The object the effect animates: `Arc`, `sRGB`, `Bauhaus`, ...
    pub object: String,
    /// Secondary selections - `split`, `blend`, `verts`, ...
    pub options: std::collections::BTreeMap<String, serde_json::Value>,
    /// The run duration in seconds.
    pub duration_secs: f64,
    /// One spec per component column.
    pub components: Vec<ComponentSpec>,
}

/// Captures a component panel as a spec - names resolved against
/// the trajectory list the role offers.
pub fn spec_for_panel(title: &str, role: &str, panel: &ComponentPanel) -> ComponentSpec {
    let knobs = component_knobs(panel);
    let choices = role_choices(role);
    ComponentSpec {
        title: title.to_string(),
        role: role.to_string(),
        trajectory: choices.get(knobs.trajectory as usize).map(|name| name.to_string()).unwrap_or_default(),
        easing: EASINGS.get(knobs.easing as usize).map(|name| name.to_string()).unwrap_or_default(),
        knobs,
    }
}

/// Writes a knob snapshot back onto a panel's widgets - the inverse
/// of `component_knobs`.
pub fn apply_knobs(panel: &ComponentPanel, knobs: &ComponentKnobs) {
    panel.trajectory.set_selected(knobs.trajectory);
    panel.easing.set_selected(knobs.easing);
    panel.overshoot.set_value(knobs.overshoot);
    panel.amplitude.set_value(knobs.amplitude);
    panel.period.set_value(knobs.period);
    panel.easing_steps.set_value(f64::from(knobs.easing_steps));
    panel.steps.set_value(f64::from(knobs.trajectory_steps));
    panel.jump.set_selected(u32::from(knobs.jump_start));
    panel.bx1.set_value(knobs.bezier[0]);
    panel.by1.set_value(knobs.bezier[1]);
    panel.bx2.set_value(knobs.bezier[2]);
    panel.by2.set_value(knobs.bezier[3]);
    panel.repeat.set_value(f64::from(knobs.repeat));
    panel.cycle_easing.set_selected(knobs.cycle_easing);
    panel.delay.set_value(knobs.delay);
    panel.radius.set_value(knobs.radius);
    panel.turns.set_value(knobs.turns);
    panel.y_turns.set_value(knobs.y_turns);
    panel.tilt.set_value(knobs.tilt);
    panel.windings.set_value(knobs.windings);
    panel.depth.set_value(knobs.depth);
    for (spin, value) in panel.center.iter().zip(knobs.center.iter()) {
        spin.set_value(*value);
    }
    panel.phase.set_value(knobs.phase);
    panel.climb.lower.set(knobs.climb[0]);
    panel.climb.upper.set(knobs.climb[1]);
    panel.range.lower.set(knobs.range[0]);
    panel.range.upper.set(knobs.range[1]);
    for (spin, value) in panel.radii.iter().zip(knobs.radii.iter()) {
        spin.set_value(*value);
    }
    for (spin, value) in panel.spiro.iter().zip(knobs.spiro.iter()) {
        spin.set_value(*value);
    }
    panel.exponent.set_value(knobs.exponent);
    panel.step.set_value(knobs.step);
    panel.waves.set_value(knobs.waves);
    panel.height.set_value(knobs.height);
    panel.decay.set_value(knobs.decay);
    panel.verts.set_value(knobs.verts);
    panel.lead_easing.set_selected(knobs.lead_easing);
    panel.trail_easing.set_selected(knobs.trail_easing);
    panel.hold_easing.set_selected(knobs.hold_easing);
    panel.hold_mode.set_selected(knobs.hold_mode);
    panel.hold_to.set_value(knobs.hold_to);
    panel.keys_count.set_value(f64::from(knobs.key_count));
    for ((row, spins), (t, v)) in panel.key_rows.iter().zip(knobs.key_points.iter()) {
        let _ = row;
        spins[0].set_value(*t);
        spins[1].set_value(*v);
    }
    for ((_, spins), point) in panel.point_rows.iter().zip(knobs.points.iter()) {
        spins[0].set_value(point.x);
        spins[1].set_value(point.y);
        spins[2].set_value(point.z);
    }
    panel.lead_trail.lower.set(knobs.lead);
    panel.lead_trail.upper.set(knobs.trail);
}

/// Applies a spec to a panel: swaps the trajectory model to the
/// spec's role first (the model decides which names exist), then
/// restores the full knob snapshot.
pub fn apply_spec(panel: &ComponentPanel, spec: &ComponentSpec) {
    panel.trajectory.set_model(Some(&StringList::new(role_choices(&spec.role))));
    panel.trajectory.set_selected(0);
    apply_knobs(panel, &spec.knobs);
    if let Some(index) = role_choices(&spec.role).iter().position(|name| *name == spec.trajectory) {
        panel.trajectory.set_selected(index as u32);
    }
    if let Some(index) = EASINGS.iter().position(|name| *name == spec.easing) {
        panel.easing.set_selected(index as u32);
    }
}

/// The index of a dropdown's current label - dropdowns are
/// name-keyed so JSON stays readable.
pub fn selected_name(dropdown: &DropDown, items: &[&str]) -> String {
    items.get(dropdown.selected() as usize).map(|name| name.to_string()).unwrap_or_default()
}

/// Selects a dropdown entry by name - unknown names keep the
/// current selection.
pub fn select_by_name(dropdown: &DropDown, items: &[&str], name: &str) {
    if let Some(index) = items.iter().position(|item| *item == name) {
        dropdown.set_selected(index as u32);
    }
}

/// The file widgets of the Effect group: a path entry plus the
/// save / load / emit buttons.
pub struct EffectWidgets {
    /// The effect file path, default `effect.json`.
    pub path: gtk4::Entry,
    /// Saves the current configuration as JSON.
    pub save: gtk4::Button,
    /// Loads a JSON effect back onto the widgets.
    pub load: gtk4::Button,
    /// Emits the current configuration as Rust code next to the
    /// JSON path (`<name>.rs`).
    pub emit: gtk4::Button,
}

/// Appends the framed Effect group to `controls`.
pub fn effect_group(controls: &GtkBox) -> EffectWidgets {
    let effect_box = GtkBox::new(Orientation::Vertical, 6);
    let row = GtkBox::new(Orientation::Horizontal, 8);
    let text = Label::new(Some("File"));
    text.set_width_chars(11);
    text.set_halign(gtk4::Align::Start);
    let path = gtk4::Entry::new();
    path.set_text("effect.json");
    path.set_hexpand(true);
    row.append(&text);
    row.append(&path);
    effect_box.append(&row);
    let buttons = GtkBox::new(Orientation::Horizontal, 8);
    buttons.set_homogeneous(true);
    let save = gtk4::Button::with_label("Save");
    let load = gtk4::Button::with_label("Load");
    let emit = gtk4::Button::with_label("Emit Rust");
    buttons.append(&save);
    buttons.append(&load);
    buttons.append(&emit);
    effect_box.append(&buttons);
    controls.append(&group_frame("Effect", &effect_box));
    EffectWidgets { path, save, load, emit }
}

/// Serializes the effect to pretty JSON at `path`.
pub fn save_effect(effect: &Effect, path: &str) {
    match serde_json::to_string_pretty(effect) {
        Ok(json) => {
            if let Err(error) = std::fs::write(path, json) {
                eprintln!("effect save failed: {error}");
            }
        }
        Err(error) => eprintln!("effect serialize failed: {error}"),
    }
}

/// Reads an effect from a JSON file.
pub fn load_effect(path: &str) -> Option<Effect> {
    match std::fs::read_to_string(path) {
        Ok(json) => match serde_json::from_str(&json) {
            Ok(effect) => Some(effect),
            Err(error) => {
                eprintln!("effect parse failed: {error}");
                None
            }
        },
        Err(error) => {
            eprintln!("effect read failed: {error}");
            None
        }
    }
}

/// Emits the effect as Rust source: library constructor calls that
/// rebuild every component's eased, timed trajectory. The generated
/// file needs `smearor-commons-animation` and `-geometry`.
pub fn emit_effect_rust(effect: &Effect) -> String {
    let mut out = String::new();
    out.push_str("//! Generated by the animation playground's Emit button.\n");
    out.push_str(&format!("//! {} effect on {:?} - {} component(s).\n\n", effect.demo, effect.object, effect.components.len()));
    out.push_str("use smearor_commons_animation::*;\n");
    out.push_str("use smearor_commons_geometry::Fraction;\nuse smearor_commons_geometry::Vec2;\nuse smearor_commons_geometry::Vec3;\n");
    out.push_str("use std::time::Duration;\n\n");
    out.push_str(
        "/// Lifts a scalar trajectory onto the 2D diagonal.\n#[allow(dead_code)]\nstruct LiftF64<S>(S);\n\nimpl<S: Trajectory<Value = f64>> Trajectory for LiftF64<S> {\n    type Value = Vec2<()>;\n    fn value_at(&self, t: Fraction) -> Self::Value { Vec2::new(self.0.value_at(t), 0.85 - 0.7 * self.0.value_at(t)) }\n}\n\n",
    );
    out.push_str(
        "/// Lifts a scalar trajectory into 3D.\n#[allow(dead_code)]\nstruct LiftF64To3<S>(S);\n\nimpl<S: Trajectory<Value = f64>> Trajectory for LiftF64To3<S> {\n    type Value = Vec3<()>;\n    fn value_at(&self, t: Fraction) -> Self::Value { Vec3::new(self.0.value_at(t), 0.85 - 0.7 * self.0.value_at(t), 0.15 + 0.7 * self.0.value_at(t)) }\n}\n\n",
    );
    out.push_str(
        "/// Lifts a planar trajectory into 3D at mid depth.\n#[allow(dead_code)]\nstruct LiftVec2<S>(S);\n\nimpl<S: Trajectory<Value = Vec2<()>>> Trajectory for LiftVec2<S> {\n    type Value = Vec3<()>;\n    fn value_at(&self, t: Fraction) -> Self::Value { let v = self.0.value_at(t); Vec3::new(v.x, v.y, 0.5) }\n}\n\n",
    );
    for (index, spec) in effect.components.iter().enumerate() {
        let value = match spec.role.as_str() {
            "xy" => "Vec2<()>",
            "xyz" => "Vec3<()>",
            _ => "f64",
        };
        out.push_str(&format!(
            "/// {} - {} eased with {}.\npub fn component_{index}() -> TimedAnimation<Box<dyn Trajectory<Value = {value}>>> {{\n",
            spec.title, spec.trajectory, spec.easing
        ));
        out.push_str(&format!(
            "    TimedAnimation::new(\n        Box::new(Eased::new(\n            {},\n            {}\n        )),\n        Duration::from_secs_f64({:?}),\n        EasingFunction::Linear,\n    )\n}}\n\n",
            emit_trajectory(spec, 12),
            emit_easing_by_name(&spec.easing, &spec.knobs),
            effect.duration_secs
        ));
    }
    out
}

/// The `EasingFunction` expression of the spec's own easing
/// selection.
fn emit_easing(knobs: &ComponentKnobs) -> String {
    emit_easing_by_name(EASINGS.get(knobs.easing as usize).copied().unwrap_or("Linear"), knobs)
}

/// The `EasingFunction` expression for a named easing - the
/// parameterized variants read the spec's sliders.
fn emit_easing_by_name(name: &str, knobs: &ComponentKnobs) -> String {
    match name {
        "Linear" => "EasingFunction::Linear".into(),
        "EaseIn" => "EasingFunction::EaseIn".into(),
        "EaseOut" => "EasingFunction::EaseOut".into(),
        "EaseInOut" => "EasingFunction::EaseInOut".into(),
        "EaseInCubic" => "EasingFunction::EaseInCubic".into(),
        "EaseOutCubic" => "EasingFunction::EaseOutCubic".into(),
        "EaseInOutCubic" => "EasingFunction::EaseInOutCubic".into(),
        "EaseInQuart" => "EasingFunction::EaseInQuart".into(),
        "EaseOutQuart" => "EasingFunction::EaseOutQuart".into(),
        "EaseInOutQuart" => "EasingFunction::EaseInOutQuart".into(),
        "EaseInQuint" => "EasingFunction::EaseInQuint".into(),
        "EaseOutQuint" => "EasingFunction::EaseOutQuint".into(),
        "EaseInOutQuint" => "EasingFunction::EaseInOutQuint".into(),
        "EaseInSine" => "EasingFunction::EaseInSine".into(),
        "EaseOutSine" => "EasingFunction::EaseOutSine".into(),
        "EaseInOutSine" => "EasingFunction::EaseInOutSine".into(),
        "EaseInExpo" => "EasingFunction::EaseInExpo".into(),
        "EaseOutExpo" => "EasingFunction::EaseOutExpo".into(),
        "EaseInOutExpo" => "EasingFunction::EaseInOutExpo".into(),
        "EaseInCirc" => "EasingFunction::EaseInCirc".into(),
        "EaseOutCirc" => "EasingFunction::EaseOutCirc".into(),
        "EaseInOutCirc" => "EasingFunction::EaseInOutCirc".into(),
        "Overshoot" => format!("EasingFunction::Overshoot {{ overshoot_amount: {:?} }}", knobs.overshoot),
        "BackIn" => format!("EasingFunction::BackIn {{ overshoot_amount: {:?} }}", knobs.overshoot),
        "BackOut" => format!("EasingFunction::BackOut {{ overshoot_amount: {:?} }}", knobs.overshoot),
        "Elastic" => format!("EasingFunction::Elastic {{ amplitude: {:?}, period: {:?} }}", knobs.amplitude, knobs.period),
        "Bounce" => "EasingFunction::Bounce".into(),
        "CubicBezier" => format!(
            "EasingFunction::CubicBezier {{ x1: {:?}, y1: {:?}, x2: {:?}, y2: {:?} }}",
            knobs.bezier[0], knobs.bezier[1], knobs.bezier[2], knobs.bezier[3]
        ),
        "Steps" => format!(
            "EasingFunction::Steps {{ steps: {}, jump: StepsJump::{} }}",
            knobs.easing_steps,
            if knobs.jump_start { "Start" } else { "End" }
        ),
        _ => "EasingFunction::Linear".into(),
    }
}

/// The inner-trajectory expression of the scalar decorators - a
/// `Span` over the spec's configured value range.
fn emit_span_inner(knobs: &ComponentKnobs) -> String {
    format!("Span::new({:?}, {:?})", knobs.range[0], knobs.range[1])
}

/// The trajectory expression for a component spec - mirrors the
/// runtime builders, emitting library constructor calls.
fn emit_trajectory(spec: &ComponentSpec, _indent: usize) -> String {
    let k = &spec.knobs;
    let scalar = |name: &str| -> Option<String> {
        match name {
            "ThreePhaseSpan" => {
                let lead = emit_easing_by_name(PHASE_EASINGS[k.lead_easing as usize % PHASE_EASINGS.len()], k);
                let trail = emit_easing_by_name(PHASE_EASINGS[k.trail_easing as usize % PHASE_EASINGS.len()], k);
                Some(format!(
                    "ThreePhaseSpan::new({:?}, {:?}, {:?}, Fraction::new({:?}), Fraction::new({:?})).with_easing({lead}, {trail})",
                    k.range[0],
                    (k.range[0] + k.range[1]) / 2.0,
                    k.range[1],
                    k.lead,
                    k.trail
                ))
            }
            "Keyframes" => {
                let keys: Vec<String> = k
                    .key_points
                    .iter()
                    .take(k.key_count.max(2) as usize)
                    .map(|(t, v)| format!("(Fraction::new({t:?}), {v:?})"))
                    .collect();
                Some(format!(
                    "Keyframes::new({}, vec![{}])",
                    keys.first().cloned().unwrap_or_default(),
                    keys.get(1..).unwrap_or(&[]).join(", ")
                ))
            }
            "Repeat" => Some(format!("Repeat::new({}, {})", emit_span_inner(k), k.repeat)),
            "Mirror" => Some(format!("Mirror::new({})", emit_span_inner(k))),
            "Delay" => Some(format!("Delay::new({}, Fraction::new({:?}))", emit_span_inner(k), k.delay)),
            "Steps" => {
                let n = k.trajectory_steps.max(2);
                Some(format!(
                    "Steps::new({:?}, (1..{n}).map(|i| {:?} + i as f64 / ({n} - 1) as f64 * ({:?} - {:?})))",
                    k.range[0], k.range[0], k.range[1], k.range[0]
                ))
            }
            "Span" => Some(emit_span_inner(k)),
            _ => None,
        }
    };
    let c = || format!("Vec2::new({:?}, {:?})", k.center[0], k.center[1]);
    let planar = |name: &str| -> Option<String> {
        match name {
            "Orbit" => Some(format!("Orbit::new({}, {:?}).with_turns({:?})", c(), k.radius, k.turns)),
            "Lissajous" => Some(format!(
                "Lissajous::new({}, {:?}, {:?}, {:?}).with_phase({:?})",
                c(),
                k.radius,
                k.turns,
                k.y_turns,
                k.phase * std::f64::consts::TAU
            )),
            "Spiral" => Some(format!("Spiral::new({}, {:?}).with_turns({:?})", c(), k.radius, k.turns)),
            "PolygonPath" => Some(format!("PolygonPath::new({})", emit_points2(k))),
            "CatmullRom" => Some(format!("CatmullRom::new({})", emit_points2(k))),
            "Lemniskate" => Some(format!("Lemniskate::new({}, {:?}).with_turns({:?})", c(), k.radius, k.turns)),
            "Ellipse" => Some(format!(
                "Ellipse::new({}, {:?}, {:?}).with_rotation({:?}).with_turns({:?})",
                c(),
                k.radii[0],
                k.radii[1],
                k.phase * std::f64::consts::TAU,
                k.turns
            )),
            "Rose" => Some(format!(
                "Rose::new({}, {:?}, {:?}).with_phase({:?}).with_turns({:?})",
                c(),
                k.radius,
                k.windings,
                k.phase * std::f64::consts::TAU,
                k.turns
            )),
            "Spiro" => Some(format!("Spiro::new({}, {:?}, {:?}, {:?}).with_turns({:?})", c(), k.spiro[0], k.spiro[1], k.spiro[2], k.turns)),
            "Superellipse" => Some(format!(
                "Superellipse::new({}, {:?}, {:?}, {:?}).with_turns({:?})",
                c(),
                k.radii[0],
                k.radii[1],
                k.exponent,
                k.turns
            )),
            "StarPath" => Some(format!(
                "StarPath::regular({}, {:?}, {}).with_step({})",
                c(),
                k.radius,
                k.verts as usize,
                (k.step as usize).min((k.verts as usize).saturating_sub(1).max(1))
            )),
            "WaveTrail" => Some(format!("WaveTrail::new({}, {}, {:?}, {:?})", emit_point2(k, 0), emit_point2(k, 1), k.height, k.waves)),
            "BouncePath" => Some(format!(
                "BouncePath::new({}, {}, {:?}, {:?}, {:?})",
                emit_point2(k, 0),
                emit_point2(k, 1),
                k.height,
                k.turns,
                k.decay
            )),
            "BezierPath" => Some(format!("BezierPath::new({}, vec![{}])", emit_point2(k, 0), emit_points2_rest(k))),
            _ => None,
        }
    };
    let spatial = |name: &str| -> Option<String> {
        match name {
            "Helix" => Some(format!("Helix::new({}, {:?}, {:?}, {:?}).with_turns({:?})", c(), k.radius, k.climb[0], k.climb[1], k.turns)),
            "SphericalOrbit" => Some(format!(
                "SphericalOrbit::new({}, {:?}, {:?}).with_turns({:?})",
                c(),
                k.radius,
                k.tilt * std::f64::consts::FRAC_PI_2,
                k.turns
            )),
            "TorusKnot" => Some(format!(
                "TorusKnot::new({}, {:?}, {}, {})",
                c(),
                k.radius,
                k.turns.round().max(1.0) as u32,
                k.windings.round().max(1.0) as u32
            )),
            "Figure8" => Some(format!("Figure8::new({}, {:?}, {:?}).with_turns({:?})", c(), k.radius, k.depth, k.turns)),
            "ConicalHelix" => Some(format!(
                "ConicalHelix::new({}, {:?}, {:?}, {:?}, {:?}).with_turns({:?})",
                c(),
                k.radii[0],
                k.radii[1],
                k.climb[0],
                k.climb[1],
                k.turns
            )),
            "Viviani" => Some(format!("Viviani::new({}, {:?}).with_turns({:?})", c(), k.radius, k.turns)),
            "Mobius" => Some(format!("Mobius::new({}, {:?}, {:?})", c(), k.radius, k.depth)),
            "SphereLissajous" => Some(format!("SphereLissajous::new({}, {:?}, {:?}, {:?})", c(), k.radius, k.turns, k.y_turns)),
            "Lorenz" => Some(format!("Lorenz::new({}, {:?})", c(), k.radius)),
            "BezierPath" => Some(format!("BezierPath::new({}, vec![{}])", emit_point3(k, 0), emit_points3_rest(k))),
            _ => None,
        }
    };
    match spec.role.as_str() {
        "xy" => planar(&spec.trajectory)
            .or_else(|| scalar(&spec.trajectory).map(|inner| format!("LiftF64({inner})")))
            .unwrap_or_else(|| "LiftF64(Span::new(0.0_f64, 1.0))".into()),
        "xyz" => spatial(&spec.trajectory)
            .or_else(|| scalar(&spec.trajectory).map(|inner| format!("LiftF64To3({inner})")))
            .unwrap_or_else(|| "LiftF64To3(Span::new(0.0_f64, 1.0))".into()),
        _ => scalar(&spec.trajectory).unwrap_or_else(|| emit_span_inner(k)),
    }
}

/// A `Vec2::new` literal of control point `i`.
fn emit_point2(k: &ComponentKnobs, i: usize) -> String {
    let point = k.points.get(i).copied().unwrap_or_default();
    format!("Vec2::new({:?}, {:?})", point.x, point.y)
}

/// The control points as a `vec![Vec2; ...]` literal.
fn emit_points2(k: &ComponentKnobs) -> String {
    let points: Vec<String> = k.points.iter().map(|p| format!("Vec2::new({:?}, {:?})", p.x, p.y)).collect();
    format!("vec![{}]", points.join(", "))
}

/// The control points after the first as a `vec![Vec2; ...]` literal.
fn emit_points2_rest(k: &ComponentKnobs) -> String {
    let points: Vec<String> = k.points.iter().skip(1).map(|p| format!("Vec2::new({:?}, {:?})", p.x, p.y)).collect();
    points.join(", ")
}

/// A `Vec3::new` literal of control point `i`.
fn emit_point3(k: &ComponentKnobs, i: usize) -> String {
    let point = k.points.get(i).copied().unwrap_or_default();
    format!("Vec3::new({:?}, {:?}, {:?})", point.x, point.y, point.z)
}

/// The control points after the first as a `vec![Vec3; ...]` literal.
fn emit_points3_rest(k: &ComponentKnobs) -> String {
    let points: Vec<String> = k.points.iter().skip(1).map(|p| format!("Vec3::new({:?}, {:?}, {:?})", p.x, p.y, p.z)).collect();
    points.join(", ")
}
