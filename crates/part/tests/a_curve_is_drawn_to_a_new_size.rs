//! A circle and an arc drawn to a new size by hand, about the centre they
//! already have.
//!
//! Closes #375.
//! - dragging a circle's outline changes its radius and leaves its centre
//!   alone — `a_circle_drawn_to_a_new_size_keeps_the_centre_it_had`
//! - a point held on the curve follows the new size —
//!   `a_point_held_on_the_rim_follows_the_new_size`
//! - dragging an arc's curve changes its reach and keeps the sweep it was
//!   drawn with — `an_arc_drawn_to_a_new_size_keeps_its_sweep`
//! - the step undoes in one — `one_undo_takes_back_a_size_drawn_by_hand`
//! - and survives compaction —
//!   `compacting_the_history_keeps_the_size_the_shape_ends_at`
//! - a click on either still selects it — no test: a click is not a drag, and
//!   only `response.drag_started()` reaches the grab (the canvas has no net,
//!   see docs/code-map.md)
//! - a size the drawing cannot take leaves the shape as it was — no test: it
//!   is `Sketch::resize_circle`, held beside the geometry by
//!   `a_size_a_typed_value_forbids_leaves_the_circle_as_it_was`
//! - dragging the centre, and a box from empty space, are untouched — no
//!   test: the grab is reached only where no point, no selection and no
//!   annotation was taken, and the box only where nothing at all was

use cao_part::{Operation, PartDocument, PointRef};
use cao_sketch::{ArcId, CircleId, PointId, WorkPlane};
use chrono::{DateTime, Utc};
use glam::DVec2;

fn at(text: &str) -> DateTime<Utc> {
    text.parse().expect("a date")
}

/// A circle of radius 10 about (20, 30), and a quarter arc beside it.
fn a_circle_and_an_arc() -> PartDocument {
    let mut document = PartDocument::new("Test", at("2026-01-02T09:00:00Z"));
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddCircle {
        sketch: 0,
        center: PointRef::New(DVec2::new(20.0, 30.0)),
        radius: 10.0,
        rim: Vec::new(),
        construction: false,
    });
    document.apply(Operation::AddArc {
        sketch: 0,
        center: PointRef::New(DVec2::new(60.0, 30.0)),
        start: PointRef::New(DVec2::new(70.0, 30.0)),
        end: PointRef::New(DVec2::new(60.0, 40.0)),
        construction: false,
    });
    document
}

#[test]
fn a_circle_drawn_to_a_new_size_keeps_the_centre_it_had() {
    let mut document = a_circle_and_an_arc();

    document.apply(Operation::ResizeCircle {
        sketch: 0,
        circle: CircleId(0),
        reach: 16.0,
    });

    let drawing = &document.sketches()[0];
    let circle = drawing.circle(CircleId(0));
    assert!((circle.radius - 16.0).abs() < 1e-6, "{}", circle.radius);
    let centre = drawing.point(circle.center);
    assert!(
        centre.distance(DVec2::new(20.0, 30.0)) < 1e-9,
        "the centre stayed where it was, not at {centre}",
    );
}

#[test]
fn a_point_held_on_the_rim_follows_the_new_size() {
    let mut document = a_circle_and_an_arc();
    document.apply(Operation::AddPoint {
        sketch: 0,
        position: DVec2::new(30.0, 30.0),
        on: vec![cao_sketch::Support::Circle(CircleId(0))],
    });
    let held = PointId(document.sketches()[0].points().len() - 1);

    document.apply(Operation::ResizeCircle {
        sketch: 0,
        circle: CircleId(0),
        reach: 16.0,
    });

    let drawing = &document.sketches()[0];
    let circle = drawing.circle(CircleId(0));
    let reach = drawing.point(held).distance(drawing.point(circle.center));
    assert!(
        (reach - 16.0).abs() < 1e-6,
        "the point held on the rim stands {reach} out, and the rim 16",
    );
}

#[test]
fn an_arc_drawn_to_a_new_size_keeps_its_sweep() {
    let mut document = a_circle_and_an_arc();
    let sweep = document.sketches()[0].arc_sweep(ArcId(0));

    document.apply(Operation::ResizeArc {
        sketch: 0,
        arc: ArcId(0),
        reach: 25.0,
    });

    let drawing = &document.sketches()[0];
    let reach = drawing.arc_radius(ArcId(0));
    assert!((reach - 25.0).abs() < 1e-6, "the curve stands {reach} out");
    let now = drawing.arc_sweep(ArcId(0));
    assert!(
        (now - sweep).abs() < 1e-6,
        "and it still runs {sweep} round, not {now}",
    );
}

#[test]
fn one_undo_takes_back_a_size_drawn_by_hand() {
    let mut document = a_circle_and_an_arc();

    document.apply(Operation::ResizeCircle {
        sketch: 0,
        circle: CircleId(0),
        reach: 16.0,
    });
    document.undo();

    let radius = document.sketches()[0].circle(CircleId(0)).radius;
    assert!((radius - 10.0).abs() < 1e-9, "back to {radius}");
}

#[test]
fn compacting_the_history_keeps_the_size_the_shape_ends_at() {
    let mut document = a_circle_and_an_arc();
    document.apply(Operation::ResizeCircle {
        sketch: 0,
        circle: CircleId(0),
        reach: 16.0,
    });
    document.apply(Operation::ResizeArc {
        sketch: 0,
        arc: ArcId(0),
        reach: 25.0,
    });

    document.compact_history();

    let drawing = &document.sketches()[0];
    assert!((drawing.circle(CircleId(0)).radius - 16.0).abs() < 1e-6);
    assert!((drawing.arc_radius(ArcId(0)) - 25.0).abs() < 1e-6);
}
