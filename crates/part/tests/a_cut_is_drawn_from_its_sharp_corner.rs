//! A fillet or a chamfer is a property of a sharp corner: the corner stays two
//! whole traits meeting at a point, and what is drawn in its place is worked
//! out from the corner, its sides and the value every time the drawing
//! changes. Every gesture on a rounded corner is a gesture on a sharp one.
//!
//! This replaces what #316 asked: the values typed are no longer dimensions
//! of the drawing, the corner is no longer a construction point held on both
//! lines, and the sides are no longer trimmed. What #316 asked that still
//! stands — one gesture, one step, one value for every corner — is held here.
//!
//! Closes #617.
//! - a cut is a property of its corner, the two traits whole up to it, the
//!   curve or the cut computed from them —
//!   `a_rounded_corner_leaves_both_traits_whole_and_draws_its_curve`
//! - a corner is named by its point or by its two traits; at a point with
//!   three traits or more the tool asks for the two, and an X, a T and two
//!   traits that do not touch are refused —
//!   `a_corner_named_by_its_point_or_by_its_two_traits_is_cut_the_same`,
//!   `two_traits_that_do_not_touch_are_refused`. The tool asking for two
//!   traits is `corner_at`, unchanged, held in the sketch crate by
//!   `a_point_where_a_third_trait_runs_in_is_no_corner_to_take`; an X and a T
//!   share no end, which is the refusal tested here
//! - corners laid in one gesture share one value, and change together —
//!   `corners_laid_together_follow_one_value`
//! - the corner keeps every rule and dimension it had —
//!   `the_corner_keeps_the_rules_and_dimensions_it_had`
//! - the value follows a variable or a formula, and the arc grows as it is
//!   typed — `corners_laid_together_follow_one_value` for the variable; the
//!   preview is the tool laying the cut on a copy of the drawing, held in the
//!   sketch crate by
//!   `a_previewed_fillet_leaves_the_drawing_untouched_and_hands_back_the_cut_it_would_lay`
//! - a value too big is held back, a warning says why, and the field offers
//!   the largest value that fits; a second Enter applies it — no test: it is
//!   the tool's, held in `cao_app` by
//!   `a_value_too_big_is_held_back_and_the_field_offers_the_largest`, the
//!   share it offers by `the_tightest_side_says_how_much_of_the_values_fits`
//!   in the sketch crate
//! - two cuts meeting exactly are allowed, and so is a chamfer reaching the
//!   far end of a side — `a_rectangle_rounded_to_half_its_height_is_raised_as_an_obround`;
//!   the chamfer follows the same reading of a side, held in the sketch crate
//!   by `two_cuts_meeting_exactly_fit_and_leave_no_straight_piece_between_them`
//! - the « Construction » toggle, when the cut is laid, decides whether it is
//!   construction — `a_cut_laid_in_construction_leaves_the_outline_sharp`;
//!   the toggle read by the tool is `cao_app`'s, held by
//!   `a_cut_laid_while_construction_is_on_is_laid_in_construction`
//! - one Ctrl+Z takes the whole gesture back —
//!   `the_whole_gesture_is_taken_back_by_one_undo`
//! - an equal chamfer is one value, distances from the sharp corner, the
//!   angle from the first side clicked —
//!   `an_equal_chamfer_takes_one_value_from_both_sides`; the angle is held in
//!   the sketch crate by `an_angled_chamfer_measures_its_angle_from_the_first_side`
//! - the outline runs from tangent point to tangent point through the curve,
//!   and the tint, the areas and the extrusion follow it —
//!   `a_plate_rounded_all_round_is_raised_with_its_rounded_corners`
//! - the sharp corner shows its point and dashes, the value reads as a
//!   dimension on each corner — no test: the canvas draws them
//!   (`render/cuts.rs`), and nothing under the canvas asserts what is painted
//! - hovering a cut lights it, its corner and its group, and a click selects
//!   it whole — no test: the click is held in the sketch crate by
//!   `a_press_on_the_curve_of_a_fillet_takes_the_fillet`; the lighting is the
//!   canvas's, with no net
//! - the centre, the ends and the middle of an arc or a cut are snaps — no
//!   test: held in the sketch crate by
//!   `the_centre_of_a_fillet_is_a_place_to_start_from`,
//!   `the_ends_of_a_fillet_are_places_to_start_from` and
//!   `the_middle_of_a_fillet_is_a_place_to_start_from`
//! - the corner point drags like a sharp corner, and grabbing an arc drags
//!   the corner point — `dragging_a_rounded_corner_moves_it_as_a_sharp_corner`;
//!   the arc grabbed is held in the sketch crate by
//!   `a_press_on_the_curve_of_a_fillet_drags_its_corner`
//! - a side's free end turns the side about the corner, and a whole side
//!   moves as on a sharp outline —
//!   `dragging_the_free_end_of_a_rounded_side_turns_it_as_on_a_sharp_corner`,
//!   `moving_a_rounded_side_moves_it_as_on_a_sharp_outline`
//! - a cut stands in the angle under 180°, and changes side only through
//!   180° — no test: held in the sketch crate by
//!   `a_cut_stands_in_the_angle_under_half_a_turn` and
//!   `a_cut_changes_side_when_its_corner_is_taken_through_half_a_turn`
//! - a cut that no longer fits turns red without stopping the drag; the
//!   outline, the area and the extrusion take the sharp corner, and « Ajout
//!   de matière » still works — `a_cut_that_does_not_fit_raises_the_sharp_corner`;
//!   the red is the canvas's
//! - a rounded outline drags as fast as a sharp one — no test: a cut lays
//!   nothing the solver reads, held in the sketch crate by
//!   `a_rounded_corner_settles_as_its_sharp_corner_would`, which lands the
//!   very same points
//! - « Cote de trait » on a rounded side measures up to the sharp corner, and
//!   rules on the sides and on the corner work as on a sharp corner —
//!   `a_side_dimension_measures_up_to_the_sharp_corner`,
//!   `the_corner_keeps_the_rules_and_dimensions_it_had`
//! - « Ajuster », « Diviser », deleting, « Symétrie » and the patterns —
//!   `dividing_a_rounded_side_keeps_its_cut`,
//!   `erasing_a_rounded_side_takes_its_cut`,
//!   `a_rounded_corner_mirrored_is_rounded_on_its_copy`; a trim erases the
//!   piece it takes and lays what is left as a division does, which is how
//!   the sketch crate holds it, by
//!   `the_piece_holding_the_corner_erased_after_a_division_takes_the_cut_away`
//! - the cut names its corner by its point, and drags recorded after it
//!   replay without drifting; a cut that no longer fits on replay is red, and
//!   its step is built —
//!   `a_cut_named_by_its_point_replays_on_its_corner_after_the_drags_that_followed`,
//!   `a_variable_that_makes_a_cut_too_big_leaves_its_step_built`. The matter
//!   raised from an area keeps it through cuts that stop fitting or meet:
//!   `an_area_raised_from_a_rounded_plate_is_still_raised_as_an_obround`
//! - an arc raises an exact cylindrical face —
//!   `a_plate_rounded_all_round_is_raised_with_its_rounded_corners`, held to
//!   the exact kernel's arithmetic
//! - #565, #587, #590, #591 and #592 — no test: their criteria are read back
//!   against this issue in the pull request
//!
//! What #316 asked that still stands:
//! - a cut's values are typed lengths, so the first of them fixes what the
//!   part measures — `the_first_cut_a_part_is_given_fixes_its_scale`
//! - compacting the history keeps the cuts, a group still one group —
//!   `a_compacted_part_keeps_its_cuts_and_their_groups`

