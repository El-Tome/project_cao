//! The cut a chamfer makes: a prism raised along the edge over the wedge
//! between the edge's two faces and the plane set back by the distance
//! along each, cut from the body — or, where the faces meet inwards, joined
//! to it, filling the corner. Its wall on that plane is the one face the
//! step names.

mod prism;

use super::Exact;
use super::wedge::{Meeting, Together, cut_across};
use crate::brep::{Declined, EdgeName};

impl Exact {
    /// The body with the edge each name finds cut off, or filled in where its
    /// faces meet inwards, set back by `distance` along each of its two faces,
    /// or by the most every edge gives: alone, and on a face two of them share,
    /// parallel and facing each other across it, with the other laid; and the
    /// one distance taken. The faces the cuts lay are numbered, in the order
    /// the names come, where the steps' numbers ended, and each trimmed, as a
    /// CAD tool trims it, by the faces it meets at the edge's ends — past the
    /// end corner where one of them leaves the edge at an obtuse angle, or a
    /// round leaves it at a few degrees either way. Two set-backs on such a
    /// face stop short of crossing; where they meet, the two flats meet in a
    /// ridge on a top, in a valley in a slot, and a cut's and a fill's that
    /// lie in one plane there — a tread's nose and foot set back by half its
    /// depth — are one slope, answering to both edges' numbers, the face they
    /// shared gone, as [`Exact::sloped`] lays it. Where matter lies past an
    /// end of an edge cut off, behind the one flat face it runs into there —
    /// the inner corner of an L, a pocket's rim, the foot of a block standing
    /// on another — its flat stops flush on that face, which grows by the
    /// flat's end and keeps its number, as [`Exact::cut_flush`] stops it. Two
    /// edges meeting at a convex corner are mitred there, from where their
    /// flats cross on the face they share down to the corner's third edge,
    /// left sharp. Two cut off where they meet beside a concave edge left
    /// sharp — the L's two top edges at its inner corner, two edges of a
    /// pocket's rim — each stop flush there and run on past the corner into a
    /// mitre, from where their flats cross on the face they share down to the
    /// concave edge, which ends one distance short of the corner, no face laid
    /// there. Three edges meeting at a convex corner are closed there by a
    /// triangle, its corners where each two flats' edges cross on the face
    /// they share, numbered after the flats. Flats that would meet anywhere
    /// else are not measured, and declined where they meet until #792. A
    /// distance shorter than the kernel can set a cut back from an edge is
    /// declined first; `Lost`, an edge its name no longer finds. Unsupported:
    /// a list of no edge, an edge that is not straight between two flat faces,
    /// one running on tangentially into the next edge of a face, one edge
    /// named twice, two edges meeting at any other corner where an edge meets
    /// inwards — two filled in, a cut and a fill, the three edges of a
    /// pocket's corner —, more than three edges meeting at a corner, an edge
    /// whose flat would be one slope with two others', or one of a slope
    /// meeting another edge of the list at a corner, a face so nearly flush
    /// with the chamfer's plane that the prism cannot stand off it, and a cut
    /// that, carried along the edge's line, takes matter beyond the faces it
    /// meets there — past an end where the face the edge runs into is curved,
    /// or is not one face — seen as a face of the prism but its chamfer left
    /// on the body, the chamfer in two pieces or with a corner on neither of
    /// the edge's faces, or a face the body held, but the edge's two, taken
    /// away whole.
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
            Meeting {
                together: Together::Sloped,
                closing: cut_across,
            },
        )?;
        Ok((cut, taken))
    }
}
