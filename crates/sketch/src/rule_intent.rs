//! What the constraint tool is pointed at, and what it means once it has been
//! shown enough.
//!
//! Two of the nine rules are not constraints at all: a point brought onto
//! another, or two circles onto a single centre, are merges — holding them
//! apart with an equation would leave two points sitting on top of each other
//! for ever, which is exactly what the drawing does not want. `RuleIntent`
//! keeps that apart from `Constraint` so the caller, which does know how a
//! merge is recorded, can tell the two apart.

use crate::constraints::{Constraint, SketchAxis};
use crate::sketch::{CircleId, Element, PointId, SegmentId, Sketch};

/// What the constraint tool has been pointed at: a piece of the drawing, or
/// one of the sketch's own axes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RulePick {
    Element(Element),
    Axis(SketchAxis),
}

/// Which rule the constraint tool is about to lay down.
///
/// A rule is placed by pointing at what it speaks of: two traits for a right
/// angle, a point and a trait for a coincidence. The tool holds what has been
/// picked so far and lays the rule down as soon as it has enough.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rule {
    Perpendicular,
    Parallel,
    Equal,
    Coincident,
    Collinear,
    Tangent,
    Midpoint,
    Fixed,
    Concentric,
}

impl Rule {
    /// How many things it needs before it can be laid down.
    pub fn arity(self) -> usize {
        match self {
            Self::Fixed => 1,
            _ => 2,
        }
    }
}

/// The step a rule becomes, once it has been shown what it speaks of.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RuleIntent {
    Constrain(Constraint),
    Merge { kept: PointId, dropped: PointId },
}

