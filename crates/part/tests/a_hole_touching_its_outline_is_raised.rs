//! An area whose hole touches its outline is raised, pinched at the touch.
//!
//! Closes #609.
//! - the drawing the application lays — a rectangle, a circle, « Tangence »
//!   to one side — is raised, the plate carrying its pinched hole —
//!   `the_drawing_the_application_lays_is_raised_pinched_at_the_touch`
//! - the campaign shape tangent to two sides raises both its areas —
//!   `the_campaign_shape_tangent_to_two_sides_raises_both_its_areas`
//! - a hole well inside its outline keeps building —
//!   `a_hole_well_inside_its_outline_keeps_building`
//! - a profile the kernel rightly declines stays declined, with its message —
//!   `an_elliptic_hole_touching_its_outline_stays_declined`
//! - a part saved with the declined extrusion rebuilds to the built state —
//!   no test: `REBUILT_BY` moved, and the geometry-cache tests beside it hold
//!   that a cache written by an older build is replayed

use std::f64::consts::PI;

use cao_part::history::{ExtrusionMode, Operation, PointRef};
use cao_part::{History, PartDocument, PartState};
use cao_sketch::{
    Area, CircleId, Constraint, LaidFrom, PointId, SegmentId, SketchAxis, Support, WorkPlane,
};
use cao_solid::Declined;
use glam::DVec2;

fn clicked(history: &History, sketch: usize, places: &[DVec2]) -> Vec<Area> {
    PartState::rebuild(history).areas_at(sketch, places)
}

fn last_step(history: &History) -> u32 {
    history.steps().last().expect("a step").operations()[0]
}

fn applied_live(history: &History) -> PartDocument {
    let now = "2026-10-20T09:00:00Z".parse().expect("a date");
    let mut part = PartDocument::new("Pinched", now);
    for operation in history.operations() {
        part.apply(operation.clone());
    }
    part
}

fn sketch_history() -> History {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    history
}

fn extruded(history: &mut History, places: &[DVec2]) -> PartDocument {
    history.push(Operation::Extrude {
        sketch: 0,
        areas: clicked(history, 0, places),
        distance: 10.0.into(),
        mode: ExtrusionMode::Add,
    });
    applied_live(history)
}

/// The drawing the application records when a rectangle is dropped on the
/// axes, a circle is drawn on the U axis, and « Tangence » pulls the first
/// side down onto the circle: read from a part the application saved, solver
/// landing included, so the test raises what the user drew and not a shape
/// laid conveniently on the lattice.
fn the_application_s_drawing() -> History {
    let mut history = sketch_history();
    history.push(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::Held {
            at: DVec2::new(0.0, 42.542388900675135),
            on: vec![Support::Axis(SketchAxis::V)],
        },
        opposite: PointRef::New(DVec2::new(125.0, -50.0)),
        construction: false,
    });
    for (first, second) in [(0, 1), (1, 2), (2, 3)] {
        history.push(Operation::Constrain {
            sketch: 0,
            constraint: Constraint::Perpendicular {
                first: SegmentId(first),
                second: SegmentId(second),
            },
        });
    }
    history.push(Operation::AddCircle {
        sketch: 0,
        center: PointRef::Held {
            at: DVec2::new(60.465268548529664, 0.0),
            on: vec![Support::Axis(SketchAxis::U)],
        },
        radius: 35.366209548835926,
        rim: vec![PointRef::Held {
            at: DVec2::new(95.83147809736559, 0.0),
            on: vec![Support::Axis(SketchAxis::U)],
        }],
        construction: false,
    });
    history.push(Operation::Constrain {
        sketch: 0,
        constraint: Constraint::Tangent {
            circle: CircleId(0),
            segment: SegmentId(0),
            at: None,
            from: LaidFrom::Curve,
        },
    });
    history
}

/// The corners of the rectangle as the solver left them, in the order the
/// sides run: what the plate measures is their quadrilateral less the disc.
fn quadrilateral_area(state: &PartState) -> f64 {
    let corners: Vec<DVec2> = [1, 3, 2, 4]
        .iter()
        .map(|rank| state.sketches[0].points()[*rank])
        .collect();
    let mut doubled = 0.0;
    for (rank, corner) in corners.iter().enumerate() {
        doubled += corner.perp_dot(corners[(rank + 1) % corners.len()]);
    }
    (doubled / 2.0).abs()
}

