//! Where a rule's marks are written: on each of the things it holds, so that
//! pointing at one of them says what it is caught up in.

use glam::DVec2;

use crate::constraints::Constraint;
use crate::sketch::{CircleId, Element, PointId, SegmentId, Sketch};

impl Sketch {
    /// Where a rule's marks are written: on each of the things it holds, so
    /// that pointing at one of them says what it is caught up in.
    ///
    /// A right angle is the exception: its mark belongs *in* the corner,
    /// which is the only place it reads as an angle rather than as a note
    /// about two traits.
    pub fn rule_marks(&self, constraint: Constraint) -> Vec<DVec2> {
        let middle = |segment: SegmentId| {
            (segment.0 < self.segments().len() && !self.is_erased_segment(segment)).then(|| {
                let (start, end) = self.endpoints(segment);
                (start + end) * 0.5
            })
        };
        let point = |id: PointId| (id.0 < self.points().len()).then(|| self.point(id));
        let circle = |id: CircleId| {
            (id.0 < self.circles().len()).then(|| {
                let round = self.circle(id);
                self.point(round.center) + DVec2::splat(round.radius * 0.7)
            })
        };
        let both = |first: SegmentId, second: SegmentId| {
            [middle(first), middle(second)]
                .into_iter()
                .flatten()
                .collect()
        };

        match constraint {
            Constraint::Perpendicular { first, second } => match self.corner_of(first, second) {
                Some(at) => vec![at],
                None => both(first, second),
            },
            Constraint::Parallel { first, second }
            | Constraint::Equal { first, second }
            | Constraint::Collinear { first, second } => both(first, second),
            Constraint::EqualRadius { first, second } => [circle(first), circle(second)]
                .into_iter()
                .flatten()
                .collect(),
            Constraint::AxisCollinear { segment, .. } => middle(segment).into_iter().collect(),
            Constraint::OnSegment { point: held, .. }
            | Constraint::Midpoint { point: held, .. } => point(held).into_iter().collect(),
            // Where the circle actually touches, not somewhere beside it: three
            // tangencies of one circle would otherwise all land on the same spot.
            Constraint::Tangent {
                circle: round,
                segment,
                at,
            } => at
                .and_then(point)
                .or_else(|| {
                    (round.0 < self.circles().len())
                        .then(|| self.foot_on_segment(self.circle(round).center, segment))
                        .flatten()
                })
                .into_iter()
                .collect(),
            Constraint::OnCircle { point: held, .. } => point(held).into_iter().collect(),
            Constraint::Fixed { element } => match element {
                Element::Point(held) => point(held).into_iter().collect(),
                Element::Segment(held) => middle(held).into_iter().collect(),
                Element::Circle(held) => circle(held).into_iter().collect(),
            },
        }
    }

    /// Just inside the corner two traits make, along the bisector.
    ///
    /// They have to actually meet: two traits held square without touching
    /// have no corner to write in, and the marks then go on the traits
    /// themselves.
    fn corner_of(&self, first: SegmentId, second: SegmentId) -> Option<DVec2> {
        let (pivot, a, b) = self.corner_points(first, second)?;
        let reach = (a.distance(pivot).min(b.distance(pivot))) * 0.25;
        let inward = ((a - pivot).normalize_or_zero() + (b - pivot).normalize_or_zero())
            .normalize_or(DVec2::X);
        Some(pivot + inward * reach)
    }

    /// The rule whose nearest mark sits under the cursor.
    pub fn nearest_rule(&self, cursor: DVec2, tolerance: f64) -> Option<Constraint> {
        self.constraints()
            .iter()
            .filter_map(|constraint| {
                let nearest = self
                    .rule_marks(*constraint)
                    .into_iter()
                    .map(|at| at.distance(cursor))
                    .min_by(f64::total_cmp)?;
                Some((*constraint, nearest))
            })
            .filter(|(_, distance)| *distance <= tolerance)
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(constraint, _)| constraint)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plane::WorkPlane;

    #[test]
    fn a_right_angle_writes_its_mark_in_the_corner() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let pivot = sketch.add_point(DVec2::new(0.0, 0.0));
        let a = sketch.add_point(DVec2::new(40.0, 0.0));
        let b = sketch.add_point(DVec2::new(0.0, 20.0));
        let first = sketch.add_segment(pivot, a);
        let second = sketch.add_segment(pivot, b);
        let constraint = Constraint::Perpendicular { first, second };
        sketch.add_constraint(constraint);

        let marks = sketch.rule_marks(constraint);
        assert_eq!(
            marks.len(),
            1,
            "a right angle writes one mark, in the corner"
        );
        let mark = marks[0];
        assert!(
            mark.x > 0.0 && mark.x < 20.0 && mark.y > 0.0 && mark.y < 20.0,
            "the mark should sit inside the corner, not on the traits: {mark}"
        );
    }

    #[test]
    fn two_traits_held_square_without_touching_write_on_themselves() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let a = sketch.add_point(DVec2::new(0.0, 0.0));
        let b = sketch.add_point(DVec2::new(40.0, 0.0));
        let c = sketch.add_point(DVec2::new(0.0, 30.0));
        let d = sketch.add_point(DVec2::new(0.0, 70.0));
        let first = sketch.add_segment(a, b);
        let second = sketch.add_segment(c, d);
        let constraint = Constraint::Perpendicular { first, second };
        sketch.add_constraint(constraint);

        let marks = sketch.rule_marks(constraint);
        assert_eq!(
            marks,
            vec![DVec2::new(20.0, 0.0), DVec2::new(0.0, 50.0)],
            "with no shared corner, each trait carries its own mark, at its middle"
        );
    }

    #[test]
    fn a_tangency_writes_where_the_circle_touches() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let start = sketch.add_point(DVec2::new(-40.0, 0.0));
        let end = sketch.add_point(DVec2::new(40.0, 0.0));
        let line = sketch.add_segment(start, end);
        let center = sketch.add_point(DVec2::new(0.0, 25.0));
        let circle = sketch.add_circle(center, 25.0);

        sketch.add_constraint(Constraint::Tangent {
            circle,
            segment: line,
            at: None,
        });
        let constraint = *sketch
            .constraints()
            .iter()
            .find(|rule| matches!(rule, Constraint::Tangent { .. }))
            .expect("the tangency was added");

        let marks = sketch.rule_marks(constraint);
        assert_eq!(
            marks,
            vec![DVec2::new(0.0, 0.0)],
            "the mark sits where the circle actually touches the line"
        );
    }

    #[test]
    fn a_rule_whose_mark_is_far_from_the_cursor_is_not_picked() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let a = sketch.add_point(DVec2::new(0.0, 0.0));
        let b = sketch.add_point(DVec2::new(40.0, 0.0));
        let c = sketch.add_point(DVec2::new(0.0, 30.0));
        let d = sketch.add_point(DVec2::new(0.0, 70.0));
        let first = sketch.add_segment(a, b);
        let second = sketch.add_segment(c, d);
        sketch.add_constraint(Constraint::Parallel { first, second });

        assert_eq!(sketch.nearest_rule(DVec2::new(200.0, 200.0), 5.0), None);
        assert!(sketch.nearest_rule(DVec2::new(20.0, 1.0), 5.0).is_some());
    }
}
