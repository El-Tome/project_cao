//! Closes #592.
//! - where a frame of drag spends its time beside an angled chamfer is
//!   measured and written down, before anything is changed — no test: a
//!   measurement, written on the issue on 2026-10-14 and on 2026-10-15
//! - one frame of drag of every point of the part's sketch stays under a
//!   budget set from that measurement, held by a test with a timing a slower
//!   machine still meets (#502) — no test: set once a drag's tries are cut
//!   down too, by the commits that follow
//! - what each point lands on is unchanged, unless that is what the
//!   measurement shows to be wrong — the measurement showed the ends of the
//!   cuts beside the angled chamfers stopping short of the hand for want of
//!   passes, and the owner chose on 2026-10-15 that they follow it —
//!   `an_end_of_a_cut_beside_the_angled_chamfers_follows_the_hand_all_the_way`

use cao_part::PartState;
use cao_part::history::{ChamferAsked, Operation, PointRef};
use cao_sketch::{Corner, LengthOutcome, PointId, SegmentId, Sketch, WorkPlane};
use glam::DVec2;

fn side(start: PointRef, end: PointRef) -> Operation {
    Operation::AddSegment {
        sketch: 0,
        start,
        end,
        construction: false,
    }
}

fn drag(point: usize, x: f64, y: f64) -> Operation {
    Operation::MovePoint {
        sketch: 0,
        point: PointId(point),
        position: DVec2::new(x, y),
        merged_into: None,
        on: Vec::new(),
        let_go: false,
    }
}

fn new(x: f64, y: f64) -> PointRef {
    PointRef::New(DVec2::new(x, y))
}

fn at(point: usize) -> PointRef {
    PointRef::Existing(PointId(point))
}

fn between(first: usize, second: usize) -> Corner {
    Corner::Between(SegmentId(first), SegmentId(second))
}

/// The owner's part of #592, as they drew it: an outline of eleven sides, five
/// of its corners rounded, two chamfered by equal distances and three by a
/// distance and an angle — one of those on a corner seventeen degrees from
/// straight — and points dragged in between, as they were.
fn the_owner_s_part() -> Sketch {
    let operations = vec![
        Operation::CreateSketch {
            plane: WorkPlane::YZ,
            on: None,
        },
        side(new(150.0, 350.0), new(775.0, 200.0)),
        side(at(2), new(1025.0, 750.0000000000001)),
        side(at(3), new(1550.0, 625.0)),
        side(at(4), new(1550.0, 1075.0)),
        side(at(5), new(699.9999999999999, 1250.0)),
        side(at(6), new(1775.0, 1325.0)),
        side(at(7), new(675.0, 1425.0)),
        side(at(8), new(274.99999999999994, 1250.0)),
        side(at(9), new(624.9999999999999, 775.0000000000001)),
        side(at(10), new(-74.99999999999989, 675.0)),
        side(at(11), at(1)),
        Operation::Fillet {
            sketch: 0,
            corners: [11, 1, 2, 3, 10]
                .map(|corner| Corner::At(PointId(corner)))
                .to_vec(),
            radius: 100.0.into(),
        },
        Operation::Chamfer {
            sketch: 0,
            corners: vec![between(23, 3), Corner::At(PointId(5))],
            mode: ChamferAsked::Equal(100.0.into()),
        },
        Operation::MoveSegment {
            sketch: 0,
            segment: SegmentId(39),
            by: DVec2::new(-9.714226073090803, -47.1833837835837),
        },
        drag(30, 2124.9999999999977, 949.9999999999989),
        drag(27, 1399.9999999999984, 524.9999999999994),
        drag(6, 794.2481845369024, 1182.897464847822),
        drag(28, 1587.7888652158786, 581.2395419854124),
        Operation::Chamfer {
            sketch: 0,
            corners: vec![between(36, 5), between(5, 6), between(7, 6)],
            mode: ChamferAsked::Angled {
                along: 100.0.into(),
                degrees: 20.0.into(),
            },
        },
        drag(33, 1399.9999999999984, 1199.9999999999986),
        drag(33, 1349.754921747137, 1085.2404001337504),
    ];
    let mut state = PartState::default();
    for operation in &operations {
        state.apply(operation);
    }
    state.sketches.remove(0)
}

/// How far the hand goes from the press, as the issue measured it.
const HAND: DVec2 = DVec2::new(20.0, 15.0);

fn dragged(sketch: &Sketch, point: PointId, by: DVec2) -> (Sketch, LengthOutcome) {
    let pull = sketch.pull(point, 1.0);
    let mut moved = sketch.clone();
    let outcome = moved.settle_pulled(&pull, sketch.point(point) + by, 1.0);
    (moved, outcome)
}

#[test]
fn an_end_of_a_cut_beside_the_angled_chamfers_follows_the_hand_all_the_way() {
    let sketch = the_owner_s_part();
    for point in [PointId(30), PointId(33)] {
        for by in [HAND, -HAND] {
            let (moved, outcome) = dragged(&sketch, point, by);

            assert_eq!(outcome, LengthOutcome::Exact, "p{} by {by}", point.0);
            let missed = moved.point(point).distance(sketch.point(point) + by);
            assert!(missed < 1e-6, "p{} by {by} stops {missed} short", point.0);
        }
    }
}
