//! Raising matter from a sketch, and taking it away again.
//!
//! An integration test rather than a colocated one: every door it goes through
//! is a public one, and `state.rs` is long past the length a single file is
//! meant to hold.

use cao_part::history::{ExtrusionMode, Operation, PointRef};
use cao_part::{History, PartState};
use cao_sketch::{DimensionTarget, WorkPlane};
use cao_solid::Mesh;
use glam::{DVec2, DVec3};

fn volume(mesh: &Mesh) -> f64 {
    mesh.triangles()
        .iter()
        .map(|[a, b, c]| a.dot(b.cross(*c)) / 6.0)
        .sum()
}

fn rectangle(history: &mut History, min: DVec2, max: DVec2) {
    history.push(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(min),
        opposite: PointRef::New(max),
        construction: false,
    });
}

fn sketch_history() -> History {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    history
}

#[test]
fn an_extrusion_turns_an_area_into_matter() {
    let mut history = sketch_history();
    rectangle(&mut history, DVec2::ZERO, DVec2::new(10.0, 20.0));
    history.push(Operation::Extrude {
        sketch: 0,
        areas: PartState::rebuild(&history).areas_at(0, &[DVec2::new(5.0, 10.0)]),
        distance: 4.0,
        mode: ExtrusionMode::Add,
    });

    let state = PartState::rebuild(&history);
    assert!((volume(&state.body) - 800.0).abs() < 1.0);
    let (min, max) = state.body.bounds().expect("a volume");
    assert!((max.z - min.z - 4.0).abs() < 1e-3, "the height");
}

/// The clicked area is found again by the point, not by its rank: drawing
/// something else afterwards must not move the extrusion.
#[test]
fn an_extrusion_still_names_the_same_area_after_another_shape_is_drawn() {
    let mut history = sketch_history();
    rectangle(&mut history, DVec2::ZERO, DVec2::new(10.0, 10.0));
    rectangle(&mut history, DVec2::new(40.0, 40.0), DVec2::new(50.0, 60.0));
    history.push(Operation::Extrude {
        sketch: 0,
        areas: PartState::rebuild(&history).areas_at(0, &[DVec2::new(45.0, 50.0)]),
        distance: 2.0,
        mode: ExtrusionMode::Add,
    });

    let state = PartState::rebuild(&history);
    assert!((volume(&state.body) - 400.0).abs() < 1.0, "the second area");
    let (min, _) = state.body.bounds().expect("a volume");
    assert!(min.x > 39.0, "in the right place: {min}");
}

/// Two circles one inside the other make a tube: the tool does not fill the
/// middle.
#[test]
fn two_circles_extrude_to_a_tube() {
    let mut history = sketch_history();
    history.push(Operation::AddCircle {
        sketch: 0,
        center: PointRef::New(DVec2::ZERO),
        radius: 10.0,
        rim: Vec::new(),
        construction: false,
    });
    history.push(Operation::AddCircle {
        sketch: 0,
        center: PointRef::Existing(cao_sketch::PointId(1)),
        radius: 6.0,
        rim: Vec::new(),
        construction: false,
    });
    history.push(Operation::Extrude {
        sketch: 0,
        areas: PartState::rebuild(&history).areas_at(0, &[DVec2::new(8.0, 0.0)]),
        distance: 5.0,
        mode: ExtrusionMode::Add,
    });

    let state = PartState::rebuild(&history);
    let expected = std::f64::consts::PI * (100.0 - 36.0) * 5.0;
    let made = volume(&state.body);
    assert!(
        (made - expected).abs() / expected < 0.03,
        "{made} / {expected}"
    );
}

