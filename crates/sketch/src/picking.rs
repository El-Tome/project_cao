//! What a gesture takes hold of: what sits under the cursor, what a box
//! catches, and which points a held selection would carry.
//!
//! The tolerance arrives in world units — the front-end knows how many of them
//! a pixel is worth, and this does not.

use glam::DVec2;

use crate::constraints::{Constraint, DimensionTarget};
use crate::sketch::{Element, PointId, Sketch};

/// What the selection tool is holding, and what pressing the delete key would
/// take away.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Selection {
    Element(Element),
    Dimension(DimensionTarget),
    Rule(Constraint),
}

/// How much easier a wide target is to hit than the trait it is drawn over. An
/// annotation and a rule mark are read rather than aimed at, so they answer to
/// a looser reach than the geometry does.
const WIDE_TARGET_REACH: f64 = 1.5;

impl Sketch {
    /// What the cursor is over, in the order a click should take it.
    ///
    /// A point before a trait before a circle before an annotation: the smaller
    /// the target, the harder it is to hit on purpose, so the smaller one wins.
    /// The origin is never picked — it is there to be measured from, not moved.
    ///
    /// `anchors` says where each annotation is drawn, which only the front-end
    /// knows.
    pub fn pick(
        &self,
        cursor: DVec2,
        tolerance: f64,
        anchors: &[(DimensionTarget, DVec2)],
    ) -> Option<Selection> {
        if let Some(point) = self
            .nearest_point(cursor, tolerance)
            .filter(|point| !self.is_origin(*point))
        {
            return Some(Selection::Element(Element::Point(point)));
        }
        if let Some(segment) = self.nearest_segment(cursor, tolerance) {
            return Some(Selection::Element(Element::Segment(segment)));
        }
        if let Some(circle) = self.nearest_circle(cursor, tolerance) {
            return Some(Selection::Element(Element::Circle(circle)));
        }
        let wide = tolerance * WIDE_TARGET_REACH;
        if let Some(target) = self.nearest_dimension(anchors, cursor, wide) {
            return Some(Selection::Dimension(target));
        }
        self.nearest_rule(cursor, wide).map(Selection::Rule)
    }

    /// Everything a box dragged from `from` to `to` takes hold of.
    ///
    /// Whole elements only: a trait counts when both its ends are in the box.
    /// Half a trait cannot be deleted, so letting the box claim it would say
    /// something the drawing cannot do.
    pub fn inside_band(
        &self,
        from: DVec2,
        to: DVec2,
        anchors: &[(DimensionTarget, DVec2)],
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
        for (target, at) in anchors {
            if inside(*at) {
                caught.push(Selection::Dimension(*target));
            }
        }
        caught
    }

    /// The points a held selection would carry, were it dragged.
    ///
    /// A trait gives both its ends, a circle its centre, and an annotation or a
    /// rule nothing: they are drawn from the geometry rather than placed. The
    /// origin never moves, so it is never carried.
    pub fn points_of(&self, selection: &[Selection]) -> Vec<PointId> {
        let mut points: Vec<PointId> = Vec::new();
        let mut take = |point: PointId| {
            if !self.is_origin(point) && !points.contains(&point) {
                points.push(point);
            }
        };
        for held in selection {
            let Selection::Element(element) = held else {
                continue;
            };
            match element {
                Element::Point(id) => take(*id),
                Element::Segment(id) => {
                    if let Some(segment) = self.segments().get(id.0) {
                        take(segment.start);
                        take(segment.end);
                    }
                }
                Element::Circle(id) => {
                    if let Some(circle) = self.circles().get(id.0) {
                        take(circle.center);
                    }
                }
            }
        }
        points
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plane::WorkPlane;

    #[test]
    fn a_point_wins_over_the_trait_it_sits_on() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let start = sketch.add_point(DVec2::new(10.0, 0.0));
        let end = sketch.add_point(DVec2::new(50.0, 0.0));
        sketch.add_segment(start, end);

        let picked = sketch.pick(DVec2::new(10.2, 0.0), 1.0, &[]);

        assert_eq!(
            picked,
            Some(Selection::Element(Element::Point(start))),
            "the cursor is within reach of both, and the point is the harder aim"
        );
    }
    #[test]
    fn a_trait_with_one_end_outside_the_band_is_not_taken() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let inside = sketch.add_point(DVec2::new(10.0, 10.0));
        let outside = sketch.add_point(DVec2::new(90.0, 10.0));
        let held = sketch.add_segment(inside, outside);

        let caught = sketch.inside_band(DVec2::new(0.0, 0.0), DVec2::new(50.0, 50.0), &[]);

