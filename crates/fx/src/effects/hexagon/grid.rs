use smearor_commons_geometry::Vec2;

/// BMW `getHexagons` - the procedural hexagonal grid: `p` in grid
/// space, returns cell-relative `coords` ([0,0] = cell center),
/// `edge` = distance to the closest cell edge (`1` at the center,
/// `0` at the edge), and `glow` = squared distance to the closest
/// cell center (`/1.5`). Two overlapping cell lattices offset by
/// half a period tile the plane.
pub(super) struct HexCell {
    /// Cell-relative coordinates - `result.xy`.
    pub coords: Vec2<()>,
    /// Distance to the closest edge - `result.z`.
    pub edge: f64,
    /// Squared distance to the closest center - `result.w`.
    pub glow: f64,
}

/// Edge length of one hexagonal cell - `sqrt(4/3)`.
const EDGE_LENGTH: f64 = 1.154_700_538_379_251_5; // sqrt(4.0/3.0)

/// See [`HexCell`].
pub(super) fn hex_cell(p: Vec2<()>) -> HexCell {
    let scale = Vec2::<()>::new(3.0 * EDGE_LENGTH, 2.0);
    let wrap = |q: Vec2<()>, offset: f64| {
        Vec2::<()>::new(
            crate::fract((q.x + offset * scale.x) / scale.x) * scale.x - scale.x * 0.5,
            crate::fract((q.y + offset * scale.y) / scale.y) * scale.y - scale.y * 0.5,
        )
    };
    let a = wrap(p, 0.0);
    let b = wrap(p, 0.5);
    let dist_a = (a.x.abs() / EDGE_LENGTH + a.y.abs() * 0.5).max(a.y.abs());
    let dist_b = (b.x.abs() / EDGE_LENGTH + b.y.abs() * 0.5).max(b.y.abs());
    HexCell {
        coords: if dist_a < dist_b { a } else { b },
        edge: 1.0 - dist_a.min(dist_b),
        glow: (a.x * a.x + a.y * a.y).min(b.x * b.x + b.y * b.y) / 1.5,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Cell centers report `coords ≈ 0`, `edge = 1`, `glow = 0`.
    #[test]
    fn cell_center_is_the_origin() {
        let cell = hex_cell(Vec2::<()>::new(0.0, 0.0));
        assert!(cell.coords.x.abs() < 1e-9 && cell.coords.y.abs() < 1e-9);
        assert!((cell.edge - 1.0).abs() < 1e-9, "edge {} at the center", cell.edge);
        assert!(cell.glow.abs() < 1e-9);
    }

    /// The grid tiles: `p` and `p + scale` produce identical cells.
    #[test]
    fn grid_repeats_at_scale() {
        let scale = Vec2::<()>::new(3.0 * 1.154_700_538_379_251_5, 2.0);
        let p = Vec2::<()>::new(0.37, 0.71);
        let a = hex_cell(p);
        let b = hex_cell(p + scale);
        assert!((a.edge - b.edge).abs() < 1e-9);
        assert!((a.coords.x - b.coords.x).abs() < 1e-9 && (a.coords.y - b.coords.y).abs() < 1e-9);
    }

    /// `edge` falls to 0 at cell borders, `glow` peaks there too.
    #[test]
    fn edge_drops_at_borders() {
        // Scan a horizontal line - edge must hit ~0 somewhere.
        let min_edge = (0..200).map(|i| hex_cell(Vec2::<()>::new(i as f64 / 40.0, 0.0)).edge).fold(1.0_f64, f64::min);
        assert!(min_edge < 0.05, "border never reached: {min_edge}");
    }
}
