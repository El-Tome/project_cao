//! What app · screens/viewport/input/trim.rs is held to.
//!
//! Closes #286.
//! - nothing is drawn where a click would do nothing —
//!   `a_click_that_would_cut_nothing_shows_nothing`, and no other tool shows
//!   it at all — `no_tool_but_the_trim_shows_what_would_go`
//! - what is shown is read off the very call the click commits — no test: it
//!   is `cut_under`, the one the click already goes through, and the two tests
//!   above are what would break if a second reading of the cursor crept in

use super::*;
use cao_sketch::WorkPlane;

/// A trait laid across the top of a curve that grazes it, so that one click
/// is within reach of both.
fn a_trait_touching_a_curve() -> Sketch {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let left = sketch.add_point(DVec2::ZERO);
    let right = sketch.add_point(DVec2::new(10.0, 0.0));
    sketch.add_segment(left, right);
    let centre = sketch.add_point(DVec2::new(5.0, -5.0));
    let east = sketch.add_point(DVec2::new(10.0, -5.0));
    let west = sketch.add_point(DVec2::new(0.0, -5.0));
    sketch.add_arc(centre, east, west);
    sketch
}

#[test]
fn a_click_within_reach_of_both_a_trait_and_a_curve_cuts_the_trait() {
    let sketch = a_trait_touching_a_curve();

    let cut = cut_under(&sketch, 0, DVec2::new(5.0, 0.0), 0.5);

    assert!(
        matches!(cut, Some(Operation::Trim { .. })),
        "the click cut {cut:?}",
    );
}

#[test]
fn a_click_the_trait_is_out_of_reach_of_cuts_the_curve() {
    let sketch = a_trait_touching_a_curve();

    let cut = cut_under(&sketch, 0, DVec2::new(1.47, -1.47), 0.5);

    assert!(
        matches!(cut, Some(Operation::TrimArc { .. })),
        "the click cut {cut:?}",
    );
}

#[test]
fn a_click_on_a_round_neither_a_trait_nor_a_curve_reaches_cuts_the_circle() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(40.0, 0.0));
    sketch.add_circle(centre, 10.0);

    let cut = cut_under(&sketch, 0, DVec2::new(50.0, 0.0), 0.5);

    assert!(
        matches!(cut, Some(Operation::TrimCircle { between: None, .. })),
        "a round with nothing on it goes whole, and the click says so: {cut:?}",
    );
}

/// A trait long enough to be cut somewhere other than at its ends, with a
/// point sitting on it a third of the way along.
fn a_trait_with_a_point_on_it() -> Sketch {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let left = sketch.add_point(DVec2::ZERO);
    let right = sketch.add_point(DVec2::new(9.0, 0.0));
    sketch.add_segment(left, right);
    sketch.add_point(DVec2::new(3.0, 0.0));
    sketch
}

#[test]
fn no_tool_but_the_trim_shows_what_would_go() {
    let sketch = a_trait_with_a_point_on_it();
    let mut editor = SketchEditor {
        tool: Tool::Trim,
        ..SketchEditor::default()
    };

    assert!(
        previewed(&sketch, &editor, 0, DVec2::new(6.0, 0.0), 0.5).is_some(),
        "the trim tool over a stretch it can cut shows nothing",
    );

    for tool in [Tool::Select, Tool::Split, Tool::Line] {
        editor.tool = tool;
        assert!(
            previewed(&sketch, &editor, 0, DVec2::new(6.0, 0.0), 0.5).is_none(),
            "{tool:?} is drawing what the trim tool would take away",
        );
    }
}

#[test]
fn a_click_that_would_cut_nothing_shows_nothing() {
    let sketch = a_trait_with_a_point_on_it();
    let editor = SketchEditor {
        tool: Tool::Trim,
        ..SketchEditor::default()
    };

    assert!(
        previewed(&sketch, &editor, 0, DVec2::new(6.0, 7.0), 0.5).is_none(),
        "a cursor nowhere near the drawing is shown a stretch going",
    );
}
