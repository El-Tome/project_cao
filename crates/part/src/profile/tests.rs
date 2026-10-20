//! The loops a pinched outline welds, taken apart again.

use std::f64::consts::TAU;

use super::*;

fn welded_from_the_touch() -> Contour {
    Contour {
        corners: vec![
            DVec2::new(0.0, 10.0),
            DVec2::new(10.0, 10.0),
            DVec2::new(10.0, 0.0),
            DVec2::new(-10.0, 0.0),
            DVec2::new(-10.0, 10.0),
            DVec2::new(1e-9, 10.0),
        ],
        runs: vec![
            Run::Straight,
            Run::Straight,
            Run::Straight,
            Run::Straight,
            Run::Straight,
            Run::Round {
                center: DVec2::new(0.0, 7.0),
                turn: -TAU,
            },
        ],
    }
}

#[test]
fn a_loop_opening_on_the_touch_parts_into_the_rectangle_and_its_hole() {
    let (outline, holes) = pinched_apart(welded_from_the_touch());

    assert_eq!(outline.runs, vec![Run::Straight; 5], "{outline:?}");
    assert_eq!(holes.len(), 1, "{holes:?}");
    assert_eq!(holes[0].corners.len(), 1, "{holes:?}");
    assert!(
        matches!(holes[0].runs[0], Run::Round { turn, .. } if turn == -TAU),
        "{holes:?}",
    );
}

#[test]
fn a_loop_opening_mid_side_parts_the_same() {
    let whole = welded_from_the_touch();
    let count = whole.corners.len();
    let turned = |by: usize| Contour {
        corners: (0..count)
            .map(|at| whole.corners[(at + by) % count])
            .collect(),
        runs: (0..count).map(|at| whole.runs[(at + by) % count]).collect(),
    };
    for by in 1..count {
        let (outline, holes) = pinched_apart(turned(by));

        assert_eq!(outline.runs.len(), 5, "turned by {by}: {outline:?}");
        assert_eq!(holes.len(), 1, "turned by {by}: {holes:?}");
        assert!(
            matches!(holes[0].runs[..], [Run::Round { turn, .. }] if turn == -TAU),
            "turned by {by}: {holes:?}",
        );
    }
}

#[test]
fn a_loop_through_no_corner_twice_passes_whole() {
    let plain = Contour::rectangle(DVec2::ZERO, DVec2::new(4.0, 3.0));

    let (outline, holes) = pinched_apart(plain.clone());

    assert_eq!(outline, plain);
    assert!(holes.is_empty(), "{holes:?}");
}
