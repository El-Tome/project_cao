//! A corner held on a sketch axis stays on it when a second size is typed, and
//! the size moves only what it has to.
//!
//! Closes #530.
//! - a point held on an axis stays on it: after the second value, the corner
//!   is on V, to the solver's tolerance —
//!   `a_corner_held_on_v_stays_on_it_when_the_second_side_is_typed`
//! - a solution that keeps everything is the one found: only the top right
//!   corner rises; the held corner, the left side and the bottom stay where
//!   they are, and the bottom keeps its 48.48 mm —
//!   `the_side_typed_second_raises_its_far_corner_alone`
//! - the same shape turned a full turn about V gives a solid —
//!   `the_shape_turned_a_full_turn_about_v_is_a_solid`
//! - every variant of the rectangle, the two sides typed either way round,
//!   keeps the corner on V and its values, and changes one side nobody typed
//!   at most — `every_variant_of_the_rectangle_keeps_its_corner_and_its_sides`
//! - a four-sided shape drawn off square, its bottom level with the held
//!   corner to about 0.1°, keeps the corner on V too —
//!   `a_shape_off_square_with_its_bottom_about_level_keeps_its_corner_on_v`
//! - a four-sided shape drawn off square, its bottom typed then its right
//!   side, moves the far corner of the right side alone —
//!   `a_shape_off_square_typed_along_its_bottom_then_its_right_side_moves_one_corner`
//! - when no place holds both the values and the rule, the value is refused,
//!   never answered exact with the rule left untrue —
//!   `a_value_no_place_on_the_axis_can_meet_is_refused`
//! - the line tool's case: right angles laid, the second length is a
//!   reference and nothing moves —
//!   `with_its_right_angles_the_second_side_is_a_reference_and_nothing_moves`
//! - found by an independent review of the first version, each a way it went
//!   wrong, held here: the left side typed by its corners or by its height is
//!   kept as its length is —
//!   `the_left_side_typed_by_its_corners_or_by_its_height_is_kept_too`; a
//!   third side typed turns the far corner round rather than sliding the held
//!   one — `a_third_side_typed_turns_the_far_corner_rather_than_sliding_the_held_one`;
//!   a top held level still takes a bottom typed after both sides —
//!   `with_its_top_held_level_a_bottom_typed_after_both_sides_still_lands`
//! - #456, #455 and #471 do not break — no test: held by the tests already
//!   naming them, in `solver/orientation/tests.rs` and
//!   `sketch/settling/nearest/tests.rs`, unchanged and green
//! - the case has its row in "The cases, against the code" of
//!   `docs/sketch.md` — no test: it is prose
//! - the issue has no "Done when"; these are its "What it does" and "What
//!   must not break", transcribed at the opening — no test: a reading of the
//!   issue, for the owner to confirm

use std::f64::consts::PI;

use cao_part::history::ExtrusionMode;
use cao_part::{DimensionOutcome, Operation, Outcome, PartDocument, PointRef, RevolutionAxis};
use cao_sketch::{
    Constraint, DimensionTarget, LengthOutcome, PointId, SegmentId, Sketch, SketchAxis, Support,
    WorkPlane,
};
use glam::DVec2;

/// The issue's drawing: a rectangle from a corner held on V to its opposite.
const HELD: DVec2 = DVec2::new(0.0, 3.5305);
const OPPOSITE: DVec2 = DVec2::new(12.622, 10.3535);
const LEFT_TYPED: f64 = 26.208;
const RIGHT_TYPED: f64 = 40.904;

/// The rectangle tool's corners: the one held, the opposite, then the bottom
/// right and the top left.
const CORNER: PointId = PointId(1);
const TOP_RIGHT: PointId = PointId(2);
const BOTTOM_RIGHT: PointId = PointId(3);
const TOP_LEFT: PointId = PointId(4);
/// And its sides: bottom, right, top, left.
const BOTTOM: SegmentId = SegmentId(0);
const RIGHT: SegmentId = SegmentId(1);
const LEFT: SegmentId = SegmentId(3);

/// How far a point nothing moves may be found from where it was.
const STILL: f64 = 1e-6;

/// How far the solver lets a side lying on V slide along it while it turns a
/// typed top round the corner a value moved: three thousandths here, against
/// the 1.6 of the whole drop when the corner slid.
const MENDED: f64 = 0.01;

