//! A chamfer (#696): one straight edge of a body cut off by a flat face, set
//! back by a distance along each of the edge's two faces — or filled in by
//! one, where they meet inwards (#778) — or several edges in one step, at one
//! distance (#698).

use std::slice;

use super::{Body, Exact, Matter};
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
    /// cuts it, at `distance`, in the order the names come: each flat face
    /// numbered on its own, in that order, from where the steps' numbers
    /// ended. The first edge declined declines the step.
    pub fn chamfer_edges(&self, edges: &[EdgeName], distance: f64) -> Result<Chamfered, Declined> {
        let Matter::Exact(exact) = &self.matter else {
            return Err(Declined::Lost(Lost::Gone));
        };
        let (exact, taken) = exact.cut_off_each(edges, distance, Exact::chamfered)?;
        Ok(Chamfered {
            body: Body::of_exact(exact),
            asked: distance,
            taken,
        })
    }
}
