//! Joining two points, and taking something away.

use super::*;

/// Two ends laid on top of each other are one corner, not two.
#[test]
fn merging_two_points_joins_what_they_held() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let left = sketch.add_point(DVec2::new(-10.0, 0.0));
    let meeting = sketch.add_point(DVec2::new(0.0, 10.0));
    let twin = sketch.add_point(DVec2::new(0.0, 10.0));
    let right = sketch.add_point(DVec2::new(10.0, 0.0));
    let first = sketch.add_segment(left, meeting);
    let second = sketch.add_segment(twin, right);

    sketch.merge_points(meeting, twin);

    assert!(sketch.is_erased_point(twin));
    assert_eq!(sketch.segments()[second.0].start, meeting);
    assert_eq!(sketch.segments()[first.0].end, meeting);
    assert_eq!(sketch.live_segments().count(), 2, "both segments stay");
}

/// A segment whose two ends became one has no length and no direction.
#[test]
fn merging_the_ends_of_a_line_takes_the_line() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let a = sketch.add_point(DVec2::new(0.0, 0.0));
    let b = sketch.add_point(DVec2::new(1.0, 0.0));
    let short = sketch.add_segment(a, b);

    sketch.merge_points(a, b);
    assert!(sketch.is_erased_segment(short));
}

/// The origin is never the one that gives way.
#[test]
fn merging_onto_the_origin_keeps_the_origin() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let stray = sketch.add_point(DVec2::ZERO);
    let far = sketch.add_point(DVec2::new(10.0, 0.0));
    sketch.add_segment(stray, far);

    sketch.merge_points(stray, Sketch::ORIGIN);

    assert!(!sketch.is_erased_point(Sketch::ORIGIN));
    assert!(sketch.is_erased_point(stray));
    assert_eq!(sketch.segments()[0].start, Sketch::ORIGIN);
}

#[test]
fn merging_moves_a_dimension_onto_the_point_that_stays() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let kept = sketch.add_point(DVec2::new(10.0, 0.0));
    let twin = sketch.add_point(DVec2::new(10.0, 0.0));
    let far = sketch.add_point(DVec2::new(10.0, 20.0));
    sketch.add_segment(twin, far);
    sketch.set_dimension(
        DimensionTarget::Distance {
            from: Sketch::ORIGIN,
            to: twin,
        },
        10.0,
        false,
    );

    sketch.merge_points(kept, twin);

    assert!(
        sketch
            .dimension_of(DimensionTarget::Distance {
                from: Sketch::ORIGIN,
                to: kept,
            })
            .is_some(),
        "the dimension follows the point that was kept"
    );
}

/// Deleting must not shift the rank of what stays: a dimension already
/// recorded against a segment would then measure another one.
#[test]
fn erasing_a_segment_leaves_the_others_where_they_were() {
    let (mut sketch, [base, side, third]) = triangle();
    sketch.set_dimension(DimensionTarget::Length(third), 40.0, false);

    sketch.erase(Element::Segment(base));

    assert!(sketch.is_erased_segment(base));
    assert!(!sketch.is_erased_segment(side));
    assert_eq!(sketch.live_segments().count(), 2);
    assert!(
        sketch
            .dimension_of(DimensionTarget::Length(third))
            .is_some(),
        "the dimension on the third side is untouched"
    );
    assert_eq!(sketch.nearest_segment(DVec2::new(50.0, 0.0), 1.0), None);
}

/// A dimension measuring something deleted would report on nothing.
#[test]
fn erasing_takes_the_dimensions_that_measured_it() {
    let (mut sketch, [base, _, _]) = triangle();
    sketch.set_dimension(DimensionTarget::Length(base), 40.0, false);
    assert_eq!(sketch.dimensions().len(), 1);

    sketch.erase(Element::Segment(base));
    assert!(sketch.dimensions().is_empty());
}

/// A segment without its point is not geometry.
#[test]
fn erasing_a_point_takes_what_leaned_on_it() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corner = sketch.add_point(DVec2::new(10.0, 0.0));
    let far = sketch.add_point(DVec2::new(10.0, 10.0));
    let touching = sketch.add_segment(Sketch::ORIGIN, corner);
    let apart = sketch.add_segment(corner, far);
    sketch.add_circle(corner, 3.0);

    sketch.erase(Element::Point(corner));

    assert!(sketch.is_erased_segment(touching));
    assert!(sketch.is_erased_segment(apart));
    assert_eq!(sketch.live_circles().count(), 0);
    assert!(!sketch.is_erased_point(far), "the point opposite stays");
}

/// The origin is what everything else is measured from.
#[test]
fn the_origin_cannot_be_erased() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    sketch.erase(Element::Point(Sketch::ORIGIN));
    assert!(!sketch.is_erased_point(Sketch::ORIGIN));
}

#[test]
fn an_erased_shape_no_longer_encloses_an_area() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners = [
        sketch.add_point(DVec2::ZERO),
        sketch.add_point(DVec2::new(10.0, 0.0)),
        sketch.add_point(DVec2::new(10.0, 10.0)),
        sketch.add_point(DVec2::new(0.0, 10.0)),
    ];
    let mut sides = Vec::new();
    for index in 0..4 {
        sides.push(sketch.add_segment(corners[index], corners[(index + 1) % 4]));
    }
    assert_eq!(sketch.regions().len(), 1);

    sketch.erase(Element::Segment(sides[0]));
    assert!(sketch.regions().is_empty());
}

#[test]
fn clicking_back_onto_a_corner_reuses_it() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let first = sketch.point_at(DVec2::new(5.0, 5.0), 0.5);
    let again = sketch.point_at(DVec2::new(5.2, 5.1), 0.5);
    let elsewhere = sketch.point_at(DVec2::new(40.0, 5.0), 0.5);

    assert_eq!(first, again);
    assert_ne!(first, elsewhere);
    assert_eq!(sketch.points().len(), 3, "the origin plus the two placed");
}

/// Clicking where the origin sits must join it rather than lay a second
/// point on top: that is how a drawing gets pinned without thinking about
/// it.
#[test]
fn clicking_the_origin_joins_it() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    assert_eq!(
        sketch.point_at(DVec2::new(0.05, -0.05), 0.5),
        Sketch::ORIGIN
    );
    assert_eq!(sketch.points().len(), 1);
}
