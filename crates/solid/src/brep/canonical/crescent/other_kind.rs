//! Decision 8 across kinds (#574): a circular wall and an elliptic one, one
//! of each operand, about one axis and within `Scale::HAIR` tolerances of
//! each other all round — an ellipse a hair from round about a circle of
//! its size, raised at a finer tolerance than the boolean's — are one
//! surface. Left two, they stand within the tolerance of each other over
//! bands no ruling tells apart, and the pair was declined.
//!
//! The second operand's wall is taken for the first's where it carries no
//! corner, a whole ring: nothing it built stands a hair off it but its
//! rims, cut again on the first's wall. A wall carrying a corner is left
//! two.

use std::f64::consts::TAU;

use crate::brep::conic::{Arm, Conic, Round};
use crate::brep::curve::{Circle, Curve};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, EllipticCylinder, Plane, Surface};
use crate::brep::topology::{Body, EdgeId, SurfaceId};

/// Under this share of the tolerance, two axes are one.
const ROUNDING: f64 = 1e-6;

/// The second operand with each whole wall standing about the axis of one
/// of the first's of the other kind, within `Scale::HAIR` tolerances of it
/// all round, laid on that wall; none when none is.
pub(super) fn laid_across(first: &Body, second: &Body, scale: Scale) -> Option<Body> {
    let mut body: Option<Body> = None;
    for rank in 0..second.surfaces.len() {
        let current = body.as_ref().unwrap_or(second);
        let own = current.surfaces[rank];
        let Some(known) = first
            .surfaces
            .iter()
            .copied()
            .find(|known| across(known, &own, scale))
        else {
            continue;
        };
        if let Some(laid) = laid_on(current, rank, known, scale) {
            body = Some(laid);
        }
    }
    body
}

/// Whether a circular wall and an elliptic one, either way round, stand
/// about one axis and within `Scale::HAIR` tolerances of each other all
/// round: furthest apart at the ends of the ellipse's half-axes.
fn across(known: &Surface, own: &Surface, scale: Scale) -> bool {
    let (round, oval) = match (known, own) {
        (Surface::Cylinder(round), Surface::Elliptic(oval))
        | (Surface::Elliptic(oval), Surface::Cylinder(round)) => (round, oval),
        _ => return false,
    };
    let eps = scale.eps();
    let parallel = round.axis.cross(oval.axis).length() * 2.0 * scale.reach() <= eps;
    let between = round.origin - oval.origin;
    let off = (between - oval.axis * oval.axis.dot(between)).length();
    let apart = (oval.major - round.radius)
        .abs()
        .max((oval.minor - round.radius).abs());
    parallel && off <= ROUNDING * eps && apart <= Scale::HAIR * eps
}

/// The body with its wall of rank `rank` put for `known`, and each of its
/// rims cut again on it on the plane the rim's cap stands on; none where a
/// corner stands on the wall, or an edge beside it is anything but a whole
/// rim.
fn laid_on(body: &Body, rank: usize, known: Surface, scale: Scale) -> Option<Body> {
    let id = SurfaceId(rank as u32);
    if body.vertices.iter().any(|vertex| vertex.on.contains(&id)) {
        return None;
    }
    let mut laid = body.clone();
    laid.surfaces[rank] = known;
    for face in body
        .face_ids()
        .filter(|face| body.face(*face).surface == id)
    {
        for coedge in body.face(face).loops.iter().flatten() {
            let edge = body.edge(coedge.edge);
            if edge.ends.is_some() {
                return None;
            }
            let cap = capped(body, coedge.edge, id)?;
            let rim = match known {
                Surface::Cylinder(round) => rim_of_round(&round, cap),
                Surface::Elliptic(oval) => rim_of_oval(&oval, cap, scale)?,
                Surface::Plane(_) | Surface::Cone(_) | Surface::Sphere(_) | Surface::Torus(_) => {
                    return None;
                }
            };
            let old = body.curve(edge.curve);
            let at = old.point(edge.from);
            let reversed = old
                .derivative(edge.from)
                .dot(rim.derivative(rim.parameter(at)))
                < 0.0;
            let to = match rim {
                Curve::Conic(conic) => conic.period()?,
                Curve::Line(_) | Curve::Circle(_) | Curve::Meet(_) => TAU,
            };
            laid.curves[edge.curve.0 as usize] = rim;
            let stretch = &mut laid.edges[coedge.edge.0 as usize];
            (stretch.from, stretch.to) = (0.0, to);
            if reversed {
                for use_of in laid
                    .faces
                    .iter_mut()
                    .flat_map(|face| face.loops.iter_mut().flatten())
                {
                    if use_of.edge == coedge.edge {
                        use_of.forward = !use_of.forward;
                    }
                }
            }
        }
    }
    Some(laid)
}

/// The plane of the cap a whole rim of the wall `wall` bounds.
fn capped(body: &Body, edge: EdgeId, wall: SurfaceId) -> Option<Plane> {
    body.faces
        .iter()
        .filter(|face| face.surface != wall)
        .filter(|face| {
            face.loops
                .iter()
                .flatten()
                .any(|coedge| coedge.edge == edge)
        })
        .find_map(|face| match body.surface(face.surface) {
            Surface::Plane(plane) => Some(*plane),
            _ => None,
        })
}

/// The rim a cap's plane cuts a circular wall along.
fn rim_of_round(round: &Cylinder, cap: Plane) -> Curve {
    Curve::Circle(Circle::on(
        round,
        (cap.origin - round.origin).dot(round.axis),
    ))
}

/// The rim a cap's plane cuts an elliptic wall along, whole.
fn rim_of_oval(oval: &EllipticCylinder, cap: Plane, scale: Scale) -> Option<Curve> {
    Conic::cut(&cap, &Round::Elliptic(*oval), scale)
        .into_iter()
        .find(|conic| conic.arm == Arm::Whole)
        .map(Curve::Conic)
}
