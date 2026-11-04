//! What a body's edges and vertices answer to (#582): the numbers of the
//! faces they lie between, which a step pointing at one finds it again by.

use super::{Body, Matter};
use crate::brep::{EdgeName, VertexName};

impl Body {
    /// The name of every edge, ascending; none on flats, which name their
    /// faces alone.
    pub fn edge_names(&self) -> Vec<EdgeName> {
        match &self.matter {
            Matter::Exact(exact) => exact.edge_names(),
            Matter::Flats(_) => Vec::new(),
        }
    }

    /// The name of every vertex, ascending; none on flats.
    pub fn vertex_names(&self) -> Vec<VertexName> {
        match &self.matter {
            Matter::Exact(exact) => exact.vertex_names(),
            Matter::Flats(_) => Vec::new(),
        }
    }
}
