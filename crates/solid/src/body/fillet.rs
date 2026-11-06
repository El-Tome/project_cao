//! A fillet (#697): one straight edge of a body rounded off by a cylinder of
//! a radius tangent to the edge's two faces.

use super::{Body, Matter};
use crate::brep::{Declined, EdgeName, Lost};

/// The body a fillet left, the radius it was asked, and the radius the
/// edge's two faces gave it.
#[derive(Clone, Debug, PartialEq)]
pub struct Filleted {
    pub body: Body,
    pub asked: f64,
    pub taken: f64,
}

impl Body {
    /// The edge a name finds rounded off by a cylinder of `radius` tangent to
    /// its two faces, or of the largest radius they give. The round it lays
    /// is numbered where the steps' numbers ended. Flats name no edge: every
    /// name is gone, as [`Body::lost`] says.
    pub fn fillet(&self, edge: &EdgeName, radius: f64) -> Result<Filleted, Declined> {
        let Matter::Exact(exact) = &self.matter else {
            return Err(Declined::Lost(Lost::Gone));
        };
        let (exact, taken) = exact.filleted(edge, radius)?;
        Ok(Filleted {
            body: Body::of_exact(exact),
            asked: radius,
            taken,
        })
    }
}
