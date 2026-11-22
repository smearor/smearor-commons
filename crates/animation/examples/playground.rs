//! GTK playground for the animation crate: pick a trajectory and an
//! easing function, tune their parameters with sliders and watch the
//! result on the frame clock.
//!
//! Run with `cargo run -p smearor-commons-animation --features gtk4 --example playground`.

use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Box as GtkBox;
use gtk4::DrawingArea;
use gtk4::DropDown;
use gtk4::Label;
use gtk4::Orientation;
use gtk4::Scale;
use gtk4::StringList;
use gtk4::glib;
use gtk4::prelude::*;
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
use smearor_commons_animation::TimedAnimation;
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

const APP_ID: &str = "io.smearor.commons_animation.playground";

/// The scalar trajectories - they run in every dimension via the
/// `LiftF64`/`LiftF64To3` wrappers.
const SCALAR_TRAJECTORIES: &[&str] = &["Span", "ThreePhaseSpan", "Keyframes", "Repeat", "Mirror", "Delay", "Steps"];

/// The 2D list adds the planar natives - bezier path and orbit.
const TRAJECTORIES_2D: &[&str] = &[
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
const TRAJECTORIES_3D: &[&str] = &[
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
fn compositions(dimension: u32) -> &'static [&'static str] {
    match dimension {
        1 => &["2D", "1D + 1D"],
        2 => &["3D", "1D + 1D + 1D", "2D + 1D", "1D + 2D"],
        _ => &["1D"],
    }
}

/// The axis group each component slot drives in `(dimension,
/// composition)` - `"xy"` takes a planar trajectory, `"xyz"` a
/// spatial one, single letters scalars.
fn component_roles(dimension: u32, composition: u32) -> &'static [&'static str] {
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
fn role_choices(role: &str) -> &'static [&'static str] {
    match role.len() {
        2 => TRAJECTORIES_2D,
        3 => TRAJECTORIES_3D,
        _ => SCALAR_TRAJECTORIES,
    }
}

