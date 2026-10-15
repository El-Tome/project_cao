//! Two parallel walls: the lines they meet along, the steps of either's
//! grid withheld beside them, and the rays two walls at one place share.

use std::collections::BTreeMap;
use std::f64::consts::TAU;

use glam::DVec3;

use super::Wall;
use super::facing::Facing;
use crate::brep::scale::Scale;
use crate::brep::surface::Cylinder;
use crate::brep::tessellation::sampling::divisions;
use crate::brep::topology::SurfaceId;

/// Gives each wall the rays every wall at one place with it took on its own.
///
/// Two walls of one axis and one radius to within the kernel's tolerance —
/// decided apart while it was finer, two surfaces still — each hold every
/// circle of either, and a face a hair wide between a circle of each, a step
/// from one to the other, is cut into nothing only where the two are sampled
/// on the same rays. Each kept to its own, one takes the rays of a curve it
/// meets another wall along and the other does not, and the step's chords
/// cross.
pub(super) fn one_place(walls: &[Wall], eps: f64, own: &mut BTreeMap<SurfaceId, Vec<DVec3>>) {
    for (at, (one, first)) in walls.iter().enumerate() {
        for (other, second) in &walls[at + 1..] {
            let between = second.origin - first.origin;
            let offset = between - first.axis * first.axis.dot(between);
            let parallel = first.axis.cross(second.axis).length() <= Scale::RELATIVE;
            if !parallel || offset.length() > eps || (first.radius - second.radius).abs() > eps {
                continue;
            }
            let [mine, theirs] = [one, other].map(|id| own.get(id).cloned().unwrap_or_default());
            own.entry(*one).or_default().extend(theirs);
            own.entry(*other).or_default().extend(mine);
        }
    }
}

/// The directions from the axes of two parallel cylinders to each line their
/// walls meet along: the one they touch along, side by side or one inside
/// the other, or the two they cross along.
///
/// Every circle of either is sampled on those lines, vertex or not: the steps
/// of the grid beside them are withheld, and a ring passing one with no
/// vertex there — its wall meeting the other only further along, or not at
/// all where the other's wall is gone — would otherwise cut across it by a
/// chord several steps long.
pub(super) fn meeting_lines(one: &Cylinder, other: &Cylinder, eps: f64) -> Vec<[DVec3; 2]> {
    let between = other.origin - one.origin;
    let flat = between - one.axis * one.axis.dot(between);
    let apart = flat.length();
    if apart == 0.0 {
        return Vec::new();
    }
    let way = flat / apart;
    let (near, far) = ((one.radius - other.radius).abs(), one.radius + other.radius);
    if (apart - far).abs() <= eps {
        return vec![[way, -way]];
    }
    if (apart - near).abs() <= eps {
        let out = if one.radius >= other.radius {
            way
        } else {
            -way
        };
        return vec![[out, out]];
    }
    if apart < near || apart > far {
        return Vec::new();
    }
    let along =
        apart / 2.0 + (one.radius - other.radius) * (one.radius + other.radius) / (2.0 * apart);
    let aside = (one.radius * one.radius - along * along).max(0.0).sqrt();
    let square = one.axis.cross(way);
    [-1.0, 1.0]
        .into_iter()
        .map(|side| {
            let line = way * along + square * side * aside;
            [line.normalize(), (line - flat).normalize()]
        })
        .collect()
}

/// The steps of a cylinder's grid standing within the pair's room of another
/// cylinder parallel to it, where the two face each other, each with the
/// stretches of height they face each other over: none unless their circles
/// come that close somewhere.
pub(super) fn touching(
    (id, cylinder): &Wall,
    other: &Cylinder,
    facing: &Facing,
    tolerance: f64,
    eps: f64,
) -> Vec<(usize, [f64; 2])> {
    let between = cylinder.origin - other.origin;
    let apart = (between - other.axis * other.axis.dot(between)).length();
    let (low, high) = (
        (cylinder.radius - other.radius).abs(),
        cylinder.radius + other.radius,
    );
    if apart < low - eps || apart > high + eps {
        return Vec::new();
    }
    let steps = divisions(cylinder.radius, tolerance);
    (0..steps)
        .flat_map(|step| {
            let angle = TAU * step as f64 / steps as f64;
            let point = cylinder.origin + cylinder.radial(angle) * cylinder.radius;
            let over = if facing.crowds(cylinder, point) {
                facing.withheld(*id, point)
            } else {
                Vec::new()
            };
            over.into_iter().map(move |stretch| (step, stretch))
        })
        .collect()
}
