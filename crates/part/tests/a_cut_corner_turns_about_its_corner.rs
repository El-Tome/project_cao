//! Closes #565.
//! - after a fillet on a right angle, dragging the free end of one side turns
//!   that side about the corner: the corner stays, the side follows the
//!   cursor, the fillet keeps its radius and stays tangent to both sides, the
//!   other side does not move —
//!   `dragging_the_free_end_of_a_rounded_side_turns_it_about_the_corner`
//! - the same after a chamfer, the cut keeping its values —
//!   `dragging_the_free_end_of_a_chamfered_side_turns_it_about_the_corner`;
//!   both free ends, anywhere the cut still fits —
//!   `either_free_end_dragged_wherever_the_cut_fits_turns_its_side_about_the_corner`
//! - an angle typed between the two sides of a rounded or chamfered corner
//!   turns the second side clicked about the corner, and the corner stays —
//!   `an_angle_typed_between_the_sides_of_a_cut_corner_turns_the_second_about_the_corner`;
//!   the angle the corner carried before it was cut, read between the
//!   stretches the cut laid back in, the same when it is typed again —
//!   `the_angle_a_corner_carried_typed_again_turns_the_second_side_about_the_corner`
//! - the flip the owner saw is reproduced as a test, and no longer happens:
//!   140° typed inside a fillet of three rolled the side over the other one,
//!   the corner reading −40° —
//!   `a_wide_angle_typed_inside_a_rounded_corner_opens_it_and_no_longer_flips_the_side_across`;
//!   5° typed at a chamfer brought the cut out past the corner, inside out,
//!   and every value from 5° to 175° in each of the four quarters, typed from
//!   either side, now turns the side without a flip wherever the cut fits —
//!   `every_angle_typed_at_a_cut_corner_turns_the_side_and_never_flips_it`
//!
//! What the issue says must not break, held by tests already in the gate that
//! pass unchanged:
//! - #316's own criteria — no test: `a_corner_keeps_what_was_typed.rs` passes
//!   unchanged; it settles with the solver alone, and the gestures themselves
//!   are held below
//! - the fillet's tangency rules (#303) — no test: their tests pass unchanged,
//!   and every fillet above is asked to graze both its sides
//! - the tests of the two cuts — no test: `rounding_a_corner.rs`,
//!   `chamfering_a_corner.rs`, `fillet/tests.rs` and `chamfer/tests.rs` pass
//!   unchanged
//! - a fillet keeps the quarter a corner's angle measures (#564) — no test:
//!   `a_fillet_keeps_the_quarter_a_corners_angle_measures` passes unchanged
//!
//! What a review of the change found it had to hold besides:
//! - a side dragged back through its corner is stopped short, never brought
//!   out past it —
//!   `a_free_end_dragged_back_through_the_corner_never_brings_its_side_out_past_it`
//! - the same for a selection moved, which settles another way —
//!   `a_selection_moved_through_the_corner_never_brings_a_side_out_past_it`
//! - a trait between two cut corners turns about the corner the angle is typed
//!   at — `an_angle_typed_at_the_second_rounded_corner_of_a_trait_turns_about_that_corner`
//! - a rule laid from a side to anything else holds the side as before —
//!   `a_side_of_a_rounded_corner_made_equal_to_another_trait_keeps_its_length`
//! - a side that cannot turn, held level, moves with its corner as it would
//!   uncut — `a_rounded_side_held_level_moves_with_its_corner_rather_than_turn`
//! - « Perpendiculaire » between the two sides turns the second about the
//!   corner — `the_sides_of_a_rounded_corner_made_perpendicular_turn_the_second_about_it`
//! - the ends of a cut dragged by hand, and a side of a chamfer moved, on the
//!   gesture's own path — `dragging_the_end_of_a_cut_keeps_what_was_typed`,
//!   `moving_a_side_still_carries_the_corner_and_the_cut`. A side of a fillet
//!   moved is refused, here as on `main`: #587
//!
//! Beside the criteria, what was decided on the way: an angle the fillet
//! cannot be laid at with the corner where it stands, the sides too short for
//! it, rolls the corner along the first side as before, the first side staying
//! and the fillet whole —
//! `an_angle_too_tight_for_the_fillet_rolls_the_corner_along_the_first_side`;
//! and a free end dragged past where the fillet fits stops where it still
//! does — `a_free_end_dragged_past_where_the_fillet_fits_stops_where_it_still_does`.

