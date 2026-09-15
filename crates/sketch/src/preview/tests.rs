use glam::DVec2;

use super::*;
use crate::axis::ChosenAxis;
use crate::chamfer::Chamfer;
use crate::constraints::SketchAxis;
use crate::plane::WorkPlane;
use crate::sketch::SegmentId;

const CORNER: DVec2 = DVec2::new(2.0, 1.0);

/// A right angle standing clear of the origin, one side east and one north.
fn a_right_angle() -> (Sketch, SegmentId, SegmentId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let pivot = sketch.add_point(CORNER);
    let east = sketch.add_point(CORNER + DVec2::new(10.0, 0.0));
    let north = sketch.add_point(CORNER + DVec2::new(0.0, 10.0));
    let along = sketch.add_segment(pivot, east);
    let up = sketch.add_segment(pivot, north);
    (sketch, along, up)
}

#[test]
fn a_previewed_fillet_leaves_the_drawing_untouched_and_hands_back_the_curve_it_would_lay() {
    let (sketch, along, up) = a_right_angle();

    let shown = sketch
        .preview(|trial| trial.fillet(along, up, 3.0))
        .expect("a corner that can be rounded");

    assert!(
        sketch.arcs().is_empty(),
        "the drawing the user still has in front of them has not been rounded"
    );
    assert_eq!(
        shown.sketch.arcs().len(),
        1,
        "the drawing shown carries the curve the click would lay"
    );
    assert!(
        matches!(shown.laid.as_slice(), [Element::Arc(_)]),
        "the curve is what the preview names as new, got {:?}",
        shown.laid
    );
}

#[test]
fn a_previewed_chamfer_names_the_cut_it_would_lay_across_the_corner() {
    let (sketch, along, up) = a_right_angle();

    let shown = sketch
        .preview(|trial| trial.chamfer(along, up, Chamfer::Equal(3.0)))
        .expect("a corner that can be cut");

    assert!(
        matches!(shown.laid.as_slice(), [Element::Segment(_)]),
        "the straight cut is what the preview names as new, got {:?}",
        shown.laid
    );
}

#[test]
fn a_previewed_mirror_names_every_copy_it_would_lay() {
    let (sketch, along, up) = a_right_angle();
    let held = [Element::Segment(along), Element::Segment(up)];

    let shown = sketch
        .preview(|trial| trial.mirror(&held, ChosenAxis::Sketch(SketchAxis::U)))
        .expect("an axis that names a direction");

    let copies = shown
        .laid
        .iter()
        .filter(|laid| matches!(laid, Element::Segment(_)))
        .count();
    assert_eq!(copies, 2, "both sides are copied, and both are shown");
    assert_eq!(
        shown.laid.len(),
        5,
        "the three corners the two sides stand on are shown too, got {:?}",
        shown.laid
    );
}

#[test]
fn a_preview_of_a_corner_too_tight_to_round_shows_nothing_at_all() {
    let (sketch, along, up) = a_right_angle();

    assert!(
        sketch
            .preview(|trial| trial.fillet(along, up, 500.0))
            .is_none(),
        "a preview of what the click would refuse is a preview that lies"
    );
}