use std::f64::consts::PI;

use cao_part::history::{ExtrusionMode, Operation, PointRef};
use cao_part::{Formula, History, Outcome, PartDocument, PartState, VariableChange, VariableId};
use cao_sketch::{
    Area, Chamfer, ChosenAxis, Constraint, Corner, CornerCutId, CutShape, DimensionTarget, Element,
    PointId, SegmentId, Sketch, SketchAxis, WorkPlane,
};
use glam::DVec2;

/// How near the arithmetic an exact volume or area comes.
const EXACT: f64 = 1e-9;
/// How near two places must stand to be the same place after a solve.
const SAME: f64 = 1e-6;

fn near(made: f64, expected: f64, what: &str) {
    assert!(
        (made - expected).abs() <= EXACT * expected.abs().max(1.0),
        "{what}: {made} where the arithmetic gives {expected}",
    );
}

/// A sketch on XY with a rectangle `width` by `height` from the origin: its
/// sides in order, each sharing its end with the next.
fn a_plate(width: f64, height: f64) -> History {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    history.push(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(width, height)),
        construction: false,
    });
    history
}

/// The four corners of the plate, each named by the side before it and the
/// side after.
fn the_four_corners() -> Vec<Corner> {
    (0..4)
        .map(|side| Corner::Between(SegmentId(side), SegmentId((side + 1) % 4)))
        .collect()
}

