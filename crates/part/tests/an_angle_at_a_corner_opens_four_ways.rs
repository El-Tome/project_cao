//! An angle between two traits, put down the way the dimension tool puts it
//! down: one trait clicked, the other clicked, then the place.
//!
//! Closes #485.
//! - at a corner, the dimension measures the quarter it is put down in: between
//!   the two traits, between one trait and the other's prolongation, or between
//!   the two prolongations — `at_a_corner_each_quarter_gives_its_angle`
//! - dragging a placed dimension moves its value, never which angle it
//!   measures — `a_corner_angle_dragged_into_another_quarter_still_measures_its_own`
//! - a value typed drives the traits on the side the dimension measures —
//!   `a_value_typed_on_a_trait_and_a_prolongation_turns_that_angle`
//! - trimming carries the angle over to the piece kept, its quarter with it;
//!   compaction keeps it too —
//!   `a_cut_and_a_compaction_carry_a_corner_quarter_over_to_what_is_kept`
//! - a corner carries one angle: the two traits clicked again open the one
//!   already there — `the_same_corner_clicked_again_gives_the_angle_already_there`
//! - every angle a tool lays by itself measures what it measures today — no
//!   test: the chamfer, the rectangle, the corner, a trait drawn at a typed
//!   angle and one drawn on from another (#561) all build theirs with
//!   `DimensionTarget::corner`, both arms along the traits, and their own tests
//!   pass as they were written
//!
//! The drawing of a prolongation is held in `crates/sketch/tests/annotation.rs`,
//! the preview in `crates/app/src/screens/viewport/render/preview/tests.rs`.

use cao_part::{Operation, PartDocument, PointRef};
use cao_sketch::{Along, DimensionTarget, PointId, SegmentId, WorkPlane};
use glam::DVec2;

/// The horizontal trait, from the left up to the corner.
const ALONG: SegmentId = SegmentId(0);
/// The slanted trait, from the corner up and to the right.
const SLANTED: SegmentId = SegmentId(1);

/// How far the cursor may be from a trait and still be on it.
const SNAP: f64 = 2.0;

/// A direct reading of the drawing, with no solver between.
const EXACT: f64 = 1e-6;

/// What the solver leaves of an angle it was asked for.
const SOLVED: f64 = 1e-3;

const LEFT: DVec2 = DVec2::new(0.0, 20.0);
const CORNER: DVec2 = DVec2::new(60.0, 20.0);
const UP_RIGHT: DVec2 = DVec2::new(100.0, 70.0);

/// 51.3°, the slanted trait against the horizontal's prolongation.
fn acute() -> f64 {
    50.0_f64.atan2(40.0).to_degrees()
}

fn obtuse() -> f64 {
    180.0 - acute()
}

/// The issue's corner: a trait coming in from the left, and one leaving it up
/// and to the right.
fn a_corner() -> PartDocument {
    let mut document = PartDocument::new("part", "2026-10-10T09:00:00Z".parse().expect("a date"));
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(LEFT),
        end: PointRef::New(CORNER),
        construction: false,
    });
    let corner = document.sketches()[0].segments()[ALONG.0].end;
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(corner),
        end: PointRef::New(UP_RIGHT),
        construction: false,
    });
    document
}