use std::f64::consts::PI;

use cao_part::PartState;
use cao_part::history::{ChamferAsked, Operation, PointRef};
use cao_sketch::{
    Constraint, Corner, DimensionTarget, PointId, SegmentId, Sketch, SketchAxis, Toward, WorkPlane,
};
use glam::DVec2;

const CORNER: DVec2 = DVec2::new(2.0, 1.0);
const PIVOT: PointId = PointId(1);
const SIDE: f64 = 10.0;
const RADIUS: f64 = 3.0;
const CUT_BACK: f64 = 2.0;
/// What the solver promises of a value it had to reach by moving points: a
/// hundred-thousandth of the drawing's size, a dozen units across here.
const SETTLED: f64 = 1e-4;

#[derive(Clone, Copy, Debug)]
enum Cut {
    Fillet,
    Chamfer,
}

/// A right angle standing clear of the sketch's own origin, one side running
/// east and the other north, ten units each.
fn a_right_angle() -> Vec<Operation> {
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
    ]
}

fn cut_by(cut: Cut) -> Operation {
    let corners = vec![Corner::Between(SegmentId(0), SegmentId(1))];
    match cut {
        Cut::Fillet => Operation::Fillet {
            sketch: 0,
            corners,
            radius: RADIUS.into(),
        },
        Cut::Chamfer => Operation::Chamfer {
            sketch: 0,
            corners,
            mode: ChamferAsked::Equal(CUT_BACK.into()),
        },
    }
}

fn replay(operations: &[Operation]) -> PartState {
    let mut state = PartState::default();
    for operation in operations {
        state.apply(operation);
    }
    state
}

/// Where a side ends, `bearing` radians round from east, seen from the corner.
fn far_end_at(bearing: f64) -> DVec2 {
    CORNER + DVec2::from_angle(bearing) * SIDE
}

/// One side of the corner as the cut left it: the piece still drawn, its end
/// at the cut and its free end.
#[derive(Clone, Copy, Debug)]
struct Side {
    piece: SegmentId,
    near: PointId,
    far: PointId,
}

/// The side whose free end stood at `far`, read off the drawing the cut left.
fn side_ending_at(sketch: &Sketch, far: DVec2) -> Side {
    let far = sketch
        .live_points()
        .find(|(_, at)| at.distance(far) < 1e-9)
        .map(|(id, _)| id)
        .expect("the free end of a side");
    sketch
        .live_segments()
        .filter(|(_, segment)| !segment.construction)
        .find_map(
            |(piece, segment)| match (segment.start == far, segment.end == far) {
                (true, _) => Some(Side {
                    piece,
                    near: segment.end,
                    far,
                }),
                (_, true) => Some(Side {
                    piece,
                    near: segment.start,
                    far,
                }),
                _ => None,
            },
        )
        .expect("what is left of the side")
}

