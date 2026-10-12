//! What sketch · corner_angle.rs is held to.
//!
//! Closes #485.
//! - a corner's angle is the same whichever trait was clicked first, its arms
//!   following their traits — `a_corners_angle_is_the_same_whichever_trait_comes_first`

use glam::DVec2;

use super::*;
use crate::plane::WorkPlane;

/// The issue's corner: a trait coming in from the left, and one leaving it up
/// and to the right, 51.3° off the horizontal's prolongation.
fn a_corner() -> (Sketch, SegmentId, SegmentId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let left = sketch.add_point(DVec2::new(0.0, 20.0));
    let pivot = sketch.add_point(DVec2::new(60.0, 20.0));
    let up_right = sketch.add_point(DVec2::new(100.0, 70.0));
    let along = sketch.add_segment(left, pivot);
    let slanted = sketch.add_segment(pivot, up_right);
    (sketch, along, slanted)
}

fn acute() -> f64 {
    50.0_f64.atan2(40.0).to_degrees()
}

fn read_past(first: SegmentId, second: SegmentId) -> DimensionTarget {
    DimensionTarget::Angle {
        first,
        first_along: Along::Prolongation,
        second,
        second_along: Along::Trait,
    }
}

#[test]
fn a_corners_angle_is_the_same_whichever_trait_comes_first() {
    let (mut sketch, along, slanted) = a_corner();
    sketch.set_dimension(read_past(along, slanted), acute(), false);

    let the_other_way_round = DimensionTarget::Angle {
        first: slanted,
        first_along: Along::Trait,
        second: along,
        second_along: Along::Prolongation,
    };
    let another_quarter = DimensionTarget::Angle {
        first: slanted,
        first_along: Along::Prolongation,
        second: along,
        second_along: Along::Trait,
    };

    assert!(sketch.dimension_of(the_other_way_round).is_some());
    assert!(sketch.dimension_of(another_quarter).is_none());
}
