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
    /// `radius` or at the one largest radius every edge gives: alone, and on a
    /// face two of them share, parallel and facing each other across it, with
    /// the other laid. Each round is numbered on its own, in the order the
    /// names come, from where the steps' numbers ended — but two edges whose
    /// set-backs meet across the face they share, which one full round takes
    /// off or, where their faces meet inwards, one round bottom fills in, the
    /// face gone, answering to both their numbers. Rounds that would meet
    /// anywhere else are declined `Unsupported`, until #792 measures them. The
    /// first edge declined declines the step.
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
