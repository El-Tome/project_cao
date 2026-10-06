//! What the ends of a circle's edge lie on besides the circle.

use glam::DVec3;

use crate::brep::curve::{Circle, Curve};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Surface};
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
}

impl Ends {
    pub(super) fn of(body: &Body, circle: &Circle, edge: &Edge) -> Ends {
        let eps = body.scale().eps();
        let touches = |surface: &Surface| match surface {
            Surface::Plane(plane) => {
                plane.normal.dot(circle.axis).abs() <= Scale::RELATIVE
                    && (plane.distance(circle.center).abs() - circle.radius).abs() <= eps
            }
            Surface::Cylinder(_) => false,
        };
        let own = eps * TOLD;
        let holds = |surface: &Surface| match surface {
            Surface::Plane(plane) => {
                plane.normal.cross(circle.axis).length() <= Scale::RELATIVE
                    && plane.distance(circle.center).abs() <= own
            }
            Surface::Cylinder(cylinder) => contact::lies_on(circle, cylinder, own),
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
            };
        };
        let meeting = |vertex: VertexId| {
            body.edge_ids()
                .map(|id| body.edge(id))
                .filter(|other| !std::ptr::eq(*other, edge))
                .filter(|other| other.ends.is_some_and(|both| both.contains(&vertex)))
                .filter_map(|other| match body.curve(other.curve) {
                    Curve::Circle(circle) => Some(*circle),
                    _ => None,
                })
                .collect::<Vec<Circle>>()
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
            Surface::Plane(_) | Surface::Cylinder(_) => true,
        }
    }
}