/// The corner cut, with its two sides: the one running east, then north.
fn cut_corner(cut: Cut) -> (Vec<Operation>, Side, Side) {
    let mut operations = a_right_angle();
    operations.push(cut_by(cut));
    let state = replay(&operations);
    let sketch = &state.sketches[0];
    let east = side_ending_at(sketch, far_end_at(0.0));
    let north = side_ending_at(sketch, far_end_at(PI / 2.0));
    (operations, east, north)
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

/// The corner where it stood, the two free ends where `east` and `north` say,
/// and the cut whole.
fn assert_cut_corner(state: &PartState, cut: Cut, sides: [(Side, DVec2); 2], case: &str) {
    let sketch = &state.sketches[0];
    let corner = sketch.point(PIVOT);
    assert!(
        corner.distance(CORNER) < SETTLED,
        "{case}: the corner stays where it was, got {corner:?}"
    );
    for (side, wanted) in sides {
        let far = sketch.point(side.far);
        assert!(
            far.distance(wanted) < SETTLED,
            "{case}: a free end at {wanted:?}, got {far:?}"
        );
    }
    assert_cut_whole(state, cut, sides.map(|(side, _)| side), case);
}

/// Each side's end at the cut on the line from the corner to its free end and
/// short of it, and the cut keeping the values it was given: a radius that
/// grazes both sides, or the same distance back along each.
fn assert_cut_whole(state: &PartState, cut: Cut, sides: [Side; 2], case: &str) {
    let sketch = &state.sketches[0];
    let corner = sketch.point(PIVOT);
    for side in sides {
        let far = sketch.point(side.far);
        let way = (far - corner).normalize();
        let near = sketch.point(side.near) - corner;
        assert!(
            way.perp_dot(near).abs() < SETTLED,
            "{case}: the end at the cut stays on the side's line, {near:?} off {way:?}"
        );
        let along = way.dot(near);
        assert!(
            along > 0.0 && along < far.distance(corner),
            "{case}: the end at the cut lies between the corner and the free end, \
             {along} along a side {} long",
            far.distance(corner)
        );
    }
    let [first, second] = sides;
    let opening = (sketch.point(first.far) - corner)
        .angle_to(sketch.point(second.far) - corner)
        .abs();
    match cut {
        Cut::Fillet => {
            let (arc, curve) = sketch.live_arcs().next().expect("the fillet");
            let reach = sketch.arc_radius(arc);
            assert!(
                (reach - RADIUS).abs() < SETTLED,
                "{case}: the fillet keeps its radius, got {reach}"
            );
            let centre = sketch.point(curve.center);
            for side in [first, second] {
                let (from, to) = sketch.endpoints(side.piece);
                let off = (centre - from).perp_dot((to - from).normalize()).abs();
                assert!(
                    (off - RADIUS).abs() < SETTLED,
                    "{case}: the fillet grazes each side, {off} off one of them"
                );
                assert!(
                    [curve.start, curve.end].contains(&side.near),
                    "{case}: the fillet still runs into each side"
                );
            }
            let sweep = sketch.arc_sweep(arc);
            assert!(
                (sweep - (PI - opening)).abs() < SETTLED,
                "{case}: the fillet turns the short way round, through {}° at a corner of {}°",
                sweep.to_degrees(),
                opening.to_degrees()
            );
        }
        Cut::Chamfer => {
            for side in [first, second] {
                let back = sketch.point(side.near).distance(corner);
                assert!(
                    (back - CUT_BACK).abs() < SETTLED,
                    "{case}: the chamfer keeps its values, {back} back along one side"
                );
            }
        }
    }
}

#[test]
fn dragging_the_free_end_of_a_rounded_side_turns_it_about_the_corner() {
    let (mut operations, east, north) = cut_corner(Cut::Fillet);
    let wanted = far_end_at(PI * 2.0 / 3.0);

    drag(&mut operations, north.far, wanted);

    assert_cut_corner(
        &replay(&operations),
        Cut::Fillet,
        [(east, far_end_at(0.0)), (north, wanted)],
        "the north side dragged to 120°",
    );
}

#[test]
fn dragging_the_free_end_of_a_chamfered_side_turns_it_about_the_corner() {
    let (mut operations, east, north) = cut_corner(Cut::Chamfer);
    let wanted = far_end_at(PI * 2.0 / 3.0);

    drag(&mut operations, north.far, wanted);

    assert_cut_corner(
        &replay(&operations),
        Cut::Chamfer,
        [(east, far_end_at(0.0)), (north, wanted)],
        "the north side dragged to 120°",
    );
}

#[test]
fn either_free_end_dragged_wherever_the_cut_fits_turns_its_side_about_the_corner() {
    for cut in [Cut::Fillet, Cut::Chamfer] {
        for opening in (1..36).map(|step| f64::from(step) * 5.0) {
            for reach in [6.0, 8.0, SIDE, 13.0, 20.0] {
                if !fits(cut, opening, reach.min(SIDE)) {
                    continue;
                }
                let (operations, east, north) = cut_corner(cut);
                let across = |bearing: f64| CORNER + DVec2::from_angle(bearing) * reach;

                let mut turned = operations.clone();
                let wanted = across(opening.to_radians());
                drag(&mut turned, north.far, wanted);
                assert_cut_corner(
                    &replay(&turned),
                    cut,
                    [(east, far_end_at(0.0)), (north, wanted)],
                    &format!("{cut:?}, the north end dragged {reach} out at {opening}°"),
                );

                let mut turned = operations;
                let wanted = across((90.0 - opening).to_radians());
                drag(&mut turned, east.far, wanted);
                assert_cut_corner(
                    &replay(&turned),
                    cut,
                    [(east, wanted), (north, far_end_at(PI / 2.0))],
                    &format!("{cut:?}, the east end dragged {reach} out, {opening}° from north"),
                );
            }
        }
    }
}

/// The arm of a side's angle, running out from the corner towards its free end
/// when `outwards`, or back past the corner the other way.
fn arm(sketch: &Sketch, side: Side, outwards: bool) -> Toward {
    let runs_out = sketch.segments()[side.piece.0].end == side.far;
    match runs_out == outwards {
        true => Toward::End,
        false => Toward::Start,
    }
}

/// Types `value` between two sides, each arm out towards its free end or
/// back past the corner as `outwards` says — which of the four quarters the
/// dimension was put down in.
fn type_angle(
    operations: &mut Vec<Operation>,
    [first, second]: [Side; 2],
    outwards: [bool; 2],
    value: f64,
) {
    let sketch = &replay(operations).sketches[0];
    let target = DimensionTarget::AngleBetween {
        first: first.piece,
        first_toward: arm(sketch, first, outwards[0]),
        second: second.piece,
        second_toward: arm(sketch, second, outwards[1]),
    };
    operations.push(Operation::SetDimension {
        sketch: 0,
        target,
        value: value.into(),
        placement: None,
    });
}

#[test]
fn an_angle_typed_between_the_sides_of_a_cut_corner_turns_the_second_about_the_corner() {
    for cut in [Cut::Fillet, Cut::Chamfer] {
        let (operations, east, north) = cut_corner(cut);

        let mut typed = operations.clone();
        type_angle(&mut typed, [east, north], [true, true], 60.0);
        assert_cut_corner(
            &replay(&typed),
            cut,
            [(east, far_end_at(0.0)), (north, far_end_at(PI / 3.0))],
            &format!("{cut:?}, 60° typed from the east side"),
        );

        let mut typed = operations;
        type_angle(&mut typed, [north, east], [true, true], 60.0);
        assert_cut_corner(
            &replay(&typed),
            cut,
            [(east, far_end_at(PI / 6.0)), (north, far_end_at(PI / 2.0))],
            &format!("{cut:?}, 60° typed from the north side"),
        );
    }
}

#[test]
fn a_wide_angle_typed_inside_a_rounded_corner_opens_it_and_no_longer_flips_the_side_across() {
    let (mut operations, east, north) = cut_corner(Cut::Fillet);

    type_angle(&mut operations, [east, north], [true, true], 140.0);

    assert_cut_corner(
        &replay(&operations),
        Cut::Fillet,
        [
            (east, far_end_at(0.0)),
            (north, far_end_at(140f64.to_radians())),
        ],
        "140° typed inside the fillet",
    );
}

/// Whether a cut still fits a corner open that wide, on sides that long, with
/// a unit to spare: a fillet of three on sides ten long needs the corner open
/// past 37°.
fn fits(cut: Cut, opening: f64, reach: f64) -> bool {
    match cut {
        Cut::Fillet => RADIUS / (opening.to_radians() / 2.0).tan() < reach - 1.0,
        Cut::Chamfer => CUT_BACK < reach - 1.0,
    }
}

#[test]
fn every_angle_typed_at_a_cut_corner_turns_the_side_and_never_flips_it() {
    for cut in [Cut::Fillet, Cut::Chamfer] {
        for outwards in [[true, true], [false, true], [true, false], [false, false]] {
            for value in (1..36).map(|step| f64::from(step) * 5.0) {
                let opening = match outwards[0] == outwards[1] {
                    true => value,
                    false => 180.0 - value,
                };
                if !fits(cut, opening, SIDE) {
                    continue;
                }
                let (operations, east, north) = cut_corner(cut);

                let mut typed = operations.clone();
                type_angle(&mut typed, [east, north], outwards, value);
                assert_cut_corner(
                    &replay(&typed),
                    cut,
                    [
                        (east, far_end_at(0.0)),
                        (north, far_end_at(opening.to_radians())),
                    ],
                    &format!("{cut:?}, {value}° typed from the east side, arms {outwards:?}"),
                );

                let mut typed = operations;
                type_angle(&mut typed, [north, east], outwards, value);
                assert_cut_corner(
                    &replay(&typed),
                    cut,
                    [
                        (east, far_end_at((90.0 - opening).to_radians())),
                        (north, far_end_at(PI / 2.0)),
                    ],
                    &format!("{cut:?}, {value}° typed from the north side, arms {outwards:?}"),
                );
            }
        }
    }
}

#[test]
fn an_angle_too_tight_for_the_fillet_rolls_the_corner_along_the_first_side() {
    let (mut operations, east, north) = cut_corner(Cut::Fillet);

    type_angle(&mut operations, [east, north], [true, true], 20.0);

    let state = replay(&operations);
    let sketch = &state.sketches[0];
    let east_end = sketch.point(east.far);
    assert!(
        east_end.distance(far_end_at(0.0)) < SETTLED,
        "the side clicked first stays, got its free end at {east_end:?}"
    );
    let corner = sketch.point(PIVOT);
    assert!(
        (corner.y - CORNER.y).abs() < SETTLED,
        "the corner rolls along the first side, got {corner:?}"
    );
    let opening = (east_end - corner)
        .angle_to(sketch.point(north.far) - corner)
        .to_degrees();
    assert!(
        (opening - 20.0).abs() < 1e-3,
        "and the angle is the one typed, got {opening}°"
    );
    assert_cut_whole(
        &state,
        Cut::Fillet,
        [east, north],
        "20° typed inside the fillet",
    );
}

#[test]
fn a_free_end_dragged_past_where_the_fillet_fits_stops_where_it_still_does() {
    let (mut operations, east, north) = cut_corner(Cut::Fillet);

    drag(&mut operations, north.far, far_end_at(20f64.to_radians()));

    let state = replay(&operations);
    let sketch = &state.sketches[0];
    let corner = sketch.point(PIVOT);
    assert!(
        corner.distance(CORNER) < SETTLED,
        "the corner stays, got {corner:?}"
    );
    let east_end = sketch.point(east.far);
    assert!(
        east_end.distance(far_end_at(0.0)) < SETTLED,
        "the other side does not move, got {east_end:?}"
    );
    let opening = (east_end - corner)
        .angle_to(sketch.point(north.far) - corner)
        .to_degrees();
    assert!(
        opening > 20.0 && opening < 90.0,
        "the side turned towards the hand and stopped short, at {opening}°"
    );
    assert_cut_whole(&state, Cut::Fillet, [east, north], "dragged to 20°");
}

#[test]
fn the_angle_a_corner_carried_typed_again_turns_the_second_side_about_the_corner() {
    for cut in [Cut::Fillet, Cut::Chamfer] {
        let mut operations = a_right_angle();
        operations.push(Operation::SetDimension {
            sketch: 0,
            target: DimensionTarget::corner(SegmentId(0), SegmentId(1)).normalised(),
            value: 90.0.into(),
            placement: None,
        });
        operations.push(cut_by(cut));
        let state = replay(&operations);
        let sketch = &state.sketches[0];
        let east = side_ending_at(sketch, far_end_at(0.0));
        let north = side_ending_at(sketch, far_end_at(PI / 2.0));
        let carried = sketch
            .dimensions()
            .iter()
            .map(|dimension| dimension.target)
            .find(|target| matches!(target, DimensionTarget::Angle { .. }))
            .expect("the angle the corner carried, hung back on the stretches");
        let DimensionTarget::Angle { first, .. } = carried else {
            unreachable!()
        };
        let stretch = sketch.segments()[first.0];
        let from_east = [stretch.start, stretch.end].contains(&east.near);

        operations.push(Operation::SetDimension {
            sketch: 0,
            target: carried,
            value: 60.0.into(),
            placement: None,
        });

        let wanted = match from_east {
            true => [(east, far_end_at(0.0)), (north, far_end_at(PI / 3.0))],
            false => [(east, far_end_at(PI / 6.0)), (north, far_end_at(PI / 2.0))],
        };
        assert_cut_corner(
            &replay(&operations),
            cut,
            wanted,
            &format!("{cut:?}, the angle the corner carried typed again at 60°"),
        );
    }
}

#[test]
fn a_free_end_dragged_back_through_the_corner_never_brings_its_side_out_past_it() {
    for cut in [Cut::Fillet, Cut::Chamfer] {
        let (mut operations, east, north) = cut_corner(cut);

        drag(&mut operations, north.far, CORNER - DVec2::new(0.0, 2.0));

        assert_cut_whole(
            &replay(&operations),
            cut,
            [east, north],
            &format!("{cut:?}, the north end dragged through the corner"),
        );
    }
}

#[test]
fn a_selection_moved_through_the_corner_never_brings_a_side_out_past_it() {
    for cut in [Cut::Fillet, Cut::Chamfer] {
        let (mut operations, east, north) = cut_corner(cut);

        operations.push(Operation::MoveMany {
            sketch: 0,
            points: vec![north.far],
            by: DVec2::new(0.0, -12.0),
        });

        assert_cut_whole(
            &replay(&operations),
            cut,
            [east, north],
            &format!("{cut:?}, the north end moved through the corner"),
        );
    }
}

/// A U standing clear of the origin: a side coming down at x = 2, a base along
/// y = 1 and a side going up at x = 12, both its corners rounded.
fn a_rounded_u() -> Vec<Operation> {
    let mut operations = vec![
        Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
        },
        Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(DVec2::new(2.0, 11.0)),
            end: PointRef::New(DVec2::new(2.0, 1.0)),
            construction: false,
        },
        Operation::AddSegment {
            sketch: 0,
            start: PointRef::Existing(PointId(2)),
            end: PointRef::New(DVec2::new(12.0, 1.0)),
            construction: false,
        },
        Operation::AddSegment {
            sketch: 0,
            start: PointRef::Existing(PointId(3)),
            end: PointRef::New(DVec2::new(12.0, 11.0)),
            construction: false,
        },
    ];
    operations.push(Operation::Fillet {
        sketch: 0,
        corners: vec![Corner::At(PointId(2)), Corner::At(PointId(3))],
        radius: 2.0.into(),
    });
    operations
}

