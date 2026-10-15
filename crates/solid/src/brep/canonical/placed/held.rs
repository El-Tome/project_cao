//! What an operand may hold against the other, read off its body: its
//! flats, its straight edges, the axes of its walls, and its cylinders and
//! elliptic walls as an axis and how far they reach from it.

use glam::{DQuat, DVec3};

use crate::brep::curve::Curve;
use crate::brep::surface::{EllipticCylinder, Plane, Surface};
use crate::brep::topology::{Body, Edge};

/// A line, as a point of it and its direction.
type Straight = (DVec3, DVec3);

/// What an operand may hold against the other: its flats, its straight
/// edges, the axes of its walls, and its cylinders and elliptic walls as an
/// axis and how far they reach from it, the second's turned and set about
/// where the two meet.
pub(super) struct Held {
    pub(super) flats: Vec<Plane>,
    pub(super) edges: Vec<Straight>,
    pub(super) axes: Vec<Straight>,
    pub(super) walls: Vec<(Straight, Reach)>,
}

/// How far a wall reaches from its axis: a cylinder its radius all round,
/// an elliptic wall as far as its support says along each direction.
#[derive(Clone, Copy)]
pub(super) enum Reach {
    Round(f64),
    Oval(EllipticCylinder),
}

impl Reach {
    /// How far the wall reaches along `across`, square to its axis and of
    /// unit length.
    pub(super) fn along(&self, across: DVec3) -> f64 {
        match self {
            Reach::Round(radius) => *radius,
            Reach::Oval(wall) => wall.support(across),
        }
    }

    /// The reach of the wall turned by `turn`, which its support reads off
    /// its turned half-axes.
    fn turned(self, turn: DQuat) -> Reach {
        match self {
            Reach::Round(_) => self,
            Reach::Oval(wall) => Reach::Oval(EllipticCylinder {
                axis: turn * wall.axis,
                u: turn * wall.u,
                v: turn * wall.v,
                ..wall
            }),
        }
    }
}

impl Held {
    pub(super) fn of(
        body: &Body,
        near: (DVec3, DVec3),
        eps: f64,
        place: impl Fn(DVec3) -> DVec3,
        turn: DQuat,
    ) -> Held {
        let line = |(point, direction): Straight| (place(point), turn * direction);
        Held {
            flats: flats(body, near, eps)
                .iter()
                .map(|flat| Plane::through(place(flat.origin), turn * flat.normal).0)
                .collect(),
            edges: edges(body, near, eps).into_iter().map(line).collect(),
            axes: axes(body).map(line).collect(),
            walls: walls(body)
                .map(|(axis, reach)| (line(axis), reach.turned(turn)))
                .collect(),
        }
    }
}

/// The flats of a body: its planes, and the plane of each circle it has an
/// edge on within `grown` of the other operand's box.
fn flats(body: &Body, near: (DVec3, DVec3), grown: f64) -> Vec<Plane> {
    let planes = body.surfaces.iter().filter_map(|surface| match surface {
        Surface::Plane(plane) => Some(*plane),
        Surface::Cylinder(_)
        | Surface::Cone(_)
        | Surface::Elliptic(_)
        | Surface::Sphere(_)
        | Surface::Torus(_) => None,
    });
    let circles = body
        .edges
        .iter()
        .filter(|edge| reaches(body, edge, near, grown))
        .filter_map(|edge| match body.curve(edge.curve) {
            Curve::Circle(circle) => Some(Plane::through(circle.center, circle.axis).0),
            _ => None,
        });
    planes.chain(circles).collect()
}

/// The axis of each wall of a body.
fn axes(body: &Body) -> impl Iterator<Item = Straight> {
    body.surfaces.iter().filter_map(|surface| match surface {
        Surface::Cylinder(cylinder) => Some((cylinder.origin, cylinder.axis)),
        Surface::Cone(cone) => Some((cone.origin, cone.axis)),
        Surface::Elliptic(wall) => Some((wall.origin, wall.axis)),
        Surface::Sphere(sphere) => Some((sphere.origin, sphere.axis)),
        Surface::Torus(torus) => Some((torus.origin, torus.axis)),
        Surface::Plane(_) => None,
    })
}

/// The cylinders and the elliptic walls of a body, as their axis and how
/// far they reach from it.
fn walls(body: &Body) -> impl Iterator<Item = (Straight, Reach)> {
    body.surfaces.iter().filter_map(|surface| match surface {
        Surface::Cylinder(cylinder) => Some((
            (cylinder.origin, cylinder.axis),
            Reach::Round(cylinder.radius),
        )),
        Surface::Elliptic(wall) => Some(((wall.origin, wall.axis), Reach::Oval(*wall))),
        Surface::Plane(_) | Surface::Cone(_) | Surface::Sphere(_) | Surface::Torus(_) => None,
    })
}

/// Each line a body has an edge on within `grown` of the other operand's
/// box.
fn edges(body: &Body, near: (DVec3, DVec3), grown: f64) -> Vec<Straight> {
    body.edges
        .iter()
        .filter(|edge| reaches(body, edge, near, grown))
        .filter_map(|edge| match body.curve(edge.curve) {
            Curve::Line(line) => Some((line.origin, line.direction)),
            _ => None,
        })
        .collect()
}

/// Whether an edge of the body reaches within `grown` of the box.
fn reaches(body: &Body, edge: &Edge, (low, high): (DVec3, DVec3), grown: f64) -> bool {
    let points = body.extremes(edge);
    let one = points
        .iter()
        .fold(DVec3::INFINITY, |least, point| least.min(*point));
    let other = points
        .iter()
        .fold(DVec3::NEG_INFINITY, |most, point| most.max(*point));
    one.cmple(high + grown).all() && (low - grown).cmple(other).all()
}
