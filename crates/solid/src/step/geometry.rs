//! The kernel's surfaces and curves as the entities of ISO 10303-42, each
//! parameterised the way the kernel's own is, so an edge keeps its sense.

use super::instances::Instances;
use crate::brep::{Curve, Surface};

pub fn surface(instances: &mut Instances, surface: &Surface) -> usize {
    match surface {
        Surface::Plane(plane) => {
            let placement = instances.placement(plane.origin, plane.normal, plane.u);
            instances.add(format!("PLANE('',#{placement})"))
        }
        other => todo!("{other:?} is not written to STEP yet"),
    }
}

pub fn curve(instances: &mut Instances, curve: &Curve) -> usize {
    match curve {
        Curve::Line(line) => {
            let origin = instances.point(line.origin);
            let direction = instances.direction(line.direction);
            let vector = instances.add(format!("VECTOR('',#{direction},1.)"));
            instances.add(format!("LINE('',#{origin},#{vector})"))
        }
        other => todo!("{other:?} is not written to STEP yet"),
    }
}
