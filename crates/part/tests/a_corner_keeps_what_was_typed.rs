//! What a chamfer and a fillet leave behind besides their geometry: the corner
//! as a point, and the values that were typed, written into the drawing as
//! dimensions that drive it.
//!
//! Closes #316.
//! - the values typed are left as dimensions, and only those —
//!   `equal_distances_are_written_along_each_side`,
//!   `two_distances_are_written_one_along_each_side`,
//!   `a_distance_and_an_angle_are_written_against_the_side_they_were_read_from`,
//!   `an_angled_chamfer_writes_both_the_values_it_was_given`,
//!   `a_fillet_writes_the_radius_that_was_typed`,
//!   `the_angle_the_two_sides_stand_at_is_never_written`
//! - the value typed applies to every corner taken, and undo removes them in
//!   one step — `every_corner_of_a_plate_is_rounded_by_the_one_gesture`,
//!   `the_whole_gesture_is_taken_back_by_one_undo`
//! - a mode that measures from the side named first still gathers several
//!   corners, though two of them share a trait —
//!   `two_corners_sharing_a_trait_are_both_cut_by_the_one_gesture`
//! - a corner too tight for the value is refused, and the others are laid
//!   anyway — `a_corner_too_tight_is_refused_and_the_others_are_rounded_anyway`
//! - a cut's values are typed lengths like any other, so the first of them
//!   fixes what the part measures —
//!   `a_chamfer_is_the_first_value_a_part_is_given_and_fixes_its_scale`
//! - the length a side carried is not lost to the cut, it is rehung on the
//!   corner — `a_length_a_side_carried_survives_the_cut_that_shortened_it`
//! - the angle the two sides stood at is not lost either, it is read between
//!   the stretches the cut took off —
//!   `an_angle_the_corner_carried_survives_the_cut_that_took_the_corner`
//! - dragging the end of a cut by hand does not change what was typed —
//!   `dragging_the_end_of_a_cut_leaves_the_distances_where_they_were`
//! - moving a side moves the cut with it and keeps its values —
//!   `moving_a_side_carries_the_corner_and_the_cut_with_it`
//! - editing one of the dimensions reshapes the chamfer —
//!   `editing_a_distance_moves_the_cut_along_the_side`
//! - erasing the corner point erases the dimensions that start from it, and
//!   the cut stays — `erasing_the_corner_takes_its_dimensions_and_leaves_the_cut`
//! - erasing a dimension erases only that dimension —
//!   `erasing_one_dimension_leaves_the_other_standing`
//! - the corner stays as a construction point, held on both sides — no test:
//!   the holds are laid in the sketch crate and asserted beside them there,
//!   for the chamfer and for the fillet alike
//! - a trait that ended on the corner stays attached to it — no test: the
//!   sketch crate already said so for the one case where the point survived,
//!   and the point is now kept in every case rather than only that one
//! - the three modes and the fillet lay the same geometry as today — no test:
//!   the two files that already assert it were left untouched and still pass
//! - a chamfer or a fillet is still one step in the history — no test: no
//!   operation was added, so the step is the same one it always was
//! - compacting the history carries the corner point and its dimensions —
//!   `a_compacted_chamfer_still_holds_the_distances_that_were_typed` and
//!   `a_compacted_chamfer_measures_the_same_part_as_the_one_it_came_from`
//! - the preview before laying is unchanged — no test: what a preview says it
//!   would lay still names only the cut and the curve, asserted where it lives

use cao_part::history::{Operation, PointRef};
use cao_part::{History, Outcome, PartState};
use cao_sketch::{
    Chamfer, Constraint, Corner, DimensionTarget, Element, PointId, SegmentId, WorkPlane,
};
use glam::DVec2;

const CORNER: DVec2 = DVec2::new(2.0, 1.0);
const EAST: SegmentId = SegmentId(0);
const NORTH: SegmentId = SegmentId(1);
const PIVOT: PointId = PointId(1);
const TOLERANCE: f64 = 1e-6;

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
            end: PointRef::New(CORNER + DVec2::new(10.0, 0.0)),
            construction: false,
        },
        Operation::AddSegment {
            sketch: 0,
            start: PointRef::Existing(PIVOT),
            end: PointRef::New(CORNER + DVec2::new(0.0, 10.0)),
            construction: false,
        },
    ]
}

