//! Decision 2's moves onto a touch for two parallel walls, an elliptic one
//! among them (#571), and how far such a touch stands from exact: read at
//! the feet of the normal the two walls share at each touch. A wall
//! touching another at both ends of one of its half-axes — an ellipse
//! inside a circle as long as its major half-axis, a circle inside an
//! ellipse as wide as its minor one — is no translation from touching at
//! both: moved onto one, it stands twice as far from the other. Grown or
//! shrunk across the two, it touches both exactly.

use glam::DVec3;

use crate::brep::relation::feet_of_touches;
use crate::brep::scale::Scale;
use crate::brep::surface::{Cylinder, EllipticCylinder, Surface};

/// Under this share of the tolerance, a touch a move leaves is as near
/// exact as it was.
const ROUNDING: f64 = 1e-3;

/// `moving`, a wall parallel to `fixed` decided to touch it along rulings,
/// moved onto its touches: moved square to the axes onto one of them, so
/// that the two share the foot of their common normal there, or grown
/// across two at the ends of one of its half-axes onto both. Of the moves
/// that leave every touch no further from exact, the one that leaves the
/// furthest nearest. None where no move does, or where the touches are
/// exact already.
pub(super) fn onto_a_touch(fixed: &Surface, moving: &Surface, scale: Scale) -> Option<Surface> {
    let feet = feet_of_touches(fixed, moving, scale);
    let room = ROUNDING * scale.eps();
    let off = |wall: &Surface, at: DVec3| wall.distance(at).abs();
    let worst = |wall: &Surface| {
        feet.iter()
            .map(|&[at, _]| off(wall, at))
            .fold(0.0, f64::max)
    };
    feet.iter()
        .filter_map(|&[on_fixed, on_moving]| translated(moving, on_fixed - on_moving))
        .chain(grown(moving, &feet, scale))
        .filter(|moved| moved != moving)
        .filter(|moved| {
            feet.iter()
                .all(|&[at, _]| off(moved, at) <= off(moving, at) + room)
        })
        .min_by(|one, other| worst(one).total_cmp(&worst(other)))
}

/// `moving` touching at the two `feet` given, at the ends of a diameter of
/// a round wall or of a half-axis of an elliptic one: its centre moved
/// midway between the feet on the fixed wall, and that radius or half-axis
/// made half their distance, which lays both feet on it at once — where
/// neither moves the wall further than decision 8's hair. None where the
/// feet are not two, or stand at the ends of no diameter or half-axis.
fn grown(moving: &Surface, feet: &[[DVec3; 2]], scale: Scale) -> Option<Surface> {
    let [[fixed_one, _], [fixed_other, _]] = feet[..] else {
        return None;
    };
    let hair = Scale::HAIR * scale.eps();
    let center = (fixed_one + fixed_other) / 2.0;
    match moving {
        Surface::Cylinder(round) => {
            let across = fixed_other - fixed_one;
            let across = across - round.axis * round.axis.dot(across);
            let radius = across.length() / 2.0;
            let shift = center - round.origin;
            let shift = shift - round.axis * round.axis.dot(shift);
            (shift.length() <= hair && (radius - round.radius).abs() <= hair)
                .then(|| Surface::Cylinder(Cylinder::about(center, round.axis, radius)))
        }
        Surface::Elliptic(oval) => {
            let across = fixed_other - fixed_one;
            let across = across - oval.axis * oval.axis.dot(across);
            let half = across.length() / 2.0;
            let way = across / (2.0 * half);
            let shift = center - oval.origin;
            let shift = shift - oval.axis * oval.axis.dot(shift);
            let (along, length, other) = if way.dot(oval.u).abs() >= way.dot(oval.v).abs() {
                (oval.u, oval.major, oval.minor)
            } else {
                (oval.v, oval.minor, oval.major)
            };
            let turned = way.cross(along).length() * (oval.major - oval.minor);
            (shift.length() <= hair && (half - length).abs() <= hair && turned <= hair).then(|| {
                Surface::Elliptic(EllipticCylinder::about(
                    center,
                    oval.axis,
                    along * half,
                    other,
                ))
            })
        }
        Surface::Plane(_) | Surface::Cone(_) | Surface::Sphere(_) | Surface::Torus(_) => None,
    }
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
        Surface::Plane(_) | Surface::Cone(_) | Surface::Sphere(_) | Surface::Torus(_) => None,
    }
}
