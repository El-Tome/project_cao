//! The samples of curves standing just inside a wall they do not lie on.
//!
//! A wall's chords sag towards its axis, by up to a chord's sag between two
//! steps of its grid. A curve passing inside the wall closer than that — the
//! rim of a bore's cap ending inside a lying post, a hair under its wall —
//! has its samples outside the wall's triangles, and the faces it bounds
//! poke through them. A vertex there gives the wall's circles a ray through
//! it; a sample of such a curve does too, where the wall holds a face, and
//! however little inside it stands, as a vertex does: the kernel puts no
//! vertex where a cap's rim passes the line its plane leaves on the wall
//! by its tolerance, and seen from the wall's axis the rim stands inside
//! by less than that wherever the cap leans to the wall. A stand-in wall,
//! no surface of the body — under a cone's rim — holds no face: a sample
//! counts there on the plane of one of its circles.

pub(super) mod oval;

use std::collections::BTreeMap;
use std::f64::consts::PI;

use glam::DVec3;

use super::super::contact::{self, Wall, lies_on};
use super::{Samples, divisions_in};
use crate::brep::conic::Round;
use crate::brep::curve::{self, Curve, Line};
use crate::brep::domain::Location;
use crate::brep::scale::Scale;
use crate::brep::surface::Cylinder;
use crate::brep::topology::{Body, Edge, SurfaceId};

/// For each wall of the body, the samples of the curves not parallel to it
/// standing inside it closer than twice a chord's sag where it holds a face:
/// circles about another axis and the curves two other cylinders meet along.
/// A wall that is no surface of the body — under a cone's rim, or under the
/// circles a swallowed cylinder left on a cap — holds no face, and takes
/// those on the plane of one of its circles: that circle's chords sag there
/// as a cylinder's rim's do.
pub(super) fn walls(
    body: &Body,
    walls: &[Wall],
    samples: &Samples,
    tolerance: f64,
) -> BTreeMap<SurfaceId, Vec<DVec3>> {
    let eps = body.scale().eps();
    let mut beneath: BTreeMap<SurfaceId, Vec<DVec3>> = BTreeMap::new();
    for (id, cylinder) in walls {
        let sag = cylinder.radius
            * (1.0 - (PI / divisions_in(cylinder.radius, tolerance, samples.span) as f64).cos());
        let rims: Vec<DVec3> = body
            .edge_ids()
            .filter_map(|edge| match body.curve(body.edge(edge).curve) {
                Curve::Circle(circle) if lies_on(circle, cylinder, eps) => Some(circle.center),
                _ => None,
            })
            .collect();
        let held = (id.0 as usize) < body.surfaces.len();
        let holds = |point: DVec3| {
            if !held {
                return rims
                    .iter()
                    .any(|centre| (point - *centre).dot(cylinder.axis).abs() <= eps);
            }
            let at = cylinder.parameters(point);
            body.face_ids().any(|face| {
                body.face(face).surface == *id
                    && !matches!(body.locate(face, at, eps), Ok(Location::Outside))
            })
        };
        for edge in body.edge_ids() {
            if let Curve::Line(line) = body.curve(body.edge(edge).curve) {
                for point in through_rims(line, body.edge(edge), &rims, cylinder) {
                    let inside = cylinder.distance(point);
                    if (-2.0 * sag..0.0).contains(&inside) && holds(point) {
                        beneath.entry(*id).or_default().push(point);
                    }
                }
            }
            let across = match body.curve(body.edge(edge).curve) {
                Curve::Circle(circle) => {
                    circle.axis.cross(cylinder.axis).length() > Scale::RELATIVE
                }
                Curve::Meet(meet) => {
                    !contact::held(body, edge, meet).contains(&curve::Wall::Cylinder(*cylinder))
                }
                Curve::Conic(conic) => conic.round != Round::Cylinder(*cylinder),
                Curve::Line(_) => false,
            };
            if !across {
                continue;
            }
            for sample in samples.edge(edge) {
                if samples.is_vertex(*sample) {
                    continue;
                }
                let point = samples.point(*sample);
                let inside = cylinder.distance(point);
                if (-2.0 * sag..0.0).contains(&inside) && holds(point) {
                    beneath.entry(*id).or_default().push(point);
                }
            }
        }
    }
    beneath
}

/// Where a line's edge passes through the planes of a wall's rims, each
/// given by its centre: a rim's chords sag inside the wall there as the
/// wall's do, and a line leaving the wall's face a hair inside its rim — the
/// side of a pocket passing a stepped turn's step 6e-4 inside the step's
/// inner rim (5550106720) — passes under the triangles of the face the rim
/// bounds on its plane unless the rim is sampled there.
fn through_rims(line: &Line, edge: &Edge, rims: &[DVec3], cylinder: &Cylinder) -> Vec<DVec3> {
    let across = line.direction.dot(cylinder.axis);
    if across.abs() <= Scale::RELATIVE
        || line.direction.cross(cylinder.axis).length() <= Scale::RELATIVE
    {
        return Vec::new();
    }
    rims.iter()
        .map(|centre| (*centre - line.origin).dot(cylinder.axis) / across)
        .filter(|at| (edge.from..=edge.to).contains(at))
        .map(|at| line.point(at))
        .collect()
}
