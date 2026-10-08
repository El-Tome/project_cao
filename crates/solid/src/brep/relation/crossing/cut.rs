//! A line a plane cuts a cylinder along, against a round whose axis stands
//! square to it — a circle in the plane, or a cylinder perpendicular to the
//! first: how far the line passes outside the round, read off the two radii
//! rather than off the line as rounded.
//!
//! Where the round's radius is the line's offset and the cylinder's radius
//! summed — a corner round touching the side a turned wall touches too — a
//! plane a hair `s` off the cylinder's axis cuts it a hair inside the round,
//! by `s² / 2r`: far below what a coordinate holds, so the line, rounded,
//! passes on the round or outside it, a touch, though it crosses it at a
//! slant `√(2R · s² / 2r)` either side, many tolerances apart (5365239969).

use glam::DVec3;

use crate::brep::curve::Line;
use crate::brep::surface::{Cylinder, Plane};

/// The cylinder a plane along its axis cuts along two lines.
pub(super) struct Cut<'a> {
    pub cylinder: &'a Cylinder,
    pub plane: &'a Plane,
}

impl Cut<'_> {
    /// How far `line`, one of the two the plane cuts the cylinder along,
    /// passes outside the round of `radius` about the line through `centre`
    /// along `axis`, square to the cylinder's; below nought inside it.
    pub fn gap(&self, line: &Line, centre: DVec3, axis: DVec3, radius: f64) -> f64 {
        let Cut { cylinder, plane } = *self;
        let away = plane.distance(cylinder.origin);
        let foot = cylinder.origin - plane.normal * away;
        let across = plane.normal.cross(cylinder.axis).normalize();
        let half = ((cylinder.radius - away.abs()) * (cylinder.radius + away.abs())).sqrt();
        let short = away * away / (cylinder.radius + half);
        let between = cylinder.axis.cross(axis).normalize();
        let side = (line.origin - foot).dot(across).signum();
        let base = (cylinder.origin - centre).dot(between) - away * plane.normal.dot(between);
        let along = side * across.dot(between);
        let sign = (base + along * half).signum();
        sign * (base + along * cylinder.radius) - radius - sign * along * short
    }
}
