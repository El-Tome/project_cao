//! What a point laid down by a tool lands on, and what that lands it in.
//!
//! The drawing decides what runs through a place; what this adds is the
//! reading a click asks for — a point already there is joined rather than
//! landed on, and a place where three curves meet holds a point no better
//! than two of them do.

use cao_part::history::PointRef;
use cao_sketch::{PointId, Sketch, Support};
use glam::DVec2;

use crate::screens::SketchContext;

/// How many things one point is held on at most. Two is a crossing, which is
/// already a place a point cannot move away from; a third would take nothing
/// more from it and leave one more mark to read.
const AT_A_CROSSING: usize = 2;

/// What a point laid at this place is held by.
pub(crate) fn landed_on(sketch: &Sketch, place: DVec2) -> Vec<Support> {
    let mut on = sketch.supports_at(place);
    on.truncate(AT_A_CROSSING);
    on
}

/// What a point the drawing already has is held by, once dropped there.
fn dropped_on(sketch: &Sketch, point: PointId, place: DVec2) -> Vec<Support> {
    let mut on = sketch.supports_for(point, place);
    on.truncate(AT_A_CROSSING);
    on
}

/// What a dragged point is held by, dropped at `landing`: what ran through
/// the place before the drag and still does in the drawing it `settled` — the
/// one shown and the one the replay rebuilds, so every hold laid is met
/// there. A trait the settling moved away is not held on, nor one the point
/// was carried onto by its own rule, a trait's middle or a tangency's contact.
/// Only a point nothing held `before`: one sliding along its own trait would
/// otherwise catch on the first crossing it went over.
pub(crate) fn held_at_drop(
    before: &Sketch,
    settled: &Sketch,
    point: PointId,
    landing: DVec2,
) -> Vec<Support> {
    if !before.holds_on(point).is_empty() {
        return Vec::new();
    }
    let was_there = dropped_on(before, point, landing);
    dropped_on(settled, point, landing)
        .into_iter()
        .filter(|support| was_there.contains(support))
        .collect()
}

/// A point already there, or a new one — held on whatever it lands on.
pub(crate) fn point_ref_at(
    context: &SketchContext<'_>,
    index: usize,
    position: DVec2,
    snap: f64,
) -> PointRef {
    let sketch = &context.document.sketches()[index];
    match sketch.nearest_point(position, snap) {
        Some(point) => PointRef::Existing(point),
        None => born_at(sketch, position),
    }
}

/// The same, where the tool has already settled that this is a new point.
pub(crate) fn born_at(sketch: &Sketch, place: DVec2) -> PointRef {
    match landed_on(sketch, place) {
        on if on.is_empty() => PointRef::New(place),
        on => PointRef::Held { at: place, on },
    }
}

#[cfg(test)]
mod tests;