fn replay(operations: &[Operation]) -> PartState {
    let mut state = PartState::default();
    for operation in operations {
        state.apply(operation);
    }
    state
}

fn chamfered(mode: Chamfer) -> PartState {
    let mut state = replay(&a_right_angle());
    state.apply(&Operation::Chamfer {
        sketch: 0,
        corners: vec![Corner::Between(EAST, NORTH)],
        mode,
    });
    state
}

/// The trait the chamfer laid across the corner: the only one whose two ends
/// are both new, so neither runs out to where a side used to end.
fn the_cut(state: &PartState) -> SegmentId {
    let sketch = &state.sketches[0];
    let far = |id: PointId| sketch.point(id).distance(CORNER) > 6.0;
    sketch
        .live_segments()
        .filter(|(_, segment)| !segment.construction)
        .find(|(_, segment)| !far(segment.start) && !far(segment.end))
        .map(|(id, _)| id)
        .expect("the cut laid across the corner")
}

/// What the drawing actually measures for a target, rather than the number
/// that was typed into it — the typed number never moves on its own, so an
/// assertion against it cannot fail.
fn measured(state: &PartState, target: DimensionTarget) -> f64 {
    let sketch = &state.sketches[0];
    match target {
        DimensionTarget::Distance { from, to } => sketch.point(from).distance(sketch.point(to)),
        other => panic!("only distances are measured back here, got {other:?}"),
    }
}

/// Every length the drawing holds, in millimetres, whatever trait carries it.
fn lengths(state: &PartState) -> Vec<f64> {
    state.sketches[0]
        .dimensions()
        .iter()
        .filter(|dimension| !dimension.is_angle())
        .map(|dimension| dimension.value)
        .collect()
}

fn angles(state: &PartState) -> Vec<f64> {
    state.sketches[0]
        .dimensions()
        .iter()
        .filter(|dimension| dimension.is_angle())
        .map(|dimension| dimension.value)
        .collect()
}

#[test]
fn equal_distances_are_written_along_each_side() {
    let state = chamfered(Chamfer::Equal(3.0));

    let mut written = lengths(&state);
    written.sort_by(f64::total_cmp);
    assert_eq!(written.len(), 2, "one distance per side, got {written:?}");
    assert!(
        written.iter().all(|value| (value - 3.0).abs() <= TOLERANCE),
        "both read the distance that was typed, got {written:?}"
    );
}

#[test]
fn two_distances_are_written_one_along_each_side() {
    let state = chamfered(Chamfer::Sided {
        first: 2.0,
        second: 6.0,
    });

    let mut written = lengths(&state);
    written.sort_by(f64::total_cmp);
    assert_eq!(written.len(), 2, "got {written:?}");
    assert!(
        (written[0] - 2.0).abs() <= TOLERANCE && (written[1] - 6.0).abs() <= TOLERANCE,
        "each side reads back the distance it was given, got {written:?}"
    );
}

#[test]
fn a_distance_and_an_angle_are_written_against_the_side_they_were_read_from() {
    let state = chamfered(Chamfer::Angled {
        along: 3.0,
        degrees: 60.0,
    });

    let written = lengths(&state);
    assert_eq!(
        written.len(),
        1,
        "only the first side was given a distance, got {written:?}"
    );
    assert!((written[0] - 3.0).abs() <= TOLERANCE, "got {written:?}");

    let turned = angles(&state);
    assert_eq!(turned.len(), 1, "got {turned:?}");
    assert!(
        (turned[0] - 60.0).abs() <= TOLERANCE,
        "the angle reads what was typed, measured from the stretch the cut took \
         away — got {turned:?}"
    );
}

#[test]
fn the_angle_the_two_sides_stand_at_is_never_written() {
    let state = chamfered(Chamfer::Equal(3.0));

    assert!(
        angles(&state).is_empty(),
        "nobody typed the corner's own opening, and freezing it would \
         over-constrain a drawing already dimensioned"
    );
}

#[test]
fn a_fillet_writes_the_radius_that_was_typed() {
    let mut state = replay(&a_right_angle());
    state.apply(&Operation::Fillet {
        sketch: 0,
        corners: vec![Corner::Between(EAST, NORTH)],
        radius: 3.0,
    });

    let sketch = &state.sketches[0];
    let (arc, _) = sketch.live_arcs().next().expect("the curve just laid");
    let held = sketch
        .dimension_of(DimensionTarget::ArcRadius(arc))
        .expect("the radius that was typed");
    assert!((held.value - 3.0).abs() <= TOLERANCE, "got {}", held.value);
    assert_eq!(
        sketch.dimensions().len(),
        1,
        "the tangencies hold the rest, and only what was typed is written"
    );
}

