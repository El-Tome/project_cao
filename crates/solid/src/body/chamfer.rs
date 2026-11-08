//! A chamfer (#696): one straight edge of a body cut off by a flat face, set
//! back by a distance along each of the edge's two faces — or filled in by
//! one, where they meet inwards (#778).

use super::{Body, Matter};
use crate::brep::{Declined, EdgeName, Lost};

/// The body a chamfer left, the distance it was asked, and the distance the
/// edge's two faces gave it.
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
        let Matter::Exact(exact) = &self.matter else {
            return Err(Declined::Lost(Lost::Gone));
        };
        let (exact, taken) = exact.chamfered(edge, distance)?;
        Ok(Chamfered {
            body: Body::of_exact(exact),
            asked: distance,
            taken,
        })
    }
}