/// A pocket: the second sketch digs into the block of the first.
#[test]
fn a_cut_takes_matter_away() {
    let mut history = sketch_history();
    rectangle(&mut history, DVec2::ZERO, DVec2::new(10.0, 10.0));
    history.push(Operation::Extrude {
        sketch: 0,
        areas: PartState::rebuild(&history).areas_at(0, &[DVec2::new(5.0, 5.0)]),
        distance: 10.0,
        mode: ExtrusionMode::Add,
    });

    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    history.push(Operation::AddRectangle {
        sketch: 1,
        corner: PointRef::New(DVec2::new(2.0, 2.0)),
        opposite: PointRef::New(DVec2::new(4.0, 4.0)),
        construction: false,
    });
    history.push(Operation::Extrude {
        sketch: 1,
        areas: PartState::rebuild(&history).areas_at(1, &[DVec2::new(3.0, 3.0)]),
        distance: 4.0,
        mode: ExtrusionMode::Cut,
    });

    let state = PartState::rebuild(&history);
    let made = volume(&state.body);
    assert!((made - (1000.0 - 16.0)).abs() < 2.0, "{made}");
}

/// A shape inside another leaves the middle empty from the very first
/// extrusion: the tube case, in rectangles.
#[test]
fn a_shape_inside_another_is_already_hollow() {
    let mut history = sketch_history();
    rectangle(&mut history, DVec2::ZERO, DVec2::new(10.0, 10.0));
    rectangle(&mut history, DVec2::new(2.0, 2.0), DVec2::new(4.0, 4.0));
    history.push(Operation::Extrude {
        sketch: 0,
        areas: PartState::rebuild(&history).areas_at(0, &[DVec2::new(1.0, 1.0)]),
        distance: 10.0,
        mode: ExtrusionMode::Add,
    });

    let made = volume(&PartState::rebuild(&history).body);
    assert!((made - (1000.0 - 40.0)).abs() < 2.0, "{made}");
}

/// A full revolution around an axis of the sketch.
#[test]
fn a_revolution_sweeps_an_area_around_an_axis() {
    let mut history = sketch_history();
    rectangle(&mut history, DVec2::new(3.0, 0.0), DVec2::new(5.0, 2.0));
    history.push(Operation::Revolve {
        sketch: 0,
        areas: PartState::rebuild(&history).areas_at(0, &[DVec2::new(4.0, 1.0)]),
        axis: cao_part::history::RevolutionAxis::Sketch(cao_sketch::SketchAxis::V),
        angle: 360.0,
        mode: ExtrusionMode::Add,
    });

    let made = volume(&PartState::rebuild(&history).body);
    let expected = std::f64::consts::TAU * 4.0 * 4.0;
    assert!(
        (made - expected).abs() / expected < 0.02,
        "{made} / {expected}"
    );
}

/// L'axe peut être un trait qu'on a tracé soi-même.
#[test]
fn a_revolution_can_turn_around_a_drawn_line() {
    let mut history = sketch_history();
    history.push(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(0.0, -10.0)),
        end: PointRef::New(DVec2::new(0.0, 10.0)),
        construction: false,
    });
    rectangle(&mut history, DVec2::new(3.0, 0.0), DVec2::new(5.0, 2.0));
    history.push(Operation::Revolve {
        sketch: 0,
        areas: PartState::rebuild(&history).areas_at(0, &[DVec2::new(4.0, 1.0)]),
        axis: cao_part::history::RevolutionAxis::Segment(cao_sketch::SegmentId(0)),
        angle: 360.0,
        mode: ExtrusionMode::Add,
    });

    let made = volume(&PartState::rebuild(&history).body);
    let expected = std::f64::consts::TAU * 4.0 * 4.0;
    assert!(
        (made - expected).abs() / expected < 0.02,
        "{made} / {expected}"
    );
}

/// A profile astride the axis would pass through itself: nothing is
/// produced, rather than a volume turned inside out.
#[test]
fn a_revolution_across_its_axis_makes_nothing() {
    let mut history = sketch_history();
    rectangle(&mut history, DVec2::new(-4.0, 0.0), DVec2::new(5.0, 2.0));
    history.push(Operation::Revolve {
        sketch: 0,
        areas: PartState::rebuild(&history).areas_at(0, &[DVec2::new(1.0, 1.0)]),
        axis: cao_part::history::RevolutionAxis::Sketch(cao_sketch::SketchAxis::V),
        angle: 360.0,
        mode: ExtrusionMode::Add,
    });

    assert!(PartState::rebuild(&history).body.is_empty());
}

