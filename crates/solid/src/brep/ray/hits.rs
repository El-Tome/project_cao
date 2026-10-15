//! Where a line meets a plane, a cylinder, a cone, an elliptic wall, a
//! sphere or a torus, with the surface's own normal there: linear on a
//! plane, and on a wall the quadratic solved so that neither root is found
//! by taking two close numbers from each other, a cone's kept on its own
//! nappe; a sphere's quadratic, and a torus's quartic kept on its own lobe
//! (#576).

use glam::{DVec2, DVec3};

use crate::brep::curve::Line;
use crate::brep::scale::Scale;
use crate::brep::surface::{Cone, Cylinder, EllipticCylinder, Plane, Sphere, Surface, Torus};

/// A hit: the parameter along the line, and the surface's own normal there.
type Hit = Option<(f64, DVec3)>;

pub(super) enum Hits {
    /// Parameters along the line, each with the surface's own normal there:
    /// a line meets a plane once, a cylinder, a cone or a sphere twice at
    /// most, and a torus four times at most.
    At([Hit; 4]),
    /// The line runs along the surface, within the tolerance.
    Along,
}

/// `direction` is of unit length; `scale` is the body's, within which a
/// cone's apex and rulings are told whatever `eps` the ray is cast with.
pub(super) fn hits(
    surface: &Surface,
    origin: DVec3,
    direction: DVec3,
    eps: f64,
    scale: Scale,
) -> Hits {
    match surface {
        Surface::Plane(plane) => on_plane(plane, origin, direction, eps),
        Surface::Cylinder(cylinder) => on_cylinder(cylinder, origin, direction, eps),
        Surface::Cone(cone) => on_cone(cone, origin, direction, scale),
        Surface::Elliptic(wall) => on_elliptic(wall, origin, direction, eps),
        Surface::Sphere(sphere) => on_sphere(sphere, origin, direction),
        Surface::Torus(torus) => on_torus(torus, origin, direction),
    }
}

/// Two hits at most, as every surface but a torus has.
fn two([one, other]: [Hit; 2]) -> Hits {
    Hits::At([one, other, None, None])
}

fn on_plane(plane: &Plane, origin: DVec3, direction: DVec3, eps: f64) -> Hits {
    let away = plane.distance(origin);
    let across = plane.normal.dot(direction);
    if across == 0.0 {
        return if away.abs() <= eps {
            Hits::Along
        } else {
            two([None; 2])
        };
    }
    two([Some((-away / across, plane.normal)), None])
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
            two([None; 2])
        };
    }
    let half = from.dot(towards);
    let rest = (from.length() - radius) * (from.length() + radius);
    let discriminant = half * half - speed * rest;
    if discriminant < 0.0 {
        return two([None; 2]);
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
    two(roots.map(|root| root.map(|at| (at, (from + towards * at) / radius))))
}

/// The cylinder's quadratic, read in the elliptic wall's section squashed
/// onto the unit circle, whose roots stand where the line's do.
fn on_elliptic(wall: &EllipticCylinder, origin: DVec3, direction: DVec3, eps: f64) -> Hits {
    let squashed = |v: DVec3| {
        let flat = v - wall.axis * wall.axis.dot(v);
        DVec2::new(flat.dot(wall.u) / wall.major, flat.dot(wall.v) / wall.minor)
    };
    let (from, towards) = (squashed(origin - wall.origin), squashed(direction));
    let speed = towards.length_squared();
    if speed <= f64::EPSILON * f64::EPSILON {
        return if wall.distance(origin).abs() <= eps {
            Hits::Along
        } else {
            two([None; 2])
        };
    }
    let half = from.dot(towards);
    let rest = (from.length() - 1.0) * (from.length() + 1.0);
    let discriminant = half * half - speed * rest;
    if discriminant < 0.0 {
        return two([None; 2]);
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
    two(roots.map(|root| {
        root.map(|at| {
            let theta = wall.parameters(origin + direction * at).x;
            (at, wall.normal(theta))
        })
    }))
}

/// A line through the apex, where the normal is no answer and the two roots
/// are one, or along a ruling runs along the cone as far as a ray can tell:
/// another direction settles it. Both are told within the body's own
/// tolerance, however fine the one the ray is cast with: a ray a rounding
/// from the apex finds its roots there only by chance.
fn on_cone(cone: &Cone, origin: DVec3, direction: DVec3, scale: Scale) -> Hits {
    let eps = scale.eps();
    let apex = cone.apex();
    let towards = apex - origin;
    let passing = (towards - direction * towards.dot(direction)).length();
    let held = apex.abs().max_element() <= scale.reach() + eps;
    if (held && passing <= eps) || cone.rules(&Line::through(origin, direction), scale) {
        return Hits::Along;
    }
    two(cone.roots(origin, direction).map(|root| {
        root.map(|at| {
            let theta = cone.parameters(origin + direction * at).x;
            (at, cone.normal(theta))
        })
    }))
}

/// No line runs along a sphere: its roots, each with the normal there, a
/// touch found once.
fn on_sphere(sphere: &Sphere, origin: DVec3, direction: DVec3) -> Hits {
    two(sphere.roots(origin, direction).map(|root| {
        root.map(|at| {
            let point = origin + direction * at;
            (at, sphere.normal(sphere.parameters(point)))
        })
    }))
}

/// No line runs along a torus: its roots on the own lobe, each with the
/// normal there, a touch found once.
fn on_torus(torus: &Torus, origin: DVec3, direction: DVec3) -> Hits {
    Hits::At(torus.roots(origin, direction).map(|root| {
        root.map(|at| {
            let point = origin + direction * at;
            (at, torus.normal(torus.parameters(point)))
        })
    }))
}
