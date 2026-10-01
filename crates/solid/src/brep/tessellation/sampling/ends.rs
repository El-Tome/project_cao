//! What the ends of a circle's edge lie on besides the circle.

use crate::brep::curve::Circle;
use crate::brep::scale::Scale;
use crate::brep::surface::Surface;
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
            };
        };
        Ends {
            touched: ends.map(|end| surfaces(end).iter().any(touches)),
            through: ends.map(|end| {
                surfaces(end)
                    .into_iter()
                    .filter(|surface| !holds(surface))
                    .collect()
            }),
        }
    }
}
