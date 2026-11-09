//! The triangle three chamfers at a convex corner are closed with (#699):
//! its corners where the set-backs of each two of the corner's edges cross on
//! the face the two share, the corner cut off flat through them, across the
//! point the three flats would otherwise meet at.

use glam::{DVec2, DVec3};

use super::{Prism, Wedge};
use crate::body::exact::Exact;
use crate::brep::Declined;
use crate::profile::{Contour, Frame};

/// What raises the prism a corner three edges meet at is closed with, from
/// where their set-backs cross and where the corner stands: as
/// [`cut_across`] raises it.
pub(in crate::body::exact) type Closing = fn([DVec3; 3], DVec3) -> Prism;

/// How much wider than the triangle the prism that cuts the corner off
/// stands, about the triangle's middle: the point the three flats meet at
/// stands over the triangle at a square corner, and off its middle at one
/// that is not.
const SPREAD: f64 = 3.0;

/// How far past the corner that prism reaches, as a share of how high the
/// corner stands over the triangle.
const PAST: f64 = 0.125;

/// For each corner three edges meet at, the prism `closing` raises over the
/// crossings of their set-backs, `set_back` from each edge, and the ranks of
/// the three edges. Declined `Unsupported` where a corner is to be closed
/// and `closing` is none, or where two of its edges share no face or their
/// set-backs never cross.
pub(super) fn closings(
    corners: Vec<(DVec3, [usize; 3])>,
    wedges: &[Wedge],
    set_back: impl Fn(&Wedge) -> f64,
    closing: Option<Closing>,
) -> Result<Vec<(Prism, [usize; 3])>, Declined> {
    corners
        .into_iter()
        .map(|(corner, ranks)| {
            let close = closing.ok_or(Declined::Unsupported)?;
            let three = ranks.map(|rank| &wedges[rank]);
            let crossings = crossings(three, three.map(&set_back)).ok_or(Declined::Unsupported)?;
            Ok((close(crossings, corner), ranks))
        })
        .collect()
}

/// Where the set-backs of each two of a corner's three edges, `set_back`
/// from each, cross on the face the two share: the first and the second's,
/// the second and the third's, the third and the first's. None where two
/// share no face, or their set-backs run parallel.
fn crossings(wedges: [&Wedge; 3], set_back: [f64; 3]) -> Option<[DVec3; 3]> {
    let crossing = |one: usize, other: usize| {
        let [first, second] = [wedges[one], wedges[other]];
        let (on_first, on_second) = (0..2)
            .flat_map(|i| (0..2).map(move |j| (i, j)))
            .find(|&(i, j)| first.faces[i] == second.faces[j])?;
        let from = first.point + first.into[on_first] * set_back[one];
        let to = second.point + second.into[on_second] * set_back[other];
        let normal = first.along.cross(second.along);
        let along = (to - from).cross(second.along).dot(normal) / normal.length_squared();
        along.is_finite().then(|| from + first.along * along)
    };
    Some([crossing(0, 1)?, crossing(1, 2)?, crossing(2, 0)?])
}

/// The prism a corner is cut off flat with through `crossings`: raised from
/// their plane toward `corner`, over the triangle they make spread
/// [`SPREAD`] times about its middle, to [`PAST`] past the corner. Its floor,
/// the face the cut lays, lies on that plane; what stands on the corner's
/// side of it near the corner is the point the three flats meet at.
pub(in crate::body::exact) fn cut_across(crossings: [DVec3; 3], corner: DVec3) -> Prism {
    let [first, second, third] = crossings;
    let middle = (first + second + third) / 3.0;
    let u = (second - first).normalize();
    let mut normal = u.cross(third - first).normalize();
    if normal.dot(corner - middle) < 0.0 {
        normal = -normal;
    }
    let v = normal.cross(u);
    let outline = crossings.map(|at| {
        let off = (at - middle) * SPREAD;
        DVec2::new(off.dot(u), off.dot(v))
    });
    let frame = Frame {
        origin: middle,
        u,
        v,
    };
    let travel = normal * normal.dot(corner - middle) * (1.0 + PAST);
    (Contour::straight(outline.to_vec()), frame, travel)
}

impl Exact {
    /// The body with each corner `closes` holds cut off by its prism, the
    /// prism's floor the face it lays, numbered where the steps' numbers end.
    /// Declined `Unsupported` where a cut left anything but that face, every
    /// corner of it a corner of one of the flats laid on the three edges
    /// whose ranks it holds, numbered from `start`, as [`Exact::cut_along`]
    /// reads it.
    pub(super) fn closed(
        self,
        closes: &[(Prism, [usize; 3])],
        start: u32,
    ) -> Result<Exact, Declined> {
        closes
            .iter()
            .try_fold(self, |body, ((outline, frame, travel), ranks)| {
                let tool = Exact::raised(outline, &[], *frame, *travel)?;
                let cut = body.cut_by(&Exact { next: 1, ..tool })?;
                let flats = ranks.map(|rank| start + rank as u32);
                if cut.cut_along(&[], &body, 1, &flats) {
                    Ok(cut)
                } else {
                    Err(Declined::Unsupported)
                }
            })
    }
}
