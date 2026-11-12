//! A chamfer (#696): one straight edge of a body cut off by a flat face, set
//! back by a distance along each of the edge's two faces — or filled in by
//! one, where they meet inwards (#778) — or several edges in one step, at one
//! distance (#698).

use std::slice;

use super::{Body, Matter};
use crate::brep::{Declined, EdgeName, Lost};

/// The body a chamfer left, the distance it was asked, and the distance the
/// edges' faces gave it.
#[derive(Clone, Debug, PartialEq)]
pub struct Chamfered {
    pub body: Body,
    pub asked: f64,
    pub taken: f64,
}

impl Body {
    /// The edge a name finds cut off, or filled in where its faces meet
    /// inwards, set back by `distance` along each of its two faces, or by the
    /// most they give: at most one of them taken whole. The flat face it lays
    /// is numbered where the steps' numbers ended. Flats name no edge: every
    /// name is gone, as [`Body::lost`] says.
    pub fn chamfer(&self, edge: &EdgeName, distance: f64) -> Result<Chamfered, Declined> {
        self.chamfer_edges(slice::from_ref(edge), distance)
    }

    /// The edge each name finds cut off, or filled in, as [`Body::chamfer`]
    /// cuts it, at `distance` or at the one largest distance every edge gives:
    /// alone, and for any two not meeting at a corner, with their set-backs
    /// short of reaching one point of a straight edge bounding a face of each
    /// — two parallel edges facing each other across a face they share, an
    /// edge ending on a face of another, two that are not parallel on a face
    /// they share. Each flat face is numbered on its own,
    /// in the order the names come, from where the steps' numbers ended — but
    /// an edge cut off and one filled in whose set-backs meet across the face
    /// they share, and whose flats lie in one plane there, or at an angle the
    /// kernel tells from none — a tread's nose and foot at half its depth —
    /// which are one slope, the face gone, answering to both their numbers.
    /// Where their flats lie at an angle — the tread's front leaning — they
    /// meet in a crease, the face gone, each answering to its own number,
    /// whichever is named first. A chain of such edges — a flight's noses and
    /// feet, each tread as deep as each riser is high — is one slope across
    /// it, the treads and risers gone, answering to every edge's number,
    /// whatever their order; a chain holding a crease is declined
    /// `Unsupported` before anything is laid (#926), and so is one whose
    /// flats stand a hair off one plane (#929). Such a slope or crease is
    /// laid at the distance whose set-backs meet across that face, which may
    /// stand a hair above the distance taken: its own distance is read off
    /// its faces, never from [`Chamfered::taken`]. Two edges meeting at a convex corner
    /// are mitred there, the corner's third edge left sharp, and three are
    /// closed there by a triangle, cut off at a convex corner and filled in at
    /// a concave one — a pocket's bottom —, numbered after every flat, by the
    /// lowest ranks of
    /// their edges in the list, then the next; two triangles at the ends of an
    /// edge whose flat has no width between them — the tip of a tongue as wide
    /// as two distances, its end edge and both uprights named — meet on a ridge
    /// in its place, that flat gone. Two cut off where they meet
    /// beside a concave edge left sharp — the L's top edges at its inner
    /// corner, a pocket's rim — each stop flush on the face the other runs
    /// along and are mitred past it, down to the concave edge, no face laid
    /// there, and two filled in beside a convex edge left sharp — two feet of a
    /// block standing on another — are mitred outside it the same way, and two
    /// filled in beside a concave edge left sharp, at a pocket's bottom corner,
    /// meet in a valley; at any other corner where an edge meets inwards they
    /// are declined `Unsupported`, as is an edge of a slope or a crease
    /// meeting another edge of the list at a corner. Flats that would meet
    /// only on a curved edge are not measured, and declined `Unsupported`
    /// where they meet. The first edge declined declines the step.
    pub fn chamfer_edges(&self, edges: &[EdgeName], distance: f64) -> Result<Chamfered, Declined> {
        let Matter::Exact(exact) = &self.matter else {
            return Err(Declined::Lost(Lost::Gone));
        };
        let (exact, taken) = exact.chamfered(edges, distance)?;
        Ok(Chamfered {
            body: Body::of_exact(exact),
            asked: distance,
            taken,
        })
    }

    /// Moves the faces' count past the numbers a chamfer or a fillet of
    /// `edges` edges would have given, one each, for one the kernel declined:
    /// the steps after it number their faces as if it had stood.
    pub fn count_past_cut(&mut self, edges: usize) {
        if let Matter::Exact(exact) = &mut self.matter {
            exact.count_past_turned(edges as u32);
        }
    }
}
