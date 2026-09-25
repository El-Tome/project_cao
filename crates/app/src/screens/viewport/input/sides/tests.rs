//! What app · viewport/input/sides.rs is held to.

use cao_sketch::PointId;
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