#[test]
fn dragging_the_end_of_a_cut_leaves_the_distances_where_they_were() {
    let mut state = chamfered(Chamfer::Equal(3.0));
    let sketch = &mut state.sketches[0];
    let cut = sketch
        .live_segments()
        .map(|(id, _)| id)
        .find(|id| sketch.segments()[id.0].start != PIVOT && !sketch.segments()[id.0].construction)
        .expect("a live trait");
    let moved = sketch.segments()[cut.0].start;

    sketch.move_point(moved, sketch.point(moved) + DVec2::new(2.0, 2.0));
    sketch.resolve(1.0);

    let mut written = lengths(&state);
    written.sort_by(f64::total_cmp);
    assert!(
        written.iter().all(|value| (value - 3.0).abs() <= TOLERANCE),
        "a hand on the cut does not retype what was typed, got {written:?}"
    );
    let sketch = &state.sketches[0];
    let reach = sketch
        .point(sketch.segments()[cut.0].start)
        .distance(sketch.point(PIVOT));
    assert!(
        (reach - 3.0).abs() <= 1e-3,
        "and the drawing settles back to it, measured from the corner wherever          the sides have carried it — got {reach}"
    );
}

#[test]
fn moving_a_side_carries_the_corner_and_the_cut_with_it() {
    let mut state = chamfered(Chamfer::Equal(3.0));
    let sketch = &mut state.sketches[0];
    let far_north = sketch
        .live_points()
        .map(|(id, _)| id)
        .find(|id| sketch.point(*id).distance(CORNER + DVec2::new(0.0, 10.0)) <= TOLERANCE)
        .expect("the far end of the side running north");

    sketch.move_point(far_north, CORNER + DVec2::new(4.0, 10.0));
    sketch.resolve(1.0);

    let sketch = &state.sketches[0];
    let corner = sketch.point(PIVOT);
    assert!(
        corner.distance(CORNER) > 1e-3,
        "the corner followed the side it is held on, got {corner:?}"
    );
    for dimension in sketch.dimensions() {
        let reach = measured(&state, dimension.target);
        assert!(
            (reach - 3.0).abs() <= 1e-3,
            "and the cut kept its values, got {reach} for {:?}",
            dimension.target
        );
    }
}

#[test]
fn editing_a_distance_moves_the_cut_along_the_side() {
    let mut state = chamfered(Chamfer::Equal(3.0));
    let target = state.sketches[0].dimensions()[0].target;

    state.sketches[0].set_dimension(target, 5.0, false);
    state.sketches[0].resolve(1.0);

    let sketch = &state.sketches[0];
    let DimensionTarget::Distance { from, to } = target else {
        panic!("a distance is read from the corner, got {target:?}");
    };
    let reach = sketch.point(from).distance(sketch.point(to));
    assert!(
        (reach - 5.0).abs() <= 1e-3,
        "the cut moved out along the side to where it was retyped, got {reach}"
    );
}

#[test]
fn erasing_the_corner_takes_its_dimensions_and_leaves_the_cut() {
    let mut state = chamfered(Chamfer::Equal(3.0));
    let cut = the_cut(&state);
    let across = state.sketches[0].endpoints(cut);

    state.sketches[0].erase(Element::Point(PIVOT));
    let sketch = &state.sketches[0];

    assert!(
        sketch.dimensions().is_empty(),
        "the distances were measured from the corner, and the corner is gone"
    );
    assert!(
        !sketch.is_erased_segment(cut) && sketch.endpoints(cut) == across,
        "the cut was never part of what held it"
    );
    assert!(
        !sketch
            .constraints()
            .iter()
            .any(|rule| matches!(rule, Constraint::OnSegment { point, .. } if *point == PIVOT)),
        "and nothing still claims to hold a point that is gone"
    );
}

#[test]
fn erasing_one_dimension_leaves_the_other_standing() {
    let mut state = chamfered(Chamfer::Sided {
        first: 2.0,
        second: 6.0,
    });
    let sketch = &mut state.sketches[0];
    let target = sketch.dimensions()[0].target;

    sketch.erase_dimension(target);

    let left = lengths(&state);
    assert_eq!(left.len(), 1, "only that one went, got {left:?}");
}

