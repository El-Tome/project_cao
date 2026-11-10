//! What two cuts mitred beside a concave edge take past their corner (#801):
//! each stops flush on the face it runs into there, the other edge's second
//! face, and past both faces lies matter both prisms, carried past the
//! corner, would take. The later edge's prism takes that overlap with it, its
//! faces merged into the two flats or rounds, which then run past the corner
//! into a mitre, the concave edge left sharp and ending one value short of
//! the corner.

use std::collections::BTreeMap;

use glam::DVec3;

use super::flush::stops;
use super::{Prism, Wedge};
use crate::body::exact::Exact;
use crate::brep::{self, Declined, EdgeId, Scale, VertexId};

/// What two prisms carried past a corner both hold past the faces their edges
/// stop flush on there, every face of it numbered nought, and the points
/// where each edge's set-back lines cross the face it stops on, where an edge
/// it runs on along meets one laid before.
pub(super) struct Overlap {
    pub solid: Exact,
    pub cleared: Vec<DVec3>,
}

/// The overlap the prisms `tool` raises over the two edges of `meeting` for
/// `taken` make past the faces they stop flush on at `corner`, each edge and
/// its wedge read on `given`, the body as given, each set back `set_back` of
/// the value. Declined `Unsupported` where either edge does not stop flush
/// there; otherwise as the raise and the booleans decline.
pub(super) fn overlap(
    given: &brep::Body,
    corner: VertexId,
    meeting: [(EdgeId, &Wedge); 2],
    taken: f64,
    set_back: &impl Fn(&Wedge) -> f64,
    tool: &impl Fn(&Wedge, f64, Scale, (DVec3, DVec3)) -> Result<Prism, Declined>,
) -> Result<Overlap, Declined> {
    let at = given.vertex(corner).point;
    let bounds = given.bounds().ok_or(Declined::Unsupported)?;
    let past = |(edge, wedge): (EdgeId, &Wedge)| {
        let stop = (stops(given, edge, wedge).into_iter())
            .find(|stop| stop.corner == at)
            .ok_or(Declined::Unsupported)?;
        let prism = tool(wedge, taken, given.scale(), bounds)?;
        let (outline, frame, travel) = &prism;
        let raised = Exact::raised(outline, &[], *frame, *travel)?;
        let distance = taken * set_back(wedge);
        let crossings = (wedge.into).map(|into| stop.crossing(at + into * distance, wedge.along));
        Ok((common(&stop.beyond(&prism)?, &raised)?, crossings))
    };
    let [(one, first), (other, second)] = [past(meeting[0])?, past(meeting[1])?];
    let both = common(&one, &other)?;
    Ok(Overlap {
        solid: Exact::of(both.brep.renumbered_by(|_| 0), 0, 0, BTreeMap::new()),
        cleared: [first, second].concat(),
    })
}

/// What two solids both hold: the first less what of it the second does not.
fn common(one: &Exact, other: &Exact) -> Result<Exact, Declined> {
    one.cut_by(&one.cut_by(other)?)
}