/// How near the arithmetic a turn the flats compute comes: its top leans, so
/// it turns into a cone the exact kernel leaves to them, and their round is
/// facets, which hold a little less than the true one.
const FACETS: f64 = 1e-2;

fn a_part() -> PartDocument {
    let mut document = PartDocument::new("Probe", "2026-01-02T09:00:00Z".parse().unwrap());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document
}

/// A rectangle from `corner`, held on V, to `opposite`, its right angles
/// erased unless `square`.
fn a_rectangle_held_on_v(corner: DVec2, opposite: DVec2, square: bool) -> PartDocument {
    let mut document = a_part();
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::Held {
            at: corner,
            on: vec![Support::Axis(SketchAxis::V)],
        },
        opposite: PointRef::New(opposite),
        construction: false,
    });
    // The rectangle tool lays its three right angles as rules of their own.
    let right_angles: Vec<Constraint> = (0..3)
        .map(|rank| Constraint::Perpendicular {
            first: SegmentId(rank),
            second: SegmentId(rank + 1),
        })
        .collect();
    for constraint in &right_angles {
        document.apply(Operation::Constrain {
            sketch: 0,
            constraint: *constraint,
        });
    }
    if !square {
        document.apply(Operation::EraseMany {
            sketch: 0,
            elements: Vec::new(),
            dimensions: Vec::new(),
            constraints: right_angles,
        });
    }
    document
}

fn type_length(document: &mut PartDocument, side: SegmentId, value: f64) -> Option<Outcome> {
    type_value(document, DimensionTarget::Length(side), value)
}

fn type_value(document: &mut PartDocument, target: DimensionTarget, value: f64) -> Option<Outcome> {
    document.apply(Operation::SetDimension {
        sketch: 0,
        target,
        value: value.into(),
        placement: None,
    })
}

/// The side's length taken `factor` times, in millimetres: the first value
/// typed sets the scale, at 3.841 mm a unit.
fn taken(document: &PartDocument, side: SegmentId, factor: f64) -> f64 {
    let length = document.sketches()[0].segment_length(side);
    match document.has_scale() {
        true => document.to_millimeters(length) * factor,
        false => length * 3.841,
    }
}

/// The issue's gestures: the left side typed, which sets the scale, then the
/// right one.
fn the_issue_s_drawing() -> PartDocument {
    let mut document = a_rectangle_held_on_v(HELD, OPPOSITE, false);
    type_length(&mut document, LEFT, LEFT_TYPED);
    let outcome = type_length(&mut document, RIGHT, RIGHT_TYPED);
    assert_eq!(outcome, Some(exact()), "the second value was to land");
    document
}

fn exact() -> Outcome {
    Outcome::Dimension(DimensionOutcome::Geometry(LengthOutcome::Exact))
}

fn points(sketch: &Sketch) -> Vec<DVec2> {
    (0..sketch.points().len())
        .map(|index| sketch.point(PointId(index)))
        .collect()
}

/// The drawing's size, as the solver measures its tolerance against.
fn size(sketch: &Sketch) -> f64 {
    let all = points(sketch);
    let low = all.iter().copied().fold(DVec2::INFINITY, DVec2::min);
    let high = all.iter().copied().fold(DVec2::NEG_INFINITY, DVec2::max);
    (high - low).length()
}

/// What the solver counts as on the axis.
fn assert_on_v(sketch: &Sketch, what: &str) {
    let held = sketch.point(CORNER);
    assert!(
        held.x.abs() <= 1e-5 * size(sketch),
        "{what}: the corner held on V left it, to x = {}",
        held.x,
    );
}

#[test]
fn a_corner_held_on_v_stays_on_it_when_the_second_side_is_typed() {
    let document = the_issue_s_drawing();

    let held = document.sketches()[0].point(CORNER);
    assert!(
        held.x.abs() < 1e-9,
        "the corner held on V left it, to x = {}",
        held.x
    );
}