/// Turns what the constraint tool has been shown into what it means.
///
/// The order of the clicks does not matter: a point and a trait make the same
/// coincidence whichever comes first, so this is built from the kinds
/// gathered rather than from their order.
pub fn rule_intent(rule: Rule, picks: &[RulePick], sketch: &Sketch) -> Option<RuleIntent> {
    // The order is kept: for an equality, the first trait clicked is the one
    // whose length the other takes.
    let segments: Vec<SegmentId> = picks
        .iter()
        .filter_map(|pick| match pick {
            RulePick::Element(Element::Segment(id)) => Some(*id),
            _ => None,
        })
        .collect();
    let points: Vec<PointId> = picks
        .iter()
        .filter_map(|pick| match pick {
            RulePick::Element(Element::Point(id)) => Some(*id),
            _ => None,
        })
        .collect();
    let circles: Vec<CircleId> = picks
        .iter()
        .filter_map(|pick| match pick {
            RulePick::Element(Element::Circle(id)) => Some(*id),
            _ => None,
        })
        .collect();
    let axes: Vec<SketchAxis> = picks
        .iter()
        .filter_map(|pick| match pick {
            RulePick::Axis(axis) => Some(*axis),
            _ => None,
        })
        .collect();

    let constrain = |constraint: Constraint| Some(RuleIntent::Constrain(constraint));
    let pair = |list: &[SegmentId]| (list.len() == 2).then(|| (list[0], list[1]));

    match rule {
        Rule::Perpendicular => pair(&segments)
            .and_then(|(first, second)| constrain(Constraint::Perpendicular { first, second })),
        Rule::Parallel => pair(&segments)
            .and_then(|(first, second)| constrain(Constraint::Parallel { first, second })),
        Rule::Collinear => match (pair(&segments), segments.as_slice(), axes.as_slice()) {
            (Some((first, second)), _, _) => constrain(Constraint::Collinear { first, second }),
            // A trait laid on one of the sketch's own axes, which is the same
            // rule against a line that cannot move.
            (None, [segment], [axis]) => constrain(Constraint::AxisCollinear {
                segment: *segment,
                axis: *axis,
            }),
            _ => None,
        },
        Rule::Equal => match (pair(&segments), circles.as_slice()) {
            (Some((first, second)), _) => constrain(Constraint::Equal { first, second }),
            (None, [first, second]) => constrain(Constraint::EqualRadius {
                first: *first,
                second: *second,
            }),
            _ => None,
        },
        Rule::Tangent => match (circles.as_slice(), segments.as_slice()) {
            ([circle], [segment]) => constrain(Constraint::Tangent {
                at: None,
                circle: *circle,
                segment: *segment,
            }),
            _ => None,
        },
        Rule::Midpoint => match (points.as_slice(), segments.as_slice()) {
            ([point], [segment]) => constrain(Constraint::Midpoint {
                point: *point,
                segment: *segment,
            }),
            _ => None,
        },
        Rule::Fixed => match picks {
            [RulePick::Element(element)] => constrain(Constraint::Fixed { element: *element }),
            _ => None,
        },
        Rule::Coincident => match (points.as_slice(), segments.as_slice()) {
            ([point], [segment]) => constrain(Constraint::OnSegment {
                point: *point,
                segment: *segment,
            }),
            // Two points asked to coincide are one point: the origin is never
            // the one that gives way.
            ([first, second], []) => {
                let (kept, dropped) = match sketch.is_origin(*second) {
                    true => (*second, *first),
                    false => (*first, *second),
                };
                Some(RuleIntent::Merge { kept, dropped })
            }
            _ => None,
        },
        Rule::Concentric => match circles.as_slice() {
            [first, second] => {
                let (kept, dropped) = (sketch.circle(*first).center, sketch.circle(*second).center);
                (kept != dropped).then_some(RuleIntent::Merge { kept, dropped })
            }
            _ => None,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plane::WorkPlane;

    #[test]
    fn the_order_of_the_clicks_does_not_change_the_rule() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let start = sketch.add_point(glam::DVec2::new(0.0, 0.0));
        let end = sketch.add_point(glam::DVec2::new(10.0, 0.0));
        let point = sketch.add_point(glam::DVec2::new(5.0, 5.0));
        let segment = sketch.add_segment(start, end);

        let forward = rule_intent(
            Rule::Coincident,
            &[
                RulePick::Element(Element::Point(point)),
                RulePick::Element(Element::Segment(segment)),
            ],
            &sketch,
        );
        let backward = rule_intent(
            Rule::Coincident,
            &[
                RulePick::Element(Element::Segment(segment)),
                RulePick::Element(Element::Point(point)),
            ],
            &sketch,
        );

        assert_eq!(
            forward, backward,
            "a point and a trait coincide either way round"
        );
        assert_eq!(
            forward,
            Some(RuleIntent::Constrain(Constraint::OnSegment {
                point,
                segment
            })),
        );
    }

    #[test]
    fn two_points_asked_to_coincide_become_one() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let first = sketch.add_point(glam::DVec2::new(0.0, 0.0));
        let second = sketch.add_point(glam::DVec2::new(10.0, 10.0));

        let intent = rule_intent(
            Rule::Coincident,
            &[
                RulePick::Element(Element::Point(first)),
                RulePick::Element(Element::Point(second)),
            ],
            &sketch,
        );

        assert_eq!(
            intent,
            Some(RuleIntent::Merge {
                kept: first,
                dropped: second
            }),
        );
    }

    #[test]
    fn the_origin_is_never_the_point_that_gives_way() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let held = sketch.add_point(glam::DVec2::new(10.0, 10.0));

        let intent = rule_intent(
            Rule::Coincident,
            &[
                RulePick::Element(Element::Point(held)),
                RulePick::Element(Element::Point(Sketch::ORIGIN)),
            ],
            &sketch,
        );

        assert_eq!(
            intent,
            Some(RuleIntent::Merge {
                kept: Sketch::ORIGIN,
                dropped: held
            }),
        );
    }

    #[test]
    fn two_circles_asked_to_be_concentric_merge_their_centres() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let first_center = sketch.add_point(glam::DVec2::new(0.0, 0.0));
        let second_center = sketch.add_point(glam::DVec2::new(20.0, 0.0));
        let first = sketch.add_circle(first_center, 5.0);
        let second = sketch.add_circle(second_center, 8.0);

        let intent = rule_intent(
            Rule::Concentric,
            &[
                RulePick::Element(Element::Circle(first)),
                RulePick::Element(Element::Circle(second)),
            ],
            &sketch,
        );

        assert_eq!(
            intent,
            Some(RuleIntent::Merge {
                kept: first_center,
                dropped: second_center
            }),
        );
    }

    #[test]
    fn a_rule_shown_the_wrong_kinds_yields_nothing() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let point = sketch.add_point(glam::DVec2::new(0.0, 0.0));

        let intent = rule_intent(
            Rule::Perpendicular,
            &[RulePick::Element(Element::Point(point))],
            &sketch,
        );

        assert_eq!(intent, None, "a right angle needs two traits, not a point");
    }
}
