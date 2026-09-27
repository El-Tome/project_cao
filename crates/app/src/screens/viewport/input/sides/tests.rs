//! What app · viewport/input/sides.rs is held to.

use cao_sketch::{Curved, PointId};
use glam::DVec2;

use super::*;

#[test]
fn a_side_pulled_across_writes_how_far_it_travelled() {
    let drag = SideDrag::Across {
        by: DVec2::new(0.0, 12.5),
    };

    let written = side_operation(2, SegmentId(3), &drag);

    assert!(
        matches!(
            written,
            Some(Operation::MoveSegment { sketch: 2, segment: SegmentId(3), by }) if by == DVec2::new(0.0, 12.5)
        ),
        "{written:?}"
    );
}

#[test]
fn a_side_slid_along_writes_the_turn_of_its_shape() {
    let turn = Turn {
        points: vec![PointId(1), PointId(2)],
        about: DVec2::new(5.0, 5.0),
        angle: 0.25,
    };

    let written = side_operation(0, SegmentId(0), &SideDrag::Along(turn));

    assert!(
        matches!(&written, Some(Operation::TurnShape { points, angle, .. }) if points.len() == 2 && *angle == 0.25),
        "{written:?}"
    );
}

#[test]
fn a_hand_back_where_it_pressed_writes_nothing() {
    let still = SideDrag::Across { by: DVec2::ZERO };
    let unturned = Turn {
        points: vec![PointId(1)],
        about: DVec2::ZERO,
        angle: 0.0,
    };

    assert!(side_operation(0, SegmentId(0), &still).is_none());
    assert!(turn_operation(0, &unturned).is_none());
}

fn a_turn() -> Turn {
    Turn {
        points: vec![PointId(1), PointId(2)],
        about: DVec2::new(5.0, 5.0),
        angle: 0.25,
    }
}

#[test]
fn a_curve_turned_and_drawn_to_the_hand_writes_both_as_one_step() {
    let arc = Curved::Arc(cao_sketch::ArcId(0));

    let written = curve_turn_operation(
        1,
        arc,
        &a_turn(),
        LengthOutcome::Exact,
        Some((45.0, LengthOutcome::Exact)),
    );

    assert!(
        matches!(
            &written,
            Some(Operation::Gesture(done)) if matches!(
                done.as_slice(),
                [Operation::TurnShape { .. }, Operation::ResizeArc { reach, .. }] if *reach == 45.0
            )
        ),
        "{written:?}"
    );
}

#[test]
fn a_curve_writes_only_what_the_drawing_took() {
    let arc = Curved::Arc(cao_sketch::ArcId(0));

    let turned_alone = curve_turn_operation(
        1,
        arc,
        &a_turn(),
        LengthOutcome::Exact,
        Some((45.0, LengthOutcome::BestEffort)),
    );
    let at_its_size = curve_turn_operation(1, arc, &a_turn(), LengthOutcome::Exact, None);
    let sized_alone = curve_turn_operation(
        1,
        arc,
        &a_turn(),
        LengthOutcome::BestEffort,
        Some((45.0, LengthOutcome::Exact)),
    );
    let nothing = curve_turn_operation(
        1,
        arc,
        &a_turn(),
        LengthOutcome::BestEffort,
        Some((45.0, LengthOutcome::BestEffort)),
    );

    assert!(
        matches!(turned_alone, Some(Operation::TurnShape { .. })),
        "a size given back: {turned_alone:?}"
    );
    assert!(
        matches!(at_its_size, Some(Operation::TurnShape { .. })),
        "the size it had: {at_its_size:?}"
    );
    assert!(
        matches!(sized_alone, Some(Operation::ResizeArc { .. })),
        "a turn refused: {sized_alone:?}"
    );
    assert_eq!(nothing, None);
}
