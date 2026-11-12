//! Every edge of a body drawn under its name: what a view lights under the
//! cursor and hands to a chamfer or a fillet.

use glam::DVec3;

use crate::body::caught::quietly;
use crate::body::drawn::tolerance;
use crate::body::{Body, Matter};
use crate::brep::EdgeName;

/// An edge as the view draws it: its name, and the points it runs through,
/// where its triangles stand on them. A closed edge ends where it starts.
#[derive(Clone, Debug, PartialEq)]
pub struct DrawnEdge {
    pub name: EdgeName,
    pub points: Vec<DVec3>,
}

impl Body {
    /// Every edge the body names, once each; none on flats, which name none,
    /// nor where sampling the edges stops on a bug.
    pub fn drawn_edges(&self) -> Vec<DrawnEdge> {
        let Matter::Exact(exact) = &self.matter else {
            return Vec::new();
        };
        let brep = exact.brep();
        let lines = quietly(|| brep.edge_lines(tolerance(brep))).unwrap_or_default();
        if lines.len() != exact.edge_names().len() {
            return Vec::new();
        }
        exact
            .edge_names()
            .iter()
            .zip(lines)
            .map(|(name, points)| DrawnEdge {
                name: name.clone(),
                points,
            })
            .collect()
    }
}
