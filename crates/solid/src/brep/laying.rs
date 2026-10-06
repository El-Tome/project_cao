//! A body laid by hand, one surface, corner and edge at a time: what a raise
//! lays its walls with, and a turn its faces.

use std::f64::consts::TAU;

use glam::DVec3;

use super::curve::{Circle, Curve, Line};
use super::scale::Scale;
use super::surface::{Cylinder, Plane, Surface};
use super::topology::{Body, Coedge, CurveId, Edge, EdgeId, SurfaceId, Vertex, VertexId};

/// A body being laid, and the tolerance its surfaces are shared at.
pub(super) struct Laying {
    pub body: Body,
    eps: f64,
}

pub(super) fn reversed(coedge: Coedge) -> Coedge {
    Coedge {
        edge: coedge.edge,
        forward: !coedge.forward,
    }
}

impl Laying {
    pub fn new(eps: f64) -> Laying {
        Laying {
            body: Body {
                surfaces: Vec::new(),
                curves: Vec::new(),
                vertices: Vec::new(),
                edges: Vec::new(),
                faces: Vec::new(),
                scale: Scale::of(1.0),
                arrivals: Vec::new(),
            },
            eps,
        }
    }

    /// The plane through `point` whose matter lies against `outward`.
    pub fn plane(&mut self, point: DVec3, outward: DVec3) -> (SurfaceId, bool) {
        let (plane, turned) = Plane::through(point, outward);
        (self.surface(Surface::Plane(plane), &[point]), turned)
    }

    /// The surface's id, shared with a surface already there when the piece
    /// standing on it would stand on that one within `eps` too: each of its
    /// `corners`, and on a cylinder each point of its circles. Two runs of a
    /// contour on one line stand on one plane; two a hair apart do not.
    pub fn surface(&mut self, surface: Surface, corners: &[DVec3]) -> SurfaceId {
        let eps = self.eps;
        let same = |known: &Surface| {
            let alike = match (known, &surface) {
                (Surface::Plane(known), Surface::Plane(plane)) => {
                    known.normal.dot(plane.normal) > 0.0
                }
                (Surface::Cylinder(known), Surface::Cylinder(cylinder)) => {
                    known.axis.dot(cylinder.axis) > 0.0
                        && known.axis.cross(cylinder.axis).length() <= Scale::RELATIVE
                        && known.origin.distance(cylinder.origin)
                            + (known.radius - cylinder.radius).abs()
                            <= eps
                }
                (Surface::Plane(_), Surface::Cylinder(_))
                | (Surface::Cylinder(_), Surface::Plane(_)) => false,
            };
            alike
                && corners
                    .iter()
                    .all(|corner| known.distance(*corner).abs() <= eps)
        };
        let found = self.body.surfaces.iter().position(same);
        let index = found.unwrap_or_else(|| {
            self.body.surfaces.push(surface);
            self.body.surfaces.len() - 1
        });
        SurfaceId(index as u32)
    }

    pub fn vertex(&mut self, point: DVec3, on: &[SurfaceId]) -> VertexId {
        let mut on = on.to_vec();
        on.sort();
        on.dedup();
        self.body.vertices.push(Vertex { point, on });
        VertexId(self.body.vertices.len() as u32 - 1)
    }

    /// A straight edge from `start` to `end`, used that way.
    pub fn line(&mut self, start: VertexId, end: VertexId) -> Coedge {
        let [from, to] = [start, end].map(|id| self.body.vertex(id).point);
        let line = Line::through(from, to - from);
        let [from, to] = [from, to].map(|point| line.parameter(point));
        if from < to {
            self.edge(Curve::Line(line), Some([start, end]), from, to, true)
        } else {
            self.edge(Curve::Line(line), Some([end, start]), to, from, false)
        }
    }

    /// An arc of the cylinder's circle about `center`, from `ends[0]` turning
    /// by `sweep` about `up` to `ends[1]`, used that way.
    pub fn arc(
        &mut self,
        cylinder: &Cylinder,
        center: DVec3,
        ends: [VertexId; 2],
        sweep: f64,
        up: DVec3,
    ) -> Coedge {
        let circle = Circle::on(cylinder, (center - cylinder.origin).dot(cylinder.axis));
        let turning = sweep * cylinder.axis.dot(up).signum();
        let [start, end] = if turning > 0.0 {
            ends
        } else {
            [ends[1], ends[0]]
        };
        let from = circle.parameter(self.body.vertex(start).point);
        let span = turning.abs();
        self.edge(
            Curve::Circle(circle),
            Some([start, end]),
            from,
            from + span,
            turning > 0.0,
        )
    }

    /// A whole circle of the cylinder about `center`, used the way `sweep`
    /// turns about `up`.
    pub fn ring(&mut self, cylinder: &Cylinder, center: DVec3, sweep: f64, up: DVec3) -> Coedge {
        let circle = Circle::on(cylinder, (center - cylinder.origin).dot(cylinder.axis));
        let forward = sweep * cylinder.axis.dot(up) > 0.0;
        self.edge(Curve::Circle(circle), None, 0.0, TAU, forward)
    }

    /// A straight edge from `start` to `end` on the line `beside` runs
    /// along, used that way: the two radial lines of a half turn are one
    /// line, and their two edges can be joined into one only on one curve.
    pub fn line_on(&mut self, beside: Coedge, start: VertexId, end: VertexId) -> Coedge {
        let curve = self.body.edge(beside.edge).curve;
        let Curve::Line(line) = *self.body.curve(curve) else {
            return self.line(start, end);
        };
        let [from, to] = [start, end].map(|id| line.parameter(self.body.vertex(id).point));
        if from < to {
            self.edge_on(curve, Some([start, end]), from, to, true)
        } else {
            self.edge_on(curve, Some([end, start]), to, from, false)
        }
    }

    fn edge(
        &mut self,
        curve: Curve,
        ends: Option<[VertexId; 2]>,
        from: f64,
        to: f64,
        forward: bool,
    ) -> Coedge {
        self.body.curves.push(curve);
        let curve = CurveId(self.body.curves.len() as u32 - 1);
        self.edge_on(curve, ends, from, to, forward)
    }

    fn edge_on(
        &mut self,
        curve: CurveId,
        ends: Option<[VertexId; 2]>,
        from: f64,
        to: f64,
        forward: bool,
    ) -> Coedge {
        self.body.edges.push(Edge {
            curve,
            ends,
            from,
            to,
        });
        Coedge {
            edge: EdgeId(self.body.edges.len() as u32 - 1),
            forward,
        }
    }
}
