//! What a box dragged across the drawing takes hold of.

use glam::DVec2;

use crate::annotation::AnnotationMetrics;
use crate::arcing::bounds_of;
use crate::picking::Selection;
use crate::sketch::{Element, Sketch};

impl Sketch {
    /// Everything a box dragged from `from` to `to` takes hold of.
    ///
    /// Whole elements only: a trait counts when both its ends are in the box.
    /// Half a trait cannot be deleted, so letting the box claim it would say
    /// something the drawing cannot do.
    ///
    /// A circle, an arc and an ellipse are read off the curve rather than off the points
    /// they stand on: a centre is not part of what is drawn, and a curve can
    /// swing out of the box between two ends that are both inside it.
    pub fn inside_band(
        &self,
        from: DVec2,
        to: DVec2,
        metrics: AnnotationMetrics,
    ) -> Vec<Selection> {
        let (low, high) = (from.min(to), from.max(to));
        let inside = |point: DVec2| point.cmpge(low).all() && point.cmple(high).all();

        let mut caught = Vec::new();
        for (id, point) in self.live_points() {
            if inside(point) && !self.is_origin(id) {
                caught.push(Selection::Element(Element::Point(id)));
            }
        }
        for (id, segment) in self.live_segments() {
            if inside(self.point(segment.start)) && inside(self.point(segment.end)) {
                caught.push(Selection::Element(Element::Segment(id)));
            }
        }
        for (id, circle) in self.live_circles() {
            let center = self.point(circle.center);
            let reach = DVec2::splat(circle.radius);
            if inside(center - reach) && inside(center + reach) {
                caught.push(Selection::Element(Element::Circle(id)));
            }
        }
        for (id, _) in self.live_arcs() {
            let (lowest, highest) = bounds_of(self.arc_draft(id));
            if inside(lowest) && inside(highest) {
                caught.push(Selection::Element(Element::Arc(id)));
            }
        }
        for (id, _) in self.live_ellipses() {
            let (lowest, highest) = self.ellipse_draft(id).bounds();
            if inside(lowest) && inside(highest) {
                caught.push(Selection::Element(Element::Ellipse(id)));
            }
        }
        for (target, at) in self.anchors(metrics) {
            if inside(at) {
                caught.push(Selection::Dimension(target));
            }
        }
        caught
    }
}

#[cfg(test)]
mod tests;
