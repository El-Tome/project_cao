//! What sketch · constraints.rs is held to: every rule a drawing can carry is
//! one the solver answers for.

use glam::DVec2;

use super::*;
use crate::plane::WorkPlane;
use crate::sketch::{Element, Sketch};

/// A drawing carrying one of everything a rule can speak of, and one of every
/// kind of rule laid on it.
///
/// `name_of` below has no wildcard arm, so a new kind of rule stops the build
/// here rather than slipping past the sweep that walks this list.
fn one_of_every_kind() -> (Sketch, Vec<Constraint>) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let west = sketch.add_point(DVec2::new(0.0, 20.0));
    let east = sketch.add_point(DVec2::new(40.0, 20.0));
    let side = sketch.add_segment(west, east);
    let south = sketch.add_point(DVec2::new(20.0, 0.0));
    let north = sketch.add_point(DVec2::new(20.0, 40.0));
    let up = sketch.add_segment(south, north);

    let centre = sketch.add_point(DVec2::new(60.0, 20.0));
    let round = sketch.add_circle(centre, 8.0);
    let other_centre = sketch.add_point(DVec2::new(60.0, 50.0));
    let other_round = sketch.add_circle(other_centre, 6.0);

    let bend_centre = sketch.add_point(DVec2::new(100.0, 20.0));
    let bend_start = sketch.add_point(DVec2::new(110.0, 20.0));
    let bend_end = sketch.add_point(DVec2::new(100.0, 30.0));
    let bend = sketch.add_arc(bend_centre, bend_start, bend_end);
    let other_bend_start = sketch.add_point(DVec2::new(100.0, 60.0));
    let other_bend_end = sketch.add_point(DVec2::new(90.0, 50.0));
    let other_bend = sketch.add_arc(bend_centre, other_bend_start, other_bend_end);

    let loose = sketch.add_point(DVec2::new(20.0, 20.0));

    let rules = vec![
        Constraint::Perpendicular {
            first: side,
            second: up,
        },
        Constraint::Parallel {
            first: side,
            second: up,
        },
        Constraint::Equal {
            first: side,
            second: up,
        },
        Constraint::EqualRadius {
            first: round,
            second: other_round,
        },
        Constraint::EqualRadiusArc {
            first: bend,
            second: other_bend,
        },
        Constraint::EqualRadiusArcCircle {
            arc: bend,
            circle: round,
        },
        Constraint::OnSegment {
            point: loose,
            segment: side,
        },
        Constraint::Collinear {
            first: side,
            second: up,
        },
        Constraint::Tangent {
            circle: round,
            segment: side,
            at: None,
        },
        Constraint::ArcTangent {
            arc: bend,
            segment: side,
            at: None,
        },
        Constraint::OnCircle {
            point: loose,
            circle: round,
        },
        Constraint::OnArc {
            point: loose,
            arc: bend,
        },
        Constraint::OnAxis {
            point: loose,
            axis: SketchAxis::U,
        },
        Constraint::Midpoint {
            point: loose,
            segment: side,
        },
        Constraint::AxisCollinear {
            segment: side,
            axis: SketchAxis::U,
        },
        Constraint::Fixed {
            element: Element::Point(loose),
        },
    ];

    let mut kinds: Vec<&str> = rules.iter().map(name_of).collect();
    kinds.sort_unstable();
    kinds.dedup();
    assert_eq!(
        kinds.len(),
        rules.len(),
        "one of every kind means one of each: {kinds:?} for {} laid",
        rules.len(),
    );

    (sketch, rules)
}

fn name_of(rule: &Constraint) -> &'static str {
    match rule {
        Constraint::Perpendicular { .. } => "perpendicular",
        Constraint::Parallel { .. } => "parallel",
        Constraint::Equal { .. } => "equal",
        Constraint::EqualRadius { .. } => "equal radius",
        Constraint::EqualRadiusArc { .. } => "equal radius, arcs",
        Constraint::EqualRadiusArcCircle { .. } => "equal radius, an arc and a circle",
        Constraint::OnSegment { .. } => "on a trait",
        Constraint::Collinear { .. } => "collinear",
        Constraint::Tangent { .. } => "tangent",
        Constraint::ArcTangent { .. } => "tangent, arc",
        Constraint::OnCircle { .. } => "on a circle",
        Constraint::OnArc { .. } => "on an arc",
        Constraint::OnAxis { .. } => "on an axis",
        Constraint::Midpoint { .. } => "midpoint",
        Constraint::AxisCollinear { .. } => "on an axis, trait",
        Constraint::Fixed { .. } => "fixed",
    }
}

/// What this cannot tell: whether the equation a rule writes says what the
/// rule means. That is each rule's own test; this is the sweep that stops a
/// new one from being added with nothing behind it at all.
#[test]
fn every_kind_of_rule_asks_the_solver_for_something() {
    let (bare, rules) = one_of_every_kind();

    for rule in rules {
        let mut sketch = bare.clone();
        let before = sketch.equations(1.0).len();
        sketch.add_constraint(rule);
        let after = sketch.equations(1.0).len();

        // A rule that holds a point still asks for nothing of the solver: it
        // takes the point out of play instead, which is one pinned point and
        // no equation.
        if matches!(rule, Constraint::Fixed { .. }) {
            assert!(sketch.is_held(Element::Point(match rule {
                Constraint::Fixed {
                    element: Element::Point(point),
                } => point,
                _ => unreachable!(),
            })));
            continue;
        }
        assert!(
            after > before,
            "{} asks the solver for nothing: the drawing cannot hold it",
            name_of(&rule),
        );
    }
}
