//! « Coïncidence » between a point and a trait: a free point comes onto the
//! trait whichever was clicked first, and a point that belongs to something
//! follows the order of the clicks — the first one clicked stays, the second
//! comes to it, and nothing else moves.
//!
//! Closes #548.
//! - (1) a point placed on an arc and a trait it does not reach: it lands, the
//!   arc coming with the trait clicked first —
//!   `a_point_on_an_arc_comes_onto_a_trait_clicked_first_and_the_arc_follows` —
//!   and the trait coming with the point clicked first —
//!   `a_trait_comes_to_a_point_on_an_arc_clicked_first`
//! - (2) an arc's centre and an arc end with a trait: the first clicked does
//!   not move, and the second comes —
//!   `an_arc_centre_comes_onto_a_trait_clicked_first_with_its_arc_whole`,
//!   `an_arc_end_comes_onto_a_trait_clicked_first`,
//!   `a_trait_comes_to_an_arc_centre_or_end_clicked_first`
//! - (3) nothing but the second thing clicked and what it belongs to moves —
//!   `the_end_of_a_trait_and_another_trait_follow_the_clicks_and_nothing_else_moves`,
//!   and every test above holds a shape apart that must not move
//! - a point first, against a fixed trait, still lands, the point giving —
//!   `a_point_comes_onto_a_fixed_trait_clicked_second`
//! - a free point still comes onto the trait whichever was clicked first —
//!   `a_free_point_comes_onto_the_trait_whichever_was_clicked_first`
//! - a point held on the trait that comes keeps its place along it (#444, #446)
//!   — `a_point_held_on_the_trait_that_comes_keeps_its_place_along_it`
//! - a point tied to something by a value alone is not free: it follows the
//!   clicks — `a_point_tied_by_a_value_follows_the_clicks_as_one_tied_by_a_trait`

use cao_part::{Operation, Outcome, PartDocument, PointRef};
use cao_sketch::{
    ArcId, Constraint, Element, PointId, Rule, RuleIntent, RulePick, SegmentId, Sketch, Support,
    WorkPlane, rule_intent,
};
use glam::DVec2;

/// How far a point held still may be found from where it was: nothing moves
/// it, so nothing but rounding.
const ON: f64 = 1e-4;

/// How far off the trait's line a point the solver brought there may stand:
/// the solver stops at a hundred-thousandth of the drawing's size, and these
/// drawings are under a hundred units across.
const ON_THE_LINE: f64 = 1e-3;

fn blank() -> PartDocument {
    let mut document = PartDocument::new("Test", "2026-01-02T09:00:00Z".parse().unwrap());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document
}

fn point_at(sketch: &Sketch, place: DVec2) -> PointId {
    sketch.nearest_point(place, 1e-6).expect("a point there")
}

fn a_trait(document: &mut PartDocument, from: DVec2, to: DVec2) -> SegmentId {
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(from),
        end: PointRef::New(to),
        construction: false,
    });
    SegmentId(document.sketches()[0].segments().len() - 1)
}

/// « Coïncidence » on a point and a trait, in this order.
fn laid(
    document: &mut PartDocument,
    point: PointId,
    segment: SegmentId,
    point_first: bool,
) -> Option<Outcome> {
    let (point, line) = (
        RulePick::Element(Element::Point(point)),
        RulePick::Element(Element::Segment(segment)),
    );
    let picks = match point_first {
        true => [point, line],
        false => [line, point],
    };
    let Some(RuleIntent::Constrain(constraint)) =
        rule_intent(Rule::Coincident, &picks, &document.sketches()[0])
    else {
        panic!("a point and a trait make a coincidence");
    };
    document.apply(Operation::Constrain {
        sketch: 0,
        constraint,
    })
}

/// How far a point stands from the line a trait lies on.
fn off_the_line(sketch: &Sketch, point: PointId, segment: SegmentId) -> f64 {
    sketch
        .point_to_segment(point, segment)
        .expect("the trait is still there")
}

/// How far each point moved between two states of a drawing.
fn moved(before: &Sketch, after: &Sketch, points: &[PointId]) -> f64 {
    points
        .iter()
        .map(|point| before.point(*point).distance(after.point(*point)))
        .fold(0.0, f64::max)
}