#[test]
fn the_drawing_the_application_lays_is_raised_pinched_at_the_touch() {
    let mut history = the_application_s_drawing();
    let state = PartState::rebuild(&history);
    let quad = quadrilateral_area(&state);
    let radius = state.sketches[0].circle(CircleId(0)).radius;
    let hole_inside = state.sketches[0]
        .points()
        .get(PointId(5).0)
        .copied()
        .expect("the circle's centre");

    let part = extruded(&mut history, &[DVec2::new(10.0, -40.0)]);

    assert_eq!(
        part.declined_because(last_step(&history)),
        None,
        "the kernel raises the pinched plate"
    );
    let expected = (quad - PI * radius * radius) * 10.0;
    let made = part.body().volume();
    // The tangency is landed by the solver to its own net, and the kernel
    // snaps that touch to one corner: what is left between the two volumes is
    // that snap, orders below a measure anyone reads.
    assert!(
        (made - expected).abs() <= 1e-6 * expected,
        "{made} where the arithmetic gives {expected}",
    );
    let (min, max) = part.body().bounds().expect("a volume");
    assert!(
        min.x < hole_inside.x && hole_inside.x < max.x,
        "the plate stands where it was drawn"
    );
}

#[test]
fn the_campaign_shape_tangent_to_two_sides_raises_both_its_areas() {
    let mut history = sketch_history();
    history.push(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::new(240.0, 240.0)),
        opposite: PointRef::New(DVec2::new(330.0, 90.0)),
        construction: false,
    });
    history.push(Operation::AddCircle {
        sketch: 0,
        center: PointRef::New(DVec2::new(285.0, 165.0)),
        radius: 45.0,
        rim: Vec::new(),
        construction: false,
    });

    let part = extruded(
        &mut history,
        &[DVec2::new(285.0, 230.0), DVec2::new(285.0, 100.0)],
    );

    assert_eq!(part.declined_because(last_step(&history)), None);
    let expected = (90.0 * 150.0 - PI * 45.0 * 45.0) * 10.0;
    let made = part.body().volume();
    assert!(
        (made - expected).abs() <= 1e-9 * expected,
        "{made} where the arithmetic gives {expected}",
    );
}

#[test]
fn a_hole_well_inside_its_outline_keeps_building() {
    let mut history = sketch_history();
    history.push(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(100.0, 60.0)),
        construction: false,
    });
    history.push(Operation::AddCircle {
        sketch: 0,
        center: PointRef::New(DVec2::new(50.0, 30.0)),
        radius: 20.0,
        rim: Vec::new(),
        construction: false,
    });

    let part = extruded(&mut history, &[DVec2::new(10.0, 10.0)]);

    assert_eq!(part.declined_because(last_step(&history)), None);
    let expected = (100.0 * 60.0 - PI * 400.0) * 10.0;
    let made = part.body().volume();
    assert!(
        (made - expected).abs() <= 1e-9 * expected,
        "{made} where the arithmetic gives {expected}",
    );
}

#[test]
fn an_elliptic_hole_touching_its_outline_stays_declined() {
    let mut history = sketch_history();
    history.push(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(100.0, 60.0)),
        construction: false,
    });
    history.push(Operation::AddEllipse {
        sketch: 0,
        center: PointRef::New(DVec2::new(50.0, 40.0)),
        first: [
            PointRef::New(DVec2::new(20.0, 40.0)),
            PointRef::New(DVec2::new(80.0, 40.0)),
        ],
        second: [
            PointRef::New(DVec2::new(50.0, 20.0)),
            PointRef::New(DVec2::new(50.0, 60.0)),
        ],
        construction: false,
        drawn: None,
    });

    let part = extruded(&mut history, &[DVec2::new(10.0, 10.0)]);

    assert_eq!(
        part.declined_because(last_step(&history)),
        Some(Declined::Profile),
        "an ellipse touching another loop is still the kernel's honest no"
    );
}
