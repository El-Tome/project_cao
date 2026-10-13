//! Decision 8's analogue on an elliptic wall (#571): two parallel elliptic
//! walls, one of each operand, whose axes, half-axes and the turn between
//! their major half-axes all stand within `Scale::HAIR` tolerances are one
//! surface. Left two, they cross at four rulings or touch along two at
//! grazing angles, or stand within the tolerance of each other over a band
//! as wide as the turn, a crossing as ill conditioned as two circular walls
//! of one radius a hair apart.
//!
//! The second operand is moved square to the axis onto the first's, with
//! what it built, as decision 8 moves it onto a cylinder; decision 1 then
//! takes the wall for the first's where their half-axes and their turn
//! stand within the tolerance. Past it, a wall no corner of its operand
//! lies on, bounded by whole rims alone — an ellipse raised whole — is laid
//! on the first's wall itself and its rims cut again on it: nothing it built
//! is left a hair off. A wall carrying corners is left two. A circle and an
//! ellipse never merge this way.

use glam::DVec3;

use crate::brep::conic::{Arm, Conic, Round};
use crate::brep::curve::Curve;
use crate::brep::scale::Scale;
use crate::brep::surface::{EllipticCylinder, Surface};
use crate::brep::topology::{Body, SurfaceId};

/// Under this share of the tolerance, a wall already stands on the first's.
const ROUNDING: f64 = 1e-6;

/// The move square to the axes that lays an elliptic wall of the second
/// operand about the axis of the nearest of the first's within
/// `Scale::HAIR` tolerances of it in every measure; none where it stands on
/// it already, but for rounding.
pub(super) fn offset(first: &Body, own: &EllipticCylinder, scale: Scale) -> Option<DVec3> {
    let (distance, across) = first
        .surfaces
        .iter()
        .filter_map(|surface| match surface {
            Surface::Elliptic(known) => hair_off(known, own, scale),
            Surface::Plane(_) | Surface::Cylinder(_) | Surface::Cone(_) => None,
        })
        .min_by(|one, other| one.0.total_cmp(&other.0))?;
    (distance > ROUNDING * scale.eps()).then_some(across)
}

/// The second operand with each elliptic wall `offset` laid about the axis
/// of one of the first's, but still past the tolerance of it, laid on that
/// wall where nothing but whole rims bounds it; none when none is.
pub(super) fn reshaped(first: &Body, second: &Body, scale: Scale) -> Option<Body> {
    let mut body: Option<Body> = None;
    for rank in 0..second.surfaces.len() {
        let current = body.as_ref().unwrap_or(second);
        let Surface::Elliptic(own) = current.surfaces[rank] else {
            continue;
        };
        let Some(known) = first.surfaces.iter().find_map(|surface| match surface {
            Surface::Elliptic(known)
                if hair_off(known, &own, scale)
                    .is_some_and(|(distance, _)| distance <= ROUNDING * scale.eps())
                    && !known.within(&own, scale.eps()) =>
            {
                Some(*known)
            }
            _ => None,
        }) else {
            continue;
        };
        if let Some(laid) = laid_on(current, rank, &own, &known, scale) {
            body = Some(laid);
        }
    }
    body
}

/// How far the axis of `own` stands from that of `known`, and the move
/// square to it that lays one on the other, where the two walls are
/// parallel and within `Scale::HAIR` tolerances of each other in every
/// measure.
fn hair_off(
    known: &EllipticCylinder,
    own: &EllipticCylinder,
    scale: Scale,
) -> Option<(f64, DVec3)> {
    let eps = scale.eps();
    let parallel = known.axis.cross(own.axis).length() * 2.0 * scale.reach() <= eps;
    if !parallel || !known.within(own, Scale::HAIR * eps) {
        return None;
    }
    let between = known.origin - own.origin;
    let across = between - known.axis * known.axis.dot(between);
    Some((across.length(), across))
}

/// The body with its wall of rank `rank`, `own`, put for `known`, and its
/// rims cut again on it; none where a corner lies on the wall or an edge
/// beside it is anything but a whole rim of it.
fn laid_on(
    body: &Body,
    rank: usize,
    own: &EllipticCylinder,
    known: &EllipticCylinder,
    scale: Scale,
) -> Option<Body> {
    let id = SurfaceId(rank as u32);
    if body.vertices.iter().any(|vertex| vertex.on.contains(&id)) {
        return None;
    }
    let mut laid = body.clone();
    laid.surfaces[rank] = Surface::Elliptic(*known);
    for face in body
        .face_ids()
        .filter(|face| body.face(*face).surface == id)
    {
        for coedge in body.face(face).loops.iter().flatten() {
            let edge = body.edge(coedge.edge);
            let Curve::Conic(conic) = body.curve(edge.curve) else {
                return None;
            };
            if edge.ends.is_some() || conic.round != Round::Elliptic(*own) {
                return None;
            }
            let recut = Conic::cut(&conic.plane, &Round::Elliptic(*known), scale)
                .into_iter()
                .find(|cut| cut.arm == Arm::Whole)?;
            let period = recut.period()?;
            laid.curves[edge.curve.0 as usize] = Curve::Conic(recut);
            let stretch = &mut laid.edges[coedge.edge.0 as usize];
            stretch.to = stretch.from + period;
        }
    }
    Some(laid)
}