fn ends(sketch: &Sketch, segment: SegmentId) -> [PointId; 2] {
    let side = sketch.segments()[segment.0];
    [side.start, side.end]
}

const CENTRE: DVec2 = DVec2::new(-52.6, -75.1);

fn on_the_arc(degrees: f64) -> DVec2 {
    CENTRE + DVec2::from_angle(degrees.to_radians()) * 30.0
}

/// The owner's drawing: an arc of radius 30 with a point placed on it, a
/// trait standing about sixty away that the arc does not reach, and a trait
/// apart that nothing asks to move.
struct ArcAndTrait {
    centre: PointId,
    start: PointId,
    end: PointId,
    held: PointId,
    line: SegmentId,
    apart: SegmentId,
}

fn an_arc_and_a_trait() -> (PartDocument, ArcAndTrait) {
    let mut document = blank();
    document.apply(Operation::AddArc {
        sketch: 0,
        center: PointRef::New(CENTRE),
        start: PointRef::New(on_the_arc(0.0)),
        end: PointRef::New(on_the_arc(64.7)),
        construction: false,
    });
    document.apply(Operation::AddPoint {
        sketch: 0,
        position: on_the_arc(16.0),
        on: vec![Support::Arc(ArcId(0))],
    });
    let line = a_trait(
        &mut document,
        DVec2::new(7.5, -2.5),
        DVec2::new(10.1, -67.5),
    );
    let apart = a_trait(
        &mut document,
        DVec2::new(-90.0, 20.0),
        DVec2::new(-70.0, 40.0),
    );
    let sketch = &document.sketches()[0];
    let fixture = ArcAndTrait {
        centre: point_at(sketch, CENTRE),
        start: point_at(sketch, on_the_arc(0.0)),
        end: point_at(sketch, on_the_arc(64.7)),
        held: point_at(sketch, on_the_arc(16.0)),
        line,
        apart,
    };
    (document, fixture)
}

fn arc_shape(sketch: &Sketch) -> (f64, f64) {
    (
        sketch.arc_radius(ArcId(0)),
        sketch.arc_sweep(ArcId(0)).to_degrees(),
    )
}

#[test]
fn a_point_on_an_arc_comes_onto_a_trait_clicked_first_and_the_arc_follows() {
    let (mut document, d) = an_arc_and_a_trait();
    let before = document.sketches()[0].clone();

    let outcome = laid(&mut document, d.held, d.line, false);

    let sketch = &document.sketches()[0];
    let on_arc =
        (sketch.point(d.held).distance(sketch.point(d.centre)) - sketch.arc_radius(ArcId(0))).abs();
    assert_eq!(outcome, None, "the rule was refused");
    assert!(
        moved(&before, sketch, &ends(sketch, d.line)) < ON,
        "the trait clicked first moved"
    );
    assert!(
        off_the_line(sketch, d.held, d.line) < ON_THE_LINE,
        "the point is off the trait"
    );
    assert!(on_arc < ON, "the point left its arc by {on_arc}");
    assert!(
        moved(&before, sketch, &ends(sketch, d.apart)) < ON,
        "a trait apart moved"
    );
}

#[test]
fn a_trait_comes_to_a_point_on_an_arc_clicked_first() {
    let (mut document, d) = an_arc_and_a_trait();
    let before = document.sketches()[0].clone();

    let outcome = laid(&mut document, d.held, d.line, true);

    let sketch = &document.sketches()[0];
    assert_eq!(outcome, None, "the rule was refused");
    assert!(
        moved(&before, sketch, &[d.held, d.centre, d.start, d.end]) < ON,
        "the point clicked first, or its arc, moved"
    );
    assert!(
        off_the_line(sketch, d.held, d.line) < ON_THE_LINE,
        "the point is off the trait"
    );
    assert!(
        moved(&before, sketch, &ends(sketch, d.apart)) < ON,
        "a trait apart moved"
    );
}

