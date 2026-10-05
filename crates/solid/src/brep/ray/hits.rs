//! Where a line meets a plane or a cylinder, with the surface's own normal
//! there: linear on a plane, and on a cylinder the quadratic solved so that
//! neither root is found by taking two close numbers from each other.

use glam::DVec3;

use crate::brep::surface::{Cylinder, Plane, Surface};

pub(super) enum Hits {
    /// Parameters along the line, each with the surface's own normal there:
    /// a line meets a plane once and a cylinder twice at most.
    At([Option<(f64, DVec3)>; 2]),
    /// The line runs along the surface, within the tolerance.
    Along,
}

/// `direction` is of unit length.
pub(super) fn hits(surface: &Surface, origin: DVec3, direction: DVec3, eps: f64) -> Hits {
    match surface {
        Surface::Plane(plane) => on_plane(plane, origin, direction, eps),
        Surface::Cylinder(cylinder) => on_cylinder(cylinder, origin, direction, eps),
    }
}

fn on_plane(plane: &Plane, origin: DVec3, direction: DVec3, eps: f64) -> Hits {
    let away = plane.distance(origin);
    let across = plane.normal.dot(direction);
    if across == 0.0 {
        return if away.abs() <= eps {
            Hits::Along
        } else {
            Hits::At([None; 2])
        };
    }
    Hits::At([Some((-away / across, plane.normal)), None])
}

fn on_cylinder(cylinder: &Cylinder, origin: DVec3, direction: DVec3, eps: f64) -> Hits {
    let flat = |v: DVec3| v - cylinder.axis * cylinder.axis.dot(v);
    let (from, towards) = (flat(origin - cylinder.origin), flat(direction));
    let speed = towards.length_squared();
    let radius = cylinder.radius;
    if speed <= f64::EPSILON * f64::EPSILON {
        return if (from.length() - radius).abs() <= eps {
            Hits::Along
        } else {
            Hits::At([None; 2])
        };
    }
    let half = from.dot(towards);
    let rest = (from.length() - radius) * (from.length() + radius);
    let discriminant = half * half - speed * rest;
    if discriminant < 0.0 {
        return Hits::At([None; 2]);
    }
    let far = -(half + discriminant.sqrt().copysign(half));
    let roots = if far == 0.0 {
        [Some(0.0), None]
    } else {
        let [one, other] = [far / speed, rest / far];
        match one.total_cmp(&other) {
            std::cmp::Ordering::Greater => [Some(other), Some(one)],
            _ => [Some(one), Some(other)],
        }
    };
    Hits::At(roots.map(|root| root.map(|at| (at, (from + towards * at) / radius))))
}
