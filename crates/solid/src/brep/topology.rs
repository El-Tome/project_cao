//! What a body is made of: arenas of vertices, edges and faces, each naming
//! the others by rank.

use std::f64::consts::PI;

use glam::DVec3;

use super::curve::{Circle, Curve};
use super::scale::Scale;
use super::surface::Surface;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SurfaceId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CurveId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VertexId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EdgeId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FaceId(pub u32);

/// A corner, and the surfaces it was decided to lie on, sorted: what a later
/// step reads instead of measuring the distance again.
#[derive(Clone, Debug, PartialEq)]
pub struct Vertex {
    pub point: DVec3,
    pub on: Vec<SurfaceId>,
}

/// A stretch of a curve, from parameter `from` up to parameter `to`, which is
/// the way the edge runs. A whole closed curve has no vertex, and runs over
/// one period.
#[derive(Clone, Debug, PartialEq)]
pub struct Edge {
    pub curve: CurveId,
    pub ends: Option<[VertexId; 2]>,
    pub from: f64,
    pub to: f64,
}

/// A face's use of an edge: along the edge's own way, or against it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Coedge {
    pub edge: EdgeId,
    pub forward: bool,
}

/// A piece of a surface, bounded by loops that keep it on their left seen from
/// outside the matter. `flipped` when the matter lies on the side the
/// surface's own normal points to — a hole's wall, the bottom of a block.
#[derive(Clone, Debug, PartialEq)]
pub struct Face {
    pub surface: SurfaceId,
    pub flipped: bool,
    pub loops: Vec<Vec<Coedge>>,
}

/// A solid, as the surfaces and curves it stands on and the faces, edges and
/// vertices that bound it.
#[derive(Clone, Debug, PartialEq)]
pub struct Body {
    pub(crate) surfaces: Vec<Surface>,
    pub(crate) curves: Vec<Curve>,
    pub(crate) vertices: Vec<Vertex>,
    pub(crate) edges: Vec<Edge>,
    pub(crate) faces: Vec<Face>,
    pub(crate) scale: Scale,
    /// For each surface, the scale of the operation that brought it into the
    /// body: a raised profile's own for each of its surfaces.
    pub(crate) arrivals: Vec<Scale>,
}

impl Body {
    pub fn surface(&self, id: SurfaceId) -> &Surface {
        &self.surfaces[id.0 as usize]
    }

    pub fn curve(&self, id: CurveId) -> &Curve {
        &self.curves[id.0 as usize]
    }

    pub fn vertex(&self, id: VertexId) -> &Vertex {
        &self.vertices[id.0 as usize]
    }

    pub fn edge(&self, id: EdgeId) -> &Edge {
        &self.edges[id.0 as usize]
    }

    pub fn face(&self, id: FaceId) -> &Face {
        &self.faces[id.0 as usize]
    }

    pub fn face_ids(&self) -> impl Iterator<Item = FaceId> + '_ {
        (0..self.faces.len() as u32).map(FaceId)
    }

    pub fn edge_ids(&self) -> impl Iterator<Item = EdgeId> + '_ {
        (0..self.edges.len() as u32).map(EdgeId)
    }

    pub fn vertex_ids(&self) -> impl Iterator<Item = VertexId> + '_ {
        (0..self.vertices.len() as u32).map(VertexId)
    }

    pub fn scale(&self) -> Scale {
        self.scale
    }

    /// The scale of the operation that brought a surface into the body. A
    /// pair of its surfaces was decided when the later of the two came, at
    /// that operation's scale: a later leaf growing the reach does not decide
    /// it again.
    pub fn arrived(&self, surface: SurfaceId) -> Scale {
        self.arrivals
            .get(surface.0 as usize)
            .copied()
            .unwrap_or(self.scale)
    }

    /// The point of an edge at its parameter `t`.
    pub fn point_on(&self, edge: EdgeId, t: f64) -> DVec3 {
        self.curve(self.edge(edge).curve).point(t)
    }

    /// The parameter of a point lying on an edge, in the stretch the edge
    /// runs over: an arc may run past the end of its circle's first period,
    /// where the curve alone would give the point a turn less.
    pub fn parameter_on(&self, edge: EdgeId, point: DVec3) -> f64 {
        let stretch = self.edge(edge);
        let curve = self.curve(stretch.curve);
        let t = curve.parameter(point);
        match curve.period() {
            Some(period) => {
                let middle = (stretch.from + stretch.to) / 2.0;
                t + period * ((middle - t) / period).round()
            }
            None => t,
        }
    }

    /// The largest coordinate, in absolute value, any point of the body
    /// reaches: what its scale is taken against.
    pub fn reach(&self) -> f64 {
        let corners = self.vertices.iter().map(|vertex| vertex.point);
        let arcs = self.edges.iter().flat_map(|edge| self.extremes(edge));
        corners
            .chain(arcs)
            .map(|point| point.abs().max_element())
            .fold(0.0, f64::max)
    }

    /// The points of an edge where a coordinate is largest or smallest.
    pub(super) fn extremes(&self, edge: &Edge) -> Vec<DVec3> {
        let curve = self.curve(edge.curve);
        let mut found = vec![curve.point(edge.from), curve.point(edge.to)];
        match curve {
            Curve::Line(_) => {}
            Curve::Circle(circle) => found.extend(turning_points(circle, edge.from, edge.to)),
            Curve::Meet(_) => {
                const SAMPLES: usize = 64;
                let step = (edge.to - edge.from) / SAMPLES as f64;
                found
                    .extend((1..SAMPLES).map(|index| curve.point(edge.from + step * index as f64)));
            }
        }
        found
    }

    /// The faces using an edge, and which way each runs along it.
    pub fn uses(&self, edge: EdgeId) -> Vec<(FaceId, bool)> {
        self.face_ids()
            .flat_map(|face| {
                self.face(face)
                    .loops
                    .iter()
                    .flatten()
                    .filter(move |coedge| coedge.edge == edge)
                    .map(move |coedge| (face, coedge.forward))
            })
            .collect()
    }
}

/// The points of a circle, between two of its parameters, where a coordinate
/// is largest or smallest.
pub(super) fn turning_points(circle: &Circle, from: f64, to: f64) -> Vec<DVec3> {
    let mut found = Vec::new();
    for axis in 0..3 {
        let phase = circle.v[axis].atan2(circle.u[axis]);
        let first = ((from - phase) / PI).ceil() as i64;
        let last = ((to - phase) / PI).floor() as i64;
        found.extend((first..=last).map(|turn| circle.point(phase + turn as f64 * PI)));
    }
    found
}

#[cfg(test)]
mod tests;