/// The hang met in use, with its real measurements: a cylinder of
/// revolution thirty units in radius, then a sketch laid on its top face
/// and dug into.
///
/// At that distance from the origin a triangle came out of its own plane in
/// `f64`, and the partition of space cut it again without end.
#[test]
fn cutting_into_a_revolved_part_from_its_own_face() {
    let mut history = sketch_history();
    rectangle(
        &mut history,
        DVec2::new(-30.0, -7.5),
        DVec2::new(0.0, -52.5),
    );
    history.push(Operation::Revolve {
        sketch: 0,
        areas: PartState::rebuild(&history).areas_at(0, &[DVec2::new(-14.8, -26.5)]),
        axis: cao_part::history::RevolutionAxis::Sketch(cao_sketch::SketchAxis::V),
        angle: 360.0,
        mode: ExtrusionMode::Add,
    });

    // The plane as the click on the face gives it: its normal and its axes
    // are not exactly aligned, which is part of the case.
    let face = WorkPlane {
        origin: DVec3::new(9.729663e-07, -7.5000005, -1.2972885e-06),
        u: DVec3::new(-1.0, -1.2972883e-07, 0.0),
        v: DVec3::new(2.2439427e-14, -1.7297178e-07, 1.0),
    };
    history.push(Operation::CreateSketch {
        plane: face,
        on: None,
    });
    history.push(Operation::AddRectangle {
        sketch: 1,
        corner: PointRef::New(DVec2::new(-25.0, 32.5)),
        opposite: PointRef::New(DVec2::new(12.5, -7.5)),
        construction: false,
    });
    history.push(Operation::Extrude {
        sketch: 1,
        areas: PartState::rebuild(&history).areas_at(1, &[DVec2::new(-6.0, 12.0)]),
        distance: -10.0,
        mode: ExtrusionMode::Cut,
    });

    let state = PartState::rebuild(&history);
    assert!(!state.body.is_empty());
    assert!(volume(&state.body) > 0.0);
}

/// Two superimposed vertices become one, and the replay does it again
/// identically.
#[test]
fn merging_two_points_replays() {
    let mut history = sketch_history();
    history.push(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(-10.0, 0.0)),
        end: PointRef::New(DVec2::new(0.0, 10.0)),
        construction: false,
    });
    history.push(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(0.0, 10.0)),
        end: PointRef::New(DVec2::new(10.0, 0.0)),
        construction: false,
    });

    let before = PartState::rebuild(&history);
    assert_eq!(before.sketches[0].drawn_points().count(), 4);

    history.push(Operation::MergePoints {
        sketch: 0,
        kept: cao_sketch::PointId(2),
        dropped: cao_sketch::PointId(3),
    });

    let state = PartState::rebuild(&history);
    assert_eq!(state.sketches[0].drawn_points().count(), 3);
    assert_eq!(
        state.sketches[0].live_segments().count(),
        2,
        "both segments still hold the shared corner"
    );
}

