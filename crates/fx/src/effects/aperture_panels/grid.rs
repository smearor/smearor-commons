use smearor_commons_geometry::Vec2;
use smearor_commons_noise::Field;
use smearor_commons_noise::Simplex2D;

/// Panel-wave constants (the shader's tunables).
pub(crate) const SPREAD: f64 = 0.6;
pub(crate) const WAVE_AMP: f64 = 0.08;
pub(crate) const WAVE_FREQ: f64 = 2.0;
pub(crate) const WAVE_NOISE_PX: f64 = 300.0;
pub(crate) const BORDER_LEAD: f64 = 0.25;
pub(crate) const BORDER_TRAIL: f64 = 0.15;
pub(crate) const BORDER_PX: f64 = 1.0;
pub(crate) const EDGE_SHADE: f64 = 0.4;
pub(crate) const PERSPECTIVE: f64 = 2.5;

/// `wavePosition` - where a panel sits on the wave's leading edge
/// (`0..1`), by `wave_shape`.
pub(crate) fn wave_position(cell: Vec2<()>, ucell: Vec2<()>, frame_size: Vec2<()>, shape: u32) -> f64 {
    let pos = (cell + Vec2::<()>::new(0.5, 0.5)).component_mul(&ucell);
    let diag = (pos.x + pos.y) / (frame_size.x + frame_size.y);
    match shape {
        1 => (pos.x * pos.x + pos.y * pos.y).sqrt() / (frame_size.x * frame_size.x + frame_size.y * frame_size.y).sqrt(),
        2 => {
            let along = (pos.x - pos.y) / (frame_size.x + frame_size.y);
            (diag + WAVE_AMP * (along * WAVE_FREQ * std::f64::consts::TAU).sin() + WAVE_AMP) / (1.0 + 2.0 * WAVE_AMP)
        }
        3 => {
            let bent = diag + WAVE_AMP * (Simplex2D.at(Vec2::<()>::new(pos.x / WAVE_NOISE_PX + 17.3, pos.y / WAVE_NOISE_PX + 5.9)) * 2.0 - 1.0);
            (bent + WAVE_AMP) / (1.0 + 2.0 * WAVE_AMP)
        }
        4 => {
            let dx = pos.x - frame_size.x * 0.5;
            let dy = pos.y - frame_size.y * 0.5;
            (dx * dx + dy * dy).sqrt() / (0.5 * (frame_size.x * frame_size.x + frame_size.y * frame_size.y).sqrt())
        }
        _ => diag,
    }
}

/// `cellTiming` - flip start/end of a panel within `[border_lead,
/// 1 - border_trail]` of the animation.
pub(crate) fn cell_timing(cell: Vec2<()>, ucell: Vec2<()>, frame_size: Vec2<()>, shape: u32, lead: f64, trail: f64) -> (f64, f64) {
    let wave = wave_position(cell, ucell, frame_size, shape).clamp(0.0, 1.0);
    let span = 1.0 - lead - trail;
    let start = lead + wave * SPREAD * span;
    (start, start + (1.0 - SPREAD) * span)
}

/// `cellProgress` - flip progress (`0..1`) of a panel.
pub(crate) fn cell_progress(cell: Vec2<()>, ucell: Vec2<()>, frame_size: Vec2<()>, shape: u32, lead: f64, trail: f64, progress: f64) -> f64 {
    let (t0, t1) = cell_timing(cell, ucell, frame_size, shape, lead, trail);
    ((progress - t0) / (t1 - t0)).clamp(0.0, 1.0)
}

/// `cellBorder` - border strength (`0..1`) of a panel right now.
pub(crate) fn cell_border(cell: Vec2<()>, ucell: Vec2<()>, frame_size: Vec2<()>, shape: u32, lead: f64, trail: f64, progress: f64) -> f64 {
    let (t0, t1) = cell_timing(cell, ucell, frame_size, shape, lead, trail);
    let fade_in = crate::smoothstep(t0 - lead.max(0.001), t0, progress);
    let fade_out = crate::smoothstep(t1, t1 + trail.max(0.001), progress);
    fade_in * (1.0 - fade_out)
}

/// The inverse perspective projection: from screen position to the
/// `panelUV` on the turning panel, or `None` when the fragment falls
/// in the gap (the shader's `onPanel` test).
pub(crate) fn panel_uv(local: f64, ucell: Vec2<()>, centre: Vec2<()>, screen: Vec2<()>, dist: f64, vertical: bool) -> Option<Vec2<()>> {
    let theta = local * std::f64::consts::PI;
    let c_t = theta.cos();
    let s_t = theta.sin();
    let den = dist * c_t + screen.x * s_t;
    if den.abs() <= 0.001 {
        return None;
    }
    let mut u = dist * (screen.x - centre.x) / den;
    let s = dist / (dist - u * s_t);
    let v = screen.y / s - centre.y;
    if local > 0.5 {
        u = -u;
    }
    let m = if vertical { Vec2::<()>::new(u, v) } else { Vec2::<()>::new(v, u) };
    let panel = m.component_mul(&Vec2::<()>::new(1.0 / ucell.x, 1.0 / ucell.y)) + Vec2::<()>::new(0.5, 0.5);
    const EPS: f64 = 0.002;
    let on = dist - u * s_t > 0.0 && panel.x >= -EPS && panel.y >= -EPS && panel.x <= 1.0 + EPS && panel.y <= 1.0 + EPS;
    on.then(|| Vec2::<()>::new(panel.x.clamp(0.0, 1.0), panel.y.clamp(0.0, 1.0)))
}
