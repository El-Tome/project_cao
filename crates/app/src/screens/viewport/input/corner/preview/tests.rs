//! What app · screens/viewport/input/corner/preview.rs is held to.
//!
//! Closes #315.
//! - the preview shows every corner taken, wherever the cursor is —
//!   `every_corner_taken_is_shown_wherever_the_cursor_is`,
//!   `the_side_under_the_cursor_stands_in_for_the_second_click`

use cao_sketch::{ChamferMode, Corner, CutCurve, CutShape, CutSide, SegmentId, WorkPlane};

use super::*;

/// A right angle, one side east and one north, ten units each.
fn a_right_angle() -> (Sketch, SegmentId, SegmentId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let pivot = sketch.add_point(DVec2::new(2.0, 1.0));
    let east = sketch.add_point(DVec2::new(12.0, 1.0));
    let north = sketch.add_point(DVec2::new(2.0, 11.0));
    let along = sketch.add_segment(pivot, east);
    let up = sketch.add_segment(pivot, north);
    (sketch, along, up)
}

fn rounding(taken: Vec<Corner>, half: Option<CutSide>, radius: Option<f64>) -> SketchEditor {
    let mut editor = SketchEditor {
        tool: Tool::Fillet,
        tool_state: ToolState::Corner { taken, half },
        ..Default::default()
    };
    editor.live.field(0).locked = radius;
    editor
}

#[test]
fn a_radius_typed_on_a_corner_already_clicked_shows_the_curve_before_enter() {
    let (sketch, along, up) = a_right_angle();
    let editor = rounding(
        vec![Corner::Between(along.into(), up.into())],
        None,
        Some(3.0),
    );

    let shown = previewed(&sketch, &editor, DVec2::ZERO, 0.5, 1.0)
        .expect("a corner with both sides and a radius has a curve to show");

    assert_eq!(
        shown.cuts.len(),
        1,
        "the cut the click would lay is what is shown"
    );
    assert!(matches!(
        shown
            .sketch
            .drawn_cut(shown.cuts[0])
            .map(|drawn| drawn.curve),
        Some(CutCurve::Arc(_))
    ));
    assert_eq!(
        sketch.live_corner_cuts().count(),
        0,
        "the drawing itself is not rounded until the click"
    );
}

#[test]
fn the_side_under_the_cursor_stands_in_for_the_second_click() {
    let (sketch, along, _) = a_right_angle();
    let editor = rounding(Vec::new(), Some(CutSide::Trait(along)), Some(3.0));
    let over_the_north_side = DVec2::new(2.0, 6.0);

    assert!(
        previewed(&sketch, &editor, over_the_north_side, 0.5, 1.0).is_some(),
        "hovering the other side says which corner, so the curve can be shown"
    );
    assert!(
        previewed(&sketch, &editor, DVec2::new(40.0, 40.0), 0.5, 1.0).is_none(),
        "a cursor over nothing names no corner"
    );
}

#[test]
fn a_corner_with_no_value_typed_for_it_shows_nothing() {
    let (sketch, along, up) = a_right_angle();
    let editor = rounding(vec![Corner::Between(along.into(), up.into())], None, None);

    assert!(
        previewed(&sketch, &editor, DVec2::ZERO, 0.5, 1.0).is_none(),
        "a corner with no radius has no curve to show"
    );
}

#[test]
fn a_chamfer_shows_the_straight_cut_its_two_distances_would_leave() {
    let (sketch, along, up) = a_right_angle();
    let mut editor = SketchEditor {
        tool: Tool::Chamfer,
        chamfer_mode: ChamferMode::Sided,
        tool_state: ToolState::Corner {
            taken: vec![Corner::Between(along.into(), up.into())],
            half: None,
        },
        ..Default::default()
    };
    editor.live.field(0).locked = Some(2.0);
    editor.live.field(1).locked = Some(4.0);

    let shown = previewed(&sketch, &editor, DVec2::ZERO, 0.5, 1.0)
        .expect("a corner with both distances typed");

    assert_eq!(shown.cuts.len(), 1, "the cut is what is shown");
    assert!(matches!(
        shown
            .sketch
            .drawn_cut(shown.cuts[0])
            .map(|drawn| drawn.curve),
        Some(CutCurve::Straight { .. })
    ));
}

#[test]
fn a_value_typed_in_millimetres_is_shown_at_the_size_the_drawing_measures() {
    let (sketch, along, up) = a_right_angle();
    let editor = rounding(
        vec![Corner::Between(along.into(), up.into())],
        None,
        Some(6.0),
    );

    let shown = previewed(&sketch, &editor, DVec2::ZERO, 0.5, 2.0)
        .expect("a corner that can be rounded at three units");

    let [cut] = shown.cuts.as_slice() else {
        panic!("the cut is what is shown, got {:?}", shown.cuts);
    };
    let CutShape::Fillet { radius: reach } = shown.sketch.corner_cuts()[cut.0].shape else {
        panic!("a fillet");
    };
    assert!(
        (reach - 3.0).abs() <= 1e-9,
        "six millimetres at two per unit is three units, got {reach}"
    );
}

#[test]
fn every_corner_taken_is_shown_wherever_the_cursor_is() {
    let (sketch, along, up) = a_right_angle();
    let far = DVec2::new(-80.0, -80.0);
    let editor = rounding(
        vec![Corner::Between(along.into(), up.into())],
        None,
        Some(3.0),
    );

    let shown = previewed(&sketch, &editor, far, 0.5, 1.0)
        .expect("a corner taken shows whatever the cursor is over");

    assert_eq!(
        shown.cuts.len(),
        1,
        "the cursor names a second side only while one is still half a corner"
    );
}
