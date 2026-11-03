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
    let point = sketch
        .nearest_point(cursor, snap * 0.8)
        .filter(|point| !sketch.is_origin(*point) || rule == Rule::Coincident);
    // A cut's point no point stands on yet is something only a coincidence
    // can speak of: it hangs the other point there.
    if point.is_none()
        && rule == Rule::Coincident
        && let Some((cut, at)) = sketch.nearest_cut_point(cursor, snap * 0.8)
    {
        return Some(RulePick::CutPoint(cut, at));
    }
    // A fillet's curve is what a tangent is aimed at, before the side it
    // meets: nearer the side, the side is taken.
    if point.is_none()
        && rule == Rule::Tangent
        && let Some(cut) = sketch.cut_before_traits(cursor, snap)
    {
        return Some(RulePick::CutCurve(cut));
    }
    point
        .map(Element::Point)
        .or_else(|| sketch.nearest_segment(cursor, snap).map(Element::Segment))
        .or_else(|| sketch.nearest_circle(cursor, snap).map(Element::Circle))
        .or_else(|| sketch.nearest_arc(cursor, snap).map(Element::Arc))
        .or_else(|| sketch.nearest_ellipse(cursor, snap).map(Element::Ellipse))
        .map(RulePick::Element)
        .or_else(|| {
            matches!(rule, Rule::Collinear | Rule::Parallel | Rule::Perpendicular)
                .then(|| cao_sketch::axis_under(cursor, snap).map(RulePick::Axis))
                .flatten()
        })
}

#[cfg(test)]
mod tests;
