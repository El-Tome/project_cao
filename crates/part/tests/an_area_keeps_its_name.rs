//! What a step of matter still stands on once the drawing has moved under it.
//!
//! An integration test rather than a colocated one: every door it goes
//! through is a public one, and it is the whole chain — the click naming the
//! area, the cuts carrying that name, the replay asking the drawing for it —
//! rather than any single piece of it.
//!
//! Closes #345.
//! - an area whose drawing is dragged raises the same matter as before —
//!   `an_extrusion_follows_the_area_out_from_under_the_place_it_was_clicked`
//! - an area whose drawing is dimensioned out from under the place clicked
//!   keeps its matter —
//!   `an_extrusion_holds_its_area_when_a_value_moves_the_drawing_off_it`
//! - an area one of whose bounding traits is rounded or chamfered raises the
//!   same matter as before —
//!   `rounding_a_corner_of_an_extruded_shape_keeps_the_matter`,
//!   `chamfering_a_corner_of_an_extruded_shape_keeps_the_matter`
//! - an area one of whose bounding traits is divided raises the same matter
//!   as before — `dividing_a_border_of_an_extruded_shape_keeps_the_matter`
//! - an area one of whose bounding traits is erased raises nothing —
//!   `erasing_a_border_of_an_extruded_shape_leaves_no_matter`
//! - the name is taken at the click and never worked out again later —
//!   `an_area_drawn_after_the_click_is_not_the_one_that_was_clicked`
//! - compaction re-emits a sketch whose curves carry the names the extrusions
//!   after it point at — `compaction_hands_an_extrusion_its_area_back`
//! - undo, redo and going back to a step are unchanged — no test:
//!   `crates/part/src/history/tests.rs` walks the cursor, and nothing here
//!   touches it; a name lives in the operation, which the cursor only
//!   includes or leaves out

use cao_part::history::{ExtrusionMode, Operation, PointRef};
use cao_part::{History, PartState};
use cao_sketch::{Chamfer, DimensionTarget, Element, PointId, SegmentId, WorkPlane};
use cao_solid::Mesh;
use glam::DVec2;

fn volume(mesh: &Mesh) -> f64 {
    mesh.triangles()
        .iter()
        .map(|[a, b, c]| a.dot(b.cross(*c)) / 6.0)
        .sum()
}

/// A drawing holding one rectangle, ten across and twenty up from the origin.
fn a_rectangle() -> History {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    history.push(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(10.0, 20.0)),
        construction: false,
    });
    history
}

/// Raises four millimetres from the area the place falls in.
fn raise(history: &mut History, place: DVec2) {
    history.push(Operation::Extrude {
        sketch: 0,
        areas: PartState::rebuild(history).areas_at(0, &[place]),
        distance: 4.0,
        mode: ExtrusionMode::Add,
    });
}

#[test]
fn an_extrusion_follows_the_area_out_from_under_the_place_it_was_clicked() {
    let mut history = a_rectangle();
    let clicked = DVec2::new(5.0, 10.0);
    raise(&mut history, clicked);

    // The four corners, not the origin the drawing owns, which is point 0.
    history.push(Operation::MoveMany {
        sketch: 0,
        points: (1..5).map(PointId).collect(),
        by: DVec2::new(100.0, 100.0),
    });

    let state = PartState::rebuild(&history);
    let drawn = &state.sketches[0];
    assert!(
        !drawn.regions()[0].contains(clicked),
        "the drawing has to have left the place clicked behind, or this \
         proves nothing",
    );
    assert!(
        (volume(&state.body) - 800.0).abs() < 1.0,
        "the area is named by the four traits bounding it, and they are the \
         four traits still: {}",
        volume(&state.body),
    );
}

#[test]
fn an_extrusion_holds_its_area_when_a_value_moves_the_drawing_off_it() {
    let mut history = a_rectangle();
    // The first value typed is what fixes what a world unit is worth. Ten
    // millimetres on a side already ten units long leaves the shape alone and
    // makes one unit one millimetre.
    history.push(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(SegmentId(0)),
        value: 10.0,
        placement: None,
    });
    let clicked = DVec2::new(5.0, 10.0);
    raise(&mut history, clicked);

    history.push(Operation::SetDimension {
        sketch: 0,
        target: DimensionTarget::Length(SegmentId(1)),
        value: 2.0,
        placement: None,
    });

    let state = PartState::rebuild(&history);
    let area = &state.sketches[0].regions()[0];
    assert!(
        !area.contains(clicked),
        "the shape has to have moved out from under the place clicked, or \
         this proves nothing",
    );
    // Nothing holds this rectangle down, so where the solver puts it is its
    // own business. What is held is that the matter raised is the area the
    // drawing encloses now, and not the one it enclosed at the click.
    let enclosed: f64 = area
        .face_triangles()
        .iter()
        .map(|[a, b, c]| (b - a).perp_dot(c - a).abs() * 0.5)
        .sum();
    assert!(
        (volume(&state.body) - enclosed * 4.0).abs() < 1.0,
        "{} raised from an area of {enclosed}",
        volume(&state.body),
    );
}

