//! What two sides clicked name, and why a chamfer refuses a corner: the
//! drawing says which sides meet where, and which side refuses a chamfer; the
//! tool only says it.

use cao_sketch::{Corner, CutRefused, CutSide, PointId, Sketch};
use glam::DVec2;

/// Why a chamfer cannot take a corner, as the key of the sentence that says
/// so — nothing when it can. The drawing says which side refuses it.
pub(super) fn chamfer_refused(sketch: &Sketch, corner: Corner) -> Option<&'static str> {
    let (_, first, second) = sketch.sides_of(corner)?;
    let refused = first.refuses_a_chamfer().or(second.refuses_a_chamfer())?;
    Some(said_of(refused))
}

/// The corner two sides clicked name: where they meet — or, two sides meeting
/// at both their ends, the end nearest the click, named by its point and the
/// two sides, whatever else ends there. What they name nothing for, as the
/// key of the sentence that says why.
pub(super) fn named_by(
    sketch: &Sketch,
    first: CutSide,
    second: CutSide,
    cursor: DVec2,
) -> Result<Corner, &'static str> {
    let ends = sketch.corners_between(first, second);
    let away = |end: &PointId| sketch.point(*end).distance(cursor);
    match ends[..] {
        [] => Err("sketch.chamfer_needs_a_corner"),
        [_] => Ok(Corner::Between(first, second)),
        _ => ends
            .iter()
            .min_by(|one, other| away(one).total_cmp(&away(other)))
            .map(|end| Corner::BetweenAt(*end, first, second))
            .ok_or("sketch.chamfer_needs_a_corner"),
    }
}

/// The key of the sentence that says why a corner was refused a cut.
pub(super) fn said_of(refused: CutRefused) -> &'static str {
    match refused {
        CutRefused::ChamferOnACurve => "sketch.chamfer_needs_two_traits",
        CutRefused::NoCorner => "sketch.chamfer_needs_a_corner",
    }
}