/// The horizontal trait clicked, the slanted one, then the place: what the
/// dimension tool lays, and the angle it reads.
fn put_down(document: &mut PartDocument, place: DVec2) -> (DimensionTarget, f64) {
    let sketch = &document.sketches()[0];
    let on_the_slanted = CORNER + (UP_RIGHT - CORNER) * 0.5;
    let chosen = sketch
        .refine(DimensionTarget::Length(ALONG), on_the_slanted, SNAP)
        .expect("a second trait makes an angle");
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

fn angles_on(document: &PartDocument) -> Vec<DimensionTarget> {
    document.sketches()[0]
        .dimensions()
        .iter()
        .map(|dimension| dimension.target)
        .collect()
}

fn arms_of(target: DimensionTarget) -> Option<(Along, Along)> {
    match target {
        DimensionTarget::Angle {
            first_along,
            second_along,
            ..
        } => Some((first_along, second_along)),
        _ => None,
    }
}

#[test]
fn at_a_corner_each_quarter_gives_its_angle() {
    for (offset, expected, arms, said) in [
        (
            DVec2::new(-20.0, 30.0),
            obtuse(),
            (Along::Trait, Along::Trait),
            "between the two traits",
        ),
        (
            DVec2::new(40.0, 20.0),
            acute(),
            (Along::Prolongation, Along::Trait),
            "the slanted trait and the horizontal's prolongation",
        ),
        (
            DVec2::new(20.0, -30.0),
            obtuse(),
            (Along::Prolongation, Along::Prolongation),
            "between the two prolongations",
        ),
        (
            DVec2::new(-30.0, -10.0),
            acute(),
            (Along::Trait, Along::Prolongation),
            "the horizontal trait and the slanted one's prolongation",
        ),
    ] {
        let mut document = a_corner();
        let (target, value) = put_down(&mut document, CORNER + offset);
        assert_reads(value, expected, said);
        assert_eq!(arms_of(target), Some(arms), "{said}");
    }
}

#[test]
fn a_corner_angle_dragged_into_another_quarter_still_measures_its_own() {
    let mut document = a_corner();
    let (target, _) = put_down(&mut document, CORNER + DVec2::new(40.0, 20.0));

    document.apply(Operation::MoveDimension {
        sketch: 0,
        target,
        offset: DVec2::new(-20.0, 30.0),
    });

    assert_eq!(angles_on(&document), vec![target]);
    let read = document.measured(0, target).expect("it reads");
    assert_reads(read, acute(), "dragged between the two traits");
}

#[test]
fn a_value_typed_on_a_trait_and_a_prolongation_turns_that_angle() {
    let mut document = a_corner();
    let (target, _) = put_down(&mut document, CORNER + DVec2::new(40.0, 20.0));

    document.apply(Operation::SetDimension {
        sketch: 0,
        target,
        value: 30.0.into(),
        placement: None,
    });

    let typed_on = document.measured(0, target).expect("it reads");
    assert!(
        (typed_on - 30.0).abs() < SOLVED,
        "the angle typed on: {typed_on}°"
    );
    let between = document.measured(0, DimensionTarget::corner(ALONG, SLANTED));
    let between = between.expect("it reads");
    assert!(
        (between - 150.0).abs() < SOLVED,
        "what it leaves the traits: {between}°"
    );
}

#[test]
fn a_cut_and_a_compaction_carry_a_corner_quarter_over_to_what_is_kept() {
    let mut document = a_corner();
    let (target, _) = put_down(&mut document, CORNER + DVec2::new(20.0, -30.0));
    document.apply(Operation::AddPoint {
        sketch: 0,
        position: (CORNER + UP_RIGHT) * 0.5,
        on: Vec::new(),
    });
    let (far, middle) = (PointId(3), PointId(4));

    document.apply(Operation::Trim {
        sketch: 0,
        segment: SLANTED,
        from: middle,
        to: far,
    });
    assert_carries_both_prolongations(&document, target, "cut");
    document.compact_history();
    assert_carries_both_prolongations(&document, target, "compacted");
}

fn assert_carries_both_prolongations(document: &PartDocument, was: DimensionTarget, said: &str) {
    let kept = angles_on(document);
    let [carried] = kept[..] else {
        panic!("{said}: one angle on what is kept, got {kept:?}");
    };
    assert_eq!(arms_of(carried), arms_of(was), "{said}: {carried:?}");
    let read = document.measured(0, carried).expect("it reads");
    assert_reads(read, obtuse(), said);
}

#[test]
fn the_same_corner_clicked_again_gives_the_angle_already_there() {
    let mut document = a_corner();
    let (first, _) = put_down(&mut document, CORNER + DVec2::new(40.0, 20.0));

    let (again, value) = put_down(&mut document, CORNER + DVec2::new(-20.0, 30.0));

    assert_eq!(again, first);
    assert_eq!(angles_on(&document), vec![first]);
    assert_reads(value, acute(), "clicked again between the two traits");
}
