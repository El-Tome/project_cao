//! A point held on half an ellipse drawn by its ends stays on the curve drawn,
//! whatever gesture solves the drawing, as it does on a whole ellipse.
//!
//! Closes #531.
//! - it does not move when the shape does not: erasing, laying a rule,
//!   dragging a point or typing a value elsewhere —
//!   `a_point_held_on_half_an_ellipse_stays_where_it_is_when_a_gesture_elsewhere_solves_the_drawing`
//! - it follows the curve when the curve changes —
//!   `a_point_held_on_half_an_ellipse_follows_the_curve_when_its_rise_changes`
//! - dragged, it slides along the curve, and the curve keeps its rise —
//!   `a_point_held_on_half_an_ellipse_slides_along_it_and_the_rise_stays`
//! - trimmed at a point held on it, the new end stays on the curve —
//!   `half_an_ellipse_trimmed_at_a_point_held_on_it_keeps_the_new_end_on_the_curve`
//! - the areas follow: a trait from the curve to the chord still meets the
//!   curve once a gesture elsewhere has solved the drawing —
//!   `the_areas_of_half_an_ellipse_cut_by_a_trait_stay_as_they_were`
//! - found by the owner trying it on 2026-10-09: a rule elsewhere that moves
//!   the half ellipse leaves the point on the half drawn, not slid round to
//!   the half that is not — `a_point_held_on_half_an_ellipse_stays_on_the_half_drawn_when_a_rule_moves_it`
//! - the issue has no "Done when"; these are its "What it does", and the
//!   gestures it measured — no test: agreed with the owner on 2026-10-09

use cao_part::{Operation, PartDocument, PointRef};
use cao_sketch::{
    Constraint, DimensionTarget, Element, EllipseId, PointId, SegmentId, Sketch, Support, WorkPlane,
};
use glam::DVec2;

/// How far a point nothing moves may be found from where it was.
const STILL: f64 = 1e-4;

/// How far off the curve a point the drawing brought there may stand: the
/// solver stops at a hundred-thousandth of the drawing's size, and these
/// drawings are under two hundred units across.
const ON_THE_CURVE: f64 = 1e-3;

const CENTRE: DVec2 = DVec2::new(40.0, 20.0);
const WIDE: f64 = 30.0;
const RISE: f64 = 20.0;

/// The half ellipse sixty wide and twenty high, at a turn round it.
fn on_the_curve(degrees: f64) -> DVec2 {
    let turn = degrees.to_radians();
    CENTRE + DVec2::new(WIDE * turn.cos(), RISE * turn.sin())
}

fn blank() -> PartDocument {
    let mut document = PartDocument::new("Test", "2026-01-02T09:00:00Z".parse().unwrap());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document
}

fn a_trait(document: &mut PartDocument, start: PointRef, end: PointRef) -> SegmentId {
    document.apply(Operation::AddSegment {
        sketch: 0,
        start,
        end,
        construction: false,
    });
    SegmentId(document.sketches()[0].segments().len() - 1)
}

/// Half an ellipse as the tool lays it by its ends: the first axis across the
/// chord, the second laid out from the centre to the rise, and the half above
/// the chord drawn.
fn half_an_ellipse(document: &mut PartDocument) -> EllipseId {
    let (left, right) = (
        PointRef::New(CENTRE - DVec2::X * WIDE),
        PointRef::New(CENTRE + DVec2::X * WIDE),
    );
    let centre = PointRef::New(CENTRE);
    document.apply(Operation::AddEllipse {
        sketch: 0,
        center: centre.clone(),
        first: [left.clone(), right.clone()],
        second: [centre, PointRef::New(CENTRE + DVec2::Y * RISE)],
        construction: false,
        drawn: Some([right, left]),
    });
    EllipseId(document.sketches()[0].ellipses().len() - 1)
}

/// Half an ellipse with a trait coming up from below to end on its curve at
/// a turn, and two traits apart that the gestures elsewhere act on.
struct Drawing {
    document: PartDocument,
    ellipse: EllipseId,
    held: PointId,
    apart: [SegmentId; 2],
}

fn drawing(degrees: f64) -> Drawing {
    let mut document = blank();
    let ellipse = half_an_ellipse(&mut document);
    let line = a_trait(
        &mut document,
        PointRef::New(on_the_curve(degrees) - DVec2::Y * 40.0),
        PointRef::Held {
            at: on_the_curve(degrees),
            on: vec![Support::Ellipse(ellipse)],
        },
    );
    let apart = [
        a_trait(
            &mut document,
            PointRef::New(DVec2::new(-60.0, 60.0)),
            PointRef::New(DVec2::new(-40.0, 80.0)),
        ),
        a_trait(
            &mut document,
            PointRef::New(DVec2::new(-60.0, 20.0)),
            PointRef::New(DVec2::new(-30.0, 20.0)),
        ),
    ];
    let held = document.sketches()[0].segments()[line.0].end;
    Drawing {
        document,
        ellipse,
        held,
        apart,
    }
}

