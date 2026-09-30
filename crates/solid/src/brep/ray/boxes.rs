//! The box round each face of a body, so that a place a ray reaches far from
//! a face is known to miss it without the face being traced.
//!
//! Every point of a face lies in the box of its boundary: on a plane it lies
//! between two points of the boundary, and on a cylinder between two points
//! of the boundary on the ruling through it. The box of a line is that of its
//! ends, of an arc of a circle that of its ends and of the places each
//! coordinate turns, both exact to rounding; the curve two cylinders meet
//! along is boxed by what its height across both axes leaves each root.

use std::f64::consts::{PI, TAU};

use glam::DVec3;

use crate::brep::curve::{Circle, Curve};
use crate::brep::topology::{Body, Edge, FaceId};

pub(super) struct Boxes {
    boxes: Vec<[DVec3; 2]>,
    margin: f64,
}

impl Boxes {
    /// The boxes of every face, a place being taken to miss one when it
    /// stands further than `eps` past it — the room the face's boundary is
    /// told within — and further than twice the body's own tolerance, which
    /// is how far an edge may stand off a surface it was decided to lie on.
    pub fn of(body: &Body, eps: f64) -> Boxes {
        let edges: Vec<[DVec3; 2]> = body
            .edge_ids()
            .map(|edge| boxed(body, body.edge(edge)))
            .collect();
        let boxes = body
            .face_ids()
            .map(|face| {
                body.face(face).loops.iter().flatten().fold(
                    [DVec3::INFINITY, DVec3::NEG_INFINITY],
                    |[low, high], coedge| {
                        let [from, to] = edges[coedge.edge.0 as usize];
                        [low.min(from), high.max(to)]
                    },
                )
            })
            .collect();
        Boxes {
            boxes,
            margin: eps + 2.0 * body.scale().eps(),
        }
    }

    /// Whether `point` stands so far from a face that it can be neither in it
    /// nor within `eps` of its boundary.
    pub fn misses(&self, face: FaceId, point: DVec3) -> bool {
        let [low, high] = self.boxes[face.0 as usize];
        point.cmplt(low - self.margin).any() || point.cmpgt(high + self.margin).any()
    }
}

fn boxed(body: &Body, edge: &Edge) -> [DVec3; 2] {
    let curve = body.curve(edge.curve);
    match curve {
        Curve::Line(_) => {
            let [from, to] = [edge.from, edge.to].map(|at| curve.point(at));
            [from.min(to), from.max(to)]
        }
        Curve::Circle(circle) => arc(circle, edge.from, edge.to),
        Curve::Meet(meet) => meet.bounds(edge.from, edge.to),
    }
}

/// Each coordinate of a circle is its centre's plus the radius times a
/// cosine, whose extremes over a stretch of angle are at its ends or where
/// the cosine is one or minus one.
fn arc(circle: &Circle, from: f64, to: f64) -> [DVec3; 2] {
    let (from, to) = (from.min(to), from.max(to));
    let [start, end] = [from, to].map(|at| circle.point(at));
    let mut low = start.min(end);
    let mut high = start.max(end);
    for axis in 0..3 {
        let phase = circle.v[axis].atan2(circle.u[axis]);
        let reach = circle.radius * circle.u[axis].hypot(circle.v[axis]);
        if passes(from, to, phase) {
            high[axis] = high[axis].max(circle.center[axis] + reach);
        }
        if passes(from, to, phase + PI) {
            low[axis] = low[axis].min(circle.center[axis] - reach);
        }
    }
    [low, high]
}

/// Whether the stretch from `from` to `to` holds an angle a whole number of
/// turns from `angle`.
fn passes(from: f64, to: f64, angle: f64) -> bool {
    angle + TAU * ((from - angle) / TAU).ceil() <= to
}