        assert!(
            caught.contains(&Selection::Element(Element::Point(inside))),
            "the end that is in the box is taken on its own account: {caught:?}"
        );
        assert!(
            !caught.contains(&Selection::Element(Element::Segment(held))),
            "half a trait cannot be deleted, so the box may not claim it: {caught:?}"
        );
    }

    #[test]
    fn a_circle_is_taken_only_when_its_whole_rim_is_inside() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let center = sketch.add_point(DVec2::new(25.0, 25.0));
        let held = sketch.add_circle(center, 5.0);
        let spilling = sketch.add_point(DVec2::new(25.0, 45.0));
        let over_the_edge = sketch.add_circle(spilling, 20.0);

        let caught = sketch.inside_band(DVec2::new(0.0, 0.0), DVec2::new(50.0, 50.0), &[]);

        assert!(
            caught.contains(&Selection::Element(Element::Circle(held))),
            "a circle whose rim is entirely in the box is taken: {caught:?}"
        );
        assert!(
            !caught.contains(&Selection::Element(Element::Circle(over_the_edge))),
            "a circle whose centre is in the box but whose rim is not stays out: {caught:?}"
        );
    }

    #[test]
    fn the_origin_is_never_taken_by_a_band() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let drawn = sketch.add_point(DVec2::new(10.0, 10.0));

        let caught = sketch.inside_band(DVec2::new(-10.0, -10.0), DVec2::new(50.0, 50.0), &[]);

        assert!(
            caught.contains(&Selection::Element(Element::Point(drawn))),
            "a point the user drew is taken: {caught:?}"
        );
        assert!(
            !caught.contains(&Selection::Element(Element::Point(Sketch::ORIGIN))),
            "the origin is measured from, never moved or deleted: {caught:?}"
        );
    }

    #[test]
    fn an_annotation_is_taken_by_the_box_that_reaches_where_it_is_drawn() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let start = sketch.add_point(DVec2::new(10.0, 10.0));
        let end = sketch.add_point(DVec2::new(40.0, 10.0));
        let measured = DimensionTarget::Length(sketch.add_segment(start, end));
        let elsewhere = DimensionTarget::Distance {
            from: start,
            to: end,
        };
        let anchors = [
            (measured, DVec2::new(25.0, 20.0)),
            (elsewhere, DVec2::new(25.0, 90.0)),
        ];

        let caught = sketch.inside_band(DVec2::new(0.0, 0.0), DVec2::new(50.0, 50.0), &anchors);

        assert!(
            caught.contains(&Selection::Dimension(measured)),
            "the box reaches where that annotation is written: {caught:?}"
        );
        assert!(
            !caught.contains(&Selection::Dimension(elsewhere)),
            "an annotation drawn outside the box stays out of it: {caught:?}"
        );
    }

    #[test]
    fn a_selected_circle_carries_its_centre() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let center = sketch.add_point(DVec2::new(25.0, 25.0));
        let drawn = sketch.add_circle(center, 5.0);

        let carried = sketch.points_of(&[Selection::Element(Element::Circle(drawn))]);

        assert_eq!(
            carried,
            vec![center],
            "dragging a circle moves it whole, and its centre is what it is placed by"
        );
    }

    #[test]
    fn a_trait_carries_both_its_ends_and_never_the_origin() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let far = sketch.add_point(DVec2::new(40.0, 0.0));
        let held = sketch.add_segment(Sketch::ORIGIN, far);

        let carried = sketch.points_of(&[
            Selection::Element(Element::Segment(held)),
            Selection::Element(Element::Point(far)),
        ]);

        assert_eq!(
            carried,
            vec![far],
            "the end anchored on the origin stays put, and the other is carried once"
        );
    }

    #[test]
    fn an_annotation_is_easier_to_hit_than_the_trait_it_measures() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let start = sketch.add_point(DVec2::new(0.0, 0.0));
        let end = sketch.add_point(DVec2::new(40.0, 0.0));
        let measured = DimensionTarget::Length(sketch.add_segment(start, end));
        let written_at = DVec2::new(20.0, 20.0);
        let anchors = [(measured, written_at)];

        let just_off = DVec2::new(20.0, 21.2);
        assert_eq!(
            sketch.pick(just_off, 1.0, &anchors),
            Some(Selection::Dimension(measured)),
            "an annotation is read rather than aimed at, so it answers past the reach"
        );
        assert_eq!(
            sketch.pick(DVec2::new(20.0, 1.2), 1.0, &anchors),
            None,
            "the geometry itself answers only within the reach"
        );
    }

    #[test]
    fn the_origin_is_never_picked() {
        let sketch = Sketch::new(WorkPlane::XY);

        assert_eq!(
            sketch.pick(DVec2::new(0.1, 0.0), 1.0, &[]),
            None,
            "the origin is there to be measured from, not taken hold of"
        );
    }
}