fn round(corners: Vec<Corner>, radius: f64) -> Operation {
    Operation::Fillet {
        sketch: 0,
        corners,
        radius: radius.into(),
        construction: false,
    }
}

fn raise(history: &mut History, place: DVec2, distance: f64) {
    let areas: Vec<Area> = PartState::rebuild(history).areas_at(0, &[place]);
    history.push(Operation::Extrude {
        sketch: 0,
        areas,
        distance: distance.into(),
        mode: ExtrusionMode::Add,
    });
}

fn the_drawing(history: &History) -> Sketch {
    PartState::rebuild(history).sketches[0].clone()
}

fn the_one_area(sketch: &Sketch) -> f64 {
    let regions = sketch.regions();
    assert_eq!(regions.len(), 1, "one closed area");
    regions[0].area()
}

/// A corner standing on its own: one trait from (10, 10) to (60, 10), one up
/// to (10, 50), and the far end of the first held by nothing.
fn an_open_corner() -> History {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    history.push(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(10.0, 10.0)),
        end: PointRef::New(DVec2::new(60.0, 10.0)),
        construction: false,
    });
    history.push(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(PointId(1)),
        end: PointRef::New(DVec2::new(10.0, 50.0)),
        construction: false,
    });
    history
}

/// The open corner as it stands, and the same corner rounded.
fn sharp_and_rounded() -> (History, History) {
    let sharp = an_open_corner();
    let mut rounded = sharp.clone();
    rounded.push(round(
        vec![Corner::Between(SegmentId(0), SegmentId(1))],
        5.0,
    ));
    (sharp, rounded)
}

fn moved(history: &mut History, point: PointId, position: DVec2) {
    history.push(Operation::MovePoint {
        sketch: 0,
        point,
        position,
        merged_into: None,
        on: Vec::new(),
        let_go: false,
    });
}

fn same_points(one: &Sketch, other: &Sketch) {
    for (rank, (a, b)) in one.points().iter().zip(other.points()).enumerate() {
        assert!(a.distance(*b) < SAME, "point {rank}: {a} against {b}");
    }
}

#[test]
fn a_rounded_corner_leaves_both_traits_whole_and_draws_its_curve() {
    let mut history = a_plate(40.0, 20.0);
    let before = the_drawing(&history);

    history.push(round(
        vec![Corner::Between(SegmentId(0), SegmentId(1))],
        5.0,
    ));

    let after = the_drawing(&history);
    assert_eq!(after.live_arcs().count(), 0, "nothing is laid but the cut");
    assert_eq!(after.segments().len(), before.segments().len());
    for side in 0..4 {
        assert_eq!(
            after.endpoints(SegmentId(side)),
            before.endpoints(SegmentId(side))
        );
    }
    let drawn = after.drawn_cuts();
    assert_eq!(drawn.len(), 1);
    assert!(drawn[0].fits);
    for touch in drawn[0].touches {
        assert!(
            touch.distance(drawn[0].corner) > 5.0 - SAME,
            "the curve stands back from the corner: {:?}",
            drawn[0]
        );
    }
}

