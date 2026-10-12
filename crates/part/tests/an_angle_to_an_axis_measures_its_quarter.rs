//! An angle between a trait and a sketch axis, put down the way the dimension
//! tool puts it down: the trait clicked, the axis clicked, then the place.
//!
//! Closes #484.
//! - a trait dimensioned against the vertical axis measures the quarter the
//!   dimension is put down in, around the middle of the trait —
//!   `against_the_vertical_axis_each_quarter_about_the_middle_gives_its_angle`
//! - the same against the horizontal axis, with a horizontal line through the
//!   middle — `against_the_horizontal_axis_each_quarter_gives_its_angle_too`
//! - a trait drawn either way round gives the same angle for the same place —
//!   `a_trait_drawn_the_other_way_round_gives_the_same_angle_for_the_same_place`
//! - the preview before the click shows the angle the click lays — no test:
//!   held where the preview is drawn, in
//!   `crates/app/src/screens/viewport/render/preview/tests.rs`
//! - the arc is drawn around the middle of the trait, between the line along
//!   the axis and the half of the trait on the cursor's side — no test: held
//!   where annotations are drawn, in `crates/sketch/tests/annotation.rs`
//! - dragging a placed dimension never changes which angle it measures —
//!   `a_dimension_dragged_into_another_quarter_still_measures_its_own`
//! - a value typed drives the trait on the side the dimension measures —
//!   `a_value_typed_on_the_obtuse_angle_makes_that_one_what_was_typed`
//! - trimming carries the dimension over to the piece kept, its quarter with
//!   it, and compaction keeps it too —
//!   `a_cut_and_a_compaction_carry_the_quarter_over_to_what_is_kept`
//! - a trait whose middle lies on the axis draws no line along it — no test:
//!   held in `crates/sketch/tests/annotation.rs` too
//! - the redundancy check reads the value measured —
//!   `an_angle_nothing_is_left_to_set_is_a_readout_of_its_own_quarter`
//! - `AngleBetween` and its tests are unchanged — no test: its tests were
//!   left word for word, and pass
//!
//! Decided while writing the criteria, with the owner:
//! - the same trait and the same axis clicked again open the dimension already
//!   there — `the_same_trait_and_axis_clicked_again_give_the_angle_already_there`
//! - a trait lying along the axis cuts no quarters and measures 0°, whichever
//!   way it was drawn — `a_trait_lying_along_the_axis_measures_nothing_whichever_way_it_was_drawn`

use cao_part::{DimensionOutcome, Operation, Outcome, PartDocument, PointRef};
use cao_sketch::{AxisToward, DimensionTarget, PointId, SegmentId, SketchAxis, Toward, WorkPlane};
use glam::DVec2;

const TRAIT: SegmentId = SegmentId(0);

/// How far the cursor may be from an axis and still be on it.
const SNAP: f64 = 2.0;

/// A direct reading of the drawing, with no solver between.
const EXACT: f64 = 1e-6;

/// What the solver leaves of an angle it was asked for.
const SOLVED: f64 = 1e-3;

/// The trait of the issue: rising to the right, 11.3° off the horizontal, its
/// middle at (70, 40).
const LEFT: DVec2 = DVec2::new(20.0, 30.0);
const RIGHT: DVec2 = DVec2::new(120.0, 50.0);

const ACUTE: f64 = 78.690_067_525_979_8;
const OBTUSE: f64 = 101.309_932_474_020_2;
const SLIGHT: f64 = OBTUSE - 90.0;
const WIDE: f64 = 180.0 - SLIGHT;

/// Up and left of the middle, and so on round: the four quarters the
/// vertical through the middle cuts with the trait.
const QUARTERS: [(DVec2, f64, &str); 4] = [
    (DVec2::new(40.0, 80.0), OBTUSE, "up and left"),
    (DVec2::new(100.0, 80.0), ACUTE, "up and right"),
    (DVec2::new(40.0, 0.0), ACUTE, "down and left"),
    (DVec2::new(100.0, 0.0), OBTUSE, "down and right"),
];

fn a_trait_from(start: DVec2, end: DVec2) -> PartDocument {
    let mut document = PartDocument::new("part", "2026-10-10T09:00:00Z".parse().expect("a date"));
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(start),
        end: PointRef::New(end),
        construction: false,
    });
    document
}

