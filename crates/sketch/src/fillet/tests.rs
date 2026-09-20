use glam::DVec2;

use super::*;
use crate::constraints::Constraint;
use crate::plane::WorkPlane;
use crate::sketch::PointId;

const TOLERANCE: f64 = 1e-9;
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
fn a_fillet_rounds_a_right_angle_into_a_quarter_of_a_circle() {
    let (mut sketch, along, up, _) = a_right_angle();

    let rounded = sketch
        .fillet(along, up, 3.0)
        .expect("a corner that can be rounded");

    let reach = sketch.arc_radius(rounded.arc);
    assert!(
        (reach - 3.0).abs() <= TOLERANCE,
        "the curve turns at the radius asked, got {reach}"
    );
    let sweep = sketch.arc_sweep(rounded.arc).to_degrees();
    assert!(
        (sweep - 90.0).abs() <= 1e-9,
        "a right angle rounds through a right angle, got {sweep}"
    );
}

#[test]
fn a_fillet_meets_each_side_where_that_side_now_stops() {
    let (mut sketch, along, up, _) = a_right_angle();

    let rounded = sketch
        .fillet(along, up, 3.0)
        .expect("a corner that can be rounded");

    let arc = sketch.arc(rounded.arc);
    let touches = [sketch.point(arc.start), sketch.point(arc.end)];
    for wanted in [CORNER + DVec2::new(3.0, 0.0), CORNER + DVec2::new(0.0, 3.0)] {
        assert!(
            touches.iter().any(|at| at.distance(wanted) <= TOLERANCE),
            "the curve starts where a side stops, {wanted:?} missing from {touches:?}"
        );
    }
    for piece in &rounded.pieces {
        let (from, to) = sketch.endpoints(*piece);
        assert!(
            (from.distance(to) - 7.0).abs() <= TOLERANCE,
            "each side kept its far seven units, got {}",
            from.distance(to)
        );
    }
}

#[test]
fn a_fillet_is_held_tangent_to_both_sides_it_joins() {
    let (mut sketch, along, up, _) = a_right_angle();

    let rounded = sketch
        .fillet(along, up, 3.0)
        .expect("a corner that can be rounded");

    let tangencies = sketch
        .constraints()
        .iter()
        .filter(|rule| {
            matches!(rule, Constraint::ArcTangent { arc, segment, .. }
                if *arc == rounded.arc && rounded.pieces.contains(segment))
        })
        .count();
    assert_eq!(
        tangencies, 2,
        "without the rule the curve stops being a fillet the moment anything moves"
    );
}

#[test]
fn a_fillet_leaves_the_corner_behind_held_on_the_lines_of_both_sides() {
    let (mut sketch, along, up, pivot) = a_right_angle();

    let rounded = sketch
        .fillet(along, up, 3.0)
        .expect("a corner that can be rounded");

    assert!(
        !sketch.is_erased_point(pivot),
        "the two tools leave the same thing behind, and a chamfer leaves its corner"
    );
    assert_eq!(rounded.corner, pivot);
    let held = sketch
        .constraints()
        .iter()
        .filter(|rule| {
            matches!(rule, Constraint::OnSegment { point, segment }
                if *point == pivot && rounded.pieces.contains(segment))
        })
        .count();
    assert_eq!(held, 2, "one hold per side, so the corner follows them");
}

#[test]
fn a_fillet_too_big_for_the_sides_it_joins_is_refused() {
    let (mut sketch, along, up, _) = a_right_angle();
    let before = sketch.clone();

    let refused = sketch.fillet(along, up, 12.0);

    assert!(
        refused.is_none(),
        "there are only ten units of side to take"
    );
    assert!(!sketch.is_erased_segment(along) && !sketch.is_erased_segment(up));
    assert_eq!(sketch.points().len(), before.points().len());
}

#[test]
fn a_fillet_of_no_radius_at_all_is_refused() {
    let (mut sketch, along, up, _) = a_right_angle();

    assert!(sketch.fillet(along, up, 0.0).is_none());
}

#[test]
fn two_traits_that_share_no_corner_cannot_be_rounded() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let a = sketch.add_point(DVec2::new(0.0, 0.0));
    let b = sketch.add_point(DVec2::new(10.0, 0.0));
    let c = sketch.add_point(DVec2::new(0.0, 5.0));
    let d = sketch.add_point(DVec2::new(10.0, 5.0));
    let lower = sketch.add_segment(a, b);
    let upper = sketch.add_segment(c, d);

    assert!(sketch.fillet(lower, upper, 1.0).is_none());
}

#[test]
fn a_fillet_on_a_sharp_corner_still_turns_at_the_radius_asked() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let pivot = sketch.add_point(CORNER);
    let east = sketch.add_point(CORNER + DVec2::new(20.0, 0.0));
    let up = sketch.add_point(CORNER + DVec2::from_angle(30f64.to_radians()) * 20.0);
    let first = sketch.add_segment(pivot, east);
    let second = sketch.add_segment(pivot, up);

    let rounded = sketch
        .fillet(first, second, 2.0)
        .expect("a corner that can be rounded");

    let centre = sketch.point(sketch.arc(rounded.arc).center);
    for piece in &rounded.pieces {
        let (from, to) = sketch.endpoints(*piece);
        let span = to - from;
        let off = (centre - from).perp_dot(span.normalize()).abs();
        assert!(
            (off - 2.0).abs() <= 1e-9,
            "the curve grazes each side at the radius asked, got {off} away from one of them"
        );
    }
    let sweep = sketch.arc_sweep(rounded.arc).to_degrees();
    assert!(
        (sweep - 150.0).abs() <= 1e-9,
        "a thirty degree corner rounds through what is left of half a turn, got {sweep}"
    );
}

#[test]
fn a_fillet_turns_the_short_way_round_whichever_side_is_named_first() {
    let (mut sketch, along, up, _) = a_right_angle();

    let rounded = sketch
        .fillet(up, along, 3.0)
        .expect("a corner that can be rounded");

    let sweep = sketch.arc_sweep(rounded.arc).to_degrees();
    assert!(
        (sweep - 90.0).abs() <= 1e-9,
        "naming the sides the other way round is the same corner, got {sweep}"
    );
}
