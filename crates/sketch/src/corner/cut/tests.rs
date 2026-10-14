//! What sketch · corner/cut.rs is held to: which points are the corner a cut
//! left, and which are only held where two lines cross.

use glam::DVec2;

use crate::holding::Support;
use crate::plane::WorkPlane;
use crate::sketch::{PointId, SegmentId, Sketch};

const CORNER: DVec2 = DVec2::new(2.0, 1.0);

/// A right angle standing clear of the sketch's own origin, one side running
/// east and the other north, ten units each.
fn a_right_angle() -> (Sketch, SegmentId, SegmentId, PointId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let pivot = sketch.add_point(CORNER);
    let east = sketch.add_point(CORNER + DVec2::new(10.0, 0.0));
    let north = sketch.add_point(CORNER + DVec2::new(0.0, 10.0));
    let along = sketch.add_segment(pivot, east);
    let up = sketch.add_segment(pivot, north);
    (sketch, along, up, pivot)
}

#[test]
fn the_point_a_fillet_leaves_is_a_cut_corner_its_sides_named_by_their_free_ends() {
    let (mut sketch, along, up, pivot) = a_right_angle();
    let rounded = sketch.fillet(along, up, 3.0).expect("a corner that fits");

    let corners = sketch.cut_corners();

    assert_eq!(corners.len(), 1, "one corner cut, got {corners:?}");
    assert_eq!(corners[0].point, pivot);
    let arc = sketch.arc(rounded.arc);
    for side in corners[0].sides {
        assert!(
            [arc.start, arc.end].contains(&side.near),
            "a side's end at the cut is where the fillet runs into it"
        );
        assert!(sketch.point(side.far).distance(CORNER) > 9.0);
    }
}

#[test]
fn the_point_a_chamfer_leaves_is_a_cut_corner() {
    let (mut sketch, along, up, pivot) = a_right_angle();
    sketch
        .chamfer(along, up, crate::chamfer::Chamfer::Equal(2.0))
        .expect("a corner that fits");

    let corners = sketch.cut_corners();

    assert_eq!(corners.len(), 1, "one corner cut, got {corners:?}");
    assert_eq!(corners[0].point, pivot);
}

#[test]
fn a_point_held_where_two_lines_cross_with_nothing_cut_between_them_is_no_cut_corner() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let east_start = sketch.add_point(CORNER + DVec2::new(3.0, 0.0));
    let east_end = sketch.add_point(CORNER + DVec2::new(10.0, 0.0));
    let north_start = sketch.add_point(CORNER + DVec2::new(0.0, 3.0));
    let north_end = sketch.add_point(CORNER + DVec2::new(0.0, 10.0));
    let east = sketch.add_segment(east_start, east_end);
    let north = sketch.add_segment(north_start, north_end);
    let sharp = sketch.add_point(CORNER);
    for side in [east, north] {
        sketch.add_constraint(Support::Segment(side).holding(sharp));
    }

    assert!(
        sketch.cut_corners().is_empty(),
        "a sharp laid by hand where two lines meet was cut from nothing"
    );
}

#[test]
fn a_cut_corner_brought_out_past_one_of_its_sides_is_no_longer_whole() {
    let (mut sketch, along, up, _) = a_right_angle();
    sketch.fillet(along, up, 3.0).expect("a corner that fits");
    let before = sketch.cut_corners();
    let north = before[0]
        .sides
        .into_iter()
        .find(|side| sketch.point(side.far).y > CORNER.y + 1.0)
        .expect("the side running north");

    sketch.move_point(north.far, CORNER - DVec2::new(0.0, 2.0));

    assert!(
        !sketch.still_cut(&before),
        "its free end behind the corner, the side runs back across it"
    );
}
