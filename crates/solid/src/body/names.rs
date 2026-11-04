//! What a body's edges and vertices answer to (#582): the numbers of the
//! faces they lie between, which a step pointing at one finds it again by.

use super::{Body, Matter};
use crate::brep::{EdgeName, Lost, VertexName};

impl Body {
    /// The name of every edge, ascending; none on flats, which name their
    /// faces alone.
    pub fn edge_names(&self) -> Vec<EdgeName> {
        match &self.matter {
            Matter::Exact(exact) => exact.edge_names().to_vec(),
            Matter::Flats(_) => Vec::new(),
        }
    }

    /// The name of every vertex, ascending; none on flats.
    pub fn vertex_names(&self) -> Vec<VertexName> {
        match &self.matter {
            Matter::Exact(exact) => exact.vertex_names().to_vec(),
            Matter::Flats(_) => Vec::new(),
        }
    }

    /// What a name no longer finds in the body: nothing when as many edges
    /// lie between its two sides as it was ranked among. On flats, which
    /// name no edge, every name is gone.
    pub fn lost(&self, name: &EdgeName) -> Option<Lost> {
        match &self.matter {
            Matter::Exact(exact) => exact.lost(name),
            Matter::Flats(_) => Some(Lost::Gone),
        }
    }
}
