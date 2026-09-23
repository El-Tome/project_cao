//! What app · screens/viewport/input/reading.rs is held to.
//!
//! Closes #176.
//! - a point then another shows the distance between them —
//!   `a_point_then_another_shows_how_far_apart_they_are`
//! - a trait shows its length — `a_trait_shows_its_length`
//! - a point then a trait shows the shortest distance onto it —
//!   `a_point_then_a_trait_shows_the_distance_square_onto_it`
//! - and the same the other way round, a trait then a point —
//!   `a_trait_then_a_point_shows_the_same_distance_the_other_way_round`
//! - a trait then a parallel trait shows the gap between them —
//!   `two_parallel_traits_show_the_gap_between_them`
//! - a trait then a trait that is not parallel shows the angle —
//!   `two_traits_that_meet_show_the_angle_they_open`
//! - a circle, or an arc, shows its radius and its diameter —
//!   `a_circle_shows_a_round_the_label_says_both_ways`
//! - nothing enters the history and the part is not marked as modified —
//!   `measuring_the_whole_drawing_leaves_the_history_empty`
//! - the next measure clears the one before it —
//!   `a_second_measure_replaces_the_first_rather_than_joining_it`
//! - clicking away from everything clears what was shown —
//!   `a_click_on_nothing_clears_what_was_being_shown`
//!
//! Closes #425.
//! - clicking inside a closed area shows its surface and how far round it is —
//!   `a_click_inside_a_closed_area_reads_its_surface_and_the_way_round_it`
//! - a shape drawn inside another gives the innermost area under the cursor —
//!   `a_click_inside_two_nested_shapes_takes_the_inner_one`
//! - clicking where no area closes shows nothing and clears what was there —
//!   `a_click_where_nothing_closes_reads_no_area_and_clears_what_was_there`
//! - nothing is added to the history and the part is not marked as modified —
//!   `measuring_the_whole_drawing_leaves_the_history_empty`, which now reads an
//!   area too
//! - the area read is tinted while it is on screen — no test: the tinting is
//!   asserted in `render/reading/tests.rs`, and a bullet may only name a test
//!   of its own file
//! - a hole comes out of the surface, and the curve is honoured rather than
//!   the polygon — no test: both are geometry, asserted in
//!   `sketch/regions/measure/tests.rs`
//! - the dashed line and the label are drawn from the measure — no test: the
//!   drawing is asserted in `render/reading/tests.rs`, and a bullet may only
//!   name a test of its own file
//! - `Échap` and taking another tool clear the measure — no test: both go
//!   through `SketchEditor::reset_pending`, asserted in `screens/sketch/tests.rs`
//! - undo, redo and any change to the drawing clear it — no test: undo and
//!   redo go through `clamp_editor_to_document`, which calls `reset_pending`,
//!   and an ordinary edit through `forget_the_measure`; both are asserted in
//!   `screens/sketch/tests.rs`
//! - the smart dimension tool still places real dimensions — no test: it is
//!   untouched here, and its own tests in `sketch/measuring/tests.rs` hold it

use cao_part::PartDocument;
use cao_part::history::{Operation, PointRef};
use cao_sketch::{Measured, PointId, Reading, WorkPlane};
use chrono::Utc;

use super::*;
use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::{SketchEditor, Tool};

const TOLERANCE: f64 = 1e-9;

/// A rectangle 40 across and 25 up, and a circle of radius 12.5 well clear of
/// it.
///
/// A rectangle rather than a square on purpose: with two equal sides, the gap
/// between the two parallels and the length of either of them are the same
/// number, and a test that reads the wrong one still passes.
struct Drawing {
    document: PartDocument,
    editor: SketchEditor,
}

impl Drawing {
    fn new() -> Self {
        let mut document = PartDocument::new("part", Utc::now());
        document.apply(Operation::CreateSketch {
            plane: WorkPlane::XY,
            on: None,
        });
        Self {
            document,
            editor: SketchEditor {
                tool: Tool::Measure,
                ..SketchEditor::default()
            },
        }
    }

    /// One click of the measure tool, and whether the part changed because
    /// of it — which, for a measure, is always the answer `false`.
    fn click(&mut self, at: DVec2) -> bool {
        let lang = Catalogue::french();
        let mut extrusion = ExtrusionState::default();
        let mut context = SketchContext {
            document: &mut self.document,
            editor: &mut self.editor,
            extrusion: &mut extrusion,
            lang: &lang,
        };
        read(&mut context, 0, at, 1.0)
    }

