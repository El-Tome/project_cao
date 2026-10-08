//! A side drawn along another stays along it once the drawing has moved.
//!
//! The solver counts a rule as held within a hundred-thousandth of the
//! drawing, and the graph the areas are read from welds at a ten-millionth: a
//! side drawn along another and dragged afterwards came out leaning on it by
//! a fraction of a micron, read as two sides parting at their corner, with a
//! needle of matter between them. A drawing whose rules hold is now settled
//! further, while that gains, under what the graph welds.
//!
//! Closes #545.
//! - the owner's drawing, through `PartDocument`, raises the frame with the
//!   notch and the window as one opening, closed and uncrossed, its volume the
//!   frame's measure times its depth —
//!   `the_owner_s_notch_and_window_dragged_apart_by_a_hair_raise_as_one_opening`
//! - the same with #540's two windows inside a frame, after dragging the side
//!   they share — `windows_whose_shared_side_was_dragged_leave_the_frame_one_opening`
//! - #500's campaign breaks no drawing main does not, over the same seeds —
//!   no test: run by hand on main, in its own build directory, and on this
//!   branch
//! - the kernel's refusal of a needle still holds for a profile handed to it
//!   directly — no test: held, in the kernel, by
//!   `a_pinch_whose_walls_leave_its_corner_nearly_along_each_other_is_declined`
//!   in `crates/solid/src/brep/prism/tests.rs`

use cao_part::history::{ExtrusionMode, Operation, PointRef};
use cao_part::{History, PartState};
use cao_sketch::{Constraint, PointId, Region, SegmentId, WorkPlane};
use cao_solid::soundness;
use glam::{DVec2, DVec3};

/// The drawing of the part the owner raised on 2026-10-08, as its history
/// holds it: a frame, a notch from (10, 10) up to the frame's top, a window
/// started on the notch's lower corner, and — after the frame was raised —
/// the notch's right side dragged to the left, which left the window's side
/// leaning on it by 0.14 µm.
const THE_OWNER_S_DRAWING: &str = r#"[
    {"AddRectangle": {"sketch": 0, "corner": {"Existing": 0}, "opposite": {"New": [60.0, 60.0]}, "construction": false}},
    {"Constrain": {"sketch": 0, "constraint": {"Perpendicular": {"first": 0, "second": 1}}}},
    {"Constrain": {"sketch": 0, "constraint": {"Perpendicular": {"first": 1, "second": 2}}}},
    {"Constrain": {"sketch": 0, "constraint": {"Perpendicular": {"first": 2, "second": 3}}}},
    {"SetDimension": {"sketch": 0, "target": {"Length": 0}, "value": 30.0, "placement": [0.0, -4.732239454984665]}},
    {"SetDimension": {"sketch": 0, "target": {"Length": 1}, "value": 30.0, "placement": [4.732239454984665, 0.0]}},
    {"EraseMany": {"sketch": 0, "elements": [], "dimensions": [{"Length": 1}], "constraints": []}},
    {"EraseMany": {"sketch": 0, "elements": [], "dimensions": [{"Length": 0}], "constraints": []}},
    {"AddRectangle": {"sketch": 0, "corner": {"New": [10.0, 10.0]}, "opposite": {"Held": {"at": [30.0, 60.0], "on": [{"Segment": 2}]}}, "construction": false}},
    {"Constrain": {"sketch": 0, "constraint": {"Perpendicular": {"first": 4, "second": 5}}}},
    {"Constrain": {"sketch": 0, "constraint": {"Perpendicular": {"first": 5, "second": 6}}}},
    {"Constrain": {"sketch": 0, "constraint": {"Perpendicular": {"first": 6, "second": 7}}}},
    {"SetDimension": {"sketch": 0, "target": {"Length": 4}, "value": 10.0, "placement": [0.0, -3.7225115299224854]}},
    {"AddRectangle": {"sketch": 0, "corner": {"Existing": 6}, "opposite": {"New": [50.0, 50.0]}, "construction": false}},
    {"Constrain": {"sketch": 0, "constraint": {"Perpendicular": {"first": 8, "second": 9}}}},
    {"Constrain": {"sketch": 0, "constraint": {"Perpendicular": {"first": 9, "second": 10}}}},
    {"Constrain": {"sketch": 0, "constraint": {"Perpendicular": {"first": 10, "second": 11}}}},
    {"SetDimension": {"sketch": 0, "target": {"Length": 8}, "value": 10.0, "placement": [0.0, -3.7225115299224854]}},
    {"SetDimension": {"sketch": 0, "target": {"Length": 9}, "value": 20.0, "placement": [3.7225115299224854, 0.0]}},
    {"EraseMany": {"sketch": 0, "elements": [], "dimensions": [{"Length": 9}], "constraints": []}},
    {"EraseMany": {"sketch": 0, "elements": [], "dimensions": [{"Length": 8}], "constraints": []}},
    {"EraseMany": {"sketch": 0, "elements": [], "dimensions": [{"Length": 4}], "constraints": []}},
    {"MoveSegment": {"sketch": 0, "segment": 5, "by": [-6.661307060174668, 0.0]}}
]"#;