#[test]
fn a_corner_named_by_its_point_or_by_its_two_traits_is_cut_the_same() {
    let mut by_traits = a_plate(40.0, 20.0);
    let mut by_point = by_traits.clone();
    let corner = the_drawing(&by_traits)
        .shared_point(SegmentId(0), SegmentId(1))
        .expect("the two first sides meet");

    by_traits.push(round(
        vec![Corner::Between(SegmentId(0), SegmentId(1))],
        5.0,
    ));
    by_point.push(round(vec![Corner::At(corner)], 5.0));

    let curve = |history: &History| the_drawing(history).drawn_cuts()[0].curve;
    assert_eq!(curve(&by_traits), curve(&by_point));
}

#[test]
fn two_traits_that_do_not_touch_are_refused() {
    let mut state = PartState::rebuild(&a_plate(40.0, 20.0));

    let said = state.apply(&round(
        vec![Corner::Between(SegmentId(0), SegmentId(2))],
        5.0,
    ));

    assert_eq!(
        said,
        Some(Outcome::Cut {
            rules: 0,
            values: 0,
            refused: 1
        })
    );
    assert_eq!(state.sketches[0].live_corner_cuts().count(), 0);
}

#[test]
fn corners_laid_together_follow_one_value() {
    let mut document = PartDocument::new("Plaque", "2026-11-02T09:00:00Z".parse().expect("a date"));
    document.apply(Operation::Variable(VariableChange::Added {
        name: "arrondi".to_string(),
        formula: Formula::Number(5.0),
    }));
    for operation in a_plate(40.0, 20.0).applied_operations() {
        document.apply(operation.clone());
    }
    let radius = document
        .variables()
        .read("arrondi")
        .expect("a formula that reads");
    document.apply(Operation::Fillet {
        sketch: 0,
        corners: the_four_corners(),
        radius,
        construction: false,
    });

    document
        .change_variable(VariableChange::Edited {
            variable: VariableId(0),
            name: "arrondi".to_string(),
            formula: Formula::Number(3.0),
        })
        .expect("a smaller rounding fits");

    let sketch = &document.sketches()[0];
    let cuts: Vec<_> = sketch.live_corner_cuts().map(|(_, cut)| cut).collect();
    assert_eq!(cuts.len(), 4);
    assert!(
        cuts.iter().all(|cut| cut.group == cuts[0].group),
        "{cuts:?}"
    );
    assert!(
        cuts.iter()
            .all(|cut| cut.shape == CutShape::Fillet { radius: 3.0 }),
        "every corner follows the one value: {cuts:?}"
    );
}

#[test]
fn the_corner_keeps_the_rules_and_dimensions_it_had() {
    let mut history = a_plate(40.0, 20.0);
    history.push(Operation::Constrain {
        sketch: 0,
        constraint: Constraint::Perpendicular {
            first: SegmentId(0),
            second: SegmentId(1),
        },
    });
    history.push(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(SegmentId(0)),
        value: 40.0.into(),
        placement: None,
    });
    let before = the_drawing(&history);

    history.push(round(
        vec![Corner::Between(SegmentId(0), SegmentId(1))],
        5.0,
    ));

    let after = the_drawing(&history);
    assert_eq!(after.constraints(), before.constraints());
    assert_eq!(
        after
            .dimensions()
            .iter()
            .map(|value| value.target)
            .collect::<Vec<_>>(),
        before
            .dimensions()
            .iter()
            .map(|value| value.target)
            .collect::<Vec<_>>(),
    );
}

#[test]
fn a_rectangle_rounded_to_half_its_height_is_raised_as_an_obround() {
    let mut history = a_plate(40.0, 20.0);
    history.push(round(the_four_corners(), 10.0));

    assert!(
        the_drawing(&history)
            .drawn_cuts()
            .iter()
            .all(|cut| cut.fits)
    );
    near(
        the_one_area(&the_drawing(&history)),
        20.0 * 20.0 + PI * 100.0,
        "the obround's area",
    );
    raise(&mut history, DVec2::new(20.0, 10.0), 10.0);

    let state = PartState::rebuild(&history);
    near(
        state.body.volume(),
        (20.0 * 20.0 + PI * 100.0) * 10.0,
        "the obround raised",
    );
}

