//! How a cone meets another surface (#536). A surface of its axis — a plane
//! square to it, a cylinder or a cone about it — is decided as two lines of
//! the meridian half-plane (`revolved.rs`), a plane holding its axis as the
//! two rulings it cuts, and any other plane as the conic it cuts or the
//! rulings through the apex (#552, `sliced.rs`). A cylinder, an elliptic
//! wall or a cone off its axis meets it along the curve read along the
//! rulings of one cone (#577, `meet/ruled.rs`), unless that reading fails,
//! which declines a boolean unless the two faces stand clear of each other.

use glam::DVec3;

use super::revolved::{Meridian, in_the_box, met};
use super::{Relation, parallel, sorted, square};
use crate::brep::curve::{Line, Wall};
use crate::brep::meet::{Configuration, Meeting, steady};
use crate::brep::scale::Scale;
use crate::brep::surface::{Cone, Plane, Surface};

/// A cone and a wall off its axis, met along the curve read along the
/// rulings (`meet/ruled.rs`); unsupported where a ruling does not meet the
/// other in a true quadratic — a wall's axis along a ruling of the cone, at
/// the cone's own slope to its axis, where the curve runs off to infinity —
/// within a hair of coaxial, where the
/// curve is all but a circle of each and the boolean reads the two faces as
/// too near to part (seed 5361127255), and where an apex stands on the other
/// wall.
fn ruled(cone: &Cone, wall: &Wall, scale: Scale) -> Relation {
    let hair = Scale::of(scale.reach() * Scale::HAIR);
    if !steady(cone, wall)
        || coaxial(cone, wall.origin(), wall.axis(), hair)
        || on_the_apex(cone, wall, scale)
    {
        return Relation::Unsupported;
    }
    let meeting = Meeting::of(&Wall::Cone(*cone), wall, scale);
    if meeting.configuration == Configuration::Apart {
        Relation::Apart
    } else {
        Relation::Meet(meeting)
    }
}

/// Whether the apex of either cone of the pair stands on the other wall,
/// where every ruling of that cone meets it at once and the curve has a
/// point no ruling crosses alone.
fn on_the_apex(cone: &Cone, wall: &Wall, scale: Scale) -> bool {
    let eps = scale.eps();
    wall.surface().distance(cone.apex()).abs() <= eps
        || matches!(wall, Wall::Cone(other) if cone.distance(other.apex()).abs() <= eps)
}

pub(super) fn relation(cone: &Cone, other: &Surface, scale: Scale) -> Relation {
    match other {
        Surface::Plane(plane) => with_plane(cone, plane, scale),
        Surface::Sphere(_) | Surface::Torus(_) => {
            super::round::relation(&Surface::Cone(*cone), other, scale)
        }
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
                ruled(first, &Wall::Cone(*second), scale)
            }
        }
        Surface::Cylinder(_) | Surface::Elliptic(_) => {
            Wall::of(other).map_or(Relation::Unsupported, |wall| ruled(cone, &wall, scale))
        }
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
/// cylinder of the axis reads it with; holding it, its two rulings; at a
/// slant, what `sliced.rs` decides.
fn with_plane(cone: &Cone, plane: &Plane, scale: Scale) -> Relation {
    if parallel(plane.normal, cone.axis, scale) {
        let height = (plane.offset() - plane.normal.dot(cone.origin)) / plane.normal.dot(cone.axis);
        return met(cone, Meridian::Level(height), scale);
    }
    if square(plane.normal, cone.axis, scale) && plane.distance(cone.origin).abs() <= scale.eps() {
        return rulings(cone, plane, scale);
    }
    super::sliced::plane_and_cone(cone, plane, scale)
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