    fn reading(&self) -> Option<Reading> {
        let showing = match &self.editor.tool_state {
            ToolState::Measure { showing, .. } => (*showing)?,
            _ => return None,
        };
        let sketch = &self.document.sketches()[0];
        match showing {
            Measured::Of(target) => sketch.read(target),
            Measured::Inside(place) => sketch.read_inside(place),
        }
    }

    fn gap(&self) -> (f64, DVec2) {
        match self.reading() {
            Some(Reading::Gap { span, offsets }) => (span, offsets),
            other => panic!("a straight run was expected, got {other:?}"),
        }
    }
}

/// Lays a closed shape, its corners shared between one side and the next.
///
/// A side laying its own two ends would leave eight points where four were
/// wanted, and a loop that never closes: no area at all.
fn lay_a_shape(drawing: &mut Drawing, corners: &[DVec2]) {
    let laid: Vec<PointRef> = corners
        .iter()
        .map(|place| {
            drawing.document.apply(Operation::AddPoint {
                sketch: 0,
                position: *place,
                on: Vec::new(),
            });
            PointRef::Existing(PointId(drawing.document.sketches()[0].points().len() - 1))
        })
        .collect();
    for side in 0..laid.len() {
        drawing.document.apply(Operation::AddSegment {
            sketch: 0,
            start: laid[side].clone(),
            end: laid[(side + 1) % laid.len()].clone(),
            construction: false,
        });
    }
}

fn a_square_and_a_circle() -> Drawing {
    let mut drawing = Drawing::new();
    lay_a_shape(
        &mut drawing,
        &[
            DVec2::new(0.0, 0.0),
            DVec2::new(40.0, 0.0),
            DVec2::new(40.0, 25.0),
            DVec2::new(0.0, 25.0),
        ],
    );
    drawing.document.apply(Operation::AddCircle {
        sketch: 0,
        center: PointRef::New(DVec2::new(100.0, 100.0)),
        radius: 12.5,
        rim: Vec::new(),
        construction: false,
    });
    drawing
}

#[test]
fn a_point_then_another_shows_how_far_apart_they_are() {
    let mut drawing = a_square_and_a_circle();

    drawing.click(DVec2::new(0.0, 0.0));
    assert!(
        drawing.reading().is_none(),
        "one point alone is half a distance, and shows nothing yet",
    );
    drawing.click(DVec2::new(40.0, 25.0));

    let (span, offsets) = drawing.gap();
    assert!(
        (span - 2225.0_f64.sqrt()).abs() < TOLERANCE,
        "the diagonal of the rectangle, got {span}",
    );
    assert!(
        (offsets.x - 40.0).abs() < TOLERANCE && (offsets.y - 25.0).abs() < TOLERANCE,
        "the reach along each axis comes with it, got {offsets:?}",
    );
}

#[test]
fn a_trait_shows_its_length() {
    let mut drawing = a_square_and_a_circle();

    drawing.click(DVec2::new(20.0, 0.0));

    let (span, offsets) = drawing.gap();
    assert!(
        (span - 40.0).abs() < TOLERANCE,
        "the side of the square, got {span}",
    );
    assert!(
        (offsets.x - 40.0).abs() < TOLERANCE && offsets.y.abs() < TOLERANCE,
        "a flat trait reaches along one axis only, got {offsets:?}",
    );
}

#[test]
fn a_point_then_a_trait_shows_the_distance_square_onto_it() {
    let mut drawing = a_square_and_a_circle();

    drawing.click(DVec2::new(0.0, 25.0));
    drawing.click(DVec2::new(20.0, 0.0));

    let (span, _) = drawing.gap();
    assert!(
        (span - 25.0).abs() < TOLERANCE,
        "a top corner stands twenty-five above the bottom side, got {span}",
    );
}

#[test]
fn a_trait_then_a_point_shows_the_same_distance_the_other_way_round() {
    let mut drawing = a_square_and_a_circle();

    drawing.click(DVec2::new(20.0, 0.0));
    drawing.click(DVec2::new(0.0, 25.0));

    let (span, _) = drawing.gap();
    assert!(
        (span - 25.0).abs() < TOLERANCE,
        "which of the two was clicked first is not a thing the user should \
         have to think about, got {span}",
    );
}

#[test]
fn two_parallel_traits_show_the_gap_between_them() {
    let mut drawing = a_square_and_a_circle();

    drawing.click(DVec2::new(20.0, 0.0));
    drawing.click(DVec2::new(20.0, 25.0));

    let (span, _) = drawing.gap();
    assert!(
        (span - 25.0).abs() < TOLERANCE,
        "the gap, and not either trait's own length of forty, got {span}",
    );
}