#[test]
fn a_cut_laid_in_construction_leaves_the_outline_sharp() {
    let mut history = a_plate(40.0, 20.0);
    history.push(Operation::Fillet {
        sketch: 0,
        corners: the_four_corners(),
        radius: 5.0.into(),
        construction: true,
    });

    let sketch = the_drawing(&history);
    assert!(sketch.drawn_cuts().iter().all(|cut| cut.construction));
    near(the_one_area(&sketch), 800.0, "the sharp plate");
}

#[test]
fn the_whole_gesture_is_taken_back_by_one_undo() {
    let mut history = a_plate(40.0, 20.0);
    history.push(round(the_four_corners(), 3.0));
    assert_eq!(the_drawing(&history).live_corner_cuts().count(), 4);

    history.undo();

    assert_eq!(
        the_drawing(&history).live_corner_cuts().count(),
        0,
        "one gesture, one step: undo takes back what was done, not a quarter of it"
    );
}

#[test]
fn an_equal_chamfer_takes_one_value_from_both_sides() {
    let mut history = a_plate(40.0, 20.0);
    history.push(Operation::Chamfer {
        sketch: 0,
        corners: vec![Corner::Between(SegmentId(0), SegmentId(1))],
        mode: Chamfer::Equal(3.0).into(),
        construction: false,
    });

    let sketch = the_drawing(&history);
    let (_, cut) = sketch.live_corner_cuts().next().expect("the corner is cut");
    assert_eq!(cut.shape, CutShape::Chamfer(Chamfer::Equal(3.0)));
    let drawn = sketch.drawn_cuts()[0];
    for touch in drawn.touches {
        near(touch.distance(drawn.corner), 3.0, "taken from each side");
    }
}

#[test]
fn a_plate_rounded_all_round_is_raised_with_its_rounded_corners() {
    let mut history = a_plate(40.0, 20.0);
    history.push(round(the_four_corners(), 5.0));
    let rounded = 40.0 * 20.0 - (4.0 - PI) * 25.0;
    near(
        the_one_area(&the_drawing(&history)),
        rounded,
        "the area tinted",
    );

    raise(&mut history, DVec2::new(20.0, 10.0), 10.0);

    let state = PartState::rebuild(&history);
    near(state.body.volume(), rounded * 10.0, "the plate raised");
}

#[test]
fn dragging_a_rounded_corner_moves_it_as_a_sharp_corner() {
    let (mut sharp, mut rounded) = sharp_and_rounded();
    for history in [&mut sharp, &mut rounded] {
        moved(history, PointId(1), DVec2::new(20.0, 0.0));
    }

    same_points(&the_drawing(&sharp), &the_drawing(&rounded));
    assert!(the_drawing(&rounded).drawn_cuts()[0].fits);
}

#[test]
fn dragging_the_free_end_of_a_rounded_side_turns_it_as_on_a_sharp_corner() {
    let (mut sharp, mut rounded) = sharp_and_rounded();
    for history in [&mut sharp, &mut rounded] {
        moved(history, PointId(2), DVec2::new(50.0, 40.0));
    }

    same_points(&the_drawing(&sharp), &the_drawing(&rounded));
}

#[test]
fn moving_a_rounded_side_moves_it_as_on_a_sharp_outline() {
    let (mut sharp, mut rounded) = sharp_and_rounded();
    for history in [&mut sharp, &mut rounded] {
        history.push(Operation::MoveSegment {
            sketch: 0,
            segment: SegmentId(1),
            by: DVec2::new(-4.0, 0.0),
        });
    }

    same_points(&the_drawing(&sharp), &the_drawing(&rounded));
}

