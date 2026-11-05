//! A face on a sphere or a torus, read about a line (#576). A round face
//! bulges between its edges — a zone across the equator, a lune between its
//! meridians, a whole ball with no edge at all — so how far it stands from a
//! line at most is read off the whole surface, and its reach along a
//! direction off its edges and off where it bulges ([`Body::bulges`]). A
//! face stands apart from a round's as from a cone's: beyond a plane
//! touching the round, or, a ring, inside the hollow its tube leaves about
//! the axis.

use glam::DVec3;

use super::hollow::corners;
use super::{farthest, from_the_axis, reach};
use crate::brep::surface::Surface;
use crate::brep::topology::{Body, FaceId};

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
