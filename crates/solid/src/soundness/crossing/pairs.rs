//! Which two faces of a surface are worth weighing against each other.
//!
//! Weighing every pair of twenty thousand faces is two hundred million
//! weighings. Most pairs are nowhere near each other, and most of the rest are
//! neighbours in one fan, which cannot cross.

use std::f64::consts::TAU;

use glam::DVec3;

use super::{Face, Pair};

mod boxes;

use boxes::Boxes;

/// Hands over every two faces whose boxes come within `room` of each other,
/// save two of one fan, in the order of a sweep along the axis or diagonal
/// they spread along most — a bar pushed slantwise has every wall as long as
/// it along each axis — and hands back that way. Which faces are near is asked
/// of their boxes nested by halves rather than of every face the sweep passes.
pub(super) fn for_each_pair_near(
    faces: &[Face],
    room: f64,
    mut visit: impl FnMut(&Face, &Face),
) -> DVec3 {
    let mut way = (f64::INFINITY, DVec3::X);
    for n in 14..27 {
        let axis = DVec3::new((n % 3) as f64, (n / 3 % 3) as f64, (n / 9) as f64) - 1.0;
        let (mut low, mut high, mut spans) = (f64::INFINITY, f64::NEG_INFINITY, 0.0);
        for [from, to] in faces.iter().map(|face| face.span(axis.normalize())) {
            (low, high, spans) = (low.min(from), high.max(to), spans + to - from);
        }
        let crowding = spans / (high - low).max(f64::MIN_POSITIVE);
        if crowding < way.0 {
            way = (crowding, axis.normalize());
        }
    }
    let hot = |(at, face): (usize, &Face)| (face.span(way.1), face.bounds, face.fans, at);
    let mut swept: Vec<_> = faces.iter().enumerate().map(hot).collect();
    swept.sort_by(|one, other| one.0[0].total_cmp(&other.0[0]));
    let starts: Vec<f64> = swept.iter().map(|each| each.0[0]).collect();
    let bounds: Vec<[DVec3; 2]> = swept.iter().map(|each| each.1).collect();
    let boxes = Boxes::of(&bounds);
    let mut ahead = Vec::new();
    for (at, ([_, end], [low, high], fans, one)) in swept.iter().enumerate() {
        let past = at + 1 + starts[at + 1..].partition_point(|start| *start <= end + room);
        ahead.clear();
        boxes.near(*low, *high, room, at + 1..past, &mut ahead);
        ahead.sort_unstable();
        for &next in &ahead {
            let (_, [other_low, other_high], others, other) = &swept[next];
            let near = other_low.cmple(*high + room) & low.cmple(*other_high + room);
            if near.all() && !fans.iter().any(|fan| *fan > 0 && others.contains(fan)) {
                visit(&faces[*one], &faces[*other]);
            }
        }
    }
    way.1
}

/// Marks the faces facing one way round a corner they all have, flat and each
/// clear of the others: a fan, as the kernel cuts its faces into. No two cross,
/// and weighing them pair by pair takes a cap of five thousand sides twelve
/// million times. Hands back the neighbours, whose shared edges still count.
pub(super) fn fans(faces: &mut [Face], room: f64) -> Vec<Pair> {
    let mut corners = Vec::with_capacity(3 * faces.len());
    for (at, face) in faces.iter().enumerate() {
        let facing = (face.normal * 1e3).round().as_i64vec3().to_array();
        for corner in 0..3 {
            let bits = face.corners[corner].to_array().map(f64::to_bits);
            corners.push((bits, facing, at, corner));
        }
    }
    corners.sort_unstable();
    let mut neighbours = Vec::new();
    let one_fan = |one: &(_, _, _, _), other: &(_, _, _, _)| (one.0, one.1) == (other.0, other.1);
    for (fan, around) in corners.chunk_by(one_fan).enumerate() {
        let first = &faces[around[0].2];
        let (centre, normal) = (first.corners[around[0].3], first.normal);
        let across = normal.any_orthonormal_vector();
        let up = normal.cross(across);
        let angle = |point: DVec3| up.dot(point - centre).atan2(across.dot(point - centre));
        let flat = |point: DVec3| normal.dot(point - centre).abs() <= room;
        let mut wedges = Vec::new();
        for &(_, _, at, corner) in around {
            let [next, previous] = [1, 2].map(|step| faces[at].corners[(corner + step) % 3]);
            let (start, end) = (angle(next), angle(previous));
            if flat(next) && flat(previous) {
                wedges.push((start, end + if end < start { TAU } else { 0.0 }, at, corner));
            }
        }
        wedges.sort_by(|one, other| one.0.total_cmp(&other.0));
        let clear = (0..wedges.len()).all(|at| {
            let next = wedges.get(at + 1).map_or(wedges[0].0 + TAU, |next| next.0);
            wedges[at].1 <= next + 1e-12
        });
        for (at, &(_, _, face, corner)) in wedges.iter().enumerate() {
            if wedges.len() > 1 && clear {
                faces[face].fans[corner] = fan + 1;
                neighbours.push((face, wedges[(at + 1) % wedges.len()].2));
            }
        }
    }
    neighbours
}
