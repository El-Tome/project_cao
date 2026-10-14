//! Closes #591.
//! - on a corner rounded alone, dragging the corner point moves the corner
//!   under the cursor, the sides turning about their free ends, and the
//!   fillet keeps its radius and stays tangent to both —
//!   `the_corner_of_a_rounded_right_angle_follows_the_hand_its_sides_turning_about_their_free_ends`
//! - on an outline with every corner rounded or chamfered, dragging the corner
//!   point of one moves that corner, the corners at the other ends of its two
//!   sides stay, and every cut keeps its values —
//!   `the_corner_of_a_plate_rounded_all_round_moves_alone`,
//!   `the_corner_of_a_plate_chamfered_all_round_moves_alone`
//! - the same for a chamfer, in each of its three modes —
//!   `the_corner_of_a_chamfer_follows_the_hand_in_each_of_its_modes`
//! - beside the criteria: the corner point is taken where the hand is, where
//!   it used to stay at the crossing of its two lines (#316) —
//!   `the_corner_a_cut_left_is_taken_where_the_hand_is`
//! - beside the criteria: a free end of a side of a chamfer, dragged, turns
//!   the side about the corner in each of the chamfer's modes, as #565 has
//!   it for equal distances —
//!   `a_free_end_of_a_chamfered_side_turns_it_in_each_of_the_modes`
//! - a corner whose sides cannot turn follows as far as the drawing lets it,
//!   and its cut is never turned inside out —
//!   `a_corner_on_a_side_held_level_slides_along_it_and_stays_whole`

use std::f64::consts::PI;

use cao_part::PartState;
use cao_part::history::{ChamferAsked, Operation, PointRef};
use cao_sketch::{
    ArcId, Constraint, Corner, DimensionTarget, PointId, SegmentId, Sketch, SketchAxis, WorkPlane,
};
use glam::DVec2;

const CORNER: DVec2 = DVec2::new(2.0, 1.0);
const PIVOT: PointId = PointId(1);
const SIDE: f64 = 10.0;
/// What the solver promises of a value it had to reach by moving points: a
/// hundred-thousandth of the drawing's size, a hundred units across at most
/// here.
const SETTLED: f64 = 1e-3;

fn replay(operations: &[Operation]) -> PartState {
    let mut state = PartState::default();
    for operation in operations {
        state.apply(operation);
    }
    state
}

fn drag(operations: &mut Vec<Operation>, point: PointId, to: DVec2) {
    operations.push(Operation::MovePoint {
        sketch: 0,
        point,
        position: to,
        merged_into: None,
        on: Vec::new(),
        let_go: false,
    });
}

/// A right angle standing clear of the sketch's own origin, one side running
/// east and the other north, ten units each, its corner cut by `cut`.
fn a_cut_right_angle(cut: Operation) -> Vec<Operation> {
    vec![
        Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
        },
        Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(CORNER),
            end: PointRef::New(CORNER + DVec2::new(SIDE, 0.0)),
            construction: false,
        },
        Operation::AddSegment {
            sketch: 0,
            start: PointRef::Existing(PIVOT),
            end: PointRef::New(CORNER + DVec2::new(0.0, SIDE)),
            construction: false,
        },
        cut,
    ]
}

fn fillet_at(corners: Vec<Corner>, radius: f64) -> Operation {
    Operation::Fillet {
        sketch: 0,
        corners,
        radius: radius.into(),
    }
}

fn chamfer_at(corners: Vec<Corner>, mode: ChamferAsked) -> Operation {
    Operation::Chamfer {
        sketch: 0,
        corners,
        mode,
    }
}

fn the_right_angle() -> Vec<Corner> {
    vec![Corner::Between(SegmentId(0), SegmentId(1))]
}

fn point_at(sketch: &Sketch, place: DVec2) -> PointId {
    sketch
        .live_points()
        .find(|(_, point)| point.distance(place) < 1e-9)
        .map(|(id, _)| id)
        .expect("a point there")
}

/// What a value measures on the drawing as it stands.
fn measured(sketch: &Sketch, target: DimensionTarget) -> f64 {
    match target {
        DimensionTarget::Distance { from, to } => sketch.point(from).distance(sketch.point(to)),
        DimensionTarget::ArcRadius(arc) => sketch.arc_radius(arc),
        angle => sketch
            .opening(angle)
            .expect("an angle the drawing can read"),
    }
}

/// Every value the drawing was given still read off it.
fn assert_values_hold(sketch: &Sketch, case: &str) {
    for dimension in sketch.dimensions().iter().filter(|value| !value.driven) {
        let reads = measured(sketch, dimension.target);
        assert!(
            (reads - dimension.value).abs() < SETTLED,
            "{case}: {:?} reads {reads}, typed {}",
            dimension.target,
            dimension.value
        );
    }
}

