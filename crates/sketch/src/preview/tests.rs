use glam::DVec2;

use super::*;
use crate::axis::ChosenAxis;
use crate::chamfer::Chamfer;
use crate::constraints::SketchAxis;
use crate::corner_cut::{CornerCutId, CutShape};
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

/// The corner cut the way a click would cut it, as a preview reads it.
fn cut(
    trial: &mut Sketch,
    along: SegmentId,
    up: SegmentId,
    shape: CutShape,
) -> Option<Vec<CornerCutId>> {
    trial
        .cut_corner(along, up, shape, false, 0)
        .ok()
        .map(|id| vec![id])
}

#[test]
fn a_previewed_fillet_leaves_the_drawing_untouched_and_hands_back_the_cut_it_would_lay() {
    let (sketch, along, up) = a_right_angle();

    let shown = sketch
        .preview(|trial| cut(trial, along, up, CutShape::Fillet { radius: 3.0 }))
        .expect("a corner that can be rounded");

    assert_eq!(
        sketch.live_corner_cuts().count(),
        0,
        "the drawing the user still has in front of them has not been rounded"
    );
    assert_eq!(
        shown.cuts,
        vec![CornerCutId(0)],
        "the cut is what the preview names as new"
    );
    assert!(
        shown.laid.is_empty(),
        "no piece of the drawing is laid: {:?}",
        shown.laid
    );
    assert!(
        shown
            .sketch
            .drawn_cut(CornerCutId(0))
            .is_some_and(|drawn| drawn.fits)
    );
}

#[test]
fn a_previewed_chamfer_names_the_cut_it_would_lay_across_the_corner() {
    let (sketch, along, up) = a_right_angle();

    let shown = sketch
        .preview(|trial| cut(trial, along, up, CutShape::Chamfer(Chamfer::Equal(3.0))))
        .expect("a corner that can be cut");

    assert_eq!(shown.cuts, vec![CornerCutId(0)]);
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
fn a_preview_of_a_corner_too_tight_for_its_fillet_shows_one_that_does_not_fit() {
    let (sketch, along, up) = a_right_angle();

    let shown = sketch
        .preview(|trial| cut(trial, along, up, CutShape::Fillet { radius: 500.0 }))
        .expect("a value too big is still laid, and shown for what it is");

    assert!(
        shown
            .sketch
            .drawn_cut(CornerCutId(0))
            .is_some_and(|drawn| !drawn.fits)
    );
}
