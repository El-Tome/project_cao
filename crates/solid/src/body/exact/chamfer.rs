//! The cut a chamfer makes: a prism raised along the edge over the wedge
//! between the edge's two faces and the plane set back by the distance
//! along each, cut from the body — or, where the faces meet inwards, joined
//! to it, filling the corner. Its wall on that plane is the one face the
//! step names.

mod prism;

use super::Exact;
use crate::brep::{Declined, EdgeName};

impl Exact {
    /// The body with the edge each name finds cut off, or filled in where
    /// its faces meet inwards, set back by `distance` along each of its two
    /// faces or by the most every edge gives with the others laid, and the
    /// one distance taken; the faces the cuts lay numbered, in the order the
    /// names come, where the steps' numbers ended, and each trimmed, as a CAD
    /// tool trims it, by the faces it meets at the edge's ends — past the end
    /// corner where one of them leaves the edge at an obtuse angle, or a
    /// round leaves it at a few degrees either way. Two set-backs on a face
    /// two edges share stop short of crossing; where they meet, the two
    /// flats meet in a ridge. A distance shorter than the
    /// kernel can set a cut back from an edge is declined first; `Lost`, an
    /// edge its name no longer finds. Unsupported: an edge that is not
    /// straight between two flat faces, one running on tangentially into the
    /// next edge of a face, two edges sharing a face that are not parallel, a
    /// face so nearly flush with the chamfer's plane that the prism cannot
    /// stand off it, and a cut that, carried along the edge's line, takes
    /// matter beyond the faces it meets there — a flat wall leaving the edge
    /// at a concave corner, which the cut would run on along — seen as a face
    /// of the prism but its chamfer left on the body, the chamfer in two
    /// pieces or with a corner on neither of the edge's faces, or a face the
    /// body held, but the edge's two, taken away whole.
    pub fn chamfered(&self, names: &[EdgeName], distance: f64) -> Result<(Exact, f64), Declined> {
        let (cut, taken, _) = self.cut_off(
            names,
            distance,
            |wedge| wedge.reach,
            |_| 1.0,
            |wedge, taken, scale, bounds| {
                wedge
                    .prism(taken, scale, bounds)
                    .ok_or(Declined::Unsupported)
            },
            None,
        )?;
        Ok((cut, taken))
    }
}
