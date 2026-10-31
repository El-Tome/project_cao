//! A face on a sphere or a torus, read along a direction and about a line
//! (#576). A round face bulges between its edges — a zone across the
//! equator, a lune between its meridians, a whole ball with no edge at all —
//! so its reach along a direction is read off its edges and off the points
//! of its surface that go furthest along that direction, where they stand
//! inside the face; and how far it stands from a line at most is read off
//! the whole surface. A face stands apart from a round's as from a cone's:
//! beyond a plane touching the round, or, a ring, inside the hollow its tube
//! leaves about the axis.

use std::f64::consts::PI;

use glam::{DVec2, DVec3};

use super::hollow::corners;
use super::{farthest, from_the_axis, reach};
use crate::brep::domain::Location;
use crate::brep::surface::{Surface, Torus};
use crate::brep::topology::{Body, FaceId};

/// The points of a face's sphere or torus that go furthest along
/// `direction` or against it — a sphere's two, a torus's four, found where
/// the surface's gradient along the direction vanishes — kept where they
/// stand inside the face or on its boundary, and every one where the face's
/// domain cannot be read. None for any other surface.
pub(super) fn bulges((body, face): (&Body, FaceId), direction: DVec3) -> Vec<DVec3> {
    let surface = body.surface(body.face(face).surface);
    let candidates = match surface {
        Surface::Sphere(sphere) => [1.0, -1.0]
            .map(|way| sphere.middle() + direction * (way * sphere.radius))
            .to_vec(),
        Surface::Torus(torus) => bulges_of_a_torus(torus, direction),
        Surface::Plane(_) | Surface::Cylinder(_) | Surface::Cone(_) | Surface::Elliptic(_) => {
            return Vec::new();
        }
    };
    let eps = body.scale().eps();
    candidates
        .into_iter()
        .filter(|point| {
            !matches!(
                body.locate(face, surface.parameters(*point), eps),
                Ok(Location::Outside)
            )
        })
        .collect()
}

/// Where a torus goes furthest along `direction` or against it: in the
/// meridian half-plane of the direction's part square to the axis and in
/// the one across from it, at the angle along the meridian whose tangent is
/// the direction's part along the axis over its part across, and half a
/// turn from it; on the own lobe only.
fn bulges_of_a_torus(torus: &Torus, direction: DVec3) -> Vec<DVec3> {
    let along = direction.dot(torus.axis);
    let across = direction - torus.axis * along;
    let theta = across.dot(torus.v).atan2(across.dot(torus.u));
    let (big, radius) = (torus.center.y, torus.radius);
    let mut found = Vec::new();
    for turn in [0.0, PI] {
        let phi = along.atan2(across.length() * turn.cos());
        for half in [0.0, PI] {
            let phi = phi + half;
            if big + radius * phi.cos() >= 0.0 {
                found.push(torus.point(DVec2::new(theta + turn, phi)));
            }
        }
    }
    found
}

/// How far a face's sphere or torus stands from the line through `origin`
/// along `axis` at most, read off the whole surface: its centre's distance
/// from the line, its ring radius and its tube radius. None for any other
/// surface.
pub(super) fn all_round(surface: &Surface, origin: DVec3, axis: DVec3) -> Option<f64> {
    match surface {
        Surface::Sphere(sphere) => {
            Some(from_the_axis(sphere.middle(), origin, axis) + sphere.radius)
        }
        Surface::Torus(torus) => Some(
            from_the_axis(torus.origin + torus.axis * torus.center.x, origin, axis)
                + torus.center.y.abs()
                + torus.radius,
        ),
        Surface::Plane(_) | Surface::Cylinder(_) | Surface::Cone(_) | Surface::Elliptic(_) => None,
    }
}

/// Whether a face stands beyond a plane touching the round at the point of
/// it nearest one of the face's corners, or their mean, on the side away
/// from the round — a block beside a ball knob: a sphere lies whole on one
/// side of any plane touching it, a torus on one side of a plane touching
/// the half of its tube away from the axis, and no further.
pub(super) fn beyond(round: &Surface, face: (&Body, FaceId), room: f64) -> bool {
    let corners = corners(face);
    let mean = corners.iter().sum::<DVec3>() / corners.len().max(1) as f64;
    corners.iter().chain([&mean]).any(|&corner| {
        let (at, normal) = match round {
            Surface::Sphere(sphere) => {
                let at = sphere.nearest(corner);
                (at, sphere.normal(sphere.parameters(at)))
            }
            Surface::Torus(torus) => {
                let at = torus.nearest(corner);
                let seen = torus.parameters(at);
                let normal = torus.normal(seen);
                if normal.dot(torus.radial(seen.x)) < 0.0 {
                    return false;
                }
                (at, normal)
            }
            Surface::Plane(_) | Surface::Cylinder(_) | Surface::Cone(_) | Surface::Elliptic(_) => {
                return false;
            }
        };
        reach(face, normal).is_some_and(|[low, _]| low > normal.dot(at) + room)
    })
}

/// Whether a face stands nearer a ring's axis everywhere than its tube
/// reaches anywhere: a post standing in a ring's hole.
pub(super) fn in_the_hollow(round: &Surface, face: (&Body, FaceId), room: f64) -> bool {
    let Surface::Torus(torus) = round else {
        return false;
    };
    let inner = torus.center.y.abs() - torus.radius;
    inner > 0.0
        && farthest(face, torus.origin, torus.axis).is_some_and(|reached| reached + room < inner)
}