/// The easing functions the playground offers - parameterized
/// variants read their parameters from the sliders at build time.
const EASINGS: &[&str] = &[
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
const PHASE_EASINGS: &[&str] = &[
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
enum Params {
    None,
    Overshoot,
    Elastic,
    CubicBezier,
    Steps,
}

/// Which segment the "phase" progress bar subdivides for a given
/// trajectory.
enum PhaseShape {
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
    fn phase_at(&self, p: f64) -> (u32, u32) {
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
    fn fraction(&self, p: f64) -> f64 {
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

/// The animation under test - one boxed slot per value dimension.
/// `control_points` carries the bezier handles for drawing.
/// A draggable center marker of an orbital trajectory component.
struct CenterMark {
    /// The center in model coordinates.
    point: Vec2<()>,
    /// The component slot the marker controls.
    slot: usize,
}

enum Demo {
    /// 1D - scalar trajectories on the x axis.
    Scalar {
        /// The timed animation.
        animation: TimedAnimation<Box<dyn Trajectory<Value = f64>>>,
        /// The segment layout for the phase bar.
        phase: PhaseShape,
        /// The per-component easings - drawn as background curves.
        easings: Vec<EasingFunction>,
    },
    /// 2D - planar trajectories.
    Planar {
        /// The timed animation.
        animation: TimedAnimation<Box<dyn Trajectory<Value = Vec2<()>>>>,
        /// The segment layout for the phase bar.
        phase: PhaseShape,
        /// Bezier handles drawn as reference points.
        control_points: Vec<Vec2<()>>,
        /// Draggable centers of orbital components.
        centers: Vec<CenterMark>,
        /// The per-component easings - drawn as background curves.
        easings: Vec<EasingFunction>,
    },
    /// 3D - spatial trajectories, `z` scales the logo.
    Spatial {
        /// The timed animation.
        animation: TimedAnimation<Box<dyn Trajectory<Value = Vec3<()>>>>,
        /// The segment layout for the phase bar.
        phase: PhaseShape,
        /// Bezier handles drawn as reference points.
        control_points: Vec<Vec2<()>>,
        /// Draggable centers of orbital components.
        centers: Vec<CenterMark>,
        /// The per-component easings - drawn as background curves.
        easings: Vec<EasingFunction>,
    },
}

impl Demo {
    /// The current value as `(point, scale)` - scalar demos sit on
    /// the mid line, spatial demos scale the logo with `z`.
    fn value(&self) -> Option<(Vec2<()>, f64)> {
        match self {
            Self::Scalar { animation, .. } => animation.value().map(|v| (Vec2::new(v, 0.5), 1.0)),
            Self::Planar { animation, .. } => animation.value().map(|v| (v, 1.0)),
            Self::Spatial { animation, .. } => animation.value().map(|v| (Vec2::new(v.x, v.y), 0.4 + 0.8 * v.z.clamp(0.0, 1.0))),
        }
    }

    fn progress(&self) -> Option<Fraction> {
        match self {
            Self::Scalar { animation, .. } => animation.progress(),
            Self::Planar { animation, .. } => animation.progress(),
            Self::Spatial { animation, .. } => animation.progress(),
        }
    }

    /// The trajectory value at eased progress `t` - scalar demos
    /// report their value as x.
    fn sample(&self, t: Fraction) -> Vec2<()> {
        match self {
            Self::Scalar { animation, .. } => Vec2::new(animation.trajectory.value_at(t), 0.5),
            Self::Planar { animation, .. } => animation.trajectory.value_at(t),
            Self::Spatial { animation, .. } => {
                let v = animation.trajectory.value_at(t);
                Vec2::new(v.x, v.y)
            }
        }
    }

    /// The draggable center markers of orbital components.
    fn centers(&self) -> &[CenterMark] {
        match self {
            Self::Scalar { .. } => &[],
            Self::Planar { centers, .. } | Self::Spatial { centers, .. } => centers,
        }
    }

    /// The per-component easings - one curve per split component.
    fn easings(&self) -> &[EasingFunction] {
        match self {
            Self::Scalar { easings, .. } | Self::Planar { easings, .. } | Self::Spatial { easings, .. } => easings,
        }
    }

    fn start(&mut self) {
        match self {
            Self::Scalar { animation, .. } => animation.start(),
            Self::Planar { animation, .. } => animation.start(),
            Self::Spatial { animation, .. } => animation.start(),
        }
    }

    /// The current trajectory segment as `(index, count)`.
    fn phase_at(&self, progress: f64) -> (u32, u32) {
        match self {
            Self::Scalar { phase, .. } | Self::Planar { phase, .. } | Self::Spatial { phase, .. } => phase.phase_at(progress),
        }
    }

    /// Progress within the current trajectory segment.
    fn phase_fraction(&self, progress: f64) -> f64 {
        match self {
            Self::Scalar { phase, .. } | Self::Planar { phase, .. } | Self::Spatial { phase, .. } => phase.fraction(progress),
        }
    }

    fn is_complete(&self) -> bool {
        match self {
            Self::Scalar { animation, .. } => animation.is_complete(),
            Self::Planar { animation, .. } => animation.is_complete(),
            Self::Spatial { animation, .. } => animation.is_complete(),
        }
    }

    /// Bezier control points for drawing - empty for other shapes.
    fn control_points(&self) -> &[Vec2<()>] {
        match self {
            Self::Scalar { .. } => &[],
            Self::Planar { control_points, .. } | Self::Spatial { control_points, .. } => control_points,
        }
    }
}

/// Lifts a scalar trajectory onto the 2D diagonal.
struct LiftF64<S>(S);

impl<S: Trajectory<Value = f64>> Trajectory for LiftF64<S> {
    type Value = Vec2<()>;

    fn value_at(&self, t: Fraction) -> Self::Value {
        v2(self.0.value_at(t))
    }
}

/// Lifts a scalar trajectory into 3D.
struct LiftF64To3<S>(S);

impl<S: Trajectory<Value = f64>> Trajectory for LiftF64To3<S> {
    type Value = Vec3<()>;

    fn value_at(&self, t: Fraction) -> Self::Value {
        v3(self.0.value_at(t))
    }
}

/// Composes two scalar trajectories into a planar value - each
/// drives its own axis.
struct Compose2<X, Y> {
    x: X,
    y: Y,
}

impl<X: Trajectory<Value = f64>, Y: Trajectory<Value = f64>> Trajectory for Compose2<X, Y> {
    type Value = Vec2<()>;

    fn value_at(&self, t: Fraction) -> Vec2<()> {
        Vec2::new(self.x.value_at(t), self.y.value_at(t))
    }
}

/// A planar trajectory on `x`/`y` plus a scalar on `z`.
struct ComposeXY<XY, Z> {
    xy: XY,
    z: Z,
}

impl<XY: Trajectory<Value = Vec2<()>>, Z: Trajectory<Value = f64>> Trajectory for ComposeXY<XY, Z> {
    type Value = Vec3<()>;

    fn value_at(&self, t: Fraction) -> Vec3<()> {
        let xy = self.xy.value_at(t);
        Vec3::new(xy.x, xy.y, self.z.value_at(t))
    }
}

/// A scalar on `x` plus a planar trajectory on `y`/`z`.
struct ComposeYZ<X, YZ> {
    x: X,
    yz: YZ,
}

impl<X: Trajectory<Value = f64>, YZ: Trajectory<Value = Vec2<()>>> Trajectory for ComposeYZ<X, YZ> {
    type Value = Vec3<()>;

    fn value_at(&self, t: Fraction) -> Vec3<()> {
        let yz = self.yz.value_at(t);
        Vec3::new(self.x.value_at(t), yz.x, yz.y)
    }
}

/// The per-component parameters - trajectory choice, easing
/// choice and both parameter sets. Each split component owns one.
struct ComponentKnobs {
    trajectory: u32,
    easing: u32,
    overshoot: f64,
    amplitude: f64,
    period: f64,
    easing_steps: u32,
    trajectory_steps: u32,
    jump_start: bool,
    bezier: [f64; 4],
    repeat: u32,
    cycle_easing: u32,
    delay: f64,
    radius: f64,
    turns: f64,
    y_turns: f64,
    tilt: f64,
    windings: f64,
    depth: f64,
    center: [f64; 2],
    phase: f64,
    climb: [f64; 2],
    radii: [f64; 2],
    spiro: [f64; 3],
    exponent: f64,
    step: f64,
    waves: f64,
    height: f64,
    decay: f64,
    verts: f64,
    lead_easing: u32,
    trail_easing: u32,
    hold_easing: u32,
    hold_mode: u32,
    hold_to: f64,
    key_count: u32,
    key_points: Vec<(f64, f64)>,
    points: Vec<Vec3<()>>,
    lead: f64,
    trail: f64,
}

/// The global parameters - layout and timing, not per-component.
struct Knobs {
    duration: Duration,
    dimension: u32,
    composition: u32,
    components: [ComponentKnobs; 3],
}

/// Builds a component's easing function by name.
fn easing_by_name(name: &str, knobs: &ComponentKnobs) -> (EasingFunction, Params) {
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
fn v2(v: f64) -> Vec2<()> {
    Vec2::new(v, 0.85 - 0.7 * v)
}

/// Maps the scalar demo value into 3D - depth grows with `v`, so
/// the logo scales up along the run.
fn v3(v: f64) -> Vec3<()> {
    Vec3::new(v, 0.85 - 0.7 * v, 0.15 + 0.7 * v)
}

/// The demo's bezier control polygon - `z` only matters in 3D.
fn bezier_points() -> Vec<Vec3<()>> {
    vec![
        Vec3::new(0.05, 0.8, 0.0),
        Vec3::new(0.3, -0.2, 0.3),
        Vec3::new(0.7, 1.2, 0.7),
        Vec3::new(0.95, 0.2, 1.0),
    ]
}

/// The demo's bezier path over a control polygon - two points a
/// line, three quadratic, four cubic, beyond that de Casteljau of
/// higher order.
fn bezier_path(points: &[Vec3<()>]) -> BezierPath {
    let rest: Vec<Vec2<()>> = points.iter().skip(1).map(|p| Vec2::<()>::new(p.x, p.y)).collect();
    let first = points.first().copied().unwrap_or_default();
    BezierPath::new(Vec2::<()>::new(first.x, first.y), rest)
}

/// The demo's spatial bezier - a true volumetric curve through
/// the 3D control points.
fn bezier3_path(points: &[Vec3<()>]) -> BezierPath<Vec3<()>> {
    let rest: Vec<Vec3<()>> = points.iter().skip(1).copied().collect();
    BezierPath::new(points.first().copied().unwrap_or_default(), rest)
}

/// Builds the scalar trajectory + phase shape from the controls -
/// 2D and 3D demos lift its values onto the diagonal.
fn build_scalar_trajectory(name: &str, knobs: &ComponentKnobs) -> (Box<dyn Trajectory<Value = f64>>, PhaseShape) {
    let span = || Span::new(0.0_f64, 1.0);
    match name {
        "ThreePhaseSpan" => (
            Box::new({
                let span = ThreePhaseSpan::new(0.0_f64, 0.5, 1.0, Fraction::new(knobs.lead), Fraction::new(knobs.trail)).with_easing(
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
            (Box::new(Steps::new(0.0_f64, (1..n).map(|i| i as f64 / (n - 1) as f64))), PhaseShape::Steps(n))
        }
        _ => (Box::new(span()), PhaseShape::Identity),
    }
}

/// The shared center knob of every orbital trajectory.
fn center(knobs: &ComponentKnobs) -> Vec2<()> {
    Vec2::<()>::new(knobs.center[0], knobs.center[1])
}

/// The planar trajectory + phase shape for a 2D component - the
/// native paths, else the scalar lifted onto the diagonal.
fn planar_trajectory(name: &str, knobs: &ComponentKnobs) -> (Box<dyn Trajectory<Value = Vec2<()>>>, PhaseShape) {
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
fn spatial_trajectory(name: &str, knobs: &ComponentKnobs) -> (Box<dyn Trajectory<Value = Vec3<()>>>, PhaseShape) {
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

/// Builds the animation from the current control state - the
/// composition splits the value axes into independently driven
/// components, each with its own trajectory.
fn build_demo(knobs: &Knobs) -> Demo {
    let roles = component_roles(knobs.dimension, knobs.composition);
    let name = |slot: usize| role_choices(roles[slot])[knobs.components[slot].trajectory as usize];
    // each component eases its own progress - the clock runs linear
    let component = |slot: usize, trajectory: Box<dyn Trajectory<Value = f64>>| -> Box<dyn Trajectory<Value = f64>> {
        let (easing, _) = easing_by_name(EASINGS[knobs.components[slot].easing as usize], &knobs.components[slot]);
        Box::new(Eased::new(trajectory, easing))
    };
    let planar = |slot: usize| -> (Box<dyn Trajectory<Value = Vec2<()>>>, PhaseShape) {
        let (trajectory, phase) = planar_trajectory(name(slot), &knobs.components[slot]);
        let (easing, _) = easing_by_name(EASINGS[knobs.components[slot].easing as usize], &knobs.components[slot]);
        (Box::new(Eased::new(trajectory, easing)), phase)
    };
    let spatial = |slot: usize| -> (Box<dyn Trajectory<Value = Vec3<()>>>, PhaseShape) {
        let (trajectory, phase) = spatial_trajectory(name(slot), &knobs.components[slot]);
        let (easing, _) = easing_by_name(EASINGS[knobs.components[slot].easing as usize], &knobs.components[slot]);
        (Box::new(Eased::new(trajectory, easing)), phase)
    };
    let scalar = |slot: usize| -> (Box<dyn Trajectory<Value = f64>>, PhaseShape) {
        let (trajectory, phase) = build_scalar_trajectory(name(slot), &knobs.components[slot]);
        (component(slot, trajectory), phase)
    };
    let easings = |count: usize| {
        knobs.components[..count]
            .iter()
            .map(|c| easing_by_name(EASINGS[c.easing as usize], c).0)
            .collect()
    };
    let points = |knobs: &ComponentKnobs| knobs.points.iter().map(|p| Vec2::<()>::new(p.x, p.y)).collect();
    // orbital components own their center knob - the marker mirrors it
    let centers = |count: usize| -> Vec<CenterMark> {
        (0..count)
            .filter(|&slot| ORBITAL.contains(&name(slot)))
            .map(|slot| CenterMark {
                point: Vec2::<()>::new(knobs.components[slot].center[0], knobs.components[slot].center[1]),
                slot,
            })
            .collect()
    };
    let duration = knobs.duration;
    let linear = EasingFunction::Linear;
    match roles {
        ["x"] => {
            let (trajectory, phase) = scalar(0);
            Demo::Scalar {
                animation: TimedAnimation::new(trajectory, duration, linear),
                phase,
                easings: easings(1),
            }
        }
        ["xy"] => {
            let (trajectory, phase) = planar(0);
            Demo::Planar {
                animation: TimedAnimation::new(trajectory, duration, linear),
                phase,
                control_points: if POINT_PATHS.contains(&name(0)) {
                    points(&knobs.components[0])
                } else {
                    Vec::new()
                },
                centers: centers(1),
                easings: easings(1),
            }
        }
        ["x", "y"] => {
            let (x, phase) = scalar(0);
            let (y, _) = scalar(1);
            Demo::Planar {
                animation: TimedAnimation::new(Box::new(Compose2 { x, y }), duration, linear),
                phase,
                control_points: Vec::new(),
                centers: Vec::new(),
                easings: easings(2),
            }
        }
        ["xyz"] => {
            let (trajectory, phase) = spatial(0);
            Demo::Spatial {
                animation: TimedAnimation::new(trajectory, duration, linear),
                phase,
                control_points: if POINT_PATHS.contains(&name(0)) {
                    points(&knobs.components[0])
                } else {
                    Vec::new()
                },
                centers: centers(1),
                easings: easings(1),
            }
        }
        ["x", "y", "z"] => {
            let (x, phase) = scalar(0);
            let (y, _) = scalar(1);
            let (z, _) = scalar(2);
            Demo::Spatial {
                animation: TimedAnimation::new(Box::new(ComposeXY { xy: Compose2 { x, y }, z }), duration, linear),
                phase,
                control_points: Vec::new(),
                centers: Vec::new(),
                easings: easings(3),
            }
        }
        ["xy", "z"] => {
            let (xy, phase) = planar(0);
            let (z, _) = scalar(1);
            Demo::Spatial {
                animation: TimedAnimation::new(Box::new(ComposeXY { xy, z }), duration, linear),
                phase,
                control_points: if POINT_PATHS.contains(&name(0)) {
                    points(&knobs.components[0])
                } else {
                    Vec::new()
                },
                centers: centers(2),
                easings: easings(2),
            }
        }
        ["x", "yz"] => {
            let (x, phase) = scalar(0);
            let (yz, _) = planar(1);
            Demo::Spatial {
                animation: TimedAnimation::new(Box::new(ComposeYZ { x, yz }), duration, linear),
                phase,
                control_points: if POINT_PATHS.contains(&name(1)) {
                    points(&knobs.components[1])
                } else {
                    Vec::new()
                },
                centers: centers(2),
                easings: easings(2),
            }
        }
        _ => unreachable!("component_roles covers every composition"),
    }
}

/// The plot domain `(x_min, x_max, y_min, y_max)` - fitted to the
/// sampled trajectory, the per-component easing curves and the
/// markers. Overshoot, back and elastic leave `0..=1` by design.
fn plot_domain(demo: &Demo) -> (f64, f64, f64, f64) {
    let mut x_min = 0.0_f64;
    let mut x_max = 1.0_f64;
    let mut y_min = 0.0_f64;
    let mut y_max = 1.0_f64;
    for i in 0..=200 {
        let t = Fraction::new(i as f64 / 200.0);
        for easing in demo.easings() {
            let eased = easing.apply(t).as_f64();
            if eased.is_finite() {
                y_min = y_min.min(eased);
                y_max = y_max.max(eased);
            }
        }
        // the easings live inside the trajectories - the linear
        // sample already reflects them
        let value = demo.sample(t);
        x_min = x_min.min(value.x);
        x_max = x_max.max(value.x);
        y_min = y_min.min(value.y);
        y_max = y_max.max(value.y);
        for point in demo.control_points() {
            x_min = x_min.min(point.x);
            x_max = x_max.max(point.x);
            y_min = y_min.min(point.y);
            y_max = y_max.max(point.y);
        }
    }
    for mark in demo.centers() {
        x_min = x_min.min(mark.point.x);
        x_max = x_max.max(mark.point.x);
        y_min = y_min.min(mark.point.y);
        y_max = y_max.max(mark.point.y);
    }
    let x_pad = 0.25 * (x_max - x_min).max(0.2);
    let y_pad = 0.25 * (y_max - y_min).max(0.2);
    (x_min - x_pad, x_max + x_pad, y_min - y_pad, y_max + y_pad)
}

/// A labeled slider row for the sidebar - returns the row too, so
/// parameter groups can hide it entirely.
fn slider(parent: &GtkBox, label: &str, min: f64, max: f64, value: f64, digits: i32) -> (GtkBox, Scale) {
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
type ChangeListeners = Rc<RefCell<Vec<Box<dyn Fn()>>>>;

/// A two-grip range slider for `lead <= trail` style parameters.
/// Drawn on a `DrawingArea`; dragging grabs the nearest grip and
/// the grips can not pass each other.
struct RangeSlider {
    /// The lower grip value, `0..=upper`.
    lower: Rc<Cell<f64>>,
    /// The upper grip value, `lower..=1`.
    upper: Rc<Cell<f64>>,
    /// Change listeners run on every grip move.
    listeners: ChangeListeners,
}

impl RangeSlider {
    /// Registers a change listener - the `connect_changed` pattern
    /// of the builtin widgets.
    fn connect_changed(&self, listener: impl Fn() + 'static) {
        self.listeners.borrow_mut().push(Box::new(listener));
    }
}

/// Trajectories with a `center` knob - the center is drawn on the
/// canvas as a draggable marker.
const ORBITAL: &[&str] = &[
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
const POINT_PATHS: &[&str] = &["BezierPath", "PolygonPath", "CatmullRom", "WaveTrail", "BouncePath"];

/// A labeled two-grip range slider row for the sidebar - the
/// grips carry `min..=max` values and `marks` draws the scale.
fn range_slider(parent: &GtkBox, label: &str, min: f64, max: f64, lower: f64, upper: f64, marks: &[f64]) -> (GtkBox, RangeSlider) {
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
            context.set_source_rgb(0.2, 0.45, 0.8);
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
fn dropdown_row(parent: &GtkBox, label: &str, items: &[&str]) -> (GtkBox, DropDown) {
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

/// A bezier control-point row: `x`/`y`/`z` spin buttons behind the
/// point label. The `z` field is hidden in 2D - the caller flips it
/// per dimension.
/// A row of labeled spin buttons behind a row label - one field per
/// `(axis, min, max, value)` spec. Used for bezier control points
/// (`x`/`y`/`z`) and keyframes (`t`/`v`); fields can be hidden per
/// context by the caller.
fn spin_row(parent: &GtkBox, label: &str, specs: &[(&str, f64, f64, f64)]) -> (GtkBox, Vec<gtk4::SpinButton>) {
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

fn point_row(parent: &GtkBox, label: &str, default: Vec3<()>) -> (GtkBox, Vec<gtk4::SpinButton>) {
    spin_row(parent, label, &[("x", -0.5, 1.5, default.x), ("y", -0.5, 1.5, default.y), ("z", -1.5, 1.5, default.z)])
}

/// The default keyframes - a wavy curve the spin rows start
/// from. The "Keys" count picks the first `n`.
const DEFAULT_KEYS: [(f64, f64); 8] = [
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
fn group_frame(title: &str, child: &GtkBox) -> gtk4::Frame {
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
struct ComponentPanel {
    /// The column wrapping the whole panel.
    column: GtkBox,
    /// The axes heading on top (`"Axis x"`, `"Axis yz"`, ...).
    title: Label,
    /// The trajectory dropdown - model swapped per role.
    trajectory: DropDown,
    /// The easing dropdown.
    easing: DropDown,
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
    center_row: GtkBox,
    center: Vec<gtk4::SpinButton>,
    phase_row: GtkBox,
    phase: Scale,
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
fn component_panel(parent: &GtkBox) -> ComponentPanel {
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
    let (lead_trail_row, lead_trail) = range_slider(&trajectory_box, "Lead/Trail", 0.0, 1.0, 0.3, 0.7, &[0.0, 0.25, 0.5, 0.75, 1.0]);
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
    let (climb_row, climb) = range_slider(&trajectory_box, "Climb", -2.0, 2.0, -1.0, 1.0, &[-2.0, -1.0, 0.0, 1.0, 2.0]);
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
fn component_knobs(panel: &ComponentPanel) -> ComponentKnobs {
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
fn connect_panel(panel: &ComponentPanel, restart: &Rc<dyn Fn()>) {
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
}

/// Shows only the parameter rows a component's selection actually
/// reads - `name` is the trajectory, `params` its easing family,
/// `spatial` whether the role drives 3 axes.
fn update_panel(panel: &ComponentPanel, name: &str, params: Params, spatial: bool) {
    panel.repeat_row.set_visible(name == "Repeat");
    panel.cycle_row.set_visible(name == "Repeat");
    panel.delay_row.set_visible(name == "Delay");
    panel.steps_row.set_visible(name == "Steps");
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

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(|app| {
        let canvas = DrawingArea::builder().hexpand(true).vexpand(true).build();

        // --- controls -------------------------------------------------

        let controls = GtkBox::new(Orientation::Vertical, 6);
        controls.set_margin_top(12);
        controls.set_margin_bottom(12);
        controls.set_margin_start(12);
        controls.set_margin_end(12);
        controls.set_width_request(320);

        // the value dimension and how it splits into components
        let dimensions_box = GtkBox::new(Orientation::Vertical, 6);
        let (_dimension_row, dimension) = dropdown_row(&dimensions_box, "Axes", &["1D", "2D", "3D"]);
        let (_composition_row, composition) = dropdown_row(&dimensions_box, "Split", compositions(0));
        controls.append(&group_frame("Dimensions", &dimensions_box));

        // one column per split component - hidden slots stay invisible
        let columns = GtkBox::new(Orientation::Horizontal, 4);
        columns.set_homogeneous(true);
        controls.append(&columns);
        let panels: Rc<Vec<ComponentPanel>> = Rc::new((0..3).map(|_| component_panel(&columns)).collect());

        // playback + timing parameters
        let playback_box = GtkBox::new(Orientation::Vertical, 6);
        let (_duration_row, duration) = slider(&playback_box, "Duration", 0.25, 6.0, 1.6, 2);
        let (_pause_row, pause) = slider(&playback_box, "Pause", 0.0, 3.0, 0.5, 2);
        let looping = gtk4::CheckButton::with_label("Loop");
        looping.set_active(true);
        playback_box.append(&looping);
        let replay = gtk4::Button::with_label("Replay");
        playback_box.append(&replay);
        let total_label = Label::new(Some("Total"));
        total_label.set_halign(gtk4::Align::Start);
        playback_box.append(&total_label);
        let total_bar = gtk4::ProgressBar::new();
        playback_box.append(&total_bar);
        let phase_label = Label::new(Some("Phase"));
        phase_label.set_halign(gtk4::Align::Start);
        playback_box.append(&phase_label);
        let phase_bar = gtk4::ProgressBar::new();
        playback_box.append(&phase_bar);
        let phase_number = Label::new(Some("Phase 1 / 1"));
        phase_number.set_halign(gtk4::Align::Center);
        playback_box.append(&phase_number);
        controls.append(&group_frame("Playback", &playback_box));

        // --- state ------------------------------------------------------

        let knobs = Rc::new({
            let (duration, dimension, composition) = (duration.clone(), dimension.clone(), composition.clone());
            let panels = Rc::clone(&panels);
            move || Knobs {
                duration: Duration::from_secs_f64(duration.value()),
                dimension: dimension.selected(),
                composition: composition.selected(),
                components: [component_knobs(&panels[0]), component_knobs(&panels[1]), component_knobs(&panels[2])],
            }
        });

        let demo = Rc::new(RefCell::new(build_demo(&knobs())));
        let latest = Rc::new(Cell::new(None::<(Vec2<()>, f64)>));
        let hold = Rc::new(Cell::new(None::<std::time::Instant>));
        let logo = gtk4::cairo::ImageSurface::create_from_png(&mut std::io::Cursor::new(include_bytes!("assets/smearor.png"))).ok();

        // Visibility + rebuild: only the rows each component's
        // selection actually reads stay visible.
        let update_visibility = {
            let panels = Rc::clone(&panels);
            let dimension = dimension.clone();
            let composition = composition.clone();
            // remembers the roles the panels were last configured
            // for - model swaps fire `selected_notify`, so the
            // re-entered update must not run again
            let last_dimension = Cell::new(u32::MAX);
            let last_roles: Rc<RefCell<Vec<&'static str>>> = Rc::new(RefCell::new(Vec::new()));
            let updating = Cell::new(false);
            let knobs = Rc::clone(&knobs);
            let demo = Rc::clone(&demo);
            let hold = Rc::clone(&hold);
            move || {
                if updating.replace(true) {
                    return;
                }
                let dim = dimension.selected();
                if dim != last_dimension.replace(dim) {
                    // the dimension's own split options
                    composition.set_model(Some(&StringList::new(compositions(dim))));
                    composition.set_selected(0);
                }
                let roles = component_roles(dim, composition.selected());
                {
                    let mut last_roles = last_roles.borrow_mut();
                    for (i, panel) in panels.iter().enumerate() {
                        match roles.get(i) {
                            Some(role) => {
                                panel.column.set_visible(true);
                                panel.title.set_markup(&format!("<span size=\"large\" weight=\"bold\">Axis {role}</span>"));
                                if last_roles.get(i) != Some(role) {
                                    panel.trajectory.set_model(Some(&StringList::new(role_choices(role))));
                                    panel.trajectory.set_selected(0);
                                }
                                let name = role_choices(role)[panel.trajectory.selected() as usize];
                                let (_, params) = easing_by_name(EASINGS[panel.easing.selected() as usize], &component_knobs(panel));
                                update_panel(panel, name, params, role.len() == 3);
                            }
                            None => panel.column.set_visible(false),
                        }
                    }
                    *last_roles = roles.to_vec();
                }
                hold.set(None);
                let mut animation = demo.borrow_mut();
                *animation = build_demo(&knobs());
                animation.start();
                updating.set(false);
            }
        };

        // Rebuild + restart on every control change.
        let restart: Rc<dyn Fn()> = Rc::new({
            let update_visibility = update_visibility.clone();
            move || update_visibility()
        });
        for panel in panels.iter() {
            connect_panel(panel, &restart);
        }
        {
            let restart = Rc::clone(&restart);
            duration.connect_value_changed(move |_| restart());
        }
        for dropdown in [&dimension, &composition] {
            let restart = Rc::clone(&restart);
            dropdown.connect_selected_notify(move |_| restart());
        }
        {
            let restart = Rc::clone(&restart);
            replay.connect_clicked(move |_| restart());
        }

        // --- frame clock -------------------------------------------------

        {
            let demo = Rc::clone(&demo);
            let latest = Rc::clone(&latest);
            let canvas = canvas.clone();
            let last_phase = Rc::new(Cell::new((u32::MAX, u32::MAX)));
            {
                let hold = Rc::clone(&hold);
                let last_phase = Rc::clone(&last_phase);
                let pause = pause.clone();
                let total_bar = total_bar.clone();
                let phase_bar = phase_bar.clone();
                let phase_number = phase_number.clone();
                canvas.add_tick_callback(move |canvas, _clock| {
                    {
                        let mut animation = demo.borrow_mut();
                        if animation.is_complete() && looping.is_active() {
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
                        latest.set(animation.value());
                        let progress = animation.progress().map(|p| p.as_f64()).unwrap_or(0.0).clamp(0.0, 1.0);
                        total_bar.set_fraction(progress);
                        // the phase bar tracks what component 0's
                        // trajectory sees - its own eased progress
                        let eased0 = animation
                            .easings()
                            .first()
                            .map(|e| e.apply(Fraction::new(progress)).as_f64())
                            .unwrap_or(progress);
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
                    gtk4::glib::ControlFlow::Continue
                });
            }
        }

        canvas.set_draw_func({
            let demo = Rc::clone(&demo);
            let latest = Rc::clone(&latest);
            let logo = logo.clone();
            move |_, context, width, height| {
                let (w, h) = (f64::from(width), f64::from(height));
                let margin = 40.0;
                let (pw, ph) = (w - 2.0 * margin, h - 2.0 * margin);
                let (x_min, x_max, y_min, y_max) = plot_domain(&demo.borrow());
                let to_x = |v: f64| margin + pw * (v - x_min) / (x_max - x_min);
                let to_y = |v: f64| margin + ph * (1.0 - (v - y_min) / (y_max - y_min));

                context.set_source_rgb(0.92, 0.92, 0.92);
                context.rectangle(margin, margin, pw, ph);
                let _ = context.fill();
                // the 0 and 1 reference lines stand out on both axes
                context.set_source_rgb(0.75, 0.75, 0.75);
                for v in [0.0, 1.0] {
                    let gy = to_y(v);
                    context.move_to(margin, gy);
                    context.line_to(margin + pw, gy);
                    let gx = to_x(v);
                    context.move_to(gx, margin);
                    context.line_to(gx, margin + ph);
                }
                context.set_line_width(1.5);
                let _ = context.stroke();
                let _ = context.save();
                context.rectangle(margin, margin, pw, ph);
                context.clip();
                {
                    // one curve per component easing - a distinct
                    // shade per axis group
                    const CURVE_COLORS: [(f64, f64, f64); 3] = [(0.72, 0.72, 0.86), (0.86, 0.7, 0.66), (0.66, 0.8, 0.7)];
                    for (slot, easing) in demo.borrow().easings().iter().enumerate() {
                        let (r, g, b) = CURVE_COLORS[slot % CURVE_COLORS.len()];
                        context.set_source_rgb(r, g, b);
                        for i in 0..=200 {
                            let t = Fraction::new(i as f64 / 200.0);
                            let eased = easing.apply(t).as_f64();
                            if !eased.is_finite() {
                                continue;
                            }
                            let (x, y) = (to_x(t.as_f64()), to_y(eased));
                            if i == 0 { context.move_to(x, y) } else { context.line_to(x, y) }
                        }
                    }
                    context.set_line_width(2.0);
                    let _ = context.stroke();
                }
                let _ = context.restore();

                // The animated content on top of the curve.
                let animation = demo.borrow();
                for point in animation.control_points() {
                    let (x, y) = (to_x(point.x), to_y(point.y));
                    context.arc(x, y, 4.0, 0.0, std::f64::consts::TAU);
                    context.set_source_rgb(0.5, 0.5, 0.5);
                    let _ = context.fill();
                }
                // one draggable center crosshair per orbital component,
                // in its easing curve's color but darker
                const MARK_COLORS: [(f64, f64, f64); 3] = [(0.4, 0.4, 0.62), (0.62, 0.36, 0.3), (0.32, 0.52, 0.36)];
                for mark in animation.centers() {
                    let (x, y) = (to_x(mark.point.x), to_y(mark.point.y));
                    let (r, g, b) = MARK_COLORS[mark.slot % MARK_COLORS.len()];
                    context.set_source_rgb(r, g, b);
                    context.set_line_width(1.5);
                    context.arc(x, y, 7.0, 0.0, std::f64::consts::TAU);
                    let _ = context.stroke();
                    context.move_to(x - 10.0, y);
                    context.line_to(x + 10.0, y);
                    context.move_to(x, y - 10.0);
                    context.line_to(x, y + 10.0);
                    let _ = context.stroke();
                }
                if let Some((point, scale)) = latest.get() {
                    let (x, y) = (to_x(point.x), to_y(point.y));
                    match &logo {
                        Some(logo) => {
                            let logo_w = 192.0 * scale;
                            let logo_h = logo_w * f64::from(logo.height()) / f64::from(logo.width());
                            let _ = context.save();
                            context.translate(x - logo_w / 2.0, y - logo_h / 2.0);
                            context.scale(logo_w / f64::from(logo.width()), logo_h / f64::from(logo.height()));
                            let _ = context.set_source_surface(logo, 0.0, 0.0);
                            let _ = context.paint();
                            let _ = context.restore();
                        }
                        None => {
                            context.arc(x, y, 10.0, 0.0, std::f64::consts::TAU);
                            context.set_source_rgb(0.2, 0.4, 0.8);
                            let _ = context.fill();
                        }
                    }
                }
                let progress = animation.progress().map(|p| p.as_f64()).unwrap_or(0.0);
                let eased: Vec<String> = animation
                    .easings()
                    .iter()
                    .map(|e| format!("{:.2}", e.apply(Fraction::new(progress)).as_f64()))
                    .collect();
                // cairo text, not a Label - widget updates inside a
                // draw callback queue a resize every frame
                context.set_source_rgb(0.35, 0.35, 0.35);
                context.select_font_face("monospace", gtk4::cairo::FontSlant::Normal, gtk4::cairo::FontWeight::Normal);
                context.set_font_size(13.0);
                context.move_to(margin, margin - 14.0);
                let _ = context.show_text(&format!("t {progress:.2}   eased {}", eased.join(" | ")));
            }
        });

        // Dragging a center crosshair rewrites that component's
        // Center spins - the spin change rebuilds the demo, which
        // moves the marker under the pointer.
        {
            let grabbed = Rc::new(Cell::new(usize::MAX));
            let drag = gtk4::GestureDrag::new();
            {
                let (demo, grabbed) = (Rc::clone(&demo), Rc::clone(&grabbed));
                drag.connect_drag_begin(move |drag, x, y| {
                    let Some(canvas) = drag.widget() else { return };
                    let (w, h) = (f64::from(canvas.width()), f64::from(canvas.height()));
                    let margin = 40.0;
                    let (x_min, x_max, y_min, y_max) = plot_domain(&demo.borrow());
                    let to_px = |vx: f64, vy: f64| {
                        (
                            margin + (w - 2.0 * margin) * (vx - x_min) / (x_max - x_min),
                            margin + (h - 2.0 * margin) * (1.0 - (vy - y_min) / (y_max - y_min)),
                        )
                    };
                    // the nearest marker inside the grab radius wins
                    let mut best = usize::MAX;
                    let mut best_dist = 24.0_f64;
                    for mark in demo.borrow().centers() {
                        let (mx, my) = to_px(mark.point.x, mark.point.y);
                        let dist = ((mx - x).powi(2) + (my - y).powi(2)).sqrt();
                        if dist < best_dist {
                            best_dist = dist;
                            best = mark.slot;
                        }
                    }
                    grabbed.set(best);
                });
            }
            {
                let (demo, panels, grabbed) = (Rc::clone(&demo), Rc::clone(&panels), Rc::clone(&grabbed));
                drag.connect_drag_update(move |drag, dx, dy| {
                    let slot = grabbed.get();
                    let (Some((start_x, start_y)), Some(canvas)) = (drag.start_point(), drag.widget()) else {
                        return;
                    };
                    if slot == usize::MAX {
                        return;
                    }
                    let (w, h) = (f64::from(canvas.width()), f64::from(canvas.height()));
                    let margin = 40.0;
                    let (x, y) = {
                        // the borrow must end before the spins fire the
                        // restart that rebuilds `demo`
                        let (x_min, x_max, y_min, y_max) = plot_domain(&demo.borrow());
                        let (px, py) = (start_x + dx - margin, start_y + dy - margin);
                        (x_min + px / (w - 2.0 * margin) * (x_max - x_min), y_min + (1.0 - py / (h - 2.0 * margin)) * (y_max - y_min))
                    };
                    // the Center spin range clamps the drag target
                    panels[slot].center[0].set_value(x.clamp(-0.5, 1.5));
                    panels[slot].center[1].set_value(y.clamp(-0.5, 1.5));
                });
            }
            {
                let grabbed = Rc::clone(&grabbed);
                drag.connect_drag_end(move |_, _, _| grabbed.set(usize::MAX));
            }
            canvas.add_controller(drag);
        }

        let view = GtkBox::new(Orientation::Horizontal, 0);
        view.append(&canvas);
        let scroller = gtk4::ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Automatic)
            .vscrollbar_policy(gtk4::PolicyType::Automatic)
            .child(&controls)
            .build();
        view.append(&scroller);
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Animation Playground")
            .default_width(1150)
            .default_height(760)
            .child(&view)
            .build();
        restart();
        window.present();
    });
    app.run()
}
