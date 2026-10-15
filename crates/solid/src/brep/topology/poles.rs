//! The poles a face holds within it: a cone's apex (#536), the two ends of
//! a sphere or of a spindle torus (#576) — a whole point's tip, a ball's
//! ends — which no edge reaches, as a disc's centre is none of its
//! boundary, and yet vertices of the body: places a user dimensions from.
//! What reads a face's extents off its edges reads these too.

use std::f64::consts::PI;

use glam::DVec3;

use super::{Body, Face, FaceId, Vertex, VertexId};
use crate::brep::curve::Curve;
use crate::brep::domain::{floor, traced};
use crate::brep::scale::Scale;
use crate::brep::surface::Surface;

impl Body {
    /// Where the vertices a face holds within it stand.
    pub(in crate::brep) fn poles_held(&self, face: FaceId) -> Vec<DVec3> {
        self.face(face)
            .poles
            .iter()
            .map(|vertex| self.vertex(*vertex).point)
            .collect()
    }

    /// Each face given the vertices at the poles of its surface it holds
    /// within it, and none otherwise: a vertex of the body already standing
    /// there, or a new one on the surface alone.
    pub(in crate::brep) fn hold_poles(&mut self, faces: &mut [Face]) {
        let eps = self.scale.eps();
        for face in faces {
            face.poles = self
                .poles_within(face)
                .into_iter()
                .map(|pole| {
                    match self
                        .vertices
                        .iter()
                        .position(|vertex| vertex.point.distance(pole) <= eps)
                    {
                        Some(rank) => {
                            let on = &mut self.vertices[rank].on;
                            if let Err(at) = on.binary_search(&face.surface) {
                                on.insert(at, face.surface);
                            }
                            VertexId(rank as u32)
                        }
                        None => {
                            self.vertices.push(Vertex {
                                point: pole,
                                on: vec![face.surface],
                            });
                            VertexId(self.vertices.len() as u32 - 1)
                        }
                    }
                })
                .collect();
        }
    }

    /// The poles of the surface a face lies on that it holds within itself.
    fn poles_within(&self, face: &Face) -> Vec<DVec3> {
        let surface = *self.surface(face.surface);
        match surface {
            Surface::Cone(_) => self.apex_within(face).into_iter().collect(),
            Surface::Sphere(_) | Surface::Torus(_) => self.round_poles_within(face, &surface),
            Surface::Plane(_) | Surface::Cylinder(_) | Surface::Elliptic(_) => Vec::new(),
        }
    }

    /// The apex of the cone a face lies on, where the face holds it within
    /// itself: no corner of the face stands there, and its circles about the
    /// axis and its conics go round it once, net, a conic by the angle its
    /// trace on the cone turns through — the tip a plane at a slant cuts off
    /// a point, held by an ellipse alone. A ruling goes round nothing. A face
    /// with a corner at its apex is held by that corner already.
    fn apex_within(&self, face: &Face) -> Option<DVec3> {
        let surface = *self.surface(face.surface);
        let Surface::Cone(cone) = surface else {
            return None;
        };
        let apex = cone.apex();
        let eps = self.scale.eps();
        let mut swept = 0.0;
        for coedge in face.loops.iter().flatten() {
            let edge = self.edge(coedge.edge);
            let at_the_apex = edge
                .ends
                .iter()
                .flatten()
                .any(|vertex| self.vertex(*vertex).point.distance(apex) <= eps);
            if at_the_apex {
                return None;
            }
            let run = if coedge.forward { 1.0 } else { -1.0 };
            match self.curve(edge.curve) {
                Curve::Circle(circle)
                    if circle.axis.cross(cone.axis).length() <= Scale::RELATIVE =>
                {
                    let way = circle.axis.dot(cone.axis).signum();
                    swept += way * run * (edge.to - edge.from);
                }
                curve @ Curve::Conic(_) => {
                    if let Ok(trace) = traced(curve, &surface, edge.from, edge.to, eps) {
                        swept += run * (trace.end().x - trace.start().x);
                    }
                }
                Curve::Line(_) | Curve::Circle(_) | Curve::Meet(_) => {}
            }
        }
        (swept.abs() > PI).then_some(apex)
    }

    /// The poles of the sphere or the torus a face lies on that it holds
    /// within itself: none that a corner of the face stands at, and the
    /// others by parity along the meridian from inside the face, read off
    /// its loops' traces ([`floor::holds`]). A face whose loops cannot be
    /// traced holds none.
    fn round_poles_within(&self, face: &Face, surface: &Surface) -> Vec<DVec3> {
        let eps = self.scale.eps();
        let Ok(loops) = self.traces_of(face) else {
            return Vec::new();
        };
        floor::poles_of(surface, eps)
            .into_iter()
            .filter(|pole| {
                let cornered = face.loops.iter().flatten().any(|coedge| {
                    self.edge(coedge.edge)
                        .ends
                        .iter()
                        .flatten()
                        .any(|vertex| self.vertex(*vertex).point.distance(pole.point) <= eps)
                });
                !cornered && floor::holds(&loops, pole)
            })
            .map(|pole| pole.point)
            .collect()
    }
}
