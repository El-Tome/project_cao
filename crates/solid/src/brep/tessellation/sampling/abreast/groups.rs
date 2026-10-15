//! Which pairs of walls stand abreast, the groups the pairs join into, and
//! the point inside every wall of a group the rays are drawn from.

use glam::DVec3;

use super::rays::axis_of;
use crate::brep::surface::Surface;
use crate::brep::topology::{Body, SurfaceId};

/// Whether faces of two parallel walls stand at heights along the axis
/// that meet: two walls whose faces stand one above the other never face
/// each other, though one stands inside the other seen along the axis — a
/// post under a boss — and a step one leaves out there would leave a chord
/// across two.
pub(super) fn side_by_side([one, other]: [SurfaceId; 2], body: &Body) -> bool {
    let (_, axis) = axis_of(body.surface(one));
    let [first, second] = [one, other].map(|wall| heights(wall, axis, body));
    let eps = body.scale().eps();
    match (first, second) {
        (Some([low, high]), Some([from, to])) => from <= high + eps && low <= to + eps,
        _ => false,
    }
}

/// The least and the largest height along `axis` the ends of the edges
/// bounding a wall's faces stand at; none where it holds no face.
fn heights(wall: SurfaceId, axis: DVec3, body: &Body) -> Option<[f64; 2]> {
    let mut span: Option<[f64; 2]> = None;
    for face in body
        .face_ids()
        .filter(|face| body.face(*face).surface == wall)
    {
        for coedge in body.face(face).loops.iter().flatten() {
            let edge = body.edge(coedge.edge);
            for at in [edge.from, edge.to] {
                let height = axis.dot(body.point_on(coedge.edge, at));
                span = Some(span.map_or([height, height], |[low, high]| {
                    [low.min(height), high.max(height)]
                }));
            }
        }
    }
    span
}

/// The two walls of a pair, first the one whose axis stands inside the
/// other wall, the smaller where each stands inside the other's: a point
/// inside both, which every ray from it leaves once through each wall. None
/// where neither does.
pub(super) fn hubbed([one, other]: [SurfaceId; 2], body: &Body) -> Option<[SurfaceId; 2]> {
    let holds = |wall: SurfaceId, of: SurfaceId| {
        let (hub, _) = axis_of(body.surface(wall));
        body.surface(of).distance(hub) < 0.0
    };
    let size = |wall: SurfaceId| size(body.surface(wall));
    match (holds(one, other), holds(other, one)) {
        (true, true) if size(other) < size(one) => Some([other, one]),
        (true, _) => Some([one, other]),
        (false, true) => Some([other, one]),
        (false, false) => None,
    }
}

/// The walls pairs abreast join into groups, each wall in one: a wall
/// abreast of two others takes rays from both pairs, and each of those
/// others must take the rays of the other pair too, or its chords cross
/// the samples it never took (campaign seed 5571471048).
pub(super) fn grouped(pairs: &[[SurfaceId; 2]]) -> Vec<Vec<SurfaceId>> {
    let mut groups: Vec<Vec<SurfaceId>> = Vec::new();
    for pair in pairs {
        let mut group = pair.to_vec();
        let mut rank = groups.len();
        while rank > 0 {
            rank -= 1;
            if pair.iter().any(|wall| groups[rank].contains(wall)) {
                group.extend(groups.swap_remove(rank));
            }
        }
        group.sort();
        group.dedup();
        groups.push(group);
    }
    groups
}

/// A point inside every wall of a group, from which each ray leaves each
/// wall once: the axis of the smallest wall standing inside all the
/// others; none where no wall's does.
pub(super) fn common_hub(group: &[SurfaceId], body: &Body) -> Option<DVec3> {
    let mut by_size = group.to_vec();
    by_size.sort_by(|one, other| size(body.surface(*one)).total_cmp(&size(body.surface(*other))));
    by_size
        .into_iter()
        .map(|wall| axis_of(body.surface(wall)).0)
        .find(|hub| {
            group
                .iter()
                .all(|wall| body.surface(*wall).distance(*hub) < 0.0)
        })
}

/// How far a wall reaches from its axis at most.
fn size(wall: &Surface) -> f64 {
    match wall {
        Surface::Cylinder(round) => round.radius,
        Surface::Elliptic(oval) => oval.major,
        Surface::Plane(_) | Surface::Cone(_) | Surface::Sphere(_) | Surface::Torus(_) => {
            f64::INFINITY
        }
    }
}
