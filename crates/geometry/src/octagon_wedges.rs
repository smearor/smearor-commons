//! Iterator over the eight edge wedges of an [`Octagon`].

use crate::Fraction;
use crate::Octagon;
use crate::Polygon;

/// The eight edge wedges of an [`Octagon`], created by
/// [`Octagon::wedges`].
///
/// Each item is the trapezoidal insert on one edge: its outer side is
/// the `outer`-scaled edge `squeeze`-narrowed around its midpoint,
/// its inner side the parallel edge of the `inner`-scaled octagon -
/// the shape chip inserts, rim marks and border studs take.
#[derive(Debug, Clone)]
pub struct OctagonWedges<U> {
    /// The octagon whose edges carry the wedges.
    octagon: Octagon<U>,
    /// Radial fraction of the outer side, relative to the inradius.
    outer: Fraction,
    /// Radial fraction of the inner side, relative to the inradius.
    inner: Fraction,
    /// The outer side's shrink relative to the edge's half-length.
    squeeze: Fraction,
    /// The edge index the iterator returns next.
    next: usize,
}

impl<U> OctagonWedges<U> {
    /// An `OctagonWedges` over `octagon`'s edges - see
    /// [`Octagon::wedges`].
    pub(crate) fn new(octagon: Octagon<U>, outer: Fraction, inner: Fraction, squeeze: Fraction) -> Self {
        Self {
            octagon,
            outer,
            inner,
            squeeze,
            next: 0,
        }
    }
}

impl<U: Copy> Iterator for OctagonWedges<U> {
    type Item = Polygon<U>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next >= 8 {
            return None;
        }
        let wedge = self.octagon.wedge(self.next, self.outer, self.inner, self.squeeze);
        self.next += 1;
        Some(wedge)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = 8 - self.next;
        (remaining, Some(remaining))
    }
}

impl<U: Copy> ExactSizeIterator for OctagonWedges<U> {}