/// A selection deleted as a block is one single step, undone by one single
/// undo.
#[test]
fn a_whole_selection_goes_in_one_step() {
    let mut history = sketch_history();
    rectangle(&mut history, DVec2::ZERO, DVec2::new(10.0, 10.0));
    history.push(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(cao_sketch::SegmentId(0)),
        value: 10.0,
        placement: None,
    });
    let drawn = PartState::rebuild(&history).sketches[0]
        .live_segments()
        .count();
    assert_eq!(drawn, 4);

    history.push(Operation::EraseMany {
        sketch: 0,
        elements: vec![
            cao_sketch::Element::Segment(cao_sketch::SegmentId(0)),
            cao_sketch::Element::Segment(cao_sketch::SegmentId(2)),
        ],
        dimensions: vec![DimensionTarget::Length(cao_sketch::SegmentId(0))],
        constraints: Vec::new(),
    });

    let state = PartState::rebuild(&history);
    assert_eq!(state.sketches[0].live_segments().count(), 2);
    assert!(state.sketches[0].dimensions().is_empty());

    history.undo();
    let back = PartState::rebuild(&history);
    assert_eq!(back.sketches[0].live_segments().count(), 4);
    assert_eq!(back.sketches[0].dimensions().len(), 1);
}

/// Erasing a trait an extrusion stands on takes the matter with it, because
/// the erasure belongs to the sketch's step and is replayed before the
/// extrusion — whenever it was typed. That is #364, and what warns about it is
/// #345.
#[test]
fn a_deletion_replays_like_any_other_step() {
    let mut history = sketch_history();
    rectangle(&mut history, DVec2::ZERO, DVec2::new(10.0, 10.0));
    history.push(Operation::Extrude {
        sketch: 0,
        areas: PartState::rebuild(&history).areas_at(0, &[DVec2::new(5.0, 5.0)]),
        distance: 4.0,
        mode: ExtrusionMode::Add,
    });
    let before = volume(&PartState::rebuild(&history).body);
    assert!(before > 0.0);

    history.push(Operation::EraseMany {
        sketch: 0,
        elements: vec![cao_sketch::Element::Segment(cao_sketch::SegmentId(0))],
        dimensions: Vec::new(),
        constraints: Vec::new(),
    });

    let state = PartState::rebuild(&history);
    assert!(state.sketches[0].regions().is_empty(), "the area is open");
    assert!(
        volume(&state.body) < 1.0,
        "the extrusion has nothing left to stand on, so the matter goes: {}",
        volume(&state.body),
    );

    // And the cursor brought back before the deletion returns both.
    history.undo();
    let back = PartState::rebuild(&history);
    assert_eq!(back.sketches[0].regions().len(), 1);
    assert!((volume(&back.body) - before).abs() < 1.0);
}

/// A sketch that is not entirely constrained extrudes all the same.
#[test]
fn an_extrusion_does_not_wait_for_a_settled_sketch() {
    let mut history = sketch_history();
    rectangle(&mut history, DVec2::new(3.0, 3.0), DVec2::new(9.0, 9.0));
    history.push(Operation::Extrude {
        sketch: 0,
        areas: PartState::rebuild(&history).areas_at(0, &[DVec2::new(5.0, 5.0)]),
        distance: 1.0,
        mode: ExtrusionMode::Add,
    });

    let state = PartState::rebuild(&history);
    assert!(!state.sketches[0].is_fully_constrained(1.0));
    assert!(!state.body.is_empty());
}

/// A dimension changes the scale: an extrusion of 10 mm stays 10 mm.
#[test]
fn an_extrusion_is_given_in_millimetres() {
    let mut history = sketch_history();
    rectangle(&mut history, DVec2::ZERO, DVec2::new(1.0, 1.0));
    history.push(Operation::SetDimension {
        sketch: 0,
        target: cao_sketch::DimensionTarget::Length(cao_sketch::SegmentId(0)),
        value: 50.0,
        placement: None,
    });
    history.push(Operation::Extrude {
        sketch: 0,
        areas: PartState::rebuild(&history).areas_at(0, &[DVec2::new(0.5, 0.5)]),
        distance: 25.0,
        mode: ExtrusionMode::Add,
    });

    let state = PartState::rebuild(&history);
    let (min, max) = state.body.bounds().expect("a volume");
    let height_millimetres = (max.z - min.z) * state.scale();
    assert!(
        (height_millimetres - 25.0).abs() < 1e-3,
        "{height_millimetres}"
    );
    let _ = DVec3::ZERO;
}
