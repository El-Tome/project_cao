//! What a body's edges and vertices answer to (#582): the numbers of the
//! faces they lie between, which a step pointing at one finds it again by.

use super::{Body, Matter};
use crate::brep::{EdgeName, Lost, VertexName};

impl Body {
    /// The name of every edge, ascending; none on flats, which name their
    /// faces alone.
    pub fn edge_names(&self) -> Vec<EdgeName> {
        match &self.matter {
            Matter::Exact(exact) => ascending(exact.edge_names()),
            Matter::Flats(_) => Vec::new(),
        }
    }

    /// The name of every vertex, ascending; none on flats.
    pub fn vertex_names(&self) -> Vec<VertexName> {
        match &self.matter {
            Matter::Exact(exact) => ascending(exact.vertex_names()),
            Matter::Flats(_) => Vec::new(),
        }
    }

    /// What became of the edge a name was given to: nothing when the body
    /// still finds it, by its faces and its ends. On flats, which name no
    /// edge, every name is gone.
    pub fn lost(&self, name: &EdgeName) -> Option<Lost> {
        match &self.matter {
            Matter::Exact(exact) => exact.lost(name),
            Matter::Flats(_) => Some(Lost::Gone),
        }
    }
}

fn ascending<T: Clone + Ord>(names: &[T]) -> Vec<T> {
    let mut names = names.to_vec();
    names.sort();
    names
}