#[test]
fn the_side_typed_second_raises_its_far_corner_alone() {
    let document = the_issue_s_drawing();
    let sketch = &document.sketches()[0];

    for (point, drawn, what) in [
        (CORNER, HELD, "the held corner"),
        (TOP_LEFT, DVec2::new(0.0, OPPOSITE.y), "the top left corner"),
        (
            BOTTOM_RIGHT,
            DVec2::new(OPPOSITE.x, HELD.y),
            "the bottom right corner",
        ),
    ] {
        let at = sketch.point(point);
        assert!(
            at.distance(drawn) < STILL,
            "{what} was to stay at {drawn}, it is at {at}"
        );
    }
    let raised = DVec2::new(OPPOSITE.x, HELD.y + RIGHT_TYPED / document.scale());
    let at = sketch.point(TOP_RIGHT);
    assert!(
        at.distance(raised) < 1e-4,
        "the top right corner was to rise to {raised}, it is at {at}",
    );
    let bottom = document.to_millimeters(sketch.segment_length(BOTTOM));
    assert!(
        (bottom - 48.48).abs() < 0.01,
        "the bottom, which nobody typed, was to keep its 48.48 mm: {bottom}",
    );
}

#[test]
fn the_shape_turned_a_full_turn_about_v_is_a_solid() {
    let mut document = the_issue_s_drawing();
    let sketch = &document.sketches()[0];
    let volume = turned_about_v(
        sketch.segment_length(LEFT),
        sketch.segment_length(RIGHT),
        sketch.point(BOTTOM_RIGHT).x,
    );
    let areas = document.areas_at(0, &[DVec2::new(6.0, 7.0)]);
    document.apply(Operation::Revolve {
        sketch: 0,
        areas,
        axis: RevolutionAxis::Sketch(SketchAxis::V),
        angle: 360.0.into(),
        mode: ExtrusionMode::Add,
    });

    let made = document.body().volume();
    assert!(
        (made - volume).abs() < FACETS * volume,
        "the shape turned about V was to hold {volume}, it holds {made}",
    );
}

/// What four sides hold turned a full turn about V, one of them `left` long
/// on V, the one across `right` long and parallel to it, `wide` away, and the
/// bottom square to both: each band between x and x + dx is a height growing
/// straight from `left` to `right`, swept round a circle of x.
fn turned_about_v(left: f64, right: f64, wide: f64) -> f64 {
    2.0 * PI * wide * wide * (left / 2.0 + (right - left) / 3.0)
}

/// Where a corner held on V was drawn, the opposite corner from it, which two
/// sides are typed and how far the second one is taken.
fn variants() -> Vec<(DVec2, DVec2, SegmentId, SegmentId, f64)> {
    let mut all = Vec::new();
    for height in [-6.0, -0.5, 0.5, 3.5305, 8.0] {
        for across in [
            DVec2::new(12.622, 6.823),
            DVec2::new(-12.622, 6.823),
            DVec2::new(12.622, -6.823),
            DVec2::new(5.0, 15.0),
            DVec2::new(30.0, 4.0),
        ] {
            for (first, second) in [
                (3, 1),
                (1, 3),
                (3, 0),
                (3, 2),
                (0, 1),
                (0, 2),
                (1, 0),
                (2, 3),
            ] {
                for factor in [0.6, 1.5, 2.5] {
                    let corner = DVec2::new(0.0, height);
                    all.push((
                        corner,
                        corner + across,
                        SegmentId(first),
                        SegmentId(second),
                        factor,
                    ));
                }
            }
        }
    }
    all
}

#[test]
fn every_variant_of_the_rectangle_keeps_its_corner_and_its_sides() {
    for (corner, opposite, first, second, factor) in variants() {
        let what = format!(
            "from {corner} to {opposite}, side {} then {}, x{factor}",
            first.0, second.0
        );
        let mut document = a_rectangle_held_on_v(corner, opposite, false);
        let first_length = document.sketches()[0].segment_length(first);
        type_length(&mut document, first, first_length * 3.841);
        let before: Vec<f64> = (0..4)
            .map(|side| document.sketches()[0].segment_length(SegmentId(side)))
            .collect();
        let value = document.to_millimeters(before[second.0] * factor);

        let outcome = type_length(&mut document, second, value);

        assert_eq!(
            outcome,
            Some(exact()),
            "{what}: the second value was to land"
        );
        let sketch = &document.sketches()[0];
        assert_on_v(sketch, &what);
        assert!(
            (sketch.segment_length(first) - first_length).abs() < 1e-5 * size(sketch),
            "{what}: the side typed first lost its value",
        );
        assert!(
            (document.to_millimeters(sketch.segment_length(second)) - value).abs() < 1e-4 * value,
            "{what}: the side typed second missed its value",
        );
        let changed: Vec<usize> = (0..4)
            .filter(|side| *side != first.0 && *side != second.0)
            .filter(|side| (sketch.segment_length(SegmentId(*side)) - before[*side]).abs() > 1e-6)
            .collect();
        assert!(
            changed.len() <= 1,
            "{what}: the value changed sides nobody typed, {changed:?}, where one was enough",
        );
    }
}

