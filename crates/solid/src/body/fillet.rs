//! A fillet (#697): one straight edge of a body rounded off by a cylinder of
//! a radius tangent to the edge's two faces — or filled in by one, where they
//! meet inwards (#778) — or several edges in one step, at one radius (#698).

use std::slice;

use super::{Body, Matter};
use crate::brep::{Declined, EdgeName, Lost};

/// The body a fillet left, the radius it was asked, and the radius the
/// edges' faces gave it.
#[derive(Clone, Debug, PartialEq)]
pub struct Filleted {
    pub body: Body,
    pub asked: f64,
    pub taken: f64,
}

impl Body {
    /// The edge a name finds rounded off by a cylinder of `radius` tangent to
    /// its two faces, or of the largest radius they give — filled in by it
    /// where they meet inwards. The round it lays is numbered where the steps'
    /// numbers ended. Flats name no edge: every name is gone, as [`Body::lost`]
    /// says.
    pub fn fillet(&self, edge: &EdgeName, radius: f64) -> Result<Filleted, Declined> {
        self.fillet_edges(slice::from_ref(edge), radius)
    }

    /// The edge each name finds rounded off as [`Body::fillet`] rounds it, at
    /// `radius` or at the one largest radius every edge gives: alone, and for
    /// any two not meeting at a corner, with their set-backs short of reaching
    /// one point of a straight edge bounding a face of each — two parallel
    /// edges facing each other across a face they share, an edge ending on a
    /// face of another, two that are not parallel on a face they share. Each
    /// round is numbered on its own, in the order the
    /// names come, from where the steps' numbers ended — but two edges whose
    /// set-backs meet across the face they share, which one full round takes
    /// off or, where their faces meet inwards, one round bottom fills in, the
    /// face gone, answering to both their numbers. Such a round is laid at the
    /// radius whose set-backs meet across that face, which may stand a hair
    /// above the radius taken: its own radius is read off its face, never from
    /// [`Filleted::taken`]. Two edges meeting at a convex corner are mitred
    /// there, the corner's third edge left sharp, and three are closed there by
    /// a piece of sphere, at a concave corner — a pocket's bottom — a hollow
    /// one facing its centre, numbered after every round, by the lowest ranks
    /// of their edges in the list, then the next. Two rounded off where they
    /// meet beside a concave edge left sharp — the L's top edges at its inner
    /// corner, a pocket's rim — each stop flush on the face the other runs
    /// along and are mitred past it on an ellipse, down to the concave edge, no
    /// face laid there, and two filled in beside a convex edge left sharp — two
    /// feet of a block standing on another — are mitred outside it the same
    /// way, and two filled in beside a concave edge left sharp, at a pocket's
    /// bottom corner, meet in a valley; at any other corner where an edge meets
    /// inwards they are declined `Unsupported`. The edge at the end of a full
    /// round, bounding the face that is gone and meeting both its edges at a
    /// corner, is rounded at the full round's own radius and mitred against it
    /// on two ellipses up to the crown, as against any round at a convex
    /// corner, and so are the two edges of a pointed end, or of a notch beside
    /// its concave upright, each bounding that face and meeting one of the full
    /// round's edges and the other, mitred against both their neighbours, the
    /// three mitres meeting at the crown, and so is the upright at its end,
    /// meeting one of its edges at a convex corner, mitred against it on one
    /// ellipse from where the full round leaves the upright's face up to the
    /// crown; an edge of a full round meeting any other edge of the list at a
    /// corner, those two where the upright at the tip is named too, the
    /// upright where the edge at its end is (#837), or one of a round bottom
    /// meeting any, is declined `Unsupported`. Rounds
    /// that would meet only on a curved edge are not measured, and declined
    /// `Unsupported` where they meet. The first edge declined declines the
    /// step.
    pub fn fillet_edges(&self, edges: &[EdgeName], radius: f64) -> Result<Filleted, Declined> {
        let Matter::Exact(exact) = &self.matter else {
            return Err(Declined::Lost(Lost::Gone));
        };
        let (exact, taken) = exact.filleted(edges, radius)?;
        Ok(Filleted {
            body: Body::of_exact(exact),
            asked: radius,
            taken,
        })
    }
}