#[test]
fn an_arc_centre_comes_onto_a_trait_clicked_first_with_its_arc_whole() {
    let (mut document, d) = an_arc_and_a_trait();
    let before = document.sketches()[0].clone();
    let shape = arc_shape(&before);

    let outcome = laid(&mut document, d.centre, d.line, false);

    let sketch = &document.sketches()[0];
    let (radius, sweep) = arc_shape(sketch);
    assert_eq!(outcome, None, "the rule was refused");
    assert!(
        moved(&before, sketch, &ends(sketch, d.line)) < ON,
        "the trait clicked first moved"
    );
    assert!(
        off_the_line(sketch, d.centre, d.line) < ON_THE_LINE,
        "the centre is off the trait"
    );
    assert!(
        (radius - shape.0).abs() < ON,
        "the arc's radius went from {} to {radius}",
        shape.0
    );
    assert!(
        (sweep - shape.1).abs() < 1e-3,
        "the arc's opening went from {}° to {sweep}°",
        shape.1
    );
    assert!(
        moved(&before, sketch, &ends(sketch, d.apart)) < ON,
        "a trait apart moved"
    );
}

#[test]
fn an_arc_end_comes_onto_a_trait_clicked_first() {
    for end_first in [true, false] {
        let (mut document, d) = an_arc_and_a_trait();
        let before = document.sketches()[0].clone();
        let end = if end_first { d.start } else { d.end };

        let outcome = laid(&mut document, end, d.line, false);

        let sketch = &document.sketches()[0];
        assert_eq!(outcome, None, "the rule was refused");
        assert!(
            moved(&before, sketch, &ends(sketch, d.line)) < ON,
            "the trait clicked first moved"
        );
        assert!(
            off_the_line(sketch, end, d.line) < ON_THE_LINE,
            "the arc end is off the trait"
        );
        assert!(
            moved(&before, sketch, &ends(sketch, d.apart)) < ON,
            "a trait apart moved"
        );
    }
}

#[test]
fn a_trait_comes_to_an_arc_centre_or_end_clicked_first() {
    let (_, d) = an_arc_and_a_trait();
    for point in [d.centre, d.start, d.end] {
        let (mut document, d) = an_arc_and_a_trait();
        let before = document.sketches()[0].clone();

        let outcome = laid(&mut document, point, d.line, true);

        let sketch = &document.sketches()[0];
        assert_eq!(outcome, None, "the rule was refused");
        assert!(
            moved(&before, sketch, &[d.centre, d.start, d.end, d.held]) < ON,
            "{point:?} was clicked first, and the arc moved"
        );
        assert!(
            off_the_line(sketch, point, d.line) < ON_THE_LINE,
            "{point:?} is off the trait"
        );
        assert!(
            moved(&before, sketch, &ends(sketch, d.apart)) < ON,
            "a trait apart moved"
        );
    }
}

/// The owner's screenshot of 2026-10-09: a free upright trait, a free
/// diagonal one, and a trait apart. Placed in the drawing's own units.
fn two_traits_and_one_apart() -> (PartDocument, [SegmentId; 3]) {
    let mut document = blank();
    let upright = a_trait(
        &mut document,
        DVec2::new(-57.07, -20.98),
        DVec2::new(-56.10, -46.95),
    );
    let diagonal = a_trait(
        &mut document,
        DVec2::new(-37.56, -17.44),
        DVec2::new(-17.56, -37.44),
    );
    let apart = a_trait(
        &mut document,
        DVec2::new(-72.07, 11.10),
        DVec2::new(-71.10, -6.95),
    );
    (document, [upright, diagonal, apart])
}

#[test]
fn the_end_of_a_trait_and_another_trait_follow_the_clicks_and_nothing_else_moves() {
    for point_first in [true, false] {
        let (mut document, [upright, diagonal, apart]) = two_traits_and_one_apart();
        let before = document.sketches()[0].clone();
        let top = ends(&before, upright)[0];

        let outcome = laid(&mut document, top, diagonal, point_first);

        let sketch = &document.sketches()[0];
        let (stays, case) = match point_first {
            true => (ends(sketch, upright), "the point first: its trait"),
            false => (ends(sketch, diagonal), "the trait first: it"),
        };
        assert_eq!(outcome, None, "the rule was refused");
        assert!(moved(&before, sketch, &stays) < ON, "{case} moved");
        assert!(
            off_the_line(sketch, top, diagonal) < ON_THE_LINE,
            "the point is off the trait"
        );
        assert!(
            moved(&before, sketch, &ends(sketch, apart)) < ON,
            "a trait apart moved"
        );
    }
}