fn tangent_to(sketch: &Sketch, arc: ArcId) -> Vec<SegmentId> {
    sketch
        .constraints()
        .iter()
        .filter_map(|rule| match rule {
            Constraint::ArcTangent {
                arc: on, segment, ..
            } if *on == arc => Some(*segment),
            _ => None,
        })
        .collect()
}

/// Every fillet grazing both the traits it is held tangent to, at the radius
/// it was given, and running into each of them.
fn assert_fillets_whole(sketch: &Sketch, case: &str) {
    for (arc, curve) in sketch.live_arcs() {
        let sides = tangent_to(sketch, arc);
        assert_eq!(sides.len(), 2, "{case}: a fillet held tangent to {sides:?}");
        let centre = sketch.point(curve.center);
        for side in sides {
            let (from, to) = sketch.endpoints(side);
            let off = (centre - from).perp_dot((to - from).normalize()).abs();
            assert!(
                (off - sketch.arc_radius(arc)).abs() < SETTLED,
                "{case}: a fillet grazes its side, {off} off it"
            );
            let line = sketch.segments()[side.0];
            assert!(
                [curve.start, curve.end]
                    .iter()
                    .any(|end| [line.start, line.end].contains(end)),
                "{case}: a fillet runs into each of its sides"
            );
        }
        let sweep = sketch.arc_sweep(arc);
        assert!(
            sweep > 0.0 && sweep < PI,
            "{case}: a fillet turns the short way round, through {}°",
            sweep.to_degrees()
        );
    }
}

#[test]
fn the_corner_of_a_rounded_right_angle_follows_the_hand_its_sides_turning_about_their_free_ends() {
    let mut operations = a_cut_right_angle(fillet_at(the_right_angle(), 3.0));
    let state = replay(&operations);
    let sketch = &state.sketches[0];
    let east = point_at(sketch, CORNER + DVec2::new(SIDE, 0.0));
    let north = point_at(sketch, CORNER + DVec2::new(0.0, SIDE));
    let wanted = CORNER + DVec2::new(-2.0, -1.5);

    drag(&mut operations, PIVOT, wanted);

    let state = replay(&operations);
    let sketch = &state.sketches[0];
    let corner = sketch.point(PIVOT);
    assert!(
        corner.distance(wanted) < SETTLED,
        "the corner follows the hand, got {corner:?}"
    );
    for (far, was) in [
        (east, CORNER + DVec2::new(SIDE, 0.0)),
        (north, CORNER + DVec2::new(0.0, SIDE)),
    ] {
        assert!(
            sketch.point(far).distance(was) < SETTLED,
            "a free end stays where it was, got {:?}",
            sketch.point(far)
        );
    }
    assert_fillets_whole(sketch, "the corner of a right angle dragged");
    assert_values_hold(sketch, "the corner of a right angle dragged");
}

/// A plate standing clear of the origin, a hundred by sixty, its four corners
/// in order round it.
fn a_plate() -> (Vec<Operation>, [PointId; 4], [DVec2; 4]) {
    let low = DVec2::new(2.0, 1.0);
    let high = DVec2::new(102.0, 61.0);
    let operations = vec![
        Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
        },
        Operation::AddRectangle {
            sketch: 0,
            corner: PointRef::New(low),
            opposite: PointRef::New(high),
            construction: false,
        },
    ];
    let places = [
        low,
        DVec2::new(high.x, low.y),
        high,
        DVec2::new(low.x, high.y),
    ];
    let state = replay(&operations);
    let corners = places.map(|place| point_at(&state.sketches[0], place));
    (operations, corners, places)
}

fn a_plate_cut_all_round(cut: impl Fn(Vec<Corner>) -> Operation, case: &str) {
    let (mut operations, corners, places) = a_plate();
    operations.push(cut(corners.iter().copied().map(Corner::At).collect()));
    let wanted = places[0] + DVec2::new(-8.0, -5.0);

    drag(&mut operations, corners[0], wanted);

    let state = replay(&operations);
    let sketch = &state.sketches[0];
    let corner = sketch.point(corners[0]);
    assert!(
        corner.distance(wanted) < SETTLED,
        "{case}: the corner dragged follows the hand, got {corner:?}"
    );
    for (other, was) in corners.iter().zip(places).skip(1) {
        let now = sketch.point(*other);
        assert!(
            now.distance(was) < SETTLED,
            "{case}: every other corner stays, one moved from {was:?} to {now:?}"
        );
    }
    assert_fillets_whole(sketch, case);
    assert_values_hold(sketch, case);
}

#[test]
fn the_corner_of_a_plate_rounded_all_round_moves_alone() {
    a_plate_cut_all_round(
        |corners| fillet_at(corners, 10.0),
        "a plate rounded all round",
    );
}

#[test]
fn the_corner_of_a_plate_chamfered_all_round_moves_alone() {
    a_plate_cut_all_round(
        |corners| chamfer_at(corners, ChamferAsked::Equal(5.0.into())),
        "a plate chamfered all round",
    );
}

