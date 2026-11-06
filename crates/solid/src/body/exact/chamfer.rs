//! The cut a chamfer makes: a prism raised along the edge over the wedge
//! between the edge's two faces and the plane set back by the distance
//! along each, cut from the body. Its wall on that plane is the one face
//! the step names.

mod prism;

use super::Exact;
use super::wedge::Wedge;
use crate::brep::{Declined, EdgeName};

impl Exact {
    /// The body with the edge a name finds cut off, set back by `distance`
    /// along each of its two faces or by the most they give, and the
    /// distance taken; the face the cut lays numbered where the steps'
    /// numbers ended, and trimmed, as a CAD tool trims it, by the faces it
    /// meets at the edge's ends — past the end corner where one of them
    /// leaves the edge at an obtuse angle, or a round leaves it at a few
    /// degrees either way. A distance shorter than the kernel can set a cut
    /// back from an edge is declined first. Unsupported: an edge that is not
    /// straight between two flat faces meeting outwards, one running on
    /// tangentially into the next edge of a face, a face so nearly flush with
    /// the chamfer's plane that the prism cannot stand off it, and a cut
    /// that, carried along the edge's line, takes
    /// matter beyond the faces it meets there — a flat wall leaving the edge
    /// at a concave corner, which the cut would run on along — seen as a face
    /// of the prism but its chamfer left on the body, the chamfer in two
    /// pieces or with a corner on neither of the edge's faces, or a face the
    /// body held, but the edge's two, taken away whole.
    pub fn chamfered(&self, name: &EdgeName, distance: f64) -> Result<(Exact, f64), Declined> {
        self.cut_off(name, distance, |wedge| wedge.reach, Wedge::prism)
    }
}