/// Four traits through `corners`, the first held on V, closed back onto it:
/// the corners are points 1 to 4, the sides 0 to 3 from the first.
fn four_traits_held_on_v(corners: [DVec2; 4]) -> PartDocument {
    let mut document = a_part();
    let mut start = PointRef::Held {
        at: corners[0],
        on: vec![Support::Axis(SketchAxis::V)],
    };
    for (index, end) in [
        PointRef::New(corners[1]),
        PointRef::New(corners[2]),
        PointRef::New(corners[3]),
        PointRef::Existing(CORNER),
    ]
    .into_iter()
    .enumerate()
    {
        document.apply(Operation::AddSegment {
            sketch: 0,
            start,
            end,
            construction: false,
        });
        start = PointRef::Existing(PointId(index + 2));
    }
    document
}

#[test]
fn a_shape_off_square_typed_along_its_bottom_then_its_right_side_moves_one_corner() {
    let mut document = four_traits_held_on_v([
        DVec2::new(0.0, 2.97),
        DVec2::new(13.29, 2.99),
        DVec2::new(10.78, 14.97),
        DVec2::new(0.26, 12.0),
    ]);
    let value = taken(&document, BOTTOM, 1.0);
    type_length(&mut document, BOTTOM, value);
    let before = points(&document.sketches()[0]);
    let value = taken(&document, RIGHT, 0.9);

    let outcome = type_length(&mut document, RIGHT, value);

    assert_eq!(outcome, Some(exact()), "the right side was to land");
    let after = points(&document.sketches()[0]);
    for index in [1, 2, 4] {
        assert!(
            before[index].distance(after[index]) < STILL,
            "corner {index} moved from {} to {}",
            before[index],
            after[index],
        );
    }
}

#[test]
fn a_shape_off_square_with_its_bottom_about_level_keeps_its_corner_on_v() {
    for rise in [-0.03, -0.01, -0.002, 0.0, 0.002, 0.01, 0.03] {
        for (top_right, top_left) in [
            (DVec2::new(13.5, 10.0), DVec2::new(0.8, 9.2)),
            (DVec2::new(11.0, 12.5), DVec2::new(-1.5, 8.0)),
            (DVec2::new(14.2, 7.4), DVec2::new(0.3, 11.1)),
        ] {
            for factor in [0.7, 1.5] {
                let what =
                    format!("bottom rising {rise}, top {top_left} to {top_right}, x{factor}");
                let mut document = four_traits_held_on_v([
                    HELD,
                    DVec2::new(12.622, HELD.y + rise),
                    top_right,
                    top_left,
                ]);
                let left = SegmentId(3);
                let right = SegmentId(1);
                let length = document.sketches()[0].segment_length(left);
                type_length(&mut document, left, length * 3.841);
                let value =
                    document.to_millimeters(document.sketches()[0].segment_length(right) * factor);

                let outcome = type_length(&mut document, right, value);

                assert_eq!(
                    outcome,
                    Some(exact()),
                    "{what}: the second value was to land"
                );
                assert_on_v(&document.sketches()[0], &what);
            }
        }
    }
}

/// A trait from the origin along U, ten long, and a trait from a point held on
/// V to its end, which can never be shorter than ten: the first is trait 0,
/// the second trait 1.
fn a_trait_from_v_to_the_end_of_ten_along_u() -> PartDocument {
    let mut document = a_part();
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(Sketch::ORIGIN),
        end: PointRef::Held {
            at: DVec2::new(10.0, 0.0),
            on: vec![Support::Axis(SketchAxis::U)],
        },
        construction: false,
    });
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Held {
            at: DVec2::new(0.0, 6.0),
            on: vec![Support::Axis(SketchAxis::V)],
        },
        end: PointRef::Existing(PointId(1)),
        construction: false,
    });
    type_length(&mut document, SegmentId(0), 10.0);
    document
}

