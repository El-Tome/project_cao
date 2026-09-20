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
//!   as before, whether that trait bordered it alone or was shared with the
//!   area next to it — `dividing_a_border_of_an_extruded_shape_keeps_the_matter`,
//!   `dividing_a_border_two_areas_share_keeps_the_matter`
//! - an area one of whose bounding traits is erased raises nothing —
//!   `erasing_a_border_of_an_extruded_shape_leaves_no_matter`
//! - the name is taken at the click and never worked out again later —
//!   `an_area_drawn_after_the_click_is_not_the_one_that_was_clicked`
//! - compaction re-emits a sketch whose curves carry the names the extrusions
//!   after it point at — `compaction_hands_an_extrusion_its_area_back`,
//!   `compaction_hands_back_an_area_whose_corners_were_cut`
//! - a revolution follows its area the way an extrusion does —
//!   `a_revolution_follows_its_area_out_from_under_the_place_clicked`
//! - undo, redo and going back to a step are unchanged — no test:
//!   `crates/part/src/history/tests.rs` walks the cursor, and nothing here
//!   touches it; a name lives in the operation, which the cursor only
//!   includes or leaves out

use cao_part::history::{ExtrusionMode, Operation, PointRef, RevolutionAxis};
use cao_part::{History, PartState};
use cao_sketch::{
    Chamfer, Corner, DimensionTarget, Element, PointId, SegmentId, SketchAxis, WorkPlane,
};
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
        corners: vec![Corner::Between(SegmentId(0), SegmentId(1))],
        radius: 2.0,
    });

    let state = PartState::rebuild(&history);
    let raised = volume(&state.body);
    assert!(
        (raised - 796.54).abs() < 0.1,
        "the rounded corner takes its sliver off and nothing more: {raised}",
    );
}

#[test]
fn chamfering_a_corner_of_an_extruded_shape_keeps_the_matter() {
    let mut history = a_rectangle();
    raise(&mut history, DVec2::new(5.0, 10.0));

    history.push(Operation::Chamfer {
        sketch: 0,
        corners: vec![Corner::Between(SegmentId(0), SegmentId(1))],
        mode: Chamfer::Equal(2.0),
    });

    let state = PartState::rebuild(&history);
    let raised = volume(&state.body);
    assert!(
        (raised - 792.0).abs() < 0.1,
        "the cut corner takes its sliver off and nothing more: {raised}",
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
        let_go: false,
        on: Vec::new(),
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
    let clicked = DVec2::new(5.0, 10.0);
    raise(&mut history, clicked);

    // A smaller rectangle drawn inside the first, around the very place that
    // was clicked. The place now falls in the small one, which is innermost
    // and is what looking for the place again would find; the name says the
    // four traits that bounded what was really clicked, and the drawing has
    // left those bounding a shape with a hole in it.
    history.push(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::new(2.0, 5.0)),
        opposite: PointRef::New(DVec2::new(8.0, 15.0)),
        construction: false,
    });

    let state = PartState::rebuild(&history);
    let raised = volume(&state.body);
    assert!(
        (raised - 560.0).abs() < 1.0,
        "the ring of matter around the hole, 140 mm² four deep — not the \
         240 the small rectangle alone would give: {raised}",
    );
}

#[test]
fn dividing_a_border_two_areas_share_keeps_the_matter() {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    history.push(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(20.0, 10.0)),
        construction: false,
    });
    // An upright straight through it, so the two halves share the bottom
    // trait, the top trait and the upright itself.
    history.push(Operation::AddSegment {
        sketch: 0,
        start: PointRef::New(DVec2::new(10.0, -2.0)),
        end: PointRef::New(DVec2::new(10.0, 12.0)),
        construction: false,
    });
    history.push(Operation::Extrude {
        sketch: 0,
        areas: PartState::rebuild(&history).areas_at(0, &[DVec2::new(5.0, 5.0)]),
        distance: 4.0,
        mode: ExtrusionMode::Add,
    });
    assert!((volume(&PartState::rebuild(&history).body) - 400.0).abs() < 1.0);

    history.push(Operation::Split {
        sketch: 0,
        segments: vec![SegmentId(0), SegmentId(4)],
        arcs: Vec::new(),
        at: DVec2::new(10.0, 0.0),
    });

    assert!(
        (volume(&PartState::rebuild(&history).body) - 400.0).abs() < 1.0,
        "the bottom trait was cut in two and only one piece borders this \
         half: asking for both would lose it, and asking for neither would \
         lose what a border means — {}",
        volume(&PartState::rebuild(&history).body),
    );
}

#[test]
fn a_revolution_follows_its_area_out_from_under_the_place_clicked() {
    let mut history = History::default();
    history.push(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    history.push(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::new(4.0, 0.0)),
        opposite: PointRef::New(DVec2::new(8.0, 6.0)),
        construction: false,
    });
    let clicked = DVec2::new(6.0, 3.0);
    history.push(Operation::Revolve {
        sketch: 0,
        areas: PartState::rebuild(&history).areas_at(0, &[clicked]),
        axis: RevolutionAxis::Sketch(SketchAxis::V),
        angle: 360.0,
        mode: ExtrusionMode::Add,
    });
    let swept = volume(&PartState::rebuild(&history).body);
    assert!(swept > 1.0, "the ring was swept at all: {swept}");

    history.push(Operation::MoveMany {
        sketch: 0,
        points: (1..5).map(PointId).collect(),
        by: DVec2::new(40.0, 0.0),
    });

    let state = PartState::rebuild(&history);
    assert!(
        !state.sketches[0].regions()[0].contains(clicked),
        "the drawing has to have left the place clicked behind",
    );
    assert!(
        volume(&state.body) > swept,
        "a ring swept further out holds more than the one before it: \
         {swept} then {}",
        volume(&state.body),
    );
}

#[test]
fn compaction_hands_back_an_area_whose_corners_were_cut() {
    let mut history = a_rectangle();
    // A second, smaller shape drawn after it. Compaction renumbers from
    // nothing, and what the chamfers leave of the first shape is numbered
    // after this one — so a name still speaking of the traits the chamfers
    // took out would land on this rectangle instead.
    history.push(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::new(30.0, 0.0)),
        opposite: PointRef::New(DVec2::new(40.0, 10.0)),
        construction: false,
    });
    raise(&mut history, DVec2::new(5.0, 10.0));
    for (first, second) in [(0, 1), (2, 3)] {
        history.push(Operation::Chamfer {
            sketch: 0,
            corners: vec![Corner::Between(SegmentId(first), SegmentId(second))],
            mode: Chamfer::Equal(2.0),
        });
    }
    let before = volume(&PartState::rebuild(&history).body);
    assert!(before > 780.0, "the big rectangle was raised: {before}");

    let compacted = cao_part::compact(&history);

    let after = volume(&PartState::rebuild(&compacted).body);
    assert!(
        (after - before).abs() < 1.0,
        "compaction drops the chamfers and re-emits what they left, so a \
         name still speaking of the traits they took out names the other \
         rectangle: {before} before, {after} after",
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