#[test]
fn rounding_a_corner_of_an_extruded_shape_keeps_the_matter() {
    let mut history = a_rectangle();
    raise(&mut history, DVec2::new(5.0, 10.0));

    history.push(Operation::Fillet {
        sketch: 0,
        first: SegmentId(0),
        second: SegmentId(1),
        radius: 2.0,
    });

    let state = PartState::rebuild(&history);
    let raised = volume(&state.body);
    assert!(
        raised > 780.0 && raised < 800.0,
        "the rounded corner takes a sliver off and nothing more: {raised}",
    );
}

#[test]
fn chamfering_a_corner_of_an_extruded_shape_keeps_the_matter() {
    let mut history = a_rectangle();
    raise(&mut history, DVec2::new(5.0, 10.0));

    history.push(Operation::Chamfer {
        sketch: 0,
        first: SegmentId(0),
        second: SegmentId(1),
        mode: Chamfer::Equal(2.0),
    });

    let state = PartState::rebuild(&history);
    let raised = volume(&state.body);
    assert!(
        raised > 780.0 && raised < 800.0,
        "the cut corner takes a sliver off and nothing more: {raised}",
    );
}

#[test]
fn dividing_a_border_of_an_extruded_shape_keeps_the_matter() {
    let mut history = a_rectangle();
    raise(&mut history, DVec2::new(5.0, 10.0));

    // A trait laid across one side, and both cut where they meet: the side
    // the area was named by becomes two traits, and both still bound it.
    history.push(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(5.0, -5.0)),
        end: PointRef::New(DVec2::new(5.0, -1.0)),
        construction: false,
    });
    history.push(Operation::MovePoint {
        sketch: 0,
        point: PointId(5),
        position: DVec2::new(5.0, 5.0),
        merged_into: None,
    });
    history.push(Operation::Split {
        sketch: 0,
        segments: vec![SegmentId(0), SegmentId(4)],
        arcs: Vec::new(),
        at: DVec2::new(5.0, 0.0),
    });

    let state = PartState::rebuild(&history);
    assert!(
        (volume(&state.body) - 800.0).abs() < 1.0,
        "both pieces of the divided side still bound the area: {}",
        volume(&state.body),
    );
}

#[test]
fn erasing_a_border_of_an_extruded_shape_leaves_no_matter() {
    let mut history = a_rectangle();
    raise(&mut history, DVec2::new(5.0, 10.0));

    history.push(Operation::EraseMany {
        sketch: 0,
        elements: vec![Element::Segment(SegmentId(0))],
        dimensions: Vec::new(),
        constraints: Vec::new(),
    });

    let state = PartState::rebuild(&history);
    assert_eq!(
        state.body.triangles().len(),
        0,
        "the area has lost a border: the step stands on nothing, rather than \
         quietly raising some other area",
    );
}

#[test]
fn an_area_drawn_after_the_click_is_not_the_one_that_was_clicked() {
    let mut history = a_rectangle();
    raise(&mut history, DVec2::new(5.0, 10.0));

    // A second rectangle laid over the first, enclosing it. The place that
    // was clicked falls in both.
    history.push(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::new(-5.0, -5.0)),
        opposite: PointRef::New(DVec2::new(15.0, 25.0)),
        construction: false,
    });

    let state = PartState::rebuild(&history);
    assert!(
        (volume(&state.body) - 800.0).abs() < 1.0,
        "the name was taken at the click and says the first rectangle: {}",
        volume(&state.body),
    );
}

#[test]
fn compaction_hands_an_extrusion_its_area_back() {
    let mut history = a_rectangle();
    // A second shape, drawn first and erased afterwards, so that compaction
    // really does hand the numbers it freed to the shape that is left.
    history.push(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::new(30.0, 0.0)),
        opposite: PointRef::New(DVec2::new(40.0, 10.0)),
        construction: false,
    });
    raise(&mut history, DVec2::new(5.0, 10.0));
    history.push(Operation::EraseMany {
        sketch: 0,
        elements: (4..8).map(SegmentId).map(Element::Segment).collect(),
        dimensions: Vec::new(),
        constraints: Vec::new(),
    });

    let compacted = cao_part::compact(&history);

    let after = volume(&PartState::rebuild(&compacted).body);
    assert!(
        (after - 800.0).abs() < 1.0,
        "the re-emitted curves carry the name the extrusion points at: \
         {after} raised out of a rectangle of 800",
    );
}