fn off_the_curve(sketch: &Sketch, ellipse: EllipseId, point: PointId) -> f64 {
    sketch.distance_to_ellipse(ellipse, sketch.point(point))
}

/// Where a drag towards a place lets a point go: along what holds it, as the
/// application takes it at every frame before it records the drop.
fn dragged(sketch: &Sketch, point: PointId, toward: DVec2) -> DVec2 {
    sketch.slide(point, toward)
}

fn rise(sketch: &Sketch, ellipse: EllipseId) -> f64 {
    sketch.ellipse_draft(ellipse).second
}

/// Each gesture the issue found setting the point off, made elsewhere.
fn gestures_elsewhere(
    apart: [SegmentId; 2],
    sketch: &Sketch,
) -> Vec<(&'static str, Vec<Operation>)> {
    let end = sketch.segments()[apart[0].0].end;
    let length = |segment: SegmentId, by: f64| Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(segment),
        value: (sketch.segment_length(segment) + by).into(),
        placement: None,
    };
    vec![
        (
            "erasing a trait elsewhere",
            vec![Operation::EraseMany {
                sketch: 0,
                elements: vec![Element::Segment(apart[1])],
                dimensions: Vec::new(),
                constraints: Vec::new(),
            }],
        ),
        (
            "laying a rule elsewhere",
            vec![Operation::Constrain {
                sketch: 0,
                constraint: Constraint::Perpendicular {
                    first: apart[0],
                    second: apart[1],
                },
            }],
        ),
        (
            "dragging a point elsewhere",
            vec![Operation::MovePoint {
                sketch: 0,
                point: end,
                position: DVec2::new(-35.0, 90.0),
                merged_into: None,
                on: Vec::new(),
                let_go: false,
            }],
        ),
        (
            "typing a second value elsewhere",
            vec![length(apart[0], 0.0), length(apart[1], 5.0)],
        ),
    ]
}

#[test]
fn a_point_held_on_half_an_ellipse_stays_where_it_is_when_a_gesture_elsewhere_solves_the_drawing() {
    for degrees in [30.0, 60.0, 80.0, 120.0, 150.0] {
        let template = drawing(degrees);
        let gestures = gestures_elsewhere(template.apart, &template.document.sketches()[0]);
        for (gesture, operations) in gestures {
            let Drawing {
                mut document,
                ellipse,
                held,
                ..
            } = drawing(degrees);
            let before = document.sketches()[0].point(held);

            for operation in operations {
                document.apply(operation);
            }

            let sketch = &document.sketches()[0];
            let moved = sketch.point(held).distance(before);
            let off = off_the_curve(sketch, ellipse, held);
            assert!(
                moved < STILL,
                "{gesture}, a point held at {degrees}° moved by {moved}"
            );
            assert!(
                off < ON_THE_CURVE,
                "{gesture}, a point held at {degrees}° stands {off} off the curve"
            );
            assert!(
                (rise(sketch, ellipse) - RISE).abs() < STILL,
                "{gesture}, the rise went from {RISE} to {}",
                rise(sketch, ellipse)
            );
        }
    }
}

#[test]
fn a_point_held_on_half_an_ellipse_follows_the_curve_when_its_rise_changes() {
    let Drawing {
        mut document,
        ellipse,
        held,
        ..
    } = drawing(60.0);
    let top = document.sketches()[0].ellipse_points(ellipse)[4];

    document.apply(Operation::MovePoint {
        sketch: 0,
        point: top,
        position: CENTRE + DVec2::Y * (RISE + 10.0),
        merged_into: None,
        on: Vec::new(),
        let_go: false,
    });

    let sketch = &document.sketches()[0];
    let off = off_the_curve(sketch, ellipse, held);
    assert!(
        (rise(sketch, ellipse) - (RISE + 10.0)).abs() < STILL,
        "the rise is {}, the top was not dragged",
        rise(sketch, ellipse)
    );
    assert!(off < ON_THE_CURVE, "the point stands {off} off the curve");
}

#[test]
fn a_point_held_on_half_an_ellipse_slides_along_it_and_the_rise_stays() {
    let Drawing {
        mut document,
        ellipse,
        held,
        ..
    } = drawing(60.0);
    let landing = dragged(
        &document.sketches()[0],
        held,
        on_the_curve(100.0) + DVec2::new(3.0, 4.0),
    );

    document.apply(Operation::MovePoint {
        sketch: 0,
        point: held,
        position: landing,
        merged_into: None,
        on: Vec::new(),
        let_go: false,
    });

    let sketch = &document.sketches()[0];
    let off = off_the_curve(sketch, ellipse, held);
    let slid = sketch.point(held).distance(on_the_curve(60.0));
    assert!(off < ON_THE_CURVE, "the point stands {off} off the curve");
    assert!(slid > 1.0, "the point did not slide along the curve");
    assert!(
        (rise(sketch, ellipse) - RISE).abs() < STILL,
        "the rise went from {RISE} to {}",
        rise(sketch, ellipse)
    );
}