#[test]
fn a_point_comes_onto_a_fixed_trait_clicked_second() {
    let (mut document, [upright, diagonal, apart]) = two_traits_and_one_apart();
    document.apply(Operation::Constrain {
        sketch: 0,
        constraint: Constraint::Fixed {
            element: Element::Segment(diagonal),
        },
    });
    let before = document.sketches()[0].clone();
    let top = ends(&before, upright)[0];

    let outcome = laid(&mut document, top, diagonal, true);

    let sketch = &document.sketches()[0];
    assert_eq!(outcome, None, "the rule was refused");
    assert!(
        moved(&before, sketch, &ends(sketch, diagonal)) < ON,
        "the fixed trait moved"
    );
    assert!(
        off_the_line(sketch, top, diagonal) < ON_THE_LINE,
        "the point is off the trait"
    );
    assert!(
        moved(&before, sketch, &ends(sketch, apart)) < ON,
        "a trait apart moved"
    );
}

#[test]
fn a_free_point_comes_onto_the_trait_whichever_was_clicked_first() {
    for point_first in [true, false] {
        let (mut document, [_, diagonal, apart]) = two_traits_and_one_apart();
        document.apply(Operation::AddPoint {
            sketch: 0,
            position: DVec2::new(-10.0, -10.0),
            on: Vec::new(),
        });
        let before = document.sketches()[0].clone();
        let free = point_at(&before, DVec2::new(-10.0, -10.0));

        let outcome = laid(&mut document, free, diagonal, point_first);

        let sketch = &document.sketches()[0];
        assert_eq!(outcome, None, "the rule was refused");
        assert!(
            moved(&before, sketch, &ends(sketch, diagonal)) < ON,
            "point first: {point_first}, the trait moved to meet a free point"
        );
        assert!(
            off_the_line(sketch, free, diagonal) < ON_THE_LINE,
            "the point is off the trait"
        );
        assert!(
            moved(&before, sketch, &ends(sketch, apart)) < ON,
            "a trait apart moved"
        );
    }
}

#[test]
fn a_point_held_on_the_trait_that_comes_keeps_its_place_along_it() {
    let (mut document, [upright, diagonal, _]) = two_traits_and_one_apart();
    let [top, bottom] = ends(&document.sketches()[0], upright);
    let middle = (document.sketches()[0].point(top) + document.sketches()[0].point(bottom)) * 0.5;
    document.apply(Operation::AddPoint {
        sketch: 0,
        position: middle,
        on: vec![Support::Segment(upright)],
    });
    let held = point_at(&document.sketches()[0], middle);

    let outcome = laid(&mut document, top, diagonal, false);

    let sketch = &document.sketches()[0];
    let (from, to) = (sketch.point(top), sketch.point(bottom));
    let share = (sketch.point(held) - from).dot(to - from) / (to - from).length_squared();
    assert_eq!(outcome, None, "the rule was refused");
    assert!(
        (share - 0.5).abs() < 1e-6,
        "the point held at the middle of the trait that came stands at {share} of it"
    );
}

#[test]
fn a_point_tied_by_a_value_follows_the_clicks_as_one_tied_by_a_trait() {
    let (mut document, [upright, diagonal, apart]) = two_traits_and_one_apart();
    document.apply(Operation::AddPoint {
        sketch: 0,
        position: DVec2::new(-10.0, -10.0),
        on: Vec::new(),
    });
    let sketch = &document.sketches()[0];
    let (tied, corner) = (
        point_at(sketch, DVec2::new(-10.0, -10.0)),
        ends(sketch, upright)[0],
    );
    let gap = sketch.point(tied).distance(sketch.point(corner));
    document.apply(Operation::SetDimension {
        sketch: 0,
        target: cao_sketch::DimensionTarget::Distance {
            from: corner,
            to: tied,
        },
        value: gap.into(),
        placement: None,
    });
    let before = document.sketches()[0].clone();

    let outcome = laid(&mut document, tied, diagonal, true);

    let sketch = &document.sketches()[0];
    assert_eq!(outcome, None, "the rule was refused");
    assert!(
        moved(&before, sketch, &[tied]) < ON,
        "the point clicked first moved"
    );
    assert!(
        off_the_line(sketch, tied, diagonal) < ON_THE_LINE,
        "the point is off the trait"
    );
    assert!(
        moved(&before, sketch, &ends(sketch, apart)) < ON,
        "a trait apart moved"
    );
}
