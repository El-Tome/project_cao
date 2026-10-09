//! The apex a cone's face holds within it: a whole point's tip, which no
//! edge reaches, as a disc's centre is none of its boundary, and yet a
//! vertex of the body (#536): a place a user dimensions from. What reads a
//! face's extents off its edges reads this too.

use std::f64::consts::PI;

use glam::DVec3;

use super::{Body, Face, FaceId, Vertex, VertexId};
use crate::brep::curve::Curve;
use crate::brep::scale::Scale;
use crate::brep::surface::Surface;

impl Body {
    /// Where the vertex a face holds within it stands.
    pub(in crate::brep) fn apex_held(&self, face: FaceId) -> Option<DVec3> {
        self.face(face).apex.map(|vertex| self.vertex(vertex).point)
    }

    /// Each face given the vertex at the apex of its cone where it holds the
    /// apex within it, and none otherwise: a vertex of the body already
    /// standing there, or a new one on the cone alone.
    pub(in crate::brep) fn hold_apexes(&mut self, faces: &mut [Face]) {
        let eps = self.scale.eps();
        for face in faces {
            face.apex = self.apex_within(face).map(|apex| {
                match self
                    .vertices
                    .iter()
                    .position(|vertex| vertex.point.distance(apex) <= eps)
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
                            point: apex,
                            on: vec![face.surface],
                        });
                        VertexId(self.vertices.len() as u32 - 1)
                    }
                }
            });
        }
    }

    /// The apex of the cone a face lies on, where the face holds it within
    /// itself: no corner of the face stands there, and its circles about the
    /// axis go round it once, net. A ruling goes round nothing. A face with
    /// a corner at its apex is held by that corner already.
    fn apex_within(&self, face: &Face) -> Option<DVec3> {
        let Surface::Cone(cone) = self.surface(face.surface) else {
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
            if let Curve::Circle(circle) = self.curve(edge.curve)
                && circle.axis.cross(cone.axis).length() <= Scale::RELATIVE
            {
                let way = circle.axis.dot(cone.axis).signum();
                let run = if coedge.forward { 1.0 } else { -1.0 };
                swept += way * run * (edge.to - edge.from);
            }
        }
        (swept.abs() > PI).then_some(apex)
    }
}
