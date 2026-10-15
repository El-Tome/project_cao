//! A body turned rigidly about a point, with its surfaces, curves and
//! corners: what decision 11 makes of a second operand leaning a hair off
//! square to, or along, a wall's axis (`canonical/squared.rs`).

use std::collections::BTreeMap;

use glam::{DQuat, DVec3};

use super::{Body, Edge};
use crate::brep::conic::{Arm, Conic, Round};
use crate::brep::curve::{Circle, Curve, Line};
use crate::brep::surface::{
    Cone, Cylinder, EllipticCylinder, Plane, Sphere, Surface, Torus, canonical_direction, reference,
};

impl Body {
    /// The body turned by `turn` about `pivot` and moved by `shift`, each
    /// direction it turns to
    /// within a sine of `within` of one of `anchors` laid on that one's
    /// bits, so that a pair the turn lays square or along compares exactly;
    /// none where a canonical direction would turn round, the way its curves
    /// run with it, or the body holds a curve the turn does not rebuild — a
    /// conic but an elliptic wall's whole section, or the curve two
    /// perpendicular cylinders meet along.
    ///
    /// A circle reads its angle from the reference of its turned axis, as a
    /// wall about that axis does, and an edge on it starts nearest where it
    /// started, its extent kept: the reference moves with the axis. An
    /// elliptic wall turns its axis and its half-axes together, and the
    /// section a plane cuts it along is cut again on the two turned.
    pub(in crate::brep) fn rotated(
        &self,
        pivot: DVec3,
        turn: DQuat,
        shift: DVec3,
        anchors: &[DVec3],
        within: f64,
    ) -> Option<Body> {
        let mut turning = Turning {
            pivot,
            turn,
            shift,
            anchors,
            within,
            turned: BTreeMap::new(),
        };
        let mut moved = self.clone();
        for (rank, surface) in self.surfaces.iter().enumerate() {
            moved.surfaces[rank] = match surface {
                Surface::Plane(plane) => Surface::Plane(turning.plane(plane)?),
                Surface::Cylinder(cylinder) => Surface::Cylinder(Cylinder::about(
                    turning.place(cylinder.origin),
                    turning.direction(cylinder.axis)?,
                    cylinder.radius,
                )),
                Surface::Cone(cone) => Surface::Cone(Cone::through(
                    turning.place(cone.origin),
                    turning.direction(cone.axis)?,
                    [cone.foot, cone.foot + cone.ruling],
                )),
                Surface::Elliptic(wall) => Surface::Elliptic(turning.oval(wall)?),
                Surface::Sphere(sphere) => Surface::Sphere(Sphere::through(
                    turning.place(sphere.middle()),
                    turning.direction(sphere.axis)?,
                    0.0,
                    sphere.radius,
                )),
                Surface::Torus(torus) => Surface::Torus(Torus::through(
                    turning.place(torus.origin),
                    turning.direction(torus.axis)?,
                    torus.center,
                    torus.radius,
                    torus.middle(),
                )),
            };
        }
        for (rank, curve) in self.curves.iter().enumerate() {
            moved.curves[rank] = match curve {
                Curve::Line(line) => Curve::Line(Line::through(
                    turning.place(line.origin),
                    turning.direction(line.direction)?,
                )),
                Curve::Circle(circle) => {
                    let axis = turning.direction(circle.axis)?;
                    let u = reference(axis);
                    Curve::Circle(Circle {
                        center: turning.place(circle.center),
                        axis,
                        radius: circle.radius,
                        u,
                        v: axis.cross(u),
                    })
                }
                Curve::Conic(conic) => Curve::Conic(turning.section(conic)?),
                Curve::Meet(_) => return None,
            };
        }
        for vertex in &mut moved.vertices {
            vertex.point = turning.place(vertex.point);
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
    /// extent it had; a whole closed curve over its period still, a conic's
    /// read again, which its turn may lengthen by rounding.
    fn read_again(&self, edge: &Edge) -> Edge {
        let curve = self.curve(edge.curve);
        let Some([from, to]) = edge.ends else {
            return match (curve, curve.period()) {
                (Curve::Conic(_), Some(period)) => Edge {
                    to: edge.from + period,
                    ..edge.clone()
                },
                _ => edge.clone(),
            };
        };
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

/// A turn about `pivot` and the shift after it, and the directions of the
/// first operand a direction it turns to within a sine of `within` is laid
/// on.
struct Turning<'a> {
    pivot: DVec3,
    turn: DQuat,
    shift: DVec3,
    anchors: &'a [DVec3],
    within: f64,
    turned: BTreeMap<[u64; 3], DVec3>,
}

impl Turning<'_> {
    fn place(&self, point: DVec3) -> DVec3 {
        self.pivot + self.turn * (point - self.pivot) + self.shift
    }

    /// The direction turned and laid on an anchor within `within` of it,
    /// canonical, the same for the same bits; none where it turns round.
    fn direction(&mut self, before: DVec3) -> Option<DVec3> {
        let key = before.to_array().map(f64::to_bits);
        if let Some(found) = self.turned.get(&key) {
            return Some(*found);
        }
        let mut after = self.turn * before;
        if let Some(anchor) = self
            .anchors
            .iter()
            .find(|anchor| anchor.cross(after).length() <= self.within)
        {
            after = *anchor * anchor.dot(after).signum();
        }
        let (canonical, round) = canonical_direction(after);
        if round {
            return None;
        }
        self.turned.insert(key, canonical);
        Some(canonical)
    }

    fn plane(&mut self, plane: &Plane) -> Option<Plane> {
        Some(Plane::through(self.place(plane.origin), self.direction(plane.normal)?).0)
    }

    /// An elliptic wall turned rigidly: its axis as any direction, its
    /// major half-axis laid on an anchor square to the turned axis within
    /// `within` of it, made square to that axis otherwise, its half-axes'
    /// lengths kept to the bit. The major half-axis may turn round, which
    /// starts the wall's angle half a turn on without turning the way it
    /// runs.
    fn oval(&mut self, wall: &EllipticCylinder) -> Option<EllipticCylinder> {
        let axis = self.direction(wall.axis)?;
        let after = self.turn * wall.u;
        let u = self
            .anchors
            .iter()
            .find(|anchor| {
                anchor.cross(after).length() <= self.within && anchor.dot(axis).abs() <= self.within
            })
            .copied()
            .unwrap_or_else(|| canonical_direction(after - axis * axis.dot(after)).0);
        let center = self.place(wall.origin);
        Some(EllipticCylinder {
            origin: center - axis * axis.dot(center),
            axis,
            u,
            v: axis.cross(u),
            major: wall.major,
            minor: wall.minor,
        })
    }

    /// The whole section a plane cuts an elliptic wall along, cut again on
    /// the turned plane and wall; none for any other conic.
    fn section(&mut self, conic: &Conic) -> Option<Conic> {
        let (Round::Elliptic(wall), Arm::Whole) = (conic.round, conic.arm) else {
            return None;
        };
        let plane = self.plane(&conic.plane)?;
        Some(Conic::rim(&plane, &self.oval(&wall)?))
    }
}
