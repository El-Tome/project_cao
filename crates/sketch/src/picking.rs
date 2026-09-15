//! What a gesture takes hold of: what sits under the cursor, and which points
//! a held selection would carry. What a box catches is in [`crate::banding`].
//!
//! The tolerance arrives in world units — the front-end knows how many of them
//! a pixel is worth, and this does not.

use glam::DVec2;

use crate::annotation::AnnotationMetrics;
use crate::arc::ArcId;
use crate::constraints::{Constraint, DimensionTarget};
use crate::sketch::{Element, PointId, Sketch};
use crate::snap::onto_rim;

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
    /// A point before a trait before a curve before an annotation: the smaller
    /// the target, the harder it is to hit on purpose, so the smaller one wins.
    /// The origin is never picked — it is there to be measured from, not moved.
    pub fn pick(
        &self,
        cursor: DVec2,
        tolerance: f64,
        metrics: AnnotationMetrics,
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
        if let Some(arc) = self.nearest_arc(cursor, tolerance) {
            return Some(Selection::Element(Element::Arc(arc)));
        }
        let wide = tolerance * WIDE_TARGET_REACH;
        if let Some(target) = self.nearest_dimension(cursor, wide, metrics) {
            return Some(Selection::Dimension(target));
        }
        self.nearest_rule(cursor, wide).map(Selection::Rule)
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
            for point in self.points_it_leans_on(*element) {
                take(point);
            }
        }
        points
    }

    /// Where an arc takes a place near it.
    ///
    /// Beyond either end it is that end, not the far side of the circle the arc
    /// is a piece of: the rest of that circle is not drawn, and a click out
    /// there must find nothing.
    pub fn place_on_arc(&self, id: ArcId, position: DVec2) -> DVec2 {
        let arc = self.arc(id);
        let centre = self.point(arc.center);
        let (start, end) = (self.point(arc.start), self.point(arc.end));
        let Some(on_rim) = onto_rim(position, centre, self.arc_radius(id)) else {
            return start;
        };
        let along = ((position - centre).to_angle() - (start - centre).to_angle())
            .rem_euclid(std::f64::consts::TAU);
        if along <= self.arc_sweep(id) {
            return on_rim;
        }
        match position.distance(start) <= position.distance(end) {
            true => start,
            false => end,
        }
    }

    /// How far a place sits from an arc's curve itself.
    pub fn distance_to_arc(&self, id: ArcId, position: DVec2) -> f64 {
        self.place_on_arc(id, position).distance(position)
    }

    /// The arc whose curve passes closest to `position`.
    pub fn nearest_arc(&self, position: DVec2, tolerance: f64) -> Option<ArcId> {
        self.live_arcs()
            .map(|(id, _)| (id, self.distance_to_arc(id, position)))
            .filter(|(_, distance)| *distance <= tolerance)
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(id, _)| id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::kinds::{name_of, one_of_every_kind, somewhere_on};
    use crate::plane::WorkPlane;

    const TOLERANCE: f64 = 1e-9;

    fn quarter() -> (Sketch, ArcId) {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let centre = sketch.add_point(DVec2::ZERO);
        let east = sketch.add_point(DVec2::new(10.0, 0.0));
        let north = sketch.add_point(DVec2::new(0.0, 10.0));
        let arc = sketch.add_arc(centre, east, north);
        (sketch, arc)
    }

    #[test]
    fn a_click_on_the_part_of_the_circle_the_arc_does_not_draw_finds_nothing() {
        let (sketch, arc) = quarter();
        let corner = DVec2::splat(10.0 / 2.0_f64.sqrt());

        assert_eq!(sketch.nearest_arc(corner, 0.5), Some(arc));
        assert_eq!(sketch.nearest_arc(DVec2::new(-10.0, 0.0), 0.5), None);
        assert_eq!(sketch.nearest_arc(DVec2::new(0.0, -10.0), 0.5), None);
    }

    #[test]
    fn past_its_end_an_arc_is_as_far_away_as_that_end_is() {
        let (sketch, arc) = quarter();
        let beyond = DVec2::new(13.0, -4.0);
        let distance = sketch.distance_to_arc(arc, beyond);

        let to_the_end = beyond.distance(DVec2::new(10.0, 0.0));
        assert!(
            (distance - to_the_end).abs() < TOLERANCE,
            "{distance} away, where its end is {to_the_end} away",
        );
    }

    #[test]
    fn an_erased_arc_is_not_under_the_cursor_any_more() {
        let (mut sketch, arc) = quarter();
        sketch.erase(Element::Arc(arc));

        assert_eq!(sketch.nearest_arc(DVec2::new(10.0, 0.0), 0.5), None);
    }

    const METRICS: AnnotationMetrics = AnnotationMetrics {
        offset_pixels: 22.0,
        arrow_pixels: 8.0,
        arc_pixels: 34.0,
        pixel: 1.0,
        nudge: DVec2::ZERO,
    };

    #[test]
    fn a_click_on_the_curve_of_an_arc_takes_hold_of_the_arc() {
        let (sketch, arc) = quarter();
        let on_the_curve = DVec2::splat(10.0 / 2.0_f64.sqrt());

        assert_eq!(
            sketch.pick(on_the_curve, 1.0, METRICS),
            Some(Selection::Element(Element::Arc(arc))),
        );
    }

    #[test]
    fn a_point_wins_over_the_trait_it_sits_on() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let start = sketch.add_point(DVec2::new(10.0, 0.0));
        let end = sketch.add_point(DVec2::new(50.0, 0.0));
        sketch.add_segment(start, end);

        let picked = sketch.pick(DVec2::new(10.2, 0.0), 1.0, METRICS);

        assert_eq!(
            picked,
            Some(Selection::Element(Element::Point(start))),
            "the cursor is within reach of both, and the point is the harder aim"
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
        sketch.set_dimension(measured, 40.0, false);
        let written_at = sketch.place(measured, METRICS).unwrap().text_at;

        let just_off = written_at + DVec2::new(0.0, 1.2);
        assert_eq!(
            sketch.pick(just_off, 1.0, METRICS),
            Some(Selection::Dimension(measured)),
            "an annotation is read rather than aimed at, so it answers past the reach"
        );
        assert_eq!(
            sketch.pick(DVec2::new(20.0, 1.2), 1.0, METRICS),
            None,
            "the geometry itself answers only within the reach"
        );
    }

    #[test]
    fn the_origin_is_never_picked() {
        let sketch = Sketch::new(WorkPlane::XY);

        assert_eq!(
            sketch.pick(DVec2::new(0.1, 0.0), 1.0, METRICS),
            None,
            "the origin is there to be measured from, not taken hold of"
        );
    }

    #[test]
    fn a_click_on_the_drawing_finds_one_of_every_kind() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let drawn = one_of_every_kind(&mut sketch);

        for element in drawn {
            let at = somewhere_on(&sketch, element);
            let found = sketch.pick(at, 1.0, METRICS);

            assert_eq!(
                found,
                Some(Selection::Element(element)),
                "a click on a {} found {found:?}",
                name_of(&element),
            );
        }
    }
}
