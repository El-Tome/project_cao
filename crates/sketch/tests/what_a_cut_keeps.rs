//! What a cut keeps of the rules and the values that spoke of the trait, and
//! what it hands back as the price of the ones it could not.
//!
//! Only what is said about *direction* survives: the pieces lie on the line
//! the trait lay on. A length measures a trait that is no longer there.

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
fn a_rule_about_a_traits_direction_holds_on_both_of_its_pieces() {
    let (mut sketch, cut, other, [first, second]) = a_trait_alongside_another();
    sketch.add_constraint(Constraint::Parallel {
        first: cut,
        second: other,
    });

    let pieces = sketch
        .trim(cut, first, second)
        .expect("a cut that can be made")
        .pieces;

    for piece in pieces {
        assert!(
            sketch.constraints().contains(
                &Constraint::Parallel {
                    first: piece,
                    second: other
                }
                .normalised()
            ) || sketch.constraints().contains(
                &Constraint::Parallel {
                    first: other,
                    second: piece
                }
                .normalised()
            ),
            "{piece:?} is no longer parallel to what the trait was",
        );
    }
}

#[test]
fn a_rule_about_a_traits_length_holds_on_neither_piece() {
    let (mut sketch, cut, other, [first, second]) = a_trait_alongside_another();
    sketch.add_constraint(Constraint::Equal {
        first: cut,
        second: other,
    });

    sketch
        .trim(cut, first, second)
        .expect("a cut that can be made");

    assert!(
        !sketch
            .constraints()
            .iter()
            .any(|rule| matches!(rule, Constraint::Equal { .. })),
        "the pieces are shorter than what was measured",
    );
}

#[test]
fn a_trait_laid_on_an_axis_leaves_both_pieces_on_it() {
    let (mut sketch, cut, _, [first, second]) = a_trait_alongside_another();
    sketch.add_constraint(Constraint::AxisCollinear {
        segment: cut,
        axis: SketchAxis::U,
    });

    let pieces = sketch
        .trim(cut, first, second)
        .expect("a cut that can be made")
        .pieces;

    for piece in pieces {
        assert!(
            sketch.constraints().contains(&Constraint::AxisCollinear {
                segment: piece,
                axis: SketchAxis::U,
            }),
            "{piece:?} came off the axis the trait was laid on",
        );
    }
}

#[test]
fn an_angle_to_an_axis_is_still_read_on_both_pieces() {
    let (mut sketch, cut, _, [first, second]) = a_trait_alongside_another();
    let target = DimensionTarget::AxisAngle {
        segment: cut,
        axis: SketchAxis::U,
    };
    sketch.set_dimension(target, 30.0, false);

    let pieces = sketch
        .trim(cut, first, second)
        .expect("a cut that can be made")
        .pieces;

    for piece in pieces {
        let kept = sketch.dimension_of(DimensionTarget::AxisAngle {
            segment: piece,
            axis: SketchAxis::U,
        });
        assert_eq!(kept.map(|value| value.value), Some(30.0), "{piece:?}");
    }
}

#[test]
fn an_angle_at_a_corner_follows_the_piece_that_still_reaches_it() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corner = sketch.add_point(DVec2::new(0.0, 5.0));
    let away = sketch.add_point(DVec2::new(10.0, 5.0));
    let cut = sketch.add_segment(corner, away);
    let up = sketch.add_point(DVec2::new(0.0, 15.0));
    let other = sketch.add_segment(corner, up);
    sketch.set_dimension(
        DimensionTarget::Angle {
            first: cut,
            second: other,
        },
        90.0,
        false,
    );
    let first = sketch.add_point(DVec2::new(4.0, 5.0));
    let second = sketch.add_point(DVec2::new(6.0, 5.0));

    let pieces = sketch
        .trim(cut, first, second)
        .expect("a cut that can be made")
        .pieces;

    let measured: Vec<SegmentId> = pieces
        .iter()
        .copied()
        .filter(|piece| {
            sketch.dimensions().iter().any(|value| {
                matches!(
                    value.target,
                    DimensionTarget::Angle { first, second }
                        if first == *piece || second == *piece
                )
            })
        })
        .collect();
    let touching: Vec<SegmentId> = pieces
        .iter()
        .copied()
        .filter(|piece| {
            let ends = sketch.segments()[piece.0];
            ends.start == corner || ends.end == corner
        })
        .collect();
    assert_eq!(measured, touching);
    assert_eq!(
        touching.len(),
        1,
        "one piece stops at the corner, one does not"
    );
}

#[test]
fn a_length_typed_on_a_trait_measures_neither_piece() {
    let (mut sketch, cut, _, [first, second]) = a_trait_alongside_another();
    sketch.set_dimension(DimensionTarget::Length(cut), 40.0, false);

    sketch
        .trim(cut, first, second)
        .expect("a cut that can be made");

    assert!(
        !sketch
            .dimensions()
            .iter()
            .any(|value| matches!(value.target, DimensionTarget::Length(_))),
        "the pieces are shorter than what was typed",
    );
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
