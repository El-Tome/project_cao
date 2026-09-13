//! What a click of the constraint tool has been pointed at.

use cao_sketch::{Element, Rule, RulePick, Sketch};
use glam::DVec2;

/// Smallest target first, as everywhere else: a point is harder to hit on
/// purpose than the trait it sits on. The axes of the sketch come last, being
/// the widest thing on screen.
pub(super) fn nearest_rule_pick(
    sketch: &Sketch,
    cursor: DVec2,
    snap: f64,
    rule: Rule,
) -> Option<RulePick> {
    sketch
        .nearest_point(cursor, snap * 0.8)
        .filter(|point| !sketch.is_origin(*point) || rule == Rule::Coincident)
        .map(Element::Point)
        .or_else(|| sketch.nearest_segment(cursor, snap).map(Element::Segment))
        .or_else(|| sketch.nearest_circle(cursor, snap).map(Element::Circle))
        .or_else(|| sketch.nearest_arc(cursor, snap).map(Element::Arc))
        .map(RulePick::Element)
        .or_else(|| {
            (rule == Rule::Collinear)
                .then(|| cao_sketch::axis_under(cursor, snap).map(RulePick::Axis))
                .flatten()
        })
}
