//! How a cone meets another surface (#536). A surface of its axis — a plane
//! square to it, a cylinder or a cone about it — is decided as two lines of
//! the meridian half-plane (`revolved.rs`). Anything else is unsupported,
//! which declines a boolean unless the two faces stand clear of each other.

use glam::DVec3;

use super::revolved::{Meridian, in_the_box, met};
use super::{Relation, parallel, sorted, square};
use crate::brep::curve::Line;
use crate::brep::scale::Scale;
use crate::brep::surface::{Cone, Plane, Surface};

pub(super) fn relation(cone: &Cone, other: &Surface, scale: Scale) -> Relation {
    match other {
        Surface::Plane(plane) => with_plane(cone, plane, scale),
        Surface::Cylinder(cylinder) if coaxial(cone, cylinder.origin, cylinder.axis, scale) => {
            met(cone, Meridian::Wall(cylinder.radius), scale)
        }
        Surface::Cone(other) => {
            let (first, second) = if goes_first(cone, other) {
                (cone, other)
            } else {
                (other, cone)
            };
            if coaxial(first, second.origin, second.axis, scale) {
                met(first, Meridian::of(first, second), scale)
            } else {
                Relation::Unsupported
            }
        }
        Surface::Cylinder(_) => Relation::Unsupported,
    }
}

/// Of two cones, the one whose frame the pair is read in: an order read off
/// the cones alone, so that the answer is the same whichever came first.
fn goes_first(one: &Cone, other: &Cone) -> bool {
    let bits = |cone: &Cone| {
        [cone.origin, cone.axis]
            .into_iter()
            .flat_map(|vector| vector.to_array())
            .chain(cone.foot.to_array())
            .chain(cone.ruling.to_array())
    };
    bits(one)
        .zip(bits(other))
        .map(|(mine, theirs)| mine.total_cmp(&theirs))
        .find(|order| order.is_ne())
        .is_none_or(|order| order.is_lt())
}

/// Whether the line through `origin` along `axis` is the cone's axis within
/// the tolerance: parallel across the box, and the two a hair apart at most.
/// A surface further off is not decided against a cone; decision 8 is not
/// extended to it.
fn coaxial(cone: &Cone, origin: DVec3, axis: DVec3, scale: Scale) -> bool {
    let between = origin - cone.origin;
    parallel(axis, cone.axis, scale)
        && (between - cone.axis * cone.axis.dot(between)).length() <= scale.eps()
}

/// Square to the axis, the circle at the plane's height, from the formula a
/// cylinder of the axis reads it with.
fn with_plane(cone: &Cone, plane: &Plane, scale: Scale) -> Relation {
    if parallel(plane.normal, cone.axis, scale) {
        let height = (plane.offset() - plane.normal.dot(cone.origin)) / plane.normal.dot(cone.axis);
        return met(cone, Meridian::Level(height), scale);
    }
    if square(plane.normal, cone.axis, scale) && plane.distance(cone.origin).abs() <= scale.eps() {
        return rulings(cone, plane, scale);
    }
    Relation::Unsupported
}

/// Holding the axis, the two rulings of the cone in the plane, each built
/// through its meridian line's foot rather than through the apex, which may
/// stand a hundred million reaches away, and laid on the plane as a
/// cylinder's lines are; the apex where it stands within the box.
fn rulings(cone: &Cone, plane: &Plane, scale: Scale) -> Relation {
    let origin = cone.origin - plane.normal * plane.distance(cone.origin);
    let across = plane.normal.cross(cone.axis).normalize();
    let [h, rho] = cone.foot.to_array();
    let [w_h, w_rho] = cone.ruling.to_array();
    let ruling = |side: f64| {
        Line::through(
            origin + cone.axis * h + across * (side * rho),
            cone.axis * w_h + across * (side * w_rho),
        )
    };
    let apex = origin + cone.axis * cone.height_at(cone.apex_at());
    Relation::Rulings {
        lines: sorted([ruling(-1.0), ruling(1.0)]),
        apex: in_the_box(apex, scale).then_some(apex),
    }
}