const DEPTH: f64 = 10.0;

fn sketch_history() -> History {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    history
}

fn raise(history: &mut History, place: DVec2) {
    history.push(Operation::Extrude {
        sketch: 0,
        areas: PartState::rebuild(history).areas_at(0, &[place]),
        distance: DEPTH.into(),
        mode: ExtrusionMode::Add,
    });
}

/// The area the place falls in, as the drawing stands.
fn area_at(state: &PartState, place: DVec2) -> Region {
    state.sketches[0]
        .regions()
        .into_iter()
        .filter(|region| region.contains(place))
        .max_by_key(|region| region.depth)
        .expect("an area there")
}

fn surface(triangles: &[[DVec3; 3]]) -> f64 {
    triangles
        .iter()
        .map(|[a, b, c]| (*b - *a).cross(*c - *a).length() / 2.0)
        .sum()
}

/// The area at `place` raised alone: closed, uncrossed, its measure times the
/// depth, and no more surface than its two caps and the walls round its own
/// outline and holes — a fin standing in the matter would add to it.
fn assert_raised_whole(state: &PartState, place: DVec2) {
    let area = area_at(state, place);
    let travel = DEPTH / state.scale();
    let triangles = state.body.triangles();
    soundness::closed(&triangles).expect("raised closed");
    soundness::uncrossed(&triangles).expect("raised uncrossed");
    let volume = state.body.volume();
    let arithmetic = area.area() * travel;
    assert!(
        (volume - arithmetic).abs() <= 1e-9 * arithmetic,
        "{volume} where the arithmetic gives {arithmetic}"
    );
    let round = area.perimeter()
        + area
            .holes
            .iter()
            .map(|hole| Region {
                outline: hole.clone(),
                holes: Vec::new(),
                depth: 0,
                triangles: Vec::new(),
            })
            .map(|hole| hole.perimeter())
            .sum::<f64>();
    let expected = 2.0 * area.area() + round * travel;
    let made = surface(&triangles);
    assert!(
        (made - expected).abs() <= 1e-6 * expected,
        "a surface of {made} where caps and walls make {expected}"
    );
}

#[test]
fn the_owner_s_notch_and_window_dragged_apart_by_a_hair_raise_as_one_opening() {
    let operations: Vec<Operation> =
        serde_json::from_str(THE_OWNER_S_DRAWING).expect("the owner's drawing reads");
    let (before, after) = operations.split_at(operations.len() - 1);
    let mut history = sketch_history();
    for operation in before {
        history.push(operation.clone());
    }
    let band = DVec2::new(3.6101782680153036, 57.10949255227801);
    raise(&mut history, band);
    history.push(after[0].clone());

    let state = PartState::rebuild(&history);

    assert_raised_whole(&state, band);
}

fn rectangle(history: &mut History, corner: PointRef, opposite: DVec2) {
    history.push(Operation::AddRectangle {
        sketch: 0,
        corner,
        opposite: PointRef::New(opposite),
        construction: false,
    });
}

/// The three right angles the rectangle tool holds a rectangle by, its sides
/// being the last four segments drawn.
fn squared(history: &mut History, first: usize) {
    for side in first..first + 3 {
        history.push(Operation::Constrain {
            sketch: 0,
            constraint: Constraint::Perpendicular {
                first: SegmentId(side),
                second: SegmentId(side + 1),
            },
        });
    }
}

#[test]
fn windows_whose_shared_side_was_dragged_leave_the_frame_one_opening() {
    let mut history = sketch_history();
    rectangle(&mut history, PointRef::New(DVec2::ZERO), DVec2::splat(30.0));
    squared(&mut history, 0);
    rectangle(
        &mut history,
        PointRef::New(DVec2::splat(5.0)),
        DVec2::new(15.0, 25.0),
    );
    squared(&mut history, 4);
    let corner = PartState::rebuild(&history).sketches[0]
        .points()
        .iter()
        .position(|point| point.distance(DVec2::new(15.0, 5.0)) < 1e-9)
        .expect("window A's lower right corner");
    rectangle(
        &mut history,
        PointRef::Existing(PointId(corner)),
        DVec2::new(25.0, 25.0),
    );
    squared(&mut history, 8);
    let band = DVec2::splat(2.0);
    raise(&mut history, band);
    history.push(Operation::MoveSegment {
        sketch: 0,
        segment: SegmentId(5),
        by: DVec2::new(-3.3, 0.0),
    });

    let state = PartState::rebuild(&history);

    assert_raised_whole(&state, band);
}