#[test]
fn a_cut_that_does_not_fit_raises_the_sharp_corner() {
    let mut history = a_plate(40.0, 20.0);
    history.push(round(the_four_corners(), 15.0));
    assert!(
        the_drawing(&history)
            .drawn_cuts()
            .iter()
            .all(|cut| !cut.fits),
        "two fillets of fifteen do not fit a side of twenty"
    );

    raise(&mut history, DVec2::new(20.0, 10.0), 10.0);

    let state = PartState::rebuild(&history);
    near(state.body.volume(), 800.0 * 10.0, "the sharp plate raised");
}

#[test]
fn a_side_dimension_measures_up_to_the_sharp_corner() {
    let mut history = a_plate(40.0, 20.0);
    history.push(round(the_four_corners(), 5.0));

    history.push(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(SegmentId(0)),
        value: 50.0.into(),
        placement: None,
    });

    let sketch = the_drawing(&history);
    let (start, end) = sketch.endpoints(SegmentId(0));
    near(start.distance(end), 50.0, "the whole side");
    assert!(sketch.drawn_cuts().iter().all(|cut| cut.fits));
}

#[test]
fn dividing_a_rounded_side_keeps_its_cut() {
    let mut history = a_plate(40.0, 20.0);
    history.push(round(
        vec![Corner::Between(SegmentId(0), SegmentId(1))],
        5.0,
    ));
    let before = the_drawing(&history).drawn_cuts();
    let (start, end) = the_drawing(&history).endpoints(SegmentId(0));

    history.push(Operation::Split {
        sketch: 0,
        segments: vec![SegmentId(0)],
        arcs: Vec::new(),
        at: (start + end) / 2.0,
    });

    let after = the_drawing(&history).drawn_cuts();
    assert_eq!(after.len(), 1);
    assert_eq!(after[0].curve, before[0].curve);
}

#[test]
fn erasing_a_rounded_side_takes_its_cut() {
    let mut history = a_plate(40.0, 20.0);
    history.push(round(
        vec![Corner::Between(SegmentId(0), SegmentId(1))],
        5.0,
    ));

    history.push(Operation::EraseMany {
        sketch: 0,
        elements: vec![Element::Segment(SegmentId(1))],
        dimensions: Vec::new(),
        constraints: Vec::new(),
    });

    assert_eq!(the_drawing(&history).live_corner_cuts().count(), 0);
}

#[test]
fn a_rounded_corner_mirrored_is_rounded_on_its_copy() {
    let (_, mut rounded) = sharp_and_rounded();

    rounded.push(Operation::Mirror {
        sketch: 0,
        elements: vec![
            Element::Segment(SegmentId(0)),
            Element::Segment(SegmentId(1)),
        ],
        axis: ChosenAxis::Sketch(SketchAxis::V),
    });

    let sketch = the_drawing(&rounded);
    let drawn = sketch.drawn_cuts();
    assert_eq!(drawn.len(), 2);
    near(
        drawn[1].corner.x,
        -drawn[0].corner.x,
        "the copy stands across the axis",
    );
    assert_ne!(
        sketch.corner_cuts()[1].group,
        sketch.corner_cuts()[0].group,
        "the copy changes on its own"
    );
}

#[test]
fn a_cut_named_by_its_point_replays_on_its_corner_after_the_drags_that_followed() {
    let (mut sharp, _) = sharp_and_rounded();
    let mut rounded = an_open_corner();
    rounded.push(round(vec![Corner::At(PointId(1))], 5.0));
    for history in [&mut sharp, &mut rounded] {
        moved(history, PointId(2), DVec2::new(70.0, 30.0));
        moved(history, PointId(1), DVec2::new(15.0, 5.0));
    }

    let drawing = the_drawing(&rounded);
    same_points(&the_drawing(&sharp), &drawing);
    assert_eq!(drawing.cuts_standing_at(PointId(1)), vec![CornerCutId(0)]);
    let drawn = drawing.drawn_cuts();
    assert!(drawn[0].fits);
    assert!(
        drawn[0].corner.distance(drawing.point(PointId(1))) < SAME,
        "the cut stands on its corner where the drags left it"
    );
}

