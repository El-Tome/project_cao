//! « Coïncidence » between a point and an arc, a circle or an ellipse: a free
//! point comes onto the curve whichever was clicked first, and a point that
//! belongs to something follows the order of the clicks — the first one
//! clicked stays, the second comes to it, and nothing else moves.
//!
//! Closes #554.
//! - a free point comes onto an arc, a circle and an ellipse, whichever was
//!   clicked first — `a_free_point_comes_onto_a_curve_whichever_was_clicked_first`
//! - the end of a trait clicked first, then an arc: the arc keeps its radius
//!   and moves to pass through the point —
//!   `an_arc_comes_to_the_end_of_a_trait_clicked_first_keeping_its_radius` —
//!   and lengthens when the point falls past its ends, reaching a little past
//!   it rather than ending on it —
//!   `an_arc_lengthens_past_the_end_of_a_trait_it_does_not_reach`; the trait
//!   and the point do not move, in both
//! - the same with a circle, which only moves —
//!   `a_circle_comes_to_the_end_of_a_trait_clicked_first`
//! - and with half an ellipse, which moves and lengthens —
//!   `half_an_ellipse_left_by_a_cut_lengthens_to_the_end_of_a_trait`. Half an
//!   ellipse drawn by its ends ends on its own axis, which cannot slide: it
//!   moves until the half drawn reaches the point, without lengthening —
//!   `half_an_ellipse_drawn_by_its_ends_comes_to_a_point_without_lengthening`
//!   (decided with the owner on 2026-10-09)
//! - an arc clicked first, then the end of a trait: the point lands on the
//!   drawn part of the arc, at the nearest place, and the arc does not move —
//!   `the_end_of_a_trait_comes_onto_the_drawn_part_of_an_arc_clicked_first`;
//!   when the nearest place on the whole circle is past an end, a little short
//!   of that end rather than on it —
//!   `the_end_of_a_trait_lands_just_short_of_the_end_of_an_arc_it_is_past`.
//!   The issue said "at an end"; the owner asked on 2026-10-09 for never two
//!   points one on the other, and never one point made of two. Half an
//!   ellipse takes it the same way —
//!   `the_end_of_a_trait_comes_onto_the_drawn_half_of_an_ellipse_clicked_first`
//! - a fillet lengthens too, the trait it is tangent to turning with its end —
//!   `a_fillet_lengthens_to_a_point_clicked_first_its_trait_turning_with_its_end`.
//!   One whose shape cannot follow comes until what is drawn reaches the
//!   point, as half an ellipse drawn by its ends does — no test here: met by
//!   the campaign of `a_point_laid_on_a_trait_or_a_curve_at_random.rs`, seed
//!   8 among the gate's
//! - a fixed curve and a point clicked first: the point comes onto it —
//!   `a_point_comes_onto_a_fixed_curve_clicked_second`, and the same for a
//!   curve that cannot move whole: a fixed circle, which holds only its
//!   centre, a circle on the origin, an ellipse whose centre is fixed. None of
//!   them grows to reach the point instead
//! - a point standing on an arc's circle past what is drawn, held by nothing,
//!   still comes onto what is drawn, or the arc still lengthens to it —
//!   `a_point_on_the_circle_of_an_arc_past_its_end_still_lands_in_order`
//! - a point held past the end of an arc, where a drag may take it, stays
//!   there when the history is compacted —
//!   `a_point_held_past_the_end_of_an_arc_stays_there_when_the_history_is_compacted`
//! - nothing else moves — no test: every test above holds a trait apart and
//!   asserts it did not move

use cao_part::{Operation, Outcome, PartDocument, PointRef};
use cao_sketch::{
    ArcId, CLEAR_OF_AN_END, CircleId, Constraint, Corner, Element, EllipseId, PointId, Rule,
    RuleIntent, RulePick, SegmentId, Sketch, WorkPlane, rule_intent,
};
use glam::DVec2;

/// How far a point held still may be found from where it was: nothing moves
/// it, so nothing but rounding.
const ON: f64 = 1e-4;

/// How far off the curve a point the drawing brought there may stand: the
/// solver stops at a hundred-thousandth of the drawing's size, and these
/// drawings are under two hundred units across.
const ON_THE_CURVE: f64 = 1e-3;

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