#[test]
fn a_length_a_side_carried_survives_the_cut_that_shortened_it() {
    let mut state = replay(&a_right_angle());
    let far_east = state.sketches[0].segments()[EAST.0].end;
    state.apply(&Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(EAST),
        value: 10.0,
        placement: None,
    });

    state.apply(&Operation::Chamfer {
        sketch: 0,
        corners: vec![Corner::Between(EAST, NORTH)],
        mode: Chamfer::Equal(3.0),
    });

    let carried = state.sketches[0]
        .dimension_of(
            DimensionTarget::Distance {
                from: PIVOT,
                to: far_east,
            }
            .normalised(),
        )
        .expect("the length the side was given, now read from the corner");
    assert!(
        (carried.value - 10.0).abs() <= TOLERANCE,
        "the cut took three units off that side, but the side is still ten \
         long from the corner out — got {}",
        carried.value
    );
}

#[test]
fn an_angle_the_corner_carried_survives_the_cut_that_took_the_corner() {
    let mut state = replay(&a_right_angle());
    state.apply(&Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Angle {
            first: EAST,
            second: NORTH,
        }
        .normalised(),
        value: 90.0,
        placement: None,
    });

    state.apply(&Operation::Chamfer {
        sketch: 0,
        corners: vec![Corner::Between(EAST, NORTH)],
        mode: Chamfer::Equal(3.0),
    });

    let turned = angles(&state);
    assert_eq!(
        turned.len(),
        1,
        "the corner is gone but what it was worth is not, got {turned:?}"
    );
    assert!(
        (turned[0] - 90.0).abs() <= TOLERANCE,
        "and it still reads what it read, got {turned:?}"
    );
}

#[test]
fn a_chamfer_is_the_first_value_a_part_is_given_and_fixes_its_scale() {
    let state = chamfered(Chamfer::Equal(3.0));

    assert!(
        state.has_scale(),
        "a distance in millimetres is what tells a drawing what it is worth, \
         and a chamfer's is as good as any other — without it the part carries \
         a dimension it cannot measure"
    );
}

#[test]
fn a_compacted_chamfer_measures_the_same_part_as_the_one_it_came_from() {
    let mut history = History::default();
    for operation in a_right_angle() {
        history.push(operation);
    }
    history.push(Operation::Chamfer {
        sketch: 0,
        corners: vec![Corner::Between(EAST, NORTH)],
        mode: Chamfer::Equal(3.0),
    });
    let live = PartState::rebuild(&history);

    let compacted = PartState::rebuild(&cao_part::compact(&history));

    assert_eq!(
        compacted.millimeters_per_unit, live.millimeters_per_unit,
        "compaction replays every dimension as an operation, so a part that \
         learned its scale from its chamfer must have learned it the first \
         time round too"
    );
}

#[test]
fn an_angled_chamfer_writes_both_the_values_it_was_given() {
    let state = chamfered(Chamfer::Angled {
        along: 3.0,
        degrees: 60.0,
    });

    assert_eq!(
        state.sketches[0].dimensions().len(),
        2,
        "the distance and the angle: the stretch the angle is read against is \
         laid for this mode and no other, and the two decisions are one"
    );
}

#[test]
fn a_compacted_chamfer_still_holds_the_distances_that_were_typed() {
    let mut history = History::default();
    for operation in a_right_angle() {
        history.push(operation);
    }
    history.push(Operation::Chamfer {
        sketch: 0,
        corners: vec![Corner::Between(EAST, NORTH)],
        mode: Chamfer::Equal(3.0),
    });

    let compacted = PartState::rebuild(&cao_part::compact(&history));

    let mut written = lengths(&compacted);
    written.sort_by(f64::total_cmp);
    assert_eq!(
        written.len(),
        2,
        "compaction drops the chamfer and re-emits what it left, dimensions          included — got {written:?}"
    );
    assert!(
        written.iter().all(|value| (value - 3.0).abs() <= TOLERANCE),
        "got {written:?}"
    );
    let sketch = &compacted.sketches[0];
    let corner = sketch
        .constraints()
        .iter()
        .filter(|rule| matches!(rule, Constraint::OnSegment { .. }))
        .count();
    assert_eq!(corner, 2, "and the corner is still held on both sides");
}

