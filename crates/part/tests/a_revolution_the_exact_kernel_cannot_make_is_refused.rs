//! Closes #632.
//! - a profile with an arc turned about an axis leaning 0.1° is declined by
//!   the application's body, not turned — no test: #633 lays that section
//!   since, raised in `cao_solid` by
//!   `a_grooved_ring_about_an_axis_leaning_a_tenth_of_a_degree_or_1e_5_is_raised_exactly`
//!   in `a_groove_turned_about_a_leaning_axis.rs`; the rule, a profile the
//!   reading cannot lay is a broken step, is held through the part on that
//!   section drawn as a pulley about an axis leaning 1e-5°, which the reading
//!   still declines, by
//!   `a_pulley_whose_groove_the_reading_cannot_lay_is_a_broken_step`
//! - a turned ellipse is declined by the application's body —
//!   `a_turned_ellipse_is_a_broken_step`
//! - seed 5577103202's case through the application's body is refused at its
//!   first step, in about a second — no test: #633 raises that step since,
//!   and the case keeps every rule through the body in about a second, held
//!   in `cao_solid` by
//!   `seed_5577103202_leaning_a_tenth_of_a_degree_keeps_every_rule_through_the_body_as_fast_as_straight`
//!   in `a_groove_turned_about_a_leaning_axis.rs`
//! - a part saved with such a revolution opens, that step « non construite » —
//!   no test: held beside the cache by
//!   `a_part_saved_with_a_turned_ellipse_opens_with_that_step_declined`, on
//!   its design and on its cache, since only there are the files a part is
//!   saved to held in memory; « non construite » is what the part tree says
//!   of a step `is_declined` names
//! - the docs no longer say the flats turn a profile (`docs/exact-kernel.md`,
//!   the campaigns' notes); `REBUILT_BY` moves if a saved part rebuilds
//!   differently — no test: `REBUILT_BY` moves, held beside the cache by
//!   `a_cache_written_while_the_flats_still_turned_is_replayed`, since only
//!   there can a test tell a part opened on its cache from one replayed; the
//!   docs are prose, held by `language.rs` only
//!
//! A revolution the exact kernel cannot make, as the application builds it:
//! a broken step with its reason, the part as it stood before it and still
//! exact, and the steps after it built exactly. The flats turned it until
//! #632, and every boolean after that was theirs.
//!
//! Since #633 the pulley is built exactly about an axis leaning up to some
//! 3e-6° and from some 1.5e-5° on, as about the sketch's own axis, and
//! declined between: there its walls, fifteen long, are laid square, and
//! its groove's two ends, sixty apart, stand on two levels, so that no
//! circle is tangent to both walls.

use std::f64::consts::{FRAC_PI_2, PI};

use cao_part::history::{ExtrusionMode, Operation, PointRef, RevolutionAxis};
use cao_part::{History, PartDocument, PartState};
use cao_sketch::{Area, PointId, SegmentId, SketchAxis, WorkPlane};
use cao_solid::Declined;
use glam::DVec2;

const EXACT: f64 = 1e-9;

const ABOUT_V: RevolutionAxis = RevolutionAxis::Sketch(SketchAxis::V);

/// A block 20 by 20 by 10 about the origin, far from every turn below.
const BLOCK: f64 = 20.0 * 20.0 * 10.0;

/// A disc of 5 raised 10.
const POST: f64 = PI * 25.0 * 10.0;

/// The pulley's ring, inside its groove's end.
const IN_THE_PULLEY: DVec2 = DVec2::new(60.0, -85.0);

fn clicked(history: &History, sketch: usize, places: &[DVec2]) -> Vec<Area> {
    PartState::rebuild(history).areas_at(sketch, places)
}

fn sketch_on(history: &mut History, plane: WorkPlane) -> usize {
    let sketch = PartState::rebuild(history).sketches.len();
    history.push(Operation::CreateSketch { plane, on: None });
    sketch
}

fn next_point(history: &History, sketch: usize) -> usize {
    PartState::rebuild(history).sketches[sketch].points().len()
}

