//! What the ends of an edge lie on besides its curve: for a circle, read
//! once for its sampling; for any curve, to tell a place grazing such a
//! surface a step from an end.

use glam::DVec3;

use crate::brep::conic::Round;
use crate::brep::curve::{Circle, Curve};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Plane, Surface};
use crate::brep::tessellation::contact;
use crate::brep::topology::{Body, Edge, VertexId};

use super::TOLD;

/// What the ends of a circle's edge, its start then its end, lie on besides
/// the circle: whether a plane touching the circle's wall there, and every
/// surface other than those holding the circle — to within rounding, not the
/// kernel's tolerance: a wall a hair off the circle's own, decided apart from
/// it, is another surface however the tolerance has grown since.
pub(super) struct Ends {
    pub(super) touched: [bool; 2],
    pub(super) through: [Vec<Surface>; 2],
    /// The other circles ending at each end.
    pub(super) meeting: [Vec<Circle>; 2],
    /// The planes cutting the circle's own wall along a conic ending at
    /// each end: the other rim of a strip of that wall pinching there.
    pub(super) pinching: [Vec<Plane>; 2],
}

impl Ends {
    pub(super) fn of(body: &Body, circle: &Circle, edge: &Edge) -> Ends {
        let eps = body.scale().eps();
        let touches = |surface: &Surface| match surface {
            Surface::Plane(plane) => {
                plane.normal.dot(circle.axis).abs() <= Scale::RELATIVE
                    && (plane.distance(circle.center).abs() - circle.radius).abs() <= eps
            }
            Surface::Cylinder(_) | Surface::Cone(_) | Surface::Elliptic(_) => false,
        };
        let own = eps * TOLD;
        let holds = |surface: &Surface| match surface {
            Surface::Plane(plane) => {
                plane.normal.cross(circle.axis).length() <= Scale::RELATIVE
                    && plane.distance(circle.center).abs() <= own
            }
            Surface::Cylinder(cylinder) => contact::lies_on(circle, cylinder, own),
            Surface::Cone(cone) => cone.holds(circle, own),
            Surface::Elliptic(_) => false,
        };
        let surfaces = |vertex: VertexId| {
            body.vertex(vertex)
                .on
                .iter()
                .map(|surface| *body.surface(*surface))
                .collect::<Vec<Surface>>()
        };
        let Some(ends) = edge.ends else {
            return Ends {
                touched: [false; 2],
                through: [Vec::new(), Vec::new()],
                meeting: [Vec::new(), Vec::new()],
                pinching: [Vec::new(), Vec::new()],
            };
        };
        let ending = |vertex: VertexId| {
            body.edge_ids()
                .map(|id| body.edge(id))
                .filter(|other| !std::ptr::eq(*other, edge))
                .filter(move |other| other.ends.is_some_and(|both| both.contains(&vertex)))
                .map(|other| body.curve(other.curve))
        };
        let meeting = |vertex: VertexId| {
            ending(vertex)
                .filter_map(|curve| match curve {
                    Curve::Circle(circle) => Some(*circle),
                    _ => None,
                })
                .collect::<Vec<Circle>>()
        };
        let pinching = |vertex: VertexId| {
            ending(vertex)
                .filter_map(|curve| match curve {
                    Curve::Conic(conic) => {
                        let on_the_wall = match &conic.round {
                            Round::Cylinder(cylinder) => contact::lies_on(circle, cylinder, own),
                            Round::Cone(cone) => cone.holds(circle, own),
                            Round::Elliptic(_) => false,
                        };
                        on_the_wall.then_some(conic.plane)
                    }
                    _ => None,
                })
                .collect::<Vec<Plane>>()
        };
        Ends {
            touched: ends.map(|end| surfaces(end).iter().any(touches)),
            through: ends.map(|end| {
                surfaces(end)
                    .into_iter()
                    .filter(|surface| !holds(surface))
                    .collect()
            }),
            meeting: ends.map(meeting),
            pinching: ends.map(pinching),
        }
    }
}

impl Ends {
    /// Whether a circle of `wall` ends at the end on `side`.
    pub(super) fn closes(&self, side: usize, wall: &Cylinder, eps: f64) -> bool {
        self.meeting[side]
            .iter()
            .any(|circle| contact::lies_on(circle, wall, eps))
    }

    /// Whether a circle about `axis` all but lying on `surface` a step from
    /// the end on `side` grazes it there: a plane, or a wall square to the
    /// circle's, lying over the circle's plane; a wall parallel to the
    /// circle's only where a circle of it ends at the same vertex, as the
    /// circle crossing it at a slant elsewhere stands on no arc of it.
    pub(super) fn grazes(&self, side: usize, surface: &Surface, axis: DVec3, eps: f64) -> bool {
        match surface {
            Surface::Cylinder(wall) if wall.axis.cross(axis).length() <= Scale::RELATIVE => {
                self.closes(side, wall, eps)
            }
            Surface::Plane(_) | Surface::Cylinder(_) | Surface::Cone(_) | Surface::Elliptic(_) => {
                true
            }
        }
    }
}

/// Whether a place of an edge stands within `room` of a surface one of its
/// ends lies on and none of the curve's `own` surfaces is, `step` or less
/// from that end.
pub(super) fn grazed<'a>(
    body: &'a Body,
    edge: &'a Edge,
    own: &[Surface],
    step: f64,
) -> impl Fn(DVec3, f64) -> bool + use<'a> {
    grazed_near(body, edge, own, move |point: DVec3, end: DVec3| {
        (point - end).length() <= step
    })
}

/// Whether a place of an edge stands within `room` of a surface one of its
/// ends lies on and none of the curve's `own` surfaces is, where `near` says
/// the place stands near that end.
fn grazed_near<'a, Near>(
    body: &'a Body,
    edge: &'a Edge,
    own: &[Surface],
    near: Near,
) -> impl Fn(DVec3, f64) -> bool + use<'a, Near>
where
    Near: Fn(DVec3, DVec3) -> bool + 'a,
{
    let ends: Vec<(DVec3, Vec<Surface>)> = edge
        .ends
        .into_iter()
        .flatten()
        .map(|end| {
            let vertex = body.vertex(end);
            let surfaces = vertex
                .on
                .iter()
                .map(|id| *body.surface(*id))
                .filter(|surface| !own.contains(surface))
                .collect();
            (vertex.point, surfaces)
        })
        .collect();
    move |point: DVec3, room: f64| {
        ends.iter().any(|(end, surfaces)| {
            near(point, *end)
                && surfaces
                    .iter()
                    .any(|surface| surface.distance(point).abs() < room)
        })
    }
}
