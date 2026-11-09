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
    /// alone, and on a face two of them share, parallel and facing each other
    /// across it, with the other laid. Each flat face is numbered on its own,
    /// in the order the names come, from where the steps' numbers ended. Two
    /// edges meeting at a convex corner are mitred there, the corner's third
    /// edge left sharp, and three are closed there by a triangle, numbered
    /// after every flat, by the lowest ranks of their edges in the list, then
    /// the next; at a corner where an edge meets inwards they are declined
    /// `Unsupported`. Flats that would meet anywhere else are declined
    /// `Unsupported`, until #792 measures them. The first edge declined
    /// declines the step.
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
}