#[test]
fn an_angle_typed_at_the_second_rounded_corner_of_a_trait_turns_about_that_corner() {
    let mut operations = a_rounded_u();
    let state = replay(&operations);
    let sketch = &state.sketches[0];
    let piece_between = |from: DVec2, to: DVec2| {
        sketch
            .live_segments()
            .filter(|(_, segment)| !segment.construction)
            .find(|(id, _)| {
                let (start, end) = sketch.endpoints(*id);
                start.distance(from) < 1e-9 && end.distance(to) < 1e-9
                    || start.distance(to) < 1e-9 && end.distance(from) < 1e-9
            })
            .expect("what is left of a side")
    };
    let (base, base_piece) = piece_between(DVec2::new(4.0, 1.0), DVec2::new(10.0, 1.0));
    let (right, right_piece) = piece_between(DVec2::new(12.0, 3.0), DVec2::new(12.0, 11.0));
    let base_ends = [base_piece.start, base_piece.end].map(|end| sketch.point(end));
    let towards_the_left = match sketch.point(base_piece.end).x < sketch.point(base_piece.start).x {
        true => Toward::End,
        false => Toward::Start,
    };
    let up = match sketch.point(right_piece.end).y > sketch.point(right_piece.start).y {
        true => Toward::End,
        false => Toward::Start,
    };
    let right_far = match up {
        Toward::End => right_piece.end,
        Toward::Start => right_piece.start,
    };
    operations.push(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::AngleBetween {
            first: base,
            first_toward: towards_the_left,
            second: right,
            second_toward: up,
        },
        value: 60.0.into(),
        placement: None,
    });

    let state = replay(&operations);
    let sketch = &state.sketches[0];
    let corner = DVec2::new(12.0, 1.0);
    assert!(
        sketch.point(PointId(3)).distance(corner) < SETTLED,
        "the corner the angle is typed at stays, got {:?}",
        sketch.point(PointId(3))
    );
    let wanted = corner + DVec2::from_angle(120f64.to_radians()) * SIDE;
    let got = sketch.point(right_far);
    assert!(
        got.distance(wanted) < SETTLED,
        "the side clicked second turns about it, its free end at {wanted:?}, got {got:?}"
    );
    for (end, was) in [base_piece.start, base_piece.end]
        .into_iter()
        .zip(base_ends)
    {
        assert!(
            sketch.point(end).y - was.y < SETTLED && was.y - sketch.point(end).y < SETTLED,
            "the base clicked first stays on its line, an end moved to {:?}",
            sketch.point(end)
        );
    }
    assert!(
        sketch.point(PointId(2)).distance(DVec2::new(2.0, 1.0)) < SETTLED,
        "and the other corner does not move, got {:?}",
        sketch.point(PointId(2))
    );
}

