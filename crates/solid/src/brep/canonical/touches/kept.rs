//! What a move leaves where it was: whether two planes still cross along
//! the line they crossed along, and how far the place of a touch moves.

use crate::brep::relation::{Relation, relation};
use crate::brep::scale::Scale;
use crate::brep::surface::Surface;

/// Whether two planes crossing along a line, the first put for `before`,
/// cross along one standing further than the tolerance from it.
pub(super) fn crossed_elsewhere(
    before: &Surface,
    after: &Surface,
    other: &Surface,
    scale: Scale,
) -> bool {
    match (
        relation(before, other, scale),
        relation(after, other, scale),
    ) {
        (Relation::Line(was), Relation::Line(is)) => {
            let from = is.origin - was.origin;
            (from - was.direction * was.direction.dot(from)).length() > scale.eps()
        }
        _ => false,
    }
}

/// How far the place two surfaces touch at moves when the first is put
/// for `before`: the line they touch along, or the points two
/// perpendicular cylinders touch at.
pub(super) fn moved_along(before: &Surface, after: &Surface, other: &Surface, scale: Scale) -> f64 {
    let places = |surface: &Surface| match relation(surface, other, scale) {
        Relation::Tangent(line) => vec![line.origin],
        Relation::Parallel(rulings) => rulings
            .iter()
            .filter(|ruled| ruled.touching)
            .map(|ruled| ruled.line.origin)
            .collect(),
        found => found.points(),
    };
    let (from, to) = (places(before), places(after));
    if from.len() != to.len() {
        return f64::INFINITY;
    }
    from.iter()
        .zip(&to)
        .map(|(one, other)| one.distance(*other))
        .fold(0.0, f64::max)
}
