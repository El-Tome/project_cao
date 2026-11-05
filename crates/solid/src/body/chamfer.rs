//! A chamfer (#696): one straight edge of a body cut off by a flat face, set
//! back by a distance along each of the edge's two faces.

use super::{Body, Matter};
use crate::brep::{Declined, EdgeName};

/// The body a chamfer left, the distance it was asked, and the distance the
/// edge's two faces gave it.
#[derive(Clone, Debug, PartialEq)]
pub struct Chamfered {
    pub body: Body,
    pub asked: f64,
    pub taken: f64,
}

impl Body {
    /// The edge a name finds cut off, set back by `distance` along each of
    /// its two faces, by a flat face numbered where the steps' numbers ended.
    /// Flats name no edge.
    pub fn chamfer(&self, edge: &EdgeName, distance: f64) -> Result<Chamfered, Declined> {
        let Matter::Exact(exact) = &self.matter else {
            return Err(Declined::Unfinished);
        };
        Ok(Chamfered {
            body: Body::of_exact(exact.chamfered(edge, distance)?),
            asked: distance,
            taken: distance,
        })
    }
}
