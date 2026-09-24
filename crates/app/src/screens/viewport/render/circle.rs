//! What the circle tool shows before its shape is drawn: the diameter typed
//! into it.

use glam::DVec2;

use crate::screens::SketchContext;
use crate::screens::viewport::input::circle_from;

pub(crate) fn live_fields(
    context: &SketchContext<'_>,
    index: usize,
    cursor: DVec2,
    scale: f64,
) -> Option<([&'static str; 2], [f64; 2])> {
    // Shown whether or not the picks make a circle just now: a field that
    // disappears while being typed into cannot be typed into.
    let across = circle_from(context, index, cursor, f64::MAX)
        .map(|found| found.radius * 2.0 * scale)
        .unwrap_or_default();
    Some((["mm", ""], [across, 0.0]))
}
