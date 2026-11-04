//! The kernel's surfaces and curves as the entities of ISO 10303-42, each
//! curve parameterised the way the kernel's own runs, so an edge keeps its
//! sense.

mod conic;
mod meet;

use glam::DVec3;

use super::instances::{Instances, real};
use crate::brep::{Circle, Cone, Curve, Surface, Torus};

/// The surface's instance, and whether its normal is the kernel's.
pub fn surface(instances: &mut Instances, surface: &Surface) -> (usize, bool) {
    match surface {
        Surface::Plane(plane) => {
            let placement = instances.placement(plane.origin, plane.normal, plane.u);
            (instances.add(format!("PLANE('',#{placement})")), true)
        }
        Surface::Cylinder(cylinder) => {
            let placement = instances.placement(cylinder.origin, cylinder.axis, cylinder.u);
            let radius = instances.length(cylinder.radius);
            let surface = format!("CYLINDRICAL_SURFACE('',#{placement},{radius})");
            (instances.add(surface), true)
        }
        Surface::Cone(cone) => self::cone(instances, cone),
        Surface::Elliptic(wall) => {
            let around = wall.u.cross(wall.v).normalize();
            let placement = instances.placement(wall.origin, around, wall.u);
            let (major, minor) = (instances.length(wall.major), instances.length(wall.minor));
            let ellipse = instances.add(format!("ELLIPSE('',#{placement},{major},{minor})"));
            let along = instances.direction(around);
            let vector = instances.add(format!("VECTOR('',#{along},1.)"));
            let surface = format!("SURFACE_OF_LINEAR_EXTRUSION('',#{ellipse},#{vector})");
            (instances.add(surface), true)
        }
        Surface::Sphere(sphere) => {
            let center = sphere.origin + sphere.axis * sphere.center;
            let placement = instances.placement(center, sphere.axis, sphere.u);
            let radius = instances.length(sphere.radius);
            let surface = format!("SPHERICAL_SURFACE('',#{placement},{radius})");
            (instances.add(surface), true)
        }
        Surface::Torus(torus) => (self::torus(instances, torus), true),
    }
}

/// STEP's cone opens along its axis and is placed where its radius is no
/// less than nought: at the foot of the meridian line, or at the apex when
/// the foot stands beyond it. Its normal points away from the axis, the
/// kernel's only where the nappe opens along the kernel's axis.
fn cone(instances: &mut Instances, cone: &Cone) -> (usize, bool) {
    let opens = cone.ruling.x > 0.0;
    let axis = if opens { cone.axis } else { -cone.axis };
    let (height, radius) = if cone.foot.y >= 0.0 {
        (cone.foot.x, cone.foot.y)
    } else {
        (cone.height_at(cone.apex_at()), 0.0)
    };
    let placement = instances.placement(cone.origin + cone.axis * height, axis, cone.u);
    let radius = instances.length(radius);
    let angle = real(cone.ruling.y.atan2(cone.ruling.x.abs()));
    let surface = format!("CONICAL_SURFACE('',#{placement},{radius},{angle})");
    (instances.add(surface), opens)
}

/// A spindle's own lobe is the outer part of the torus of radius `|R|` when
/// its meridian circle's centre stands on the profile's side of the axis,
/// the inner part when across it.
fn torus(instances: &mut Instances, torus: &Torus) -> usize {
    let center = torus.origin + torus.axis * torus.center.x;
    let placement = instances.placement(center, torus.axis, torus.u);
    let (major, minor) = (torus.center.y.abs(), torus.radius);
    let radii = format!("{},{}", instances.length(major), instances.length(minor));
    if major < minor {
        let outer = super::instances::flag(torus.center.y > 0.0);
        instances.add(format!(
            "DEGENERATE_TOROIDAL_SURFACE('',#{placement},{radii},{outer})"
        ))
    } else {
        instances.add(format!("TOROIDAL_SURFACE('',#{placement},{radii})"))
    }
}

pub fn curve(instances: &mut Instances, curve: &Curve, from: f64, to: f64) -> usize {
    match curve {
        Curve::Line(line) => {
            let origin = instances.point(line.origin);
            let direction = instances.direction(line.direction);
            let vector = instances.add(format!("VECTOR('',#{direction},1.)"));
            instances.add(format!("LINE('',#{origin},#{vector})"))
        }
        Curve::Circle(circle) => self::circle(instances, circle),
        Curve::Conic(conic) => conic::written(instances, conic),
        Curve::Meet(_) => meet::written(instances, curve, from, to),
    }
}

fn circle(instances: &mut Instances, circle: &Circle) -> usize {
    let placement = instances.placement(circle.center, circle.u.cross(circle.v), circle.u);
    let radius = instances.length(circle.radius);
    instances.add(format!("CIRCLE('',#{placement},{radius})"))
}

fn placed(instances: &mut Instances, origin: DVec3, x: DVec3, y: DVec3) -> usize {
    instances.placement(origin, x.cross(y).normalize(), x)
}