fn segment(history: &mut History, sketch: usize, start: PointRef, end: PointRef) {
    history.push(Operation::AddSegment {
        sketch,
        start,
        end,
        construction: false,
    });
}

fn raise(history: &mut History, sketch: usize, place: DVec2, distance: f64) {
    history.push(Operation::Extrude {
        sketch,
        areas: clicked(history, sketch, &[place]),
        distance: distance.into(),
        mode: ExtrusionMode::Add,
    });
}

fn turn(history: &mut History, sketch: usize, place: DVec2, axis: RevolutionAxis, degrees: f64) {
    history.push(Operation::Revolve {
        sketch,
        areas: clicked(history, sketch, &[place]),
        axis,
        angle: degrees.into(),
        mode: ExtrusionMode::Add,
    });
}

/// The number the history gave its last step.
fn last_step(history: &History) -> u32 {
    history.steps().last().expect("a step").operations()[0]
}

fn applied_live(history: &History) -> PartDocument {
    let now = "2026-11-02T09:00:00Z".parse().expect("a date");
    let mut part = PartDocument::new("Refused", now);
    for operation in history.operations() {
        part.apply(operation.clone());
    }
    part
}

fn assert_near(made: f64, expected: f64, what: &str) {
    assert!(
        (made - expected).abs() <= EXACT * expected,
        "{what}: {made} where the arithmetic gives {expected}",
    );
}

/// The block, raised from a sketch of its own on XY.
fn a_block(history: &mut History) {
    let sketch = sketch_on(history, WorkPlane::XY);
    history.push(Operation::AddRectangle {
        sketch,
        corner: PointRef::New(DVec2::splat(-10.0)),
        opposite: PointRef::New(DVec2::splat(10.0)),
        construction: false,
    });
    raise(history, sketch, DVec2::ZERO, 10.0);
}

/// The post, about (0, -40) on a sketch of its own on XY: a step after the
/// turn, clear of everything.
fn a_post(history: &mut History) {
    let sketch = sketch_on(history, WorkPlane::XY);
    let center = DVec2::new(0.0, -40.0);
    history.push(Operation::AddCircle {
        sketch,
        center: PointRef::New(center),
        radius: 5.0,
        rim: Vec::new(),
        construction: false,
    });
    raise(history, sketch, center, 10.0);
}

/// A ring from 30 to 90 off the sketch's V axis, 105 long, a groove cut
/// into its outer side between 15 and 75 from its end, its sides 15 deep
/// and square to the axis, its bottom a half round of 30 tangent to them:
/// seed 5577103202's first section, on a sketch of its own on XZ.
fn a_pulley(history: &mut History) -> usize {
    let sketch = sketch_on(history, WorkPlane::XZ);
    let first = next_point(history, sketch);
    let at = DVec2::new;
    let corner = |n: usize| PointRef::Existing(PointId(first + n));
    segment(
        history,
        sketch,
        PointRef::New(at(30.0, -90.0)),
        PointRef::New(at(90.0, -90.0)),
    );
    segment(history, sketch, corner(1), PointRef::New(at(90.0, -75.0)));
    segment(history, sketch, corner(2), PointRef::New(at(75.0, -75.0)));
    history.push(Operation::AddArc {
        sketch,
        center: PointRef::New(at(75.0, -45.0)),
        start: PointRef::New(at(75.0, -15.0)),
        end: corner(3),
        construction: false,
    });
    segment(history, sketch, corner(5), PointRef::New(at(90.0, -15.0)));
    segment(history, sketch, corner(6), PointRef::New(at(90.0, 15.0)));
    segment(history, sketch, corner(7), PointRef::New(at(30.0, 15.0)));
    segment(history, sketch, corner(8), corner(0));
    sketch
}

/// A line through the origin of `sketch`, leaning `degrees` off its V axis,
/// as the axis a turn about it is handed.
fn an_axis_leaning(history: &mut History, sketch: usize, degrees: f64) -> RevolutionAxis {
    let along = DVec2::from_angle(FRAC_PI_2 - degrees.to_radians());
    let drawn = PartState::rebuild(history).sketches[sketch]
        .segments()
        .len();
    segment(
        history,
        sketch,
        PointRef::New(-along * 200.0),
        PointRef::New(along * 200.0),
    );
    RevolutionAxis::Segment(SegmentId(drawn))
}

