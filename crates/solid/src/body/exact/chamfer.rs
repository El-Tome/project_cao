//! The cut a chamfer makes: a prism raised along the edge over the wedge
//! between the edge's two faces and the plane set back by the distance
//! along each, cut from the body — or, where the faces meet inwards, joined
//! to it, filling the corner. Its wall on that plane is the one face the
//! step names.

mod prism;

use super::Exact;
use super::wedge::{Around, Closing, Meeting, Together, cut_across, joined_across};
use crate::brep::{Declined, EdgeName};

impl Exact {
    /// The body with the edge each name finds cut off, or filled in where its
    /// faces meet inwards, set back by `distance` along each of its two faces,
    /// or by the most every edge gives: alone, and for any two not meeting at
    /// a corner, with their set-backs short of reaching one point of a
    /// straight edge bounding a face of each, as [`Exact::cut_off`] reads it;
    /// and the one distance taken. The faces the cuts lay are numbered, in the
    /// order the names come, where the steps' numbers ended, and each trimmed, as a
    /// CAD tool trims it, by the faces it meets at the edge's ends — past the
    /// end corner where one of them leaves the edge at an obtuse angle, or a
    /// round leaves it at a few degrees either way. Two set-backs on such a
    /// face stop short of crossing; where they meet, the two flats meet in a
    /// ridge on a top, in a valley in a slot, and a cut's and a fill's that lie
    /// in one plane there, or at an angle the kernel tells from none — a
    /// tread's nose and foot set back by half its depth, even with its riser
    /// turned or leaning by a hair — are one slope, answering to both edges'
    /// numbers, and at an angle — the tread's front leaning — two flats
    /// meeting in a crease, each answering to its own edge's number, the face
    /// they shared gone either way, whichever edge is named first, and a
    /// chain of them — a flight's noses and feet, each tread as deep as each
    /// riser is high — one slope across it, answering to every edge's number,
    /// as [`Exact::sloped`] lays them. Where matter lies past an end of an edge
    /// cut off, behind the one flat face it runs into there — the inner corner
    /// of an L, a pocket's rim, the foot of a block standing on another — its
    /// flat stops flush on that face, which grows by the flat's end and keeps
    /// its number, as [`Exact::laid_flush`] stops it. Two edges meeting at a
    /// convex corner are mitred there, from where their flats cross on the
    /// face they share down to the corner's third edge, left sharp. Two cut
    /// off where they meet beside a concave edge left sharp — the L's two top edges
    /// at its inner corner, two edges of a pocket's rim — each stop flush there
    /// and run on past the corner into a mitre, from where their flats cross on
    /// the face they share down to the concave edge, which ends one distance
    /// short of the corner, no face laid there. Two filled in where they meet
    /// beside a convex edge left sharp — two feet of a block standing on
    /// another — each stop flush there and run on past the corner into a mitre
    /// outside the convex edge, from where their flats cross on the face they
    /// share to where they meet it, no face laid there. Two filled in where
    /// they meet beside a concave edge left sharp — two edges of a pocket's
    /// floor, or one and the upright it meets — each stop flush there and meet
    /// in a valley, the third edge ending one distance short, no face laid
    /// there. Three edges meeting at a convex corner are closed there by a
    /// triangle, its corners where each two flats' edges cross on the face they
    /// share, numbered after the flats — two such triangles at the ends of an
    /// edge whose flat has no width between them, the tip of a tongue as wide
    /// as two distances, meeting on a ridge in its place, that flat gone and
    /// its number answering to no face (#837) — and three at a concave corner,
    /// a pocket's bottom, by that triangle joined, filling the hollow the three
    /// fills leave, as [`joined_across`] lays it. The three at a standing
    /// block's foot corner, two feet and the upright above them, go round the
    /// upright: its flat laid first, stopping flush on the lower block's top,
    /// then the feet filled along the outline it left with the foot of that
    /// flat, which no name finds, numbered after the flats, the three fills
    /// meeting in mitres — the body the upright chamfered alone and then the
    /// three feet leave; and the three at the top of an L's inner corner, its
    /// mirror, the inner upright filled first, its fill stopping flush on the
    /// top, then the two top edges cut along the outline it left with the top
    /// edge of its flat, numbered after them, the three flats mitred at the
    /// top's two reflex corners. An upright named with its block's top edges
    /// is chamfered with them first, the top corner closed by its triangle,
    /// before the feet go round it: every edge of a standing block in one
    /// step. A post's upright between a floor plate and a ceiling plate goes
    /// round at both its ends, the feet under the ceiling the mirror of those
    /// on the floor, its flat's top filled with them. A foot and the upright
    /// above it, the other foot left sharp, go
    /// round the upright the same way, the fill along its flat's foot ending
    /// on the plane square to the sharp foot where the flat meets that foot's
    /// wall, its end a small face standing in the air, numbered after the
    /// fill; and a top edge at an L's inner corner and the upright below it,
    /// the other top edge left sharp, the mirror, the cut along the top of the
    /// upright's flat ending on the plane square to the sharp top edge, its
    /// end a small face dug into the top and facing back along that edge.
    /// Flats that would meet only on a curved edge are not measured,
    /// and declined where they meet.
    /// A distance shorter than the kernel can set a cut back from an edge is
    /// declined first; `Lost`, an edge its name no longer finds. Unsupported: a
    /// list of no edge, an edge that is not straight between two flat faces,
    /// one running on tangentially into the next edge of a face, one edge named
    /// twice, two edges meeting at any other corner where an edge meets inwards
    /// — a cut and a fill —, more than three edges meeting at a corner, an edge
    /// whose flat would meet one other's in a crease and another's in a slope
    /// — a leaning flight's foot — or one of a slope or a crease meeting
    /// another edge of the list at a
    /// corner, a face so nearly flush with the chamfer's plane that the prism
    /// cannot stand off it, and a cut
    /// that, carried along the edge's line, takes matter beyond the faces it
    /// meets there — past an end where the face the edge runs into is curved,
    /// or is not one face — seen as a face of the prism but its chamfer left on
    /// the body, the chamfer in two pieces or with a corner on neither of the
    /// edge's faces, or a face the body held, but the edge's two, taken away
    /// whole.
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
                closing: Closing {
                    outwards: cut_across,
                    inwards: joined_across,
                },
                around: Around::Edge,
            },
        )?;
        Ok((cut, taken))
    }
}