#[test]
fn a_side_of_a_rounded_corner_made_equal_to_another_trait_keeps_its_length() {
    let (mut operations, east, _) = cut_corner(Cut::Fillet);
    operations.push(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(20.0, 0.0)),
        end: PointRef::New(DVec2::new(20.0, 4.0)),
        construction: false,
    });
    let state = replay(&operations);
    let sketch = &state.sketches[0];
    let other = sketch
        .live_segments()
        .map(|(id, _)| id)
        .max_by_key(|id| id.0)
        .expect("the trait just drawn");
    let before = sketch.endpoints(east.piece);
    operations.push(Operation::Constrain {
        sketch: 0,
        constraint: Constraint::Equal {
            first: east.piece,
            second: other,
        },
    });

    let state = replay(&operations);
    let sketch = &state.sketches[0];
    let after = sketch.endpoints(east.piece);
    assert!(
        after.0.distance(before.0) < SETTLED && after.1.distance(before.1) < SETTLED,
        "the side clicked first stays as it was, {before:?} became {after:?}"
    );
    let (from, to) = sketch.endpoints(other);
    assert!(
        (from.distance(to) - before.0.distance(before.1)).abs() < SETTLED,
        "and the other takes its length, got {}",
        from.distance(to)
    );
}

#[test]
fn a_rounded_side_held_level_moves_with_its_corner_rather_than_turn() {
    let (mut operations, east, north) = cut_corner(Cut::Fillet);
    operations.push(Operation::Constrain {
        sketch: 0,
        constraint: Constraint::AxisParallel {
            segment: east.piece,
            axis: SketchAxis::U,
        },
    });
    let wanted = far_end_at(0.0) + DVec2::new(2.0, -3.0);

    drag(&mut operations, east.far, wanted);

    let state = replay(&operations);
    let sketch = &state.sketches[0];
    let reached = sketch.point(east.far);
    assert!(
        reached.distance(wanted) < SETTLED,
        "the free end follows the hand, got {reached:?}"
    );
    let corner = sketch.point(PIVOT);
    assert!(
        (corner.y - (CORNER.y - 3.0)).abs() < SETTLED,
        "the side cannot turn, so the corner goes down with it, got {corner:?}"
    );
    assert_cut_whole(
        &state,
        Cut::Fillet,
        [east, north],
        "a level side dragged down",
    );
}