/// A trait apart, that nothing asks to move.
fn a_trait_apart(document: &mut PartDocument) -> SegmentId {
    a_trait(document, DVec2::new(-60.0, 60.0), DVec2::new(-40.0, 80.0))
}

/// « Coïncidence » on a point and a curve, in this order.
fn laid(
    document: &mut PartDocument,
    point: PointId,
    curve: Element,
    point_first: bool,
) -> Option<Outcome> {
    let (point, curve) = (
        RulePick::Element(Element::Point(point)),
        RulePick::Element(curve),
    );
    let picks = match point_first {
        true => [point, curve],
        false => [curve, point],
    };
    let Some(RuleIntent::Constrain(constraint)) =
        rule_intent(Rule::Coincident, &picks, &document.sketches()[0])
    else {
        panic!("a point and a curve make a coincidence");
    };
    document.apply(Operation::Constrain {
        sketch: 0,
        constraint,
    })
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

const CENTRE: DVec2 = DVec2::new(40.0, 30.0);
const RADIUS: f64 = 20.0;

fn around(centre: DVec2, degrees: f64, reach: f64) -> DVec2 {
    centre + DVec2::from_angle(degrees.to_radians()) * reach
}

/// An arc of radius twenty, a quarter turn from 0° to 90° about `CENTRE`.
fn an_arc(document: &mut PartDocument) -> ArcId {
    document.apply(Operation::AddArc {
        sketch: 0,
        center: PointRef::New(CENTRE),
        start: PointRef::New(around(CENTRE, 0.0, RADIUS)),
        end: PointRef::New(around(CENTRE, 90.0, RADIUS)),
        construction: false,
    });
    ArcId(document.sketches()[0].arcs().len() - 1)
}

fn a_circle(document: &mut PartDocument) -> CircleId {
    document.apply(Operation::AddCircle {
        sketch: 0,
        center: PointRef::New(CENTRE),
        radius: RADIUS,
        rim: Vec::new(),
        construction: false,
    });
    CircleId(document.sketches()[0].circles().len() - 1)
}

const ELLIPSE_CENTRE: DVec2 = DVec2::new(40.0, -40.0);
const WIDE: f64 = 30.0;
const HIGH: f64 = 15.0;

/// Where a turn round the ellipse lands, about a centre.
fn on_the_ellipse(centre: DVec2, degrees: f64) -> DVec2 {
    let turn = degrees.to_radians();
    centre + DVec2::new(WIDE * turn.cos(), HIGH * turn.sin())
}

/// A place standing `out` off the ellipse, square to it at that turn.
fn off_the_ellipse(centre: DVec2, degrees: f64, out: f64) -> DVec2 {
    let turn = degrees.to_radians();
    let normal = DVec2::new(HIGH * turn.cos(), WIDE * turn.sin()).normalize();
    on_the_ellipse(centre, degrees) + normal * out
}

/// An ellipse thirty by fifteen with no stretch taken away, or drawn over the
/// stretch between two turns of it.
fn an_ellipse(document: &mut PartDocument, drawn: Option<[f64; 2]>) -> EllipseId {
    document.apply(Operation::AddEllipse {
        sketch: 0,
        center: PointRef::New(ELLIPSE_CENTRE),
        first: [
            PointRef::New(ELLIPSE_CENTRE - DVec2::X * WIDE),
            PointRef::New(ELLIPSE_CENTRE + DVec2::X * WIDE),
        ],
        second: [
            PointRef::New(ELLIPSE_CENTRE - DVec2::Y * HIGH),
            PointRef::New(ELLIPSE_CENTRE + DVec2::Y * HIGH),
        ],
        construction: false,
        drawn: drawn.map(|turns| turns.map(|at| PointRef::New(on_the_ellipse(ELLIPSE_CENTRE, at)))),
    });
    EllipseId(document.sketches()[0].ellipses().len() - 1)
}

/// Half an ellipse as its tool lays it: drawn between the two ends of its
/// first axis, over the half its second axis rises into.
fn half_an_ellipse_by_its_ends(document: &mut PartDocument) -> EllipseId {
    let (left, right) = (
        PointRef::New(ELLIPSE_CENTRE - DVec2::X * WIDE),
        PointRef::New(ELLIPSE_CENTRE + DVec2::X * WIDE),
    );
    let centre = PointRef::New(ELLIPSE_CENTRE);
    document.apply(Operation::AddEllipse {
        sketch: 0,
        center: centre.clone(),
        first: [left.clone(), right.clone()],
        second: [centre, PointRef::New(ELLIPSE_CENTRE + DVec2::Y * HIGH)],
        construction: false,
        drawn: Some([right, left]),
    });
    EllipseId(document.sketches()[0].ellipses().len() - 1)
}

fn arc_points(sketch: &Sketch, arc: ArcId) -> Vec<PointId> {
    let curve = sketch.arc(arc);
    vec![curve.center, curve.start, curve.end]
}

/// Where the start of an arc stands round its centre, and how far round it
/// runs, in degrees.
fn arc_run(sketch: &Sketch, arc: ArcId) -> (f64, f64) {
    let drawn = sketch.arc_draft(arc);
    (
        (drawn.start - drawn.centre).to_angle().to_degrees(),
        sketch.arc_sweep(arc).to_degrees(),
    )
}

/// How far round from an arc's start a place stands, in degrees.
fn round_the_arc(sketch: &Sketch, arc: ArcId, place: DVec2) -> f64 {
    let drawn = sketch.arc_draft(arc);
    ((place - drawn.centre).to_angle() - (drawn.start - drawn.centre).to_angle())
        .rem_euclid(std::f64::consts::TAU)
        .to_degrees()
}

fn margin() -> f64 {
    CLEAR_OF_AN_END.to_degrees()
}

#[test]
fn a_free_point_comes_onto_a_curve_whichever_was_clicked_first() {
    for point_first in [true, false] {
        for kind in ["arc", "circle", "ellipse"] {
            let mut document = blank();
            let curve = match kind {
                "arc" => Element::Arc(an_arc(&mut document)),
                "circle" => Element::Circle(a_circle(&mut document)),
                _ => Element::Ellipse(an_ellipse(&mut document, None)),
            };
            let apart = a_trait_apart(&mut document);
            let place = match kind {
                "ellipse" => off_the_ellipse(ELLIPSE_CENTRE, 60.0, 12.0),
                _ => around(CENTRE, 45.0, 50.0),
            };
            document.apply(Operation::AddPoint {
                sketch: 0,
                position: place,
                on: Vec::new(),
            });
            let before = document.sketches()[0].clone();
            let free = point_at(&before, place);
            let curve_points: Vec<PointId> = match curve {
                Element::Arc(arc) => arc_points(&before, arc),
                Element::Circle(circle) => vec![before.circle(circle).center],
                Element::Ellipse(ellipse) => before.ellipse_points(ellipse).to_vec(),
                _ => unreachable!(),
            };

            let outcome = laid(&mut document, free, curve, point_first);

            let sketch = &document.sketches()[0];
            let at = sketch.point(free);
            let off = match curve {
                Element::Arc(arc) => sketch.distance_to_arc(arc, at),
                Element::Circle(circle) => {
                    let round = sketch.circle(circle);
                    (sketch.point(round.center).distance(at) - round.radius).abs()
                }
                Element::Ellipse(ellipse) => sketch.distance_to_ellipse(ellipse, at),
                _ => unreachable!(),
            };
            let case = format!("a free point and {kind}, point first {point_first}");
            assert_eq!(outcome, None, "{case}: the rule was refused");
            assert!(
                moved(&before, sketch, &curve_points) < ON,
                "{case}: the curve moved to meet a free point"
            );
            assert!(
                sketch
                    .circles()
                    .iter()
                    .all(|round| (round.radius - RADIUS).abs() < ON),
                "{case}: the circle changed size"
            );
            assert!(off < ON_THE_CURVE, "{case}: the point stands {off} off it");
            assert!(
                moved(&before, sketch, &ends(sketch, apart)) < ON,
                "{case}: a trait apart moved"
            );
        }
    }
}

#[test]
fn an_arc_comes_to_the_end_of_a_trait_clicked_first_keeping_its_radius() {
    let mut document = blank();
    let arc = an_arc(&mut document);
    let tip = around(CENTRE, 45.0, 50.0);
    let line = a_trait(&mut document, DVec2::new(110.0, 60.0), tip);
    let apart = a_trait_apart(&mut document);
    let before = document.sketches()[0].clone();
    let point = point_at(&before, tip);

    let outcome = laid(&mut document, point, Element::Arc(arc), true);

    let sketch = &document.sketches()[0];
    let (start, sweep) = arc_run(sketch, arc);
    let reach = sketch.point(sketch.arc(arc).center).distance(tip);
    assert_eq!(outcome, None, "the rule was refused");
    assert!(
        moved(&before, sketch, &ends(sketch, line)) < ON,
        "the trait clicked first, or its end, moved"
    );
    assert!(
        (sketch.arc_radius(arc) - RADIUS).abs() < ON,
        "the arc's radius went from {RADIUS} to {}",
        sketch.arc_radius(arc)
    );
    assert!(
        (reach - RADIUS).abs() < ON_THE_CURVE,
        "the arc's circle does not pass through the point: it stands {reach} from the centre"
    );
    assert!(
        start.abs() < 1e-3 && (sweep - 90.0).abs() < 1e-3,
        "the point fell inside the arc, and the arc turned or opened: it starts at {start}° \
         and runs {sweep}°"
    );
    assert!(
        moved(&before, sketch, &ends(sketch, apart)) < ON,
        "a trait apart moved"
    );
}

#[test]
fn an_arc_lengthens_past_the_end_of_a_trait_it_does_not_reach() {
    let mut document = blank();
    let arc = an_arc(&mut document);
    let tip = around(CENTRE, -30.0, 50.0);
    let line = a_trait(&mut document, DVec2::new(100.0, -10.0), tip);
    let apart = a_trait_apart(&mut document);
    let before = document.sketches()[0].clone();
    let point = point_at(&before, tip);

    let outcome = laid(&mut document, point, Element::Arc(arc), true);

    let sketch = &document.sketches()[0];
    let (start, sweep) = arc_run(sketch, arc);
    let centre = sketch.point(sketch.arc(arc).center);
    let way = (centre - CENTRE).to_angle().to_degrees();
    assert_eq!(outcome, None, "the rule was refused");
    assert!(
        moved(&before, sketch, &ends(sketch, line)) < ON,
        "the trait clicked first, or its end, moved"
    );
    assert!(
        (sketch.arc_radius(arc) - RADIUS).abs() < ON,
        "the arc's radius went from {RADIUS} to {}",
        sketch.arc_radius(arc)
    );
    assert!(
        (way + 30.0).abs() < 1e-3,
        "the arc came at {way}°, not straight towards the point"
    );
    assert!(
        (start - (-30.0 - margin())).abs() < 1e-3,
        "the arc's start went round to {start}°, not a little past the point"
    );
    assert!(
        (sweep - (90.0 + 30.0 + margin())).abs() < 1e-3,
        "the arc runs {sweep}°: its opening did not grow to reach past the point"
    );
    assert!(
        sketch.distance_to_arc(arc, tip) < ON_THE_CURVE,
        "the point is off the arc drawn"
    );
    assert!(
        sketch.point(sketch.arc(arc).start).distance(tip) > 1.0,
        "the arc ends on the point, two points one on the other"
    );
    assert!(
        moved(&before, sketch, &ends(sketch, apart)) < ON,
        "a trait apart moved"
    );
}

#[test]
fn a_circle_comes_to_the_end_of_a_trait_clicked_first() {
    let mut document = blank();
    let circle = a_circle(&mut document);
    let tip = around(CENTRE, -30.0, 50.0);
    let line = a_trait(&mut document, DVec2::new(100.0, -10.0), tip);
    let apart = a_trait_apart(&mut document);
    let before = document.sketches()[0].clone();
    let point = point_at(&before, tip);

    let outcome = laid(&mut document, point, Element::Circle(circle), true);

    let sketch = &document.sketches()[0];
    let round = sketch.circle(circle);
    let centre = sketch.point(round.center);
    let expected = around(CENTRE, -30.0, 30.0);
    assert_eq!(outcome, None, "the rule was refused");
    assert!(
        moved(&before, sketch, &ends(sketch, line)) < ON,
        "the trait clicked first, or its end, moved"
    );
    assert!(
        (round.radius - RADIUS).abs() < ON,
        "the circle's radius went from {RADIUS} to {}",
        round.radius
    );
    assert!(
        centre.distance(expected) < ON_THE_CURVE,
        "the circle's centre went to {centre}, not straight towards the point to {expected}"
    );
    assert!(
        moved(&before, sketch, &ends(sketch, apart)) < ON,
        "a trait apart moved"
    );
}

/// The two axes of an ellipse, as the reach of each and the way the first
/// runs, in degrees.
fn axes(sketch: &Sketch, ellipse: EllipseId) -> (f64, f64, f64) {
    let drawn = sketch.ellipse_draft(ellipse);
    (
        drawn.first.length(),
        drawn.second,
        drawn.first.to_angle().to_degrees(),
    )
}

/// The turn round an ellipse a place on it stands at, in degrees from
/// nought to a whole turn.
fn turn_of(sketch: &Sketch, ellipse: EllipseId, place: DVec2) -> f64 {
    sketch
        .ellipse_draft(ellipse)
        .turn_nearest(place)
        .to_degrees()
        .rem_euclid(360.0)
}

#[test]
fn half_an_ellipse_left_by_a_cut_lengthens_to_the_end_of_a_trait() {
    let mut document = blank();
    let ellipse = an_ellipse(&mut document, Some([20.0, 160.0]));
    let tip = off_the_ellipse(ELLIPSE_CENTRE, -20.0, 10.0);
    let line = a_trait(&mut document, DVec2::new(100.0, -80.0), tip);
    let apart = a_trait_apart(&mut document);
    let before = document.sketches()[0].clone();
    let point = point_at(&before, tip);
    let shape = axes(&before, ellipse);

    let outcome = laid(&mut document, point, Element::Ellipse(ellipse), true);

    let sketch = &document.sketches()[0];
    let (opens, closes) = sketch.ellipse_ends(ellipse).expect("still half an ellipse");
    let (from, to) = (
        turn_of(sketch, ellipse, sketch.point(opens)),
        turn_of(sketch, ellipse, sketch.point(closes)),
    );
    let at = turn_of(sketch, ellipse, tip);
    assert_eq!(outcome, None, "the rule was refused");
    assert!(
        moved(&before, sketch, &ends(sketch, line)) < ON,
        "the trait clicked first, or its end, moved"
    );
    let now = axes(sketch, ellipse);
    assert!(
        (now.0 - shape.0).abs() < ON
            && (now.1 - shape.1).abs() < ON
            && (now.2 - shape.2).abs() < 1e-3,
        "the ellipse changed shape, from {shape:?} to {now:?}"
    );
    assert!(
        (at - 340.0).abs() < 1e-3,
        "the ellipse came so that the point stands at {at}°, not straight towards it"
    );
    assert!(
        (from - (340.0 - margin())).abs() < 1e-3 && (to - 160.0).abs() < 1e-3,
        "the half drawn runs from {from}° to {to}°: it did not lengthen a little past the point"
    );
    assert!(
        sketch.distance_to_ellipse(ellipse, tip) < ON_THE_CURVE,
        "the point is off the half drawn"
    );
    assert!(
        moved(&before, sketch, &ends(sketch, apart)) < ON,
        "a trait apart moved"
    );
}

#[test]
fn half_an_ellipse_drawn_by_its_ends_comes_to_a_point_without_lengthening() {
    let mut document = blank();
    let ellipse = half_an_ellipse_by_its_ends(&mut document);
    let tip = off_the_ellipse(ELLIPSE_CENTRE, -30.0, 10.0);
    let line = a_trait(&mut document, DVec2::new(100.0, -80.0), tip);
    let apart = a_trait_apart(&mut document);
    let before = document.sketches()[0].clone();
    let point = point_at(&before, tip);
    let shape = axes(&before, ellipse);
    let drawn_ends = before.ellipse_ends(ellipse);

    let outcome = laid(&mut document, point, Element::Ellipse(ellipse), true);

    let sketch = &document.sketches()[0];
    let at = turn_of(sketch, ellipse, tip);
    assert_eq!(outcome, None, "the rule was refused");
    assert!(
        moved(&before, sketch, &ends(sketch, line)) < ON,
        "the trait clicked first, or its end, moved"
    );
    let now = axes(sketch, ellipse);
    assert!(
        (now.0 - shape.0).abs() < ON
            && (now.1 - shape.1).abs() < ON
            && (now.2 - shape.2).abs() < 1e-3,
        "the ellipse changed shape, from {shape:?} to {now:?}"
    );
    assert_eq!(
        sketch.ellipse_ends(ellipse),
        drawn_ends,
        "the half no longer runs between the ends of its axis"
    );
    assert!(
        (at - margin()).abs() < 1e-3,
        "the point stands at {at}° round the half, not a little short of its end"
    );
    assert!(
        sketch.distance_to_ellipse(ellipse, tip) < ON_THE_CURVE,
        "the point is off the half drawn"
    );
    assert!(
        moved(&before, sketch, &ends(sketch, apart)) < ON,
        "a trait apart moved"
    );
}

#[test]
fn the_end_of_a_trait_comes_onto_the_drawn_part_of_an_arc_clicked_first() {
    let mut document = blank();
    let arc = an_arc(&mut document);
    let tip = around(CENTRE, 45.0, 50.0);
    let line = a_trait(&mut document, DVec2::new(110.0, 60.0), tip);
    let apart = a_trait_apart(&mut document);
    let before = document.sketches()[0].clone();
    let point = point_at(&before, tip);

    let outcome = laid(&mut document, point, Element::Arc(arc), false);

    let sketch = &document.sketches()[0];
    let landed = sketch.point(point);
    let expected = around(CENTRE, 45.0, RADIUS);
    assert_eq!(outcome, None, "the rule was refused");
    assert!(
        moved(&before, sketch, &arc_points(sketch, arc)) < ON,
        "the arc clicked first moved"
    );
    assert!(
        landed.distance(expected) < ON_THE_CURVE,
        "the point landed at {landed}, not at the nearest place on the arc, {expected}"
    );
    assert!(
        moved(&before, sketch, &[ends(sketch, line)[0]]) < ON,
        "the far end of the point's trait moved"
    );
    assert!(
        moved(&before, sketch, &ends(sketch, apart)) < ON,
        "a trait apart moved"
    );
}

#[test]
fn the_end_of_a_trait_lands_just_short_of_the_end_of_an_arc_it_is_past() {
    for (degrees, short_of) in [(-30.0, 0.0), (130.0, 90.0)] {
        let mut document = blank();
        let arc = an_arc(&mut document);
        let tip = around(CENTRE, degrees, 50.0);
        let _line = a_trait(&mut document, DVec2::new(100.0, 100.0), tip);
        let apart = a_trait_apart(&mut document);
        let before = document.sketches()[0].clone();
        let point = point_at(&before, tip);

        let outcome = laid(&mut document, point, Element::Arc(arc), false);

        let sketch = &document.sketches()[0];
        let landed = sketch.point(point);
        let round = round_the_arc(sketch, arc, landed);
        let expected = match short_of == 0.0 {
            true => margin(),
            false => 90.0 - margin(),
        };
        assert_eq!(outcome, None, "the rule was refused");
        assert!(
            moved(&before, sketch, &arc_points(sketch, arc)) < ON,
            "the arc clicked first moved"
        );
        assert!(
            sketch.distance_to_arc(arc, landed) < ON_THE_CURVE,
            "the point at {degrees}° landed off the arc drawn"
        );
        assert!(
            (round - expected).abs() < 1e-3,
            "the point at {degrees}° landed {round}° round the arc, not a little short of its \
             end at {short_of}°"
        );
        assert!(
            moved(&before, sketch, &ends(sketch, apart)) < ON,
            "a trait apart moved"
        );
    }
}

#[test]
fn the_end_of_a_trait_comes_onto_the_drawn_half_of_an_ellipse_clicked_first() {
    for half in ["cut", "by its ends"] {
        let mut document = blank();
        let ellipse = match half {
            "cut" => an_ellipse(&mut document, Some([20.0, 160.0])),
            _ => half_an_ellipse_by_its_ends(&mut document),
        };
        let tip = off_the_ellipse(ELLIPSE_CENTRE, -30.0, 10.0);
        let _line = a_trait(&mut document, DVec2::new(100.0, -80.0), tip);
        let apart = a_trait_apart(&mut document);
        let before = document.sketches()[0].clone();
        let point = point_at(&before, tip);
        let opens = match half {
            "cut" => 20.0,
            _ => 0.0,
        };

        let outcome = laid(&mut document, point, Element::Ellipse(ellipse), false);

        let sketch = &document.sketches()[0];
        let landed = sketch.point(point);
        let at = turn_of(sketch, ellipse, landed);
        assert_eq!(outcome, None, "{half}: the rule was refused");
        assert!(
            moved(&before, sketch, &sketch.ellipse_stands_on(ellipse)) < ON,
            "{half}: the ellipse clicked first moved"
        );
        assert!(
            sketch.distance_to_ellipse(ellipse, landed) < ON_THE_CURVE,
            "{half}: the point landed off the half drawn"
        );
        assert!(
            (at - (opens + margin())).abs() < 1e-3,
            "{half}: the point landed at {at}°, not a little short of the end at {opens}°"
        );
        assert!(
            moved(&before, sketch, &ends(sketch, apart)) < ON,
            "{half}: a trait apart moved"
        );
    }
}

/// What a curve stands on, and a circle's size, read off the drawing.
fn curve_shape(sketch: &Sketch, curve: Element) -> Vec<DVec2> {
    match curve {
        Element::Circle(circle) => {
            let round = sketch.circle(circle);
            vec![sketch.point(round.center), DVec2::splat(round.radius)]
        }
        Element::Arc(arc) => arc_points(sketch, arc)
            .into_iter()
            .map(|point| sketch.point(point))
            .collect(),
        Element::Ellipse(ellipse) => sketch
            .ellipse_stands_on(ellipse)
            .into_iter()
            .map(|point| sketch.point(point))
            .collect(),
        _ => unreachable!(),
    }
}

/// How far a point stands from what is drawn of a curve.
fn off_what_is_drawn(sketch: &Sketch, curve: Element, point: PointId) -> f64 {
    let place = sketch.point(point);
    match curve {
        Element::Circle(circle) => {
            let round = sketch.circle(circle);
            (sketch.point(round.center).distance(place) - round.radius).abs()
        }
        Element::Arc(arc) => sketch.distance_to_arc(arc, place),
        Element::Ellipse(ellipse) => sketch.distance_to_ellipse(ellipse, place),
        _ => unreachable!(),
    }
}

#[test]
fn a_point_comes_onto_a_fixed_curve_clicked_second() {
    for case in [
        "a fixed arc",
        "a fixed circle",
        "a circle on the origin",
        "an ellipse whose centre is fixed",
    ] {
        let mut document = blank();
        let (curve, centre) = match case {
            "a fixed arc" => (Element::Arc(an_arc(&mut document)), CENTRE),
            "a fixed circle" => (Element::Circle(a_circle(&mut document)), CENTRE),
            "a circle on the origin" => {
                document.apply(Operation::AddCircle {
                    sketch: 0,
                    center: PointRef::Existing(Sketch::ORIGIN),
                    radius: RADIUS,
                    rim: Vec::new(),
                    construction: false,
                });
                (Element::Circle(CircleId(0)), DVec2::ZERO)
            }
            _ => (
                Element::Ellipse(an_ellipse(&mut document, None)),
                ELLIPSE_CENTRE,
            ),
        };
        let fixed = match case {
            "a fixed arc" | "a fixed circle" => Some(curve),
            "an ellipse whose centre is fixed" => Some(Element::Point(point_at(
                &document.sketches()[0],
                ELLIPSE_CENTRE,
            ))),
            _ => None,
        };
        if let Some(element) = fixed {
            document.apply(Operation::Constrain {
                sketch: 0,
                constraint: Constraint::Fixed { element },
            });
        }
        let tip = around(centre, 45.0, 50.0);
        let line = a_trait(&mut document, around(centre, 30.0, 80.0), tip);
        let apart = a_trait_apart(&mut document);
        let before = document.sketches()[0].clone();
        let point = point_at(&before, tip);

        let outcome = laid(&mut document, point, curve, true);

        let sketch = &document.sketches()[0];
        let shape = curve_shape(&before, curve);
        let changed = shape
            .iter()
            .zip(curve_shape(sketch, curve))
            .map(|(was, is)| was.distance(is))
            .fold(0.0, f64::max);
        let off = off_what_is_drawn(sketch, curve, point);
        assert_eq!(outcome, None, "{case}: the rule was refused");
        assert!(
            changed < ON,
            "{case}: the curve moved or changed size, by {changed}"
        );
        assert!(
            off < ON_THE_CURVE,
            "{case}: the point stands {off} off the curve"
        );
        assert!(
            moved(&before, sketch, &[ends(sketch, line)[0]]) < ON,
            "{case}: the far end of the point's trait moved"
        );
        assert!(
            moved(&before, sketch, &ends(sketch, apart)) < ON,
            "{case}: a trait apart moved"
        );
    }
}

#[test]
fn a_point_on_the_circle_of_an_arc_past_its_end_still_lands_in_order() {
    for point_first in [true, false] {
        let mut document = blank();
        let arc = an_arc(&mut document);
        let tip = around(CENTRE, 180.0, RADIUS);
        let line = a_trait(&mut document, DVec2::new(-20.0, 30.0), tip);
        let apart = a_trait_apart(&mut document);
        let before = document.sketches()[0].clone();
        let point = point_at(&before, tip);

        let outcome = laid(&mut document, point, Element::Arc(arc), point_first);

        let sketch = &document.sketches()[0];
        let off = sketch.distance_to_arc(arc, sketch.point(point));
        assert_eq!(
            outcome, None,
            "point first {point_first}: the rule was refused"
        );
        assert!(
            off < ON_THE_CURVE,
            "point first {point_first}: the point stands {off} off what is drawn of the arc"
        );
        match point_first {
            true => assert!(
                moved(&before, sketch, &ends(sketch, line)) < ON,
                "the point and its trait, clicked first, moved"
            ),
            false => assert!(
                moved(&before, sketch, &arc_points(sketch, arc)) < ON,
                "the arc, clicked first, moved"
            ),
        }
        assert!(
            moved(&before, sketch, &ends(sketch, apart)) < ON,
            "point first {point_first}: a trait apart moved"
        );
    }
}

#[test]
fn a_point_held_past_the_end_of_an_arc_stays_there_when_the_history_is_compacted() {
    let mut document = blank();
    let arc = an_arc(&mut document);
    let tip = around(CENTRE, 45.0, 50.0);
    a_trait(&mut document, DVec2::new(110.0, 60.0), tip);
    let point = point_at(&document.sketches()[0], tip);
    laid(&mut document, point, Element::Arc(arc), false);
    let past = document.sketches()[0].slide(point, around(CENTRE, 150.0, RADIUS));
    document.apply(Operation::MovePoint {
        sketch: 0,
        point,
        position: past,
        merged_into: None,
        on: Vec::new(),
        let_go: false,
    });
    let before = document.sketches()[0].clone();
    assert!(
        before.distance_to_arc(arc, before.point(point)) > 1.0,
        "the drag did not take the point past the end of the arc"
    );

    document.compact_history();

    let sketch = &document.sketches()[0];
    let gone: Vec<DVec2> = before
        .live_points()
        .map(|(_, place)| place)
        .filter(|place| sketch.nearest_point(*place, ON).is_none())
        .collect();
    assert!(
        gone.is_empty(),
        "the compaction moved the points at {gone:?}"
    );
}

#[test]
fn a_fillet_lengthens_to_a_point_clicked_first_its_trait_turning_with_its_end() {
    let mut document = blank();
    let corner = DVec2::new(40.0, 30.0);
    a_trait(&mut document, corner, DVec2::new(70.0, 30.0));
    let at = point_at(&document.sketches()[0], corner);
    document.apply(Operation::AddSegment {
        sketch: 0,
        start: PointRef::Existing(at),
        end: PointRef::New(DVec2::new(40.0, 60.0)),
        construction: false,
    });
    document.apply(Operation::Fillet {
        sketch: 0,
        corners: vec![Corner::At(at)],
        radius: 5.0.into(),
    });
    let arc = ArcId(0);
    let tip = DVec2::new(100.0, -20.0);
    let line = a_trait(&mut document, DVec2::new(120.0, -20.0), tip);
    let apart = a_trait_apart(&mut document);
    let before = document.sketches()[0].clone();
    let point = point_at(&before, tip);
    let (radius, sweep) = (before.arc_radius(arc), before.arc_sweep(arc).to_degrees());

    let outcome = laid(&mut document, point, Element::Arc(arc), true);

    let sketch = &document.sketches()[0];
    let off = sketch.distance_to_arc(arc, tip);
    let now = sketch.arc_sweep(arc).to_degrees();
    assert_eq!(outcome, None, "the rule was refused");
    assert!(
        moved(&before, sketch, &ends(sketch, line)) < ON,
        "the point clicked first, or its trait, moved"
    );
    assert!(
        off < ON_THE_CURVE,
        "the point stands {off} off what is drawn"
    );
    assert!(
        (sketch.arc_radius(arc) - radius).abs() < ON,
        "the radius went from {radius} to {}",
        sketch.arc_radius(arc)
    );
    assert!(
        now > sweep + 1.0,
        "the fillet did not lengthen: {sweep}° to {now}°"
    );
    assert!(
        moved(&before, sketch, &ends(sketch, apart)) < ON,
        "a trait apart moved"
    );
}