#[test]
fn two_traits_that_meet_show_the_angle_they_open() {
    let mut drawing = a_square_and_a_circle();

    drawing.click(DVec2::new(20.0, 0.0));
    drawing.click(DVec2::new(0.0, 12.5));

    let Some(Reading::Opening { degrees }) = drawing.reading() else {
        panic!(
            "two sides that meet open an angle, got {:?}",
            drawing.reading()
        );
    };
    assert!(
        (degrees - 90.0).abs() < TOLERANCE,
        "the corner of the rectangle, got {degrees}",
    );
}

#[test]
fn a_circle_shows_a_round_the_label_says_both_ways() {
    let mut drawing = a_square_and_a_circle();

    drawing.click(DVec2::new(112.5, 100.0));

    let Some(Reading::Round { radius }) = drawing.reading() else {
        panic!("a circle reads as a round, got {:?}", drawing.reading());
    };
    assert!(
        (radius - 12.5).abs() < TOLERANCE,
        "the radius, which the label doubles for the diameter, got {radius}",
    );
}

#[test]
fn a_click_inside_a_closed_area_reads_its_surface_and_the_way_round_it() {
    let mut drawing = a_square_and_a_circle();

    drawing.click(DVec2::new(20.0, 12.0));

    let Some(Reading::Surface { area, perimeter }) = drawing.reading() else {
        panic!("the rectangle closes an area, got {:?}", drawing.reading());
    };
    assert!(
        (area - 1000.0).abs() < TOLERANCE,
        "forty across and twenty-five up, got {area}",
    );
    assert!(
        (perimeter - 130.0).abs() < TOLERANCE,
        "twice each side, got {perimeter}",
    );
}

#[test]
fn a_click_inside_two_nested_shapes_takes_the_inner_one() {
    let mut drawing = a_square_and_a_circle();
    lay_a_shape(
        &mut drawing,
        &[
            DVec2::new(10.0, 5.0),
            DVec2::new(30.0, 5.0),
            DVec2::new(30.0, 20.0),
            DVec2::new(10.0, 20.0),
        ],
    );

    drawing.click(DVec2::new(20.0, 12.0));

    let Some(Reading::Surface { area, .. }) = drawing.reading() else {
        panic!("a closed area was read, got {:?}", drawing.reading());
    };
    assert!(
        (area - 300.0).abs() < TOLERANCE,
        "the inner shape is what the cursor is in, and the outer one is only \
         what it is in as well; got {area}",
    );
}

#[test]
fn a_click_where_nothing_closes_reads_no_area_and_clears_what_was_there() {
    let mut drawing = a_square_and_a_circle();
    drawing.click(DVec2::new(20.0, 12.0));
    assert!(drawing.reading().is_some(), "the area was read");

    drawing.click(DVec2::new(500.0, 500.0));

    assert!(
        drawing.reading().is_none(),
        "outside everything there is no area to report",
    );
}

#[test]
fn measuring_the_whole_drawing_leaves_the_history_empty() {
    let mut drawing = a_square_and_a_circle();
    let before = drawing.document.history.operations().len();

    let changed = [
        DVec2::new(0.0, 0.0),
        DVec2::new(40.0, 25.0),
        DVec2::new(20.0, 0.0),
        DVec2::new(0.0, 12.5),
        DVec2::new(112.5, 100.0),
        DVec2::new(20.0, 12.0),
        DVec2::new(500.0, 500.0),
    ]
    .into_iter()
    .any(|at| drawing.click(at));

    assert!(
        !changed,
        "a measure that reported a change would mark the part as modified",
    );
    assert_eq!(
        drawing.document.history.operations().len(),
        before,
        "nothing a measure does is a step of the design",
    );
}

#[test]
fn a_second_measure_replaces_the_first_rather_than_joining_it() {
    let mut drawing = a_square_and_a_circle();

    drawing.click(DVec2::new(20.0, 0.0));
    let (first, _) = drawing.gap();
    drawing.click(DVec2::new(112.5, 100.0));

    assert!(
        matches!(drawing.reading(), Some(Reading::Round { .. })),
        "one measure at a time: the circle takes the place of the trait",
    );
    assert!(
        (first - 40.0).abs() < TOLERANCE,
        "the first measure was genuinely read before being replaced",
    );
}

#[test]
fn a_click_on_nothing_clears_what_was_being_shown() {
    let mut drawing = a_square_and_a_circle();

    drawing.click(DVec2::new(20.0, 0.0));
    assert!(drawing.reading().is_some(), "the trait was read");
    drawing.click(DVec2::new(500.0, 500.0));

    assert!(
        drawing.reading().is_none(),
        "a value nothing under the cursor answers for has to go",
    );
}
