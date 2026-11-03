//! Decision 13 (#644): a plane of the second operand standing parallel to a
//! straight edge of the first a plane bounds, within `Scale::HAIR`
//! tolerances of it and past rounding, is moved onto it. A cut at a slant
//! drawn through a point a hair off a block's edge passes the edge by less
//! than the tolerance or a few times it; the block's two faces along the
//! edge cross the cut along two lines a hair from the edge and from each
//! other, three planes nearly sharing one line, and the kernel read the
//! edge on the cut and its faces' lines across it: the block was refused
//! (5585500692, 5585502204). Laid through the edge, the three share it
//! exactly.
//!
//! The move is a hair only where the cut crosses every plane along the
//! edge within the hair of it. A cut a hair off parallel to one of them
//! crosses it at the gap over the lean, far from the edge, and laid through
//! the edge it would take off that face far more than the tolerance: blocks
//! that held were refused (5585501810, 5585505134). A wall along the edge
//! is not asked: a cut a hair from tangent to a cone along the ruling the
//! edge is, laid through it, touches the cone there (5585502064). Two edges
//! a hair apart, a step a hair high, are no such corner either: laid
//! through one, the cut would pass the other by less than the tolerance
//! (5585505818). Nor is an edge two walls bound: the line a pin touches a
//! disc along, which the band decisions lay, stays as they lay it, and so
//! does a block's face a hair from it (90016363).
//!
//! The second operand's plane moves along its normal with what it built on
//! it, as `carried.rs` moves it.

use glam::DVec3;

use super::carried::carried_along;
use crate::brep::curve::Curve;
use crate::brep::domain::Location;
use crate::brep::relation::{Relation, relation};
use crate::brep::scale::Scale;
use crate::brep::surface::{Plane, Surface};
use crate::brep::topology::{Body, FaceId};

/// Under this share of the tolerance, an edge stands on a plane, or along
/// it, as it is.
const ROUNDING: f64 = 1e-6;

/// The second operand with each of its planes standing a hair from a
/// straight edge of the first, parallel to it, moved onto it; none when
/// nothing moves.
pub(in crate::brep) fn edged(first: &Body, second: &Body, scale: Scale) -> Option<Body> {
    carried_along(first, second, scale, |current, rank| {
        let plane = match current.surfaces[rank] {
            Surface::Plane(plane) => plane,
            Surface::Cylinder(_)
            | Surface::Cone(_)
            | Surface::Elliptic(_)
            | Surface::Sphere(_)
            | Surface::Torus(_) => return None,
        };
        offset(first, current, rank, &plane, scale)
    })
}

/// The move along the normal that lays `plane`, of rank `rank` in `second`,
/// through the straight edges of the first operand parallel to it within
/// rounding over their length, past rounding from it and within
/// `Scale::HAIR` tolerances, whose ends or middle stand over a face of the
/// plane, and whose planes it crosses within the hair of them. None where
/// decision 1 takes the plane for one of the first's, where such an edge
/// lies on the plane already — moved, it would stand a hair off — and where
/// two such edges stand at gaps more than rounding apart. An edge two
/// walls of the first bound is passed by, and so is one a face of the
/// first nearly parallel to the plane bounds: two planes a hair apart are
/// decision 10's, which leaves them two unless a wall touches one.
fn offset(first: &Body, second: &Body, rank: usize, plane: &Plane, scale: Scale) -> Option<DVec3> {
    let eps = scale.eps();
    let own = Surface::Plane(*plane);
    if first
        .surfaces
        .iter()
        .any(|known| matches!(relation(known, &own, scale), Relation::Same { .. }))
    {
        return None;
    }
    let faces: Vec<FaceId> = second
        .face_ids()
        .filter(|face| second.face(*face).surface.0 as usize == rank)
        .collect();
    let over_a_face = |point: DVec3| {
        faces.iter().any(|face| {
            matches!(
                second.locate(*face, own.parameters(point), Scale::HAIR * eps),
                Ok(Location::Inside | Location::Boundary)
            )
        })
    };
    let mut gaps = Vec::new();
    for id in first.edge_ids() {
        let edge = first.edge(id);
        let Curve::Line(line) = first.curve(edge.curve) else {
            continue;
        };
        let along: Vec<&Surface> = first
            .uses(id)
            .into_iter()
            .map(|(face, _)| first.surface(first.face(face).surface))
            .collect();
        let walls = along
            .iter()
            .filter(|surface| !matches!(surface, Surface::Plane(_)))
            .count();
        if walls > 1 {
            continue;
        }
        let ends = [line.point(edge.from), line.point(edge.to)];
        let middle = (ends[0] + ends[1]) / 2.0;
        let [from, to] = ends.map(|end| plane.normal.dot(end - plane.origin));
        let gap = (from + to) / 2.0;
        let crossed_far = along.iter().any(|surface| {
            matches!(surface, Surface::Plane(known)
                if gap.abs() > Scale::HAIR * eps * known.normal.cross(plane.normal).length())
        });
        if (from - to).abs() > ROUNDING * eps
            || gap.abs() > Scale::HAIR * eps
            || crossed_far
            || ![ends[0], middle, ends[1]].into_iter().any(over_a_face)
        {
            continue;
        }
        if gap.abs() <= ROUNDING * eps {
            return None;
        }
        gaps.push(gap);
    }
    let gap = *gaps.first()?;
    if gaps
        .iter()
        .any(|other| (other - gap).abs() > ROUNDING * eps)
    {
        return None;
    }
    Some(plane.normal * gap)
}