/// An ellipse 10 by 6 about (50, 5) on a sketch of its own on XY, turned a
/// quarter about the sketch's V axis: an elliptic torus, which no surface of
/// the exact kernel turns into.
fn an_ellipse_turned(history: &mut History) {
    let sketch = sketch_on(history, WorkPlane::XY);
    let center = DVec2::new(50.0, 5.0);
    history.push(Operation::AddEllipse {
        sketch,
        center: PointRef::New(center),
        first: [
            PointRef::New(center - DVec2::X * 5.0),
            PointRef::New(center + DVec2::X * 5.0),
        ],
        second: [
            PointRef::New(center - DVec2::Y * 3.0),
            PointRef::New(center + DVec2::Y * 3.0),
        ],
        construction: false,
        drawn: None,
    });
    turn(history, sketch, center, ABOUT_V, 90.0);
}

/// The turn the last step of `history` is, refused with its reason, the
/// part as it stood before it, exact; and the post raised after it, built
/// exactly beside the block.
fn refused_then_the_post_is_built(mut history: History, what: &str) {
    let turned = last_step(&history);
    let part = applied_live(&history);
    assert_eq!(
        part.declined_because(turned),
        Some(Declined::Profile),
        "{what}: no profile the exact kernel reads",
    );
    assert_near(
        part.body().volume(),
        BLOCK,
        &format!("{what}: the block alone"),
    );
    assert!(part.body().is_exact(), "{what}: the part stays exact");

    a_post(&mut history);
    let part = applied_live(&history);
    assert!(part.is_declined(turned), "{what}: the turn stays refused");
    assert_eq!(
        part.declined_because(last_step(&history)),
        None,
        "{what}: the post is built",
    );
    assert!(part.body().is_exact(), "{what}: never the flats after it");
    assert_near(
        part.body().volume(),
        BLOCK + POST,
        &format!("{what}: the block and the post"),
    );
}

#[test]
fn a_pulley_whose_groove_the_reading_cannot_lay_is_a_broken_step() {
    for degrees in [360.0, 90.0] {
        let mut history = History::default();
        a_block(&mut history);
        let sketch = a_pulley(&mut history);
        let axis = an_axis_leaning(&mut history, sketch, 1e-5);
        turn(&mut history, sketch, IN_THE_PULLEY, axis, degrees);

        refused_then_the_post_is_built(
            history,
            &format!("a pulley leaning 1e-5° turned {degrees}°"),
        );

        for lean in [1e-6, 1e-4, 0.1] {
            let mut history = History::default();
            a_block(&mut history);
            let sketch = a_pulley(&mut history);
            let axis = an_axis_leaning(&mut history, sketch, lean);
            turn(&mut history, sketch, IN_THE_PULLEY, axis, degrees);
            let part = applied_live(&history);
            assert_eq!(
                part.declined_because(last_step(&history)),
                None,
                "leaning {lean}°: laid"
            );
            assert!(part.body().is_exact(), "leaning {lean}°: exact");
        }

        let mut history = History::default();
        a_block(&mut history);
        let sketch = a_pulley(&mut history);
        turn(&mut history, sketch, IN_THE_PULLEY, ABOUT_V, degrees);
        let part = applied_live(&history);
        let pappus = (321_750.0 - 33_750.0 * PI) * degrees.to_radians();
        assert_eq!(part.declined_because(last_step(&history)), None);
        assert!(part.body().is_exact(), "about its own axis: exact");
        assert_near(
            part.body().volume() - BLOCK,
            pappus,
            &format!("the pulley about its own axis turned {degrees}°"),
        );
    }
}

#[test]
fn a_turned_ellipse_is_a_broken_step() {
    let mut history = History::default();
    a_block(&mut history);
    an_ellipse_turned(&mut history);

    refused_then_the_post_is_built(history, "an ellipse turned a quarter");
}
