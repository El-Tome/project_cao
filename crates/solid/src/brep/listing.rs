//! Every face, edge and vertex a body is made of, with the exact geometry
//! behind each and which touch which — asked for by Tom on 2026-09-30, because
//! the application will need it: an edge to fillet, a vertex to dimension
//! from, a face to sketch on.
//!
//! Plain data, read off the body and nothing else, so that the rules can hold
//! it to the geometry without trusting how it was built.

use glam::DVec3;

use super::curve::Curve;
use super::surface::Surface;
use super::topology::Body;

#[derive(Clone, Debug, PartialEq)]
pub struct Listing {
    pub faces: Vec<ListedFace>,
    pub edges: Vec<ListedEdge>,
    pub vertices: Vec<DVec3>,
}

/// A face: its exact surface, whether the surface's own normal points out of
/// the matter, and its loops as edges by rank, each run along its own way or
/// against it.
#[derive(Clone, Debug, PartialEq)]
pub struct ListedFace {
    pub surface: Surface,
    pub outward: bool,
    pub loops: Vec<Vec<(usize, bool)>>,
}

/// An edge: its exact curve and the stretch of its parameter it runs over,
/// its two vertices by rank — none for a whole circle — and the faces on
/// either side, each with the way it runs along the edge.
#[derive(Clone, Debug, PartialEq)]
pub struct ListedEdge {
    pub curve: Curve,
    pub from: f64,
    pub to: f64,
    pub ends: Option<[usize; 2]>,
    pub sides: Vec<(usize, bool)>,
}

impl Body {
    pub fn listing(&self) -> Listing {
        Listing {
            faces: self
                .faces
                .iter()
                .map(|face| ListedFace {
                    surface: *self.surface(face.surface),
                    outward: !face.flipped,
                    loops: face
                        .loops
                        .iter()
                        .map(|lap| {
                            lap.iter()
                                .map(|coedge| (coedge.edge.0 as usize, coedge.forward))
                                .collect()
                        })
                        .collect(),
                })
                .collect(),
            edges: self
                .edge_ids()
                .map(|id| {
                    let edge = self.edge(id);
                    ListedEdge {
                        curve: *self.curve(edge.curve),
                        from: edge.from,
                        to: edge.to,
                        ends: edge.ends.map(|ends| ends.map(|end| end.0 as usize)),
                        sides: self
                            .uses(id)
                            .into_iter()
                            .map(|(face, forward)| (face.0 as usize, forward))
                            .collect(),
                    }
                })
                .collect(),
            vertices: self.vertices.iter().map(|vertex| vertex.point).collect(),
        }
    }
}
