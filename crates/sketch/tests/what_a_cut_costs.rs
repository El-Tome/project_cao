//! What a cut hands back as the price of what it could not carry over.
//!
//! The drawing used to lose those rules and those values in silence: the user
//! found out when the shape started moving.

use cao_sketch::{Constraint, DimensionTarget, PointId, SegmentId, Sketch, SketchAxis, WorkPlane};
use glam::DVec2;

fn a_trait_alongside_another() -> (Sketch, SegmentId, SegmentId, [PointId; 2]) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(0.0, 1.0));
    let end = sketch.add_point(DVec2::new(10.0, 1.0));
    let cut = sketch.add_segment(start, end);
    let far_start = sketch.add_point(DVec2::new(0.0, 5.0));
    let far_end = sketch.add_point(DVec2::new(10.0, 5.0));
    let other = sketch.add_segment(far_start, far_end);
    let first = sketch.add_point(DVec2::new(4.0, 1.0));
    let second = sketch.add_point(DVec2::new(6.0, 1.0));
    (sketch, cut, other, [first, second])
}

#[test]
fn a_cut_counts_the_rules_it_could_not_carry_over() {
    let (mut sketch, cut, other, [first, second]) = a_trait_alongside_another();
    sketch.add_constraint(Constraint::Parallel {
        first: cut,
        second: other,
    });
    sketch.add_constraint(Constraint::Equal {
        first: cut,
        second: other,
    });

    let trimmed = sketch
        .trim(cut, first, second)
        .expect("a cut that can be made");

    assert_eq!(
        trimmed.rules_dropped, 1,
        "the equal lengths went, the parallel followed both pieces",
    );
}

#[test]
fn a_cut_counts_the_values_it_could_not_carry_over() {
    let (mut sketch, cut, _, [first, second]) = a_trait_alongside_another();
    sketch.set_dimension(DimensionTarget::Length(cut), 40.0, false);
    sketch.set_dimension(
        DimensionTarget::AxisAngle {
            segment: cut,
            axis: SketchAxis::U,
        },
        0.0,
        false,
    );

    let trimmed = sketch
        .trim(cut, first, second)
        .expect("a cut that can be made");

    assert_eq!(
        trimmed.values_dropped, 1,
        "the length went, the angle to the axis followed both pieces",
    );
}

#[test]
fn a_cut_that_takes_the_whole_trait_loses_everything_that_spoke_of_it() {
    let (mut sketch, cut, other, [_, _]) = a_trait_alongside_another();
    let (start, end) = (sketch.segments()[cut.0].start, sketch.segments()[cut.0].end);
    sketch.add_constraint(Constraint::Parallel {
        first: cut,
        second: other,
    });
    sketch.set_dimension(DimensionTarget::Length(cut), 40.0, false);

    let trimmed = sketch
        .trim(cut, start, end)
        .expect("a cut that can be made");

    assert_eq!(trimmed.pieces, Vec::new(), "nothing of the trait is left");
    assert_eq!((trimmed.rules_dropped, trimmed.values_dropped), (1, 1));
}

#[test]
fn a_cut_nothing_was_said_about_costs_nothing() {
    let (mut sketch, cut, other, [first, second]) = a_trait_alongside_another();
    sketch.add_constraint(Constraint::AxisCollinear {
        segment: other,
        axis: SketchAxis::U,
    });

    let trimmed = sketch
        .trim(cut, first, second)
        .expect("a cut that can be made");

    assert_eq!((trimmed.rules_dropped, trimmed.values_dropped), (0, 0));
}
