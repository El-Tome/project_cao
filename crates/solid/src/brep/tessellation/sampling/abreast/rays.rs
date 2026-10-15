//! Where a ray from a point inside a wall leaves it, and the places the
//! rays of walls abreast pass through: each wall's grid, and the places an
//! elliptic wall's rims take on their own.

use std::f64::consts::TAU;

use glam::{DVec2, DVec3};

use super::super::divisions;
use super::super::elliptic::{on_the_wall, reached_by_planes};
use crate::brep::surface::Surface;
use crate::brep::topology::{Body, SurfaceId};

/// What a wall takes for a ray: a circular wall the direction from its
/// axis, an elliptic one the eccentric angle.
#[derive(Clone, Copy)]
pub(super) enum Taken {
    Ray(DVec3),
    Angle(f64),
}

/// Where the ray from `hub` along `way`, square to the wall's axis and
/// from inside it, leaves the wall, and what the wall takes for it.
pub(super) fn hit(wall: &Surface, hub: DVec3, way: DVec3) -> Option<(DVec3, Taken)> {
    match wall {
        Surface::Cylinder(round) => {
            let flat = |v: DVec3| v - round.axis * round.axis.dot(v);
            let from = flat(hub - round.origin);
            let along = from.dot(way);
            let reach = along * along - (from.length_squared() - round.radius * round.radius);
            if reach < 0.0 {
                return None;
            }
            let point = hub + way * (reach.sqrt() - along);
            let ray = flat(point - round.origin).try_normalize()?;
            Some((point, Taken::Ray(ray)))
        }
        Surface::Elliptic(oval) => {
            let squashed =
                |v: DVec3| DVec2::new(v.dot(oval.u) / oval.major, v.dot(oval.v) / oval.minor);
            let (from, pace) = (squashed(hub - oval.origin), squashed(way));
            let t = beyond(from, pace)?;
            let at = from + pace * t;
            Some((hub + way * t, Taken::Angle(at.y.atan2(at.x))))
        }
        Surface::Plane(_) | Surface::Cone(_) | Surface::Sphere(_) | Surface::Torus(_) => None,
    }
}

/// The points of a wall's grid at the height of its origin: a circle's
/// steps from its `u`, an ellipse's steps of eccentric angle from its major
/// half-axis, divided as a circle of that half-axis is.
pub(super) fn grid(wall: &Surface, tolerance: f64) -> Vec<DVec3> {
    let steps = match wall {
        Surface::Cylinder(round) => divisions(round.radius, tolerance),
        Surface::Elliptic(oval) => divisions(oval.major, tolerance),
        Surface::Plane(_) | Surface::Cone(_) | Surface::Sphere(_) | Surface::Torus(_) => {
            return Vec::new();
        }
    };
    (0..steps)
        .map(|step| wall.point(DVec2::new(TAU * step as f64 / steps as f64, 0.0)))
        .collect()
}

/// The places an elliptic wall's rims are sampled at besides its grid and
/// the rays given them (`conic.rs`), at the height of its origin; none for
/// another wall.
pub(super) fn alone(body: &Body, wall: SurfaceId, tolerance: f64) -> Vec<DVec3> {
    let Surface::Elliptic(oval) = body.surface(wall) else {
        return Vec::new();
    };
    let steps = divisions(oval.major, tolerance);
    on_the_wall(body, oval, steps)
        .into_iter()
        .chain(reached_by_planes(body, oval))
        .map(|angle| oval.point(DVec2::new(angle, 0.0)))
        .collect()
}

/// A point of a wall's axis, and its direction.
pub(super) fn axis_of(wall: &Surface) -> (DVec3, DVec3) {
    match wall {
        Surface::Cylinder(round) => (round.origin, round.axis),
        Surface::Elliptic(oval) => (oval.origin, oval.axis),
        Surface::Plane(plane) => (plane.origin, plane.normal),
        Surface::Cone(cone) => (cone.origin, cone.axis),
        Surface::Sphere(sphere) => (sphere.origin, sphere.axis),
        Surface::Torus(torus) => (torus.origin, torus.axis),
    }
}

/// How far along `pace` from `from` the unit circle is left, `from` inside
/// it.
fn beyond(from: DVec2, pace: DVec2) -> Option<f64> {
    let (a, b, c) = (
        pace.length_squared(),
        from.dot(pace),
        from.length_squared() - 1.0,
    );
    let reach = b * b - a * c;
    (a > 0.0 && reach >= 0.0).then(|| (reach.sqrt() - b) / a)
}