#[test]
fn the_corner_of_a_chamfer_follows_the_hand_in_each_of_its_modes() {
    for (mode, case) in [
        (ChamferAsked::Equal(2.0.into()), "equal distances"),
        (
            ChamferAsked::Sided {
                first: 2.0.into(),
                second: 3.0.into(),
            },
            "two distances",
        ),
        (
            ChamferAsked::Angled {
                along: 2.0.into(),
                degrees: 30.0.into(),
            },
            "a distance and an angle",
        ),
    ] {
        let mut operations = a_cut_right_angle(chamfer_at(the_right_angle(), mode));
        let wanted = CORNER + DVec2::new(1.5, -2.0);

        drag(&mut operations, PIVOT, wanted);

        let state = replay(&operations);
        let sketch = &state.sketches[0];
        let corner = sketch.point(PIVOT);
        assert!(
            corner.distance(wanted) < SETTLED,
            "{case}: the corner follows the hand, got {corner:?}"
        );
        for was in [
            CORNER + DVec2::new(SIDE, 0.0),
            CORNER + DVec2::new(0.0, SIDE),
        ] {
            assert!(
                sketch
                    .live_points()
                    .any(|(_, at)| at.distance(was) < SETTLED),
                "{case}: the free end at {was:?} stays"
            );
        }
        assert_values_hold(sketch, case);
    }
}

#[test]
fn a_corner_on_a_side_held_level_slides_along_it_and_stays_whole() {
    let mut operations = a_cut_right_angle(fillet_at(the_right_angle(), 3.0));
    let state = replay(&operations);
    let sketch = &state.sketches[0];
    let east_end = point_at(sketch, CORNER + DVec2::new(SIDE, 0.0));
    let east = sketch
        .live_segments()
        .filter(|(_, line)| !line.construction)
        .find(|(_, line)| line.start == east_end || line.end == east_end)
        .map(|(id, _)| id)
        .expect("what is left of the east side");
    operations.push(Operation::Constrain {
        sketch: 0,
        constraint: Constraint::AxisParallel {
            segment: east,
            axis: SketchAxis::U,
        },
    });

    drag(&mut operations, PIVOT, CORNER + DVec2::new(-2.0, 3.0));

    let state = replay(&operations);
    let sketch = &state.sketches[0];
    let corner = sketch.point(PIVOT);
    assert!(
        (corner.y - CORNER.y).abs() < SETTLED && corner.x < CORNER.x - 1.0,
        "the corner goes along the level side towards the hand, got {corner:?}"
    );
    assert_fillets_whole(sketch, "a corner on a level side");
    assert_values_hold(sketch, "a corner on a level side");
}

#[test]
fn the_corner_a_cut_left_is_taken_where_the_hand_is() {
    for cut in [
        fillet_at(the_right_angle(), 3.0),
        chamfer_at(the_right_angle(), ChamferAsked::Equal(2.0.into())),
    ] {
        let state = replay(&a_cut_right_angle(cut));
        let wanted = CORNER + DVec2::new(-2.0, -1.5);

        let landing = state.sketches[0].slide(PIVOT, wanted);

        assert!(
            landing.distance(wanted) < 1e-12,
            "the corner goes where the hand is, not back to the crossing: {landing:?}"
        );
    }
}

#[test]
fn a_free_end_of_a_chamfered_side_turns_it_in_each_of_the_modes() {
    for (mode, case) in [
        (ChamferAsked::Equal(2.0.into()), "equal distances"),
        (
            ChamferAsked::Sided {
                first: 2.0.into(),
                second: 3.0.into(),
            },
            "two distances",
        ),
        (
            ChamferAsked::Angled {
                along: 2.0.into(),
                degrees: 30.0.into(),
            },
            "a distance and an angle",
        ),
    ] {
        let mut operations = a_cut_right_angle(chamfer_at(the_right_angle(), mode));
        let state = replay(&operations);
        let north = point_at(&state.sketches[0], CORNER + DVec2::new(0.0, SIDE));
        let wanted = CORNER + DVec2::from_angle(PI * 2.0 / 3.0) * SIDE;

        drag(&mut operations, north, wanted);

        let state = replay(&operations);
        let sketch = &state.sketches[0];
        let corner = sketch.point(PIVOT);
        assert!(
            corner.distance(CORNER) < SETTLED,
            "{case}: the corner stays, got {corner:?}"
        );
        let reached = sketch.point(north);
        assert!(
            reached.distance(wanted) < SETTLED,
            "{case}: the free end follows the hand, got {reached:?}"
        );
        assert!(
            sketch
                .live_points()
                .any(|(_, at)| at.distance(CORNER + DVec2::new(SIDE, 0.0)) < SETTLED),
            "{case}: the other side stays"
        );
        assert_values_hold(sketch, case);
    }
}