/// Where a click lands on an axis, well away from the trait.
fn on(axis: SketchAxis) -> DVec2 {
    match axis {
        SketchAxis::U => DVec2::new(-150.0, 0.0),
        SketchAxis::V => DVec2::new(0.0, 150.0),
    }
}

/// The trait clicked, then the axis, then the place: what the dimension tool
/// lays, and the angle it reads.
fn put_down(document: &mut PartDocument, axis: SketchAxis, place: DVec2) -> (DimensionTarget, f64) {
    let sketch = &document.sketches()[0];
    let chosen = sketch
        .refine(DimensionTarget::Length(TRAIT), on(axis), SNAP)
        .expect("an axis after a trait makes an angle");
    let target = sketch.oriented(chosen, place);
    if sketch.dimension_of(target).is_none() {
        let value = document.measured(0, target).expect("the angle reads");
        document.apply(Operation::SetDimension {
            sketch: 0,
            target,
            value: value.into(),
            placement: None,
        });
    }
    let laid = document.sketches()[0]
        .dimension_of(target)
        .expect("the angle is on the drawing");
    (target, laid.value)
}

fn assert_reads(value: f64, expected: f64, said: &str) {
    assert!(
        (value - expected).abs() < EXACT,
        "{said}: expected {expected}°, read {value}°",
    );
}

#[test]
fn against_the_vertical_axis_each_quarter_about_the_middle_gives_its_angle() {
    for (place, expected, said) in QUARTERS {
        let mut document = a_trait_from(LEFT, RIGHT);
        let (_, value) = put_down(&mut document, SketchAxis::V, place);
        assert_reads(value, expected, said);
    }
}

#[test]
fn against_the_horizontal_axis_each_quarter_gives_its_angle_too() {
    for (place, expected, said) in [
        (
            DVec2::new(130.0, 45.0),
            SLIGHT,
            "right, between the horizontal and the trait",
        ),
        (DVec2::new(70.0, 90.0), WIDE, "above"),
        (
            DVec2::new(10.0, 35.0),
            SLIGHT,
            "left, between the horizontal and the trait",
        ),
        (DVec2::new(70.0, 0.0), WIDE, "below"),
    ] {
        let mut document = a_trait_from(LEFT, RIGHT);
        let (_, value) = put_down(&mut document, SketchAxis::U, place);
        assert_reads(value, expected, said);
    }
}

#[test]
fn a_trait_drawn_the_other_way_round_gives_the_same_angle_for_the_same_place() {
    for (place, expected, said) in QUARTERS {
        let mut document = a_trait_from(RIGHT, LEFT);
        let (_, value) = put_down(&mut document, SketchAxis::V, place);
        assert_reads(value, expected, said);
    }
}

fn angles_on(document: &PartDocument) -> Vec<DimensionTarget> {
    document.sketches()[0]
        .dimensions()
        .iter()
        .map(|dimension| dimension.target)
        .collect()
}

#[test]
fn a_dimension_dragged_into_another_quarter_still_measures_its_own() {
    let mut document = a_trait_from(LEFT, RIGHT);
    let (target, _) = put_down(&mut document, SketchAxis::V, QUARTERS[0].0);

    document.apply(Operation::MoveDimension {
        sketch: 0,
        target,
        offset: DVec2::new(30.0, -40.0),
    });

    assert_eq!(angles_on(&document), vec![target]);
    let laid = document.sketches()[0]
        .dimension_of(target)
        .expect("still there");
    assert_reads(laid.value, OBTUSE, "dragged down and right");
    let read = document.measured(0, target).expect("it reads");
    assert_reads(read, OBTUSE, "read again where it was dragged");
}

