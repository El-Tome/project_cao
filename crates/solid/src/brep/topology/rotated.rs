//! A body turned rigidly about a point, with its surfaces, curves and
//! corners: what decision 11 makes of a second operand leaning a hair off
//! square to, or along, a wall's axis (`canonical/squared.rs`).

use std::collections::BTreeMap;

use glam::{DQuat, DVec3};

use super::{Body, Edge};
use crate::brep::curve::{Circle, Curve, Line};
use crate::brep::surface::{Cone, Cylinder, Plane, Surface, canonical_direction, reference};

impl Body {
    /// The body turned by `turn` about `pivot` and moved by `shift`, each
    /// direction it turns to
    /// within a sine of `within` of one of `anchors` laid on that one's
    /// bits, so that a pair the turn lays square or along compares exactly;
    /// none where a canonical direction would turn round, the way its curves
    /// run with it, or the body holds a curve the turn does not rebuild — a
    /// conic or the curve two perpendicular cylinders meet along.
    ///
    /// A circle reads its angle from the reference of its turned axis, as a
    /// wall about that axis does, and an edge on it starts nearest where it
    /// started, its extent kept: the reference moves with the axis.
    pub(in crate::brep) fn rotated(
        &self,
        pivot: DVec3,
        turn: DQuat,
        shift: DVec3,
        anchors: &[DVec3],
        within: f64,
    ) -> Option<Body> {
        let mut turned: BTreeMap<[u64; 3], DVec3> = BTreeMap::new();
        let mut direction = |before: DVec3| -> Option<DVec3> {
            let key = before.to_array().map(f64::to_bits);
            if let Some(found) = turned.get(&key) {
                return Some(*found);
            }
            let mut after = turn * before;
            if let Some(anchor) = anchors
                .iter()
                .find(|anchor| anchor.cross(after).length() <= within)
            {
                after = *anchor * anchor.dot(after).signum();
            }
            let (canonical, round) = canonical_direction(after);
            if round {
                return None;
            }
            turned.insert(key, canonical);
            Some(canonical)
        };
        let place = |point: DVec3| pivot + turn * (point - pivot) + shift;
        let mut moved = self.clone();
        for (rank, surface) in self.surfaces.iter().enumerate() {
            moved.surfaces[rank] = match surface {
                Surface::Plane(plane) => {
                    Surface::Plane(Plane::through(place(plane.origin), direction(plane.normal)?).0)
                }
                Surface::Cylinder(cylinder) => Surface::Cylinder(Cylinder::about(
                    place(cylinder.origin),
                    direction(cylinder.axis)?,
                    cylinder.radius,
                )),
                Surface::Cone(cone) => Surface::Cone(Cone::through(
                    place(cone.origin),
                    direction(cone.axis)?,
                    [cone.foot, cone.foot + cone.ruling],
                )),
            };
        }
        for (rank, curve) in self.curves.iter().enumerate() {
            moved.curves[rank] = match curve {
                Curve::Line(line) => Curve::Line(Line::through(
                    place(line.origin),
                    direction(line.direction)?,
                )),
                Curve::Circle(circle) => {
                    let axis = direction(circle.axis)?;
                    let u = reference(axis);
                    Curve::Circle(Circle {
                        center: place(circle.center),
                        axis,
                        radius: circle.radius,
                        u,
                        v: axis.cross(u),
                    })
                }
                Curve::Conic(_) | Curve::Meet(_) => return None,
            };
        }
        for vertex in &mut moved.vertices {
            vertex.point = place(vertex.point);
        }
        let edges = self
            .edges
            .iter()
            .map(|edge| moved.read_again(edge))
            .collect();
        moved.edges = edges;
        Some(moved)
    }

    /// The edge's stretch read again on its turned curve, between its turned
    /// corners: on a closed curve from the start nearest its own, over the
    /// extent it had; a whole closed curve over its period still.
    fn read_again(&self, edge: &Edge) -> Edge {
        let Some([from, to]) = edge.ends else {
            return edge.clone();
        };
        let curve = self.curve(edge.curve);
        let [start, end] = [from, to].map(|vertex| self.vertex(vertex).point);
        let (from, to) = match curve.period() {
            Some(period) => {
                let near = |at: f64, point: DVec3| {
                    let read = curve.parameter(point);
                    read + period * ((at - read) / period).round()
                };
                let from = near(edge.from, start);
                (from, near(edge.to + from - edge.from, end))
            }
            None => (curve.parameter(start), curve.parameter(end)),
        };
        Edge {
            from,
            to,
            ..edge.clone()
        }
    }
}