/// A square plate, four corners, drawn as one closed outline.
fn a_plate() -> Vec<Operation> {
    let mut drawn = vec![Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    }];
    let at = |x: f64, y: f64| DVec2::new(x, y);
    let round = [at(0.0, 0.0), at(20.0, 0.0), at(20.0, 20.0), at(0.0, 20.0)];
    for (rank, corner) in round.iter().enumerate() {
        let next = round[(rank + 1) % 4];
        drawn.push(Operation::AddSegment {
            sketch: 0,
            start: match rank {
                0 => PointRef::New(*corner),
                _ => PointRef::Existing(PointId(rank + 1)),
            },
            end: match rank {
                3 => PointRef::Existing(PointId(1)),
                _ => PointRef::New(next),
            },
            construction: false,
        });
    }
    drawn
}

/// The four corner points of that plate, in the order they were laid.
fn the_four_corners() -> Vec<Corner> {
    (1..=4).map(|rank| Corner::At(PointId(rank))).collect()
}

#[test]
fn every_corner_of_a_plate_is_rounded_by_the_one_gesture() {
    let mut state = replay(&a_plate());

    state.apply(&Operation::Fillet {
        sketch: 0,
        corners: the_four_corners(),
        radius: 3.0,
    });

    assert_eq!(
        state.sketches[0].live_arcs().count(),
        4,
        "the four corners of a plate share their traits: cutting one trims what \
         the next leans on, which is why a corner is recorded by its point"
    );
}

#[test]
fn the_whole_gesture_is_taken_back_by_one_undo() {
    let mut history = History::default();
    for operation in a_plate() {
        history.push(operation);
    }
    history.push(Operation::Fillet {
        sketch: 0,
        corners: the_four_corners(),
        radius: 3.0,
    });
    assert_eq!(
        PartState::rebuild(&history).sketches[0].live_arcs().count(),
        4
    );

    history.undo();

    assert_eq!(
        PartState::rebuild(&history).sketches[0].live_arcs().count(),
        0,
        "one gesture, one step: undo takes back what was done, not a quarter of it"
    );
}

#[test]
fn a_corner_too_tight_is_refused_and_the_others_are_rounded_anyway() {
    // A long sliver: the two ends are far too sharp for a curve of this reach,
    // the apex between them is nearly flat and takes one easily.
    let mut state = replay(&[
        Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
        },
        Operation::AddSegment {
            sketch: 0,
            start: PointRef::New(DVec2::new(0.0, 0.0)),
            end: PointRef::New(DVec2::new(30.0, 0.0)),
            construction: false,
        },
        Operation::AddSegment {
            sketch: 0,
            start: PointRef::Existing(PointId(2)),
            end: PointRef::New(DVec2::new(15.0, 1.0)),
            construction: false,
        },
        Operation::AddSegment {
            sketch: 0,
            start: PointRef::Existing(PointId(3)),
            end: PointRef::Existing(PointId(1)),
            construction: false,
        },
    ]);

    let said = state.apply(&Operation::Fillet {
        sketch: 0,
        corners: (1..=3).map(|rank| Corner::At(PointId(rank))).collect(),
        radius: 3.0,
    });

    assert_eq!(
        state.sketches[0].live_arcs().count(),
        1,
        "the flat apex took its curve"
    );
    assert_eq!(
        said,
        Some(Outcome::Cut {
            rules: 0,
            values: 0,
            refused: 2
        }),
        "the two sharp ends are counted and said, not silently dropped"
    );
}

#[test]
fn two_corners_sharing_a_trait_are_both_cut_by_the_one_gesture() {
    let mut state = replay(&a_plate());

    // The two ends of one side, each naming that side first. Cutting the
    // first replaces it, so the second names a trait that is already gone.
    let said = state.apply(&Operation::Chamfer {
        sketch: 0,
        corners: vec![
            Corner::Between(SegmentId(1), SegmentId(0)),
            Corner::Between(SegmentId(1), SegmentId(2)),
        ],
        mode: Chamfer::Sided {
            first: 2.0,
            second: 6.0,
        },
    });

    assert_eq!(
        said,
        Some(Outcome::Cut {
            rules: 0,
            values: 0,
            refused: 0
        }),
        "neither corner was turned away: what a trait became is followed \
         through the cut that replaced it"
    );
    let cuts = state.sketches[0]
        .live_segments()
        .filter(|(_, segment)| !segment.construction)
        .count();
    assert_eq!(cuts, 6, "four sides, each end of one of them cut back");
}