#[test]
fn the_sides_of_a_rounded_corner_made_perpendicular_turn_the_second_about_it() {
    let mut operations = vec![
        Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
        },
        Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(CORNER),
            end: PointRef::New(far_end_at(0.0)),
            construction: false,
        },
        Operation::AddSegment {
            sketch: 0,
            start: PointRef::Existing(PIVOT),
            end: PointRef::New(far_end_at(PI / 3.0)),
            construction: false,
        },
        cut_by(Cut::Fillet),
    ];
    let state = replay(&operations);
    let sketch = &state.sketches[0];
    let east = side_ending_at(sketch, far_end_at(0.0));
    let slanted = side_ending_at(sketch, far_end_at(PI / 3.0));
    operations.push(Operation::Constrain {
        sketch: 0,
        constraint: Constraint::Perpendicular {
            first: east.piece,
            second: slanted.piece,
        },
    });

    assert_cut_corner(
        &replay(&operations),
        Cut::Fillet,
        [(east, far_end_at(0.0)), (slanted, far_end_at(PI / 2.0))],
        "a corner of 60° made square",
    );
}

#[test]
fn dragging_the_end_of_a_cut_keeps_what_was_typed() {
    let (mut operations, east, north) = cut_corner(Cut::Chamfer);
    let state = replay(&operations);
    let end = state.sketches[0].point(east.near);

    drag(&mut operations, east.near, end + DVec2::new(1.0, 1.0));

    let state = replay(&operations);
    let sketch = &state.sketches[0];
    for side in [east, north] {
        let back = sketch.point(side.near).distance(sketch.point(PIVOT));
        assert!(
            (back - CUT_BACK).abs() < SETTLED,
            "the cut keeps its values, {back} back along one side"
        );
    }
}

#[test]
fn moving_a_side_still_carries_the_corner_and_the_cut() {
    let (mut operations, east, north) = cut_corner(Cut::Chamfer);

    operations.push(Operation::MoveSegment {
        sketch: 0,
        segment: north.piece,
        by: DVec2::new(3.0, 0.0),
    });

    let state = replay(&operations);
    let corner = state.sketches[0].point(PIVOT);
    assert!(
        (corner.x - CORNER.x - 3.0).abs() < SETTLED,
        "the corner follows the side it is held on, got {corner:?}"
    );
    assert_cut_whole(&state, Cut::Chamfer, [east, north], "a side moved");
}