#[test]
fn a_value_typed_on_the_obtuse_angle_makes_that_one_what_was_typed() {
    let mut document = a_trait_from(LEFT, RIGHT);
    let (target, _) = put_down(&mut document, SketchAxis::V, QUARTERS[0].0);

    document.apply(Operation::SetDimension {
        sketch: 0,
        target,
        value: 120.0.into(),
        placement: None,
    });

    let obtuse = document.measured(0, target).expect("it reads");
    assert!(
        (obtuse - 120.0).abs() < SOLVED,
        "the angle typed on: {obtuse}°"
    );
    let up_and_right = DimensionTarget::AxisAngle {
        segment: TRAIT,
        segment_toward: Toward::End,
        axis: SketchAxis::V,
        axis_toward: AxisToward::Positive,
    };
    let acute = document.measured(0, up_and_right).expect("it reads");
    assert!(
        (acute - 60.0).abs() < SOLVED,
        "what it leaves the other side: {acute}°"
    );
}

#[test]
fn a_cut_and_a_compaction_carry_the_quarter_over_to_what_is_kept() {
    let mut document = a_trait_from(LEFT, RIGHT);
    let (target, _) = put_down(&mut document, SketchAxis::V, QUARTERS[0].0);
    document.apply(Operation::AddPoint {
        sketch: 0,
        position: (LEFT + RIGHT) * 0.5,
        on: Vec::new(),
    });
    let (left, middle) = (PointId(1), PointId(3));

    document.apply(Operation::Trim {
        sketch: 0,
        segment: TRAIT,
        from: left,
        to: middle,
    });
    assert_carries_the_obtuse_quarter(&document, target, "cut");
    document.compact_history();
    assert_carries_the_obtuse_quarter(&document, target, "compacted");
}

fn arms_of(target: DimensionTarget) -> Option<(Toward, AxisToward)> {
    match target {
        DimensionTarget::AxisAngle {
            segment_toward,
            axis_toward,
            ..
        } => Some((segment_toward, axis_toward)),
        _ => None,
    }
}

fn assert_carries_the_obtuse_quarter(document: &PartDocument, was: DimensionTarget, said: &str) {
    let kept = angles_on(document);
    let [carried] = kept[..] else {
        panic!("{said}: one angle on what is kept, got {kept:?}");
    };
    assert_eq!(arms_of(carried), arms_of(was), "{said}: {carried:?}");
    let read = document.measured(0, carried).expect("it reads");
    assert_reads(read, OBTUSE, said);
}

#[test]
fn an_angle_nothing_is_left_to_set_is_a_readout_of_its_own_quarter() {
    let mut document = a_trait_from(LEFT, RIGHT);
    put_down(&mut document, SketchAxis::U, DVec2::new(130.0, 45.0));
    let sketch = &document.sketches()[0];
    let target = sketch.oriented(
        sketch
            .refine(DimensionTarget::Length(TRAIT), on(SketchAxis::V), SNAP)
            .expect("an axis after a trait makes an angle"),
        QUARTERS[0].0,
    );

    let outcome = document.apply(Operation::SetDimension {
        sketch: 0,
        target,
        value: 45.0.into(),
        placement: None,
    });

    assert_eq!(
        outcome,
        Some(Outcome::Dimension(DimensionOutcome::Reference)),
        "the angle to the horizontal already holds the trait's direction",
    );
    let laid = document.sketches()[0].dimension_of(target).expect("laid");
    assert!(laid.driven, "nothing is left for it to set");
    assert_reads(laid.value, OBTUSE, "the readout of the quarter up and left");
}

#[test]
fn the_same_trait_and_axis_clicked_again_give_the_angle_already_there() {
    let mut document = a_trait_from(LEFT, RIGHT);
    let (first, _) = put_down(&mut document, SketchAxis::V, QUARTERS[0].0);

    let (again, value) = put_down(&mut document, SketchAxis::V, QUARTERS[1].0);

    assert_eq!(again, first);
    assert_eq!(angles_on(&document), vec![first]);
    assert_reads(value, OBTUSE, "put down again up and right");
}

#[test]
fn a_trait_lying_along_the_axis_measures_nothing_whichever_way_it_was_drawn() {
    let (west, east) = (DVec2::new(20.0, 40.0), DVec2::new(120.0, 40.0));
    for (start, end, said) in [
        (west, east, "drawn eastwards"),
        (east, west, "drawn westwards"),
    ] {
        for place in [DVec2::new(30.0, 80.0), DVec2::new(110.0, 0.0)] {
            let mut document = a_trait_from(start, end);
            let (_, value) = put_down(&mut document, SketchAxis::U, place);
            assert_reads(value, 0.0, said);
        }
    }
}
