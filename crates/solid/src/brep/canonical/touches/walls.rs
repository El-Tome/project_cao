//! Decision 2's moves onto a touch for two parallel walls, an elliptic one
//! among them (#571), and how far such a touch stands from exact: read at
//! the feet of the normal the two walls share at each touch.

use glam::DVec3;

use crate::brep::relation::feet_of_touches;
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, Surface};

/// Under this share of the tolerance, a touch a move leaves is as near
/// exact as it was.
const ROUNDING: f64 = 1e-3;

/// `moving`, a wall parallel to `fixed` decided to touch it along rulings,
/// moved onto a touch, square to the axes, so that the two share the foot of
/// their common normal there: onto the first touch whose move leaves every
/// other no further from exact. None where no move does, or where the
/// touch is exact already.
pub(super) fn onto_a_touch(fixed: &Surface, moving: &Surface, scale: Scale) -> Option<Surface> {
    let feet = feet_of_touches(fixed, moving, scale);
    let room = ROUNDING * scale.eps();
    feet.iter().find_map(|&[on_fixed, on_moving]| {
        let moved = translated(moving, on_fixed - on_moving)?;
        feet.iter()
            .all(|&[at, _]| moved.distance(at).abs() <= moving.distance(at).abs() + room)
            .then_some(moved)
            .filter(|moved| moved != moving)
    })
}

/// How far two parallel walls decided to touch along rulings stand from
/// touching exactly, at the touch that stands furthest; none where they
/// touch nowhere.
pub(super) fn off_the_touches(one: &Surface, other: &Surface, scale: Scale) -> Option<f64> {
    feet_of_touches(one, other, scale)
        .into_iter()
        .map(|[on_one, on_other]| on_one.distance(on_other))
        .reduce(f64::max)
}

/// A wall moved by `by`, canonical; none for a surface that is no wall.
fn translated(wall: &Surface, by: DVec3) -> Option<Surface> {
    match wall {
        Surface::Cylinder(round) => Some(Surface::Cylinder(Cylinder::about(
            round.origin + by,
            round.axis,
            round.radius,
        ))),
        Surface::Elliptic(oval) => Some(Surface::Elliptic(oval.moved(by))),
        Surface::Plane(_) | Surface::Cone(_) => None,
    }
}