/// A plate forty by twenty rounded by `arrondi`, raised ten from the area
/// clicked once it is rounded.
fn a_raised_rounded_plate() -> PartDocument {
    let mut document = PartDocument::new("Plaque", "2026-11-02T09:00:00Z".parse().expect("a date"));
    document.apply(Operation::Variable(VariableChange::Added {
        name: "arrondi".to_string(),
        formula: Formula::Number(5.0),
    }));
    for operation in a_plate(40.0, 20.0).applied_operations() {
        document.apply(operation.clone());
    }
    let radius = document
        .variables()
        .read("arrondi")
        .expect("a formula that reads");
    document.apply(Operation::Fillet {
        sketch: 0,
        corners: the_four_corners(),
        radius,
        construction: false,
    });
    let areas = document.areas_at(0, &[DVec2::new(20.0, 10.0)]);
    document.apply(Operation::Extrude {
        sketch: 0,
        areas,
        distance: 10.0.into(),
        mode: ExtrusionMode::Add,
    });
    document
}

fn rounded_to(document: &mut PartDocument, radius: f64) {
    document
        .change_variable(VariableChange::Edited {
            variable: VariableId(0),
            name: "arrondi".to_string(),
            formula: Formula::Number(radius),
        })
        .expect("nothing breaks: a cut never renumbers, and the area keeps its name");
}

#[test]
fn a_variable_that_makes_a_cut_too_big_leaves_its_step_built() {
    let mut document = a_raised_rounded_plate();
    near(
        document.body().volume(),
        (800.0 - (4.0 - PI) * 25.0) * 10.0,
        "the rounded plate raised",
    );

    rounded_to(&mut document, 15.0);

    assert!(
        document.sketches()[0]
            .drawn_cuts()
            .iter()
            .all(|cut| !cut.fits)
    );
    near(
        document.body().volume(),
        800.0 * 10.0,
        "the area raised from the rounded plate is still raised, its corners sharp",
    );
}

#[test]
fn an_area_raised_from_a_rounded_plate_is_still_raised_as_an_obround() {
    let mut document = a_raised_rounded_plate();

    rounded_to(&mut document, 10.0);

    assert!(
        document.sketches()[0]
            .drawn_cuts()
            .iter()
            .all(|cut| cut.fits)
    );
    near(
        document.body().volume(),
        (20.0 * 20.0 + PI * 100.0) * 10.0,
        "the short sides are gone between the arcs, and the area keeps its name",
    );
}

#[test]
fn the_first_cut_a_part_is_given_fixes_its_scale() {
    let mut state = PartState::rebuild(&an_open_corner());
    assert!(!state.has_scale());

    state.apply(&Operation::Chamfer {
        sketch: 0,
        corners: vec![Corner::Between(SegmentId(0), SegmentId(1))],
        mode: Chamfer::Equal(3.0).into(),
        construction: false,
    });

    assert!(state.has_scale());
}

#[test]
fn a_compacted_part_keeps_its_cuts_and_their_groups() {
    let mut history = a_plate(40.0, 20.0);
    history.push(round(
        vec![
            Corner::Between(SegmentId(0), SegmentId(1)),
            Corner::Between(SegmentId(1), SegmentId(2)),
        ],
        5.0,
    ));
    history.push(Operation::Chamfer {
        sketch: 0,
        corners: vec![Corner::Between(SegmentId(2), SegmentId(3))],
        mode: Chamfer::Equal(2.0).into(),
        construction: false,
    });

    let compacted = PartState::rebuild(&cao_part::compact(&history));
    let live = PartState::rebuild(&history);

    let groups = |state: &PartState| {
        let mut groups: Vec<usize> = state.sketches[0]
            .live_corner_cuts()
            .map(|(_, cut)| cut.group)
            .collect();
        groups.dedup();
        groups.len()
    };
    assert_eq!(compacted.sketches[0].drawn_cuts().len(), 3);
    assert_eq!(groups(&compacted), groups(&live));
    near(
        the_one_area(&compacted.sketches[0]),
        the_one_area(&live.sketches[0]),
        "the same rounded and chamfered plate",
    );
}