#[test]
fn a_value_no_place_on_the_axis_can_meet_is_refused() {
    let mut document = a_trait_from_v_to_the_end_of_ten_along_u();
    let before = points(&document.sketches()[0]);

    let outcome = type_length(&mut document, SegmentId(1), 8.0);

    assert_ne!(outcome, Some(exact()), "eight cannot be met from V");
    let after = points(&document.sketches()[0]);
    for (index, (was, is)) in before.iter().zip(&after).enumerate() {
        assert!(
            was.distance(*is) < STILL,
            "point {index} moved from {was} to {is}"
        );
    }
}

#[test]
fn with_its_right_angles_the_second_side_is_a_reference_and_nothing_moves() {
    let mut document = a_rectangle_held_on_v(HELD, OPPOSITE, true);
    type_length(&mut document, LEFT, LEFT_TYPED);
    let before = points(&document.sketches()[0]);

    let outcome = type_length(&mut document, RIGHT, RIGHT_TYPED);

    assert_eq!(
        outcome,
        Some(Outcome::Dimension(DimensionOutcome::Reference))
    );
    let after = points(&document.sketches()[0]);
    for (index, (was, is)) in before.iter().zip(&after).enumerate() {
        assert!(
            was.distance(*is) < STILL,
            "point {index} moved from {was} to {is}"
        );
    }
}

#[test]
fn the_left_side_typed_by_its_corners_or_by_its_height_is_kept_too() {
    for target in [
        DimensionTarget::Distance {
            from: TOP_LEFT,
            to: CORNER,
        },
        DimensionTarget::Projected {
            from: CORNER,
            to: TOP_LEFT,
            axis: SketchAxis::V,
        },
    ] {
        let mut document = a_rectangle_held_on_v(HELD, OPPOSITE, false);
        type_value(&mut document, target, LEFT_TYPED);

        let outcome = type_length(&mut document, RIGHT, RIGHT_TYPED);

        assert_eq!(outcome, Some(exact()), "{target:?}: the value was to land");
        let sketch = &document.sketches()[0];
        for (point, drawn) in [(CORNER, HELD), (TOP_LEFT, DVec2::new(0.0, OPPOSITE.y))] {
            let at = sketch.point(point);
            assert!(
                at.distance(drawn) < STILL,
                "{target:?}: a corner of the left side was to stay at {drawn}, it is at {at}",
            );
        }
    }
}

/// A rectangle held on V by its corner, `rules` laid on it, its sides typed in
/// turn, each a `factor` of what it measured: whether each value landed, and
/// how far the held corner went with the last one.
fn typed_in_turn(
    corner: DVec2,
    opposite: DVec2,
    rules: &[Constraint],
    sides: &[(SegmentId, f64)],
) -> (Vec<Option<Outcome>>, f64) {
    let mut document = a_rectangle_held_on_v(corner, opposite, false);
    for rule in rules {
        document.apply(Operation::Constrain {
            sketch: 0,
            constraint: *rule,
        });
    }
    let mut outcomes = Vec::new();
    let mut slid = 0.0;
    for (side, factor) in sides {
        let value = taken(&document, *side, *factor);
        let was = document.sketches()[0].point(CORNER);
        outcomes.push(type_length(&mut document, *side, value));
        slid = document.sketches()[0].point(CORNER).distance(was);
    }
    (outcomes, slid)
}

#[test]
fn a_third_side_typed_turns_the_far_corner_rather_than_sliding_the_held_one() {
    let (outcomes, slid) = typed_in_turn(
        HELD,
        HELD + DVec2::new(30.0, 4.0),
        &[Constraint::AxisParallel {
            segment: LEFT,
            axis: SketchAxis::V,
        }],
        &[(LEFT, 1.0), (SegmentId(2), 0.6), (RIGHT, 0.6)],
    );

    assert_eq!(
        outcomes.last(),
        Some(&Some(exact())),
        "the third value was to land"
    );
    assert!(slid < MENDED, "the held corner slid {slid} along V");
}

#[test]
fn with_its_top_held_level_a_bottom_typed_after_both_sides_still_lands() {
    let (outcomes, _) = typed_in_turn(
        HELD,
        HELD + DVec2::new(5.0, 15.0),
        &[Constraint::AxisParallel {
            segment: SegmentId(2),
            axis: SketchAxis::U,
        }],
        &[(LEFT, 1.0), (RIGHT, 0.6), (BOTTOM, 0.6)],
    );

    assert_eq!(
        outcomes.last(),
        Some(&Some(exact())),
        "the bottom was to land"
    );
}
