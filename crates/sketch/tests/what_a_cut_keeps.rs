//! What a cut keeps of the rules and the values that spoke of the trait.
//!
//! What is said about *direction* survives whole: the pieces lie on the line
//! the trait lay on. What is fastened to a place on that line follows the
//! piece that place fell on. A length measures a trait that is no longer
//! there.

use cao_sketch::{
    CircleId, Constraint, DimensionTarget, PointId, SegmentId, Sketch, SketchAxis, WorkPlane,
};
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

fn a_circle_brushing_a_long_trait() -> (Sketch, SegmentId, CircleId, PointId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(0.0, 4.0));
    let end = sketch.add_point(DVec2::new(20.0, 4.0));
    let cut = sketch.add_segment(start, end);
    let centre = sketch.add_point(DVec2::new(3.0, 7.0));
    let circle = sketch.add_circle(centre, 3.0);
    sketch.add_tangency(circle, cut);
    let contact = sketch
        .constraints()
        .iter()
        .find_map(|rule| match rule {
            Constraint::Tangent {
                at: Some(point), ..
            } => Some(*point),
            _ => None,
        })
        .expect("a tangency keeps a point where the two touch");
    (sketch, cut, circle, contact)
}

fn brushed_by(sketch: &Sketch, circle: CircleId) -> Vec<SegmentId> {
    sketch
        .constraints()
        .iter()
        .filter_map(|rule| match rule {
            Constraint::Tangent {
                circle: round,
                segment,
                ..
            } if *round == circle => Some(*segment),
            _ => None,
        })
        .collect()
}

#[test]
fn a_tangency_follows_the_piece_the_contact_sits_on() {
    let (mut sketch, cut, circle, contact) = a_circle_brushing_a_long_trait();
    let first = sketch.add_point(DVec2::new(10.0, 4.0));
    let second = sketch.add_point(DVec2::new(15.0, 4.0));

    let trimmed = sketch
        .trim(cut, first, second)
        .expect("a cut that can be made");

    assert_eq!(
        brushed_by(&sketch, circle),
        vec![trimmed.pieces[0]],
        "the contact is on the near piece, and on that one only",
    );
    assert!(
        !sketch.is_erased_point(contact),
        "the contact is still drawn"
    );
    assert_eq!(trimmed.rules_dropped, 0);
}

#[test]
fn a_tangency_whose_contact_falls_in_the_stretch_that_goes_is_lost_and_counted() {
    let (mut sketch, cut, circle, contact) = a_circle_brushing_a_long_trait();
    let first = sketch.add_point(DVec2::new(1.0, 4.0));
    let second = sketch.add_point(DVec2::new(8.0, 4.0));

    let trimmed = sketch
        .trim(cut, first, second)
        .expect("a cut that can be made");

    assert_eq!(brushed_by(&sketch, circle), Vec::new());
    assert!(
        sketch.is_erased_point(contact),
        "the place they touched is not on the drawing any more",
    );
    assert_eq!(trimmed.rules_dropped, 1);
}

fn a_point_measured_from_a_long_trait() -> (Sketch, SegmentId, PointId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(0.0, 4.0));
    let end = sketch.add_point(DVec2::new(20.0, 4.0));
    let cut = sketch.add_segment(start, end);
    let point = sketch.add_point(DVec2::new(3.0, 9.0));
    sketch.set_dimension(
        DimensionTarget::PointToSegment {
            point,
            segment: cut,
        },
        5.0,
        false,
    );
    (sketch, cut, point)
}

fn measured_from(sketch: &Sketch, point: PointId) -> Vec<SegmentId> {
    sketch
        .dimensions()
        .iter()
        .filter_map(|value| match value.target {
            DimensionTarget::PointToSegment {
                point: from,
                segment,
            } if from == point => Some(segment),
            _ => None,
        })
        .collect()
}

#[test]
fn a_distance_to_the_line_follows_the_piece_the_foot_of_it_falls_on() {
    let (mut sketch, cut, point) = a_point_measured_from_a_long_trait();
    let first = sketch.add_point(DVec2::new(10.0, 4.0));
    let second = sketch.add_point(DVec2::new(15.0, 4.0));

    let trimmed = sketch
        .trim(cut, first, second)
        .expect("a cut that can be made");

    assert_eq!(
        measured_from(&sketch, point),
        vec![trimmed.pieces[0]],
        "on one piece: both lie on the line, and twice would say it twice",
    );
    assert_eq!(trimmed.values_dropped, 0);
}

#[test]
fn a_distance_whose_foot_falls_in_the_stretch_that_goes_is_lost_and_counted() {
    let (mut sketch, cut, point) = a_point_measured_from_a_long_trait();
    let first = sketch.add_point(DVec2::new(1.0, 4.0));
    let second = sketch.add_point(DVec2::new(8.0, 4.0));

    let trimmed = sketch
        .trim(cut, first, second)
        .expect("a cut that can be made");

    assert_eq!(measured_from(&sketch, point), Vec::new());
    assert_eq!(trimmed.values_dropped, 1);
}
