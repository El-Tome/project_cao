//! What two edges of one sense mitred beside an edge of the other take past
//! their corner (#801): each stops flush on the face it runs into there, the
//! other edge's second face, and past both faces lies what both prisms,
//! carried past the corner, would hold — matter two cuts beside a concave
//! edge would take, air two fills beside a convex edge would fill. The later
//! edge's prism takes that overlap with it, cut or joined with it, its faces
//! merged into the two flats or rounds, which then run past the corner into a
//! mitre, the third edge left sharp.

use std::collections::BTreeMap;

use glam::DVec3;

use super::flush::stops;
use super::{Prism, Wedge};
use crate::body::exact::Exact;
use crate::brep::{self, Declined, EdgeId, Scale, VertexId};

/// Where the numbers of an overlap's faces start: above any a step gives and
/// below the pieces', so that a face of it left standing a while — on the
/// plane an edge stops on at one end, past where it stops at its other end,
/// until the edge laid after it there takes the overlap beyond and covers it
/// — is told apart from a face of a prism, and the step declined should one
/// stand once every edge is laid.
pub(super) const OVERLAPS: u32 = 1 << 30;

/// Where the numbers of the faces a fill's ends held in the air leave start
/// (#846): past the overlaps' and below the pieces', a stretch of [`STRIDE`]
/// for each end, so that a laying leaves them standing as no overlap is left,
/// and the step numbers each once every edge is laid.
pub(super) const ENDS: u32 = OVERLAPS + (1 << 29);

/// How many numbers each end held in the air has from [`ENDS`]: room for the
/// numbers a boolean moves it by.
pub(super) const STRIDE: u32 = 1 << 20;

/// What two prisms carried past a corner both hold past the faces their edges
/// stop flush on there, every face of it numbered [`OVERLAPS`], and the points
/// where each edge's set-back lines cross every face it stops on, there or at
/// its other end, where an edge it runs on along meets one laid before — at
/// its other end, one an overlap taken there before left, on an edge shorter
/// than its set-backs reach along it.
pub(super) struct Overlap {
    pub solid: Exact,
    pub cleared: Vec<DVec3>,
}

/// The overlap the prisms `tool` raises over the two edges of `pair` for
/// `taken` make past the faces they stop flush on at `corner`, each edge and
/// its wedge read on `given`, the body as given, each set back `set_back` of
/// the value. Declined `Unsupported` where either edge does not stop flush
/// there; otherwise as the raise and the booleans decline.
pub(super) fn overlap(
    given: &brep::Body,
    corner: VertexId,
    pair: [(EdgeId, &Wedge); 2],
    taken: f64,
    set_back: &impl Fn(&Wedge) -> f64,
    tool: &impl Fn(&Wedge, f64, Scale, (DVec3, DVec3)) -> Result<Prism, Declined>,
) -> Result<Overlap, Declined> {
    let at = given.vertex(corner).point;
    let bounds = given.bounds().ok_or(Declined::Unsupported)?;
    let past = |(edge, wedge): (EdgeId, &Wedge)| {
        let stops = stops(given, edge, wedge);
        let stop = (stops.iter())
            .find(|stop| stop.end == corner)
            .ok_or(Declined::Unsupported)?;
        let prism = tool(wedge, taken, given.scale(), bounds)?;
        let (outline, frame, travel) = &prism;
        let raised = Exact::raised(outline, &[], *frame, *travel)?;
        let distance = taken * set_back(wedge);
        let crossings: Vec<DVec3> = (stops.iter())
            .flat_map(|stop| {
                (wedge.into).map(|into| stop.crossing(at + into * distance, wedge.along))
            })
            .collect();
        Ok((common(&stop.beyond(&prism)?, &raised)?, crossings))
    };
    let [(one, first), (other, second)] = [past(pair[0])?, past(pair[1])?];
    let both = common(&one, &other)?;
    Ok(Overlap {
        solid: Exact::of(both.brep.renumbered_by(|_| OVERLAPS), 0, 0, BTreeMap::new()),
        cleared: [first, second].concat(),
    })
}

impl Exact {
    /// Declined `Unsupported` where a face of an overlap, numbered from
    /// [`OVERLAPS`] short of [`ENDS`], still stands on the body once every
    /// edge is laid: one no later edge covered.
    pub(super) fn overlaps_covered(&self) -> Result<(), Declined> {
        let left = (self.brep.face_ids())
            .any(|face| (self.brep.numbers(face).iter()).any(|n| (OVERLAPS..ENDS).contains(n)));
        if left {
            Err(Declined::Unsupported)
        } else {
            Ok(())
        }
    }
}

/// What two solids both hold: the first less what of it the second does not.
fn common(one: &Exact, other: &Exact) -> Result<Exact, Declined> {
    one.cut_by(&one.cut_by(other)?)
}