#[test]
fn half_an_ellipse_trimmed_at_a_point_held_on_it_keeps_the_new_end_on_the_curve() {
    let Drawing {
        mut document,
        ellipse,
        held,
        ..
    } = drawing(60.0);
    let (_, left) = document.sketches()[0]
        .ellipse_ends(ellipse)
        .expect("half an ellipse");

    document.apply(Operation::TrimEllipse {
        sketch: 0,
        ellipse,
        between: Some((held, left)),
    });

    let sketch = &document.sketches()[0];
    let (_, end) = sketch.ellipse_ends(ellipse).expect("still a stretch");
    let off = sketch
        .ellipse_draft(ellipse)
        .at(sketch
            .ellipse_draft(ellipse)
            .turn_nearest(sketch.point(end)))
        .distance(sketch.point(end));
    assert_eq!(end, held, "the stretch left does not end at the point");
    assert!(off < ON_THE_CURVE, "the new end stands {off} off the curve");
}

#[test]
fn the_areas_of_half_an_ellipse_cut_by_a_trait_stay_as_they_were() {
    let mut document = blank();
    let ellipse = half_an_ellipse(&mut document);
    let (right, left) = document.sketches()[0]
        .ellipse_ends(ellipse)
        .expect("half an ellipse");
    let chord = a_trait(
        &mut document,
        PointRef::Existing(left),
        PointRef::Existing(right),
    );
    a_trait(
        &mut document,
        PointRef::Held {
            at: on_the_curve(60.0),
            on: vec![Support::Ellipse(ellipse)],
        },
        PointRef::Held {
            at: DVec2::new(on_the_curve(60.0).x, CENTRE.y),
            on: vec![Support::Segment(chord)],
        },
    );
    let apart = a_trait(
        &mut document,
        PointRef::New(DVec2::new(-60.0, 20.0)),
        PointRef::New(DVec2::new(-30.0, 20.0)),
    );
    let areas = |sketch: &Sketch| -> Vec<f64> {
        let mut areas: Vec<f64> = sketch.regions().iter().map(|area| area.area()).collect();
        areas.sort_by(f64::total_cmp);
        areas
    };
    let before = areas(&document.sketches()[0]);

    document.apply(Operation::EraseMany {
        sketch: 0,
        elements: vec![Element::Segment(apart)],
        dimensions: Vec::new(),
        constraints: Vec::new(),
    });

    let after = areas(&document.sketches()[0]);
    assert_eq!(before.len(), 2, "the trait does not cut the half in two");
    assert_eq!(
        after.len(),
        before.len(),
        "the areas went from {before:?} to {after:?}"
    );
    for (was, is) in before.iter().zip(&after) {
        assert!(
            (was - is).abs() < 1e-3,
            "the areas went from {before:?} to {after:?}"
        );
    }
}

#[test]
fn a_point_held_on_half_an_ellipse_stays_on_the_half_drawn_when_a_rule_moves_it() {
    let mut document = blank();
    let (left, right) = (
        PointRef::New(DVec2::new(80.0, 80.0)),
        PointRef::New(DVec2::new(200.0, 80.0)),
    );
    let centre = PointRef::New(DVec2::new(140.0, 80.0));
    document.apply(Operation::AddEllipse {
        sketch: 0,
        center: centre.clone(),
        first: [left.clone(), right.clone()],
        second: [centre, PointRef::New(DVec2::new(140.0, 120.0))],
        construction: false,
        drawn: Some([right, left]),
    });
    let ellipse = EllipseId(0);
    let rising = a_trait(
        &mut document,
        PointRef::New(DVec2::new(175.0, 15.0)),
        PointRef::New(DVec2::new(160.0, 100.0)),
    );
    let held = document.sketches()[0].segments()[rising.0].end;
    document.apply(Operation::MovePoint {
        sketch: 0,
        point: held,
        position: DVec2::new(148.95921556069476, 119.55155542123052),
        merged_into: None,
        on: vec![Support::Ellipse(ellipse)],
        let_go: false,
    });
    let across = a_trait(
        &mut document,
        PointRef::New(DVec2::new(92.28858627645354, 178.69110424376063)),
        PointRef::New(DVec2::new(285.0, 160.0)),
    );
    document.apply(Operation::Constrain {
        sketch: 0,
        constraint: Constraint::Perpendicular {
            first: rising,
            second: across,
        },
    });

    let outcome = document.apply(Operation::Constrain {
        sketch: 0,
        constraint: Constraint::AxisPerpendicular {
            segment: across,
            axis: cao_sketch::SketchAxis::U,
        },
    });

    let sketch = &document.sketches()[0];
    let off = off_the_curve(sketch, ellipse, held);
    let way = |segment: SegmentId| {
        let (start, end) = sketch.endpoints(segment);
        (end - start).normalize()
    };
    assert_eq!(outcome, None, "the rule was refused");
    assert!(
        way(rising).dot(way(across)).abs() < 1e-6 && way(across).x.abs() < 1e-6,
        "the rules do not hold: the two traits at {} and {}",
        way(rising),
        way(across)
    );
    assert!(
        off < ON_THE_CURVE,
        "the point stands {off} off the half drawn, at {}",
        sketch.point(held)
    );
}
