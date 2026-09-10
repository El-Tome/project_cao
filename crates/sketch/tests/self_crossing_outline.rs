//! `Sketch::closed_outlines` walks the segment graph by the angular order of
//! edges around each vertex — it has no notion of two edges meeting in space
//! without sharing a vertex. A plain point drag, no tangency or dimension
//! involved, is enough to fold a polygon into a bowtie: combinatorially still
//! a closed, distinct loop, but one that draws no area a fill could make
//! sense of.

use cao_sketch::{Sketch, WorkPlane};
use glam::DVec2;

#[test]
fn a_bowtie_outline_is_not_offered_as_a_region_to_fill() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let a = sketch.add_point(DVec2::new(20.0, 5.0));
    let b = sketch.add_point(DVec2::new(10.0, 0.0));
    let c = sketch.add_point(DVec2::new(10.0, 10.0));
    let d = sketch.add_point(DVec2::new(0.0, 10.0));
    // `a` sits past the side `cd`, so the sides `ab`/`bc` and `da`/`cd` cross
    // each other instead of the loop closing on itself cleanly.
    for (from, to) in [(a, b), (b, c), (c, d), (d, a)] {
        sketch.add_segment(from, to);
    }

    assert!(
        sketch.regions().is_empty(),
        "a self-crossing outline has no simple area to fill"
    );
}

#[test]
fn dragging_a_corner_past_the_opposite_side_turns_a_square_into_a_refused_bowtie() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let a = sketch.add_point(DVec2::new(0.0, 0.0));
    let b = sketch.add_point(DVec2::new(10.0, 0.0));
    let c = sketch.add_point(DVec2::new(10.0, 10.0));
    let d = sketch.add_point(DVec2::new(0.0, 10.0));
    for (from, to) in [(a, b), (b, c), (c, d), (d, a)] {
        sketch.add_segment(from, to);
    }
    assert_eq!(sketch.regions().len(), 1, "the square starts as one region");

    // Dragging `a` past the opposite side `bc` crosses edges `ab`/`da` with
    // `bc`/`cd`, the same fold a user's drag produced in the report.
    sketch.settle_around(a, DVec2::new(20.0, 5.0), 1.0);

    assert!(
        sketch.regions().is_empty(),
        "the folded square is no longer offered as a region"
    );
}
