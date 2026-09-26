//! What a point dragged does to the shape it belongs to: it stretches the
//! shape before it turns it.
//!
//! Closes #422.
//! - 1: a rectangle pulled by a corner changes the two sides meeting there,
//!   keeps all four directions and its opposite corner, and the two corners
//!   between follow —
//!   `a_rectangle_pulled_by_a_corner_keeps_its_directions_and_its_opposite_corner`
//! - 2: the same for any shape whose traits a rule of direction ties — an L
//!   keeps its far end and its corner slides —
//!   `an_l_pulled_by_one_end_keeps_its_far_end_and_its_corner_slides`,
//!   `an_l_tied_by_a_collinear_rule_keeps_its_directions`,
//!   `an_obtuse_parallelogram_pulled_by_a_corner_keeps_its_opposite_corner`,
//!   `a_triangle_with_two_typed_angles_grows_without_turning`
//! - 3: what stays is the point holding the shape, else the centre of the
//!   curve the point is on, else the farthest point among those on the traits
//!   tied by direction; a curve's centre still carries its curve —
//!   `a_rectangle_held_at_a_corner_keeps_that_corner_and_stretches`,
//!   `a_free_tail_does_not_take_the_place_of_the_opposite_corner`,
//!   `a_trapezoid_pulled_by_a_corner_keeps_its_far_corner`,
//!   `a_circle_centred_on_a_corner_does_not_take_the_place_of_the_opposite_corner`,
//!   `a_point_held_on_a_trait_leaves_the_far_end_whichever_way_the_trait_was_drawn`,
//!   `a_circle_dragged_by_its_centre_is_carried_at_its_size`,
//!   `a_corner_that_carries_a_circle_stretches_its_rectangle_and_carries_the_circle`;
//!   a shape tied to another by a value lets that one follow —
//!   `a_shape_tied_by_a_distance_to_another_lets_that_one_follow`; two shapes
//!   sharing the origin pivot apart — `two_shapes_sharing_the_origin_pivot_apart`
//! - 4: a shape that cannot stretch at all pivots about that point, the point
//!   pulled stopping where its sizes allow; a turn a rule forbids leaves it
//!   where it is — `a_free_trait_of_typed_length_pivots_about_its_other_end`,
//!   `a_rectangle_typed_on_both_sides_pivots_about_its_opposite_corner`,
//!   `a_triangle_of_three_typed_sides_pivots_about_its_far_corner`,
//!   `a_pivot_a_rule_forbids_leaves_the_shape_where_it_is`
//! - 5: a shape that can stretch one way only follows the hand that way and no
//!   further — `a_rectangle_with_only_its_width_typed_takes_its_height_from_the_cursor`;
//!   past its reach the point stops on the way to the hand and goes no
//!   further back as the hand goes on —
//!   `a_hinge_pulled_out_of_reach_stops_on_the_way_to_the_hand`,
//!   `a_hinge_pulled_further_out_of_reach_goes_no_further_back`; a point whose
//!   one way is round a curve goes round it towards the hand —
//!   `a_corner_held_at_a_typed_distance_goes_round_towards_the_hand`
//! - 6: a trait of typed length whose other end is held still pivots about it —
//!   `a_leaning_trait_of_typed_length_hung_off_the_origin_pivots_about_it`
//! - 7: a point on a curve leaves the curve's centre in place: an arc's end
//!   slides round, its other end keeping its direction, all the way round
//!   even when its radius is typed —
//!   `an_arc_end_slides_round_its_circle_and_the_other_end_keeps_its_direction`,
//!   `an_arc_end_of_typed_radius_slides_all_the_way_round`; an ellipse whose
//!   axis is typed turns about its centre —
//!   `an_ellipse_with_a_typed_axis_turns_about_its_centre`
//! - 7, the ellipse's axis end lengthening its axis without turning it — no
//!   test: here; `an_axis_end_dragged_alone_leaves_the_centre_where_it_was` in
//!   ellipse/tests.rs holds it; taken twice as far off its axis as along it,
//!   the end follows the hand, the ellipse turning about its centre —
//!   `an_axis_end_pulled_twice_as_far_off_its_axis_as_along_it_follows_the_hand`,
//!   `an_axis_end_swung_onto_a_place_as_far_out_as_it_stands_ends_there`
//! - 8: a trait held by nothing stretches freely —
//!   `with_no_rules_only_the_dragged_point_moves`, and traits held by lengths
//!   alone still bend — `two_typed_bars_hinged_together_bend_to_reach_the_cursor`
//! - 19: the guard that keeps the way up when a value is typed stays — no
//!   test: here, `a_rectangle_does_not_turn_when_one_of_its_sides_changes` in
//!   sketch/tests/values.rs holds it, untouched
//! - 18: a shape pivoting brings the dragged point onto a grid point within
//!   reach — `a_pivoting_point_is_pulled_onto_a_grid_point_within_reach`,
//!   towards the hand before any magnet —
//!   `a_pivoting_drag_is_taken_towards_the_hand_and_joins_only_what_it_lands_on`
//! - 22: a point that stopped short of the hand is dropped on nothing —
//!   `a_point_that_stopped_short_of_the_hand_is_joined_to_nothing`
//! - 24: a corner pulled past the opposite one goes through, the shape coming
//!   out the other way round —
//!   `a_rectangle_pulled_past_its_opposite_corner_comes_out_the_other_way_round`,
//!   `a_corner_held_to_its_height_by_a_typed_width_goes_through_the_opposite_side`;
//!   not a shape with a rounded corner, which would come out crossed —
//!   `a_rounded_corner_keeps_its_rectangle_from_going_through_inside_out`
//! - 20: the verdict does not change — `a_drag_leaves_the_verdict_and_the_freedom_as_they_were`

use glam::DVec2;

use crate::constraints::{Constraint, DimensionTarget, SketchAxis};
use crate::element::Element;
use crate::plane::WorkPlane;
use crate::sketch::{PointId, SegmentId, Sketch};

/// Close enough for points the solver placed: it stops at a hundred-thousandth
/// of the drawing's size.
const SETTLED: f64 = 1e-4;

/// A turn smaller than a hundredth of a degree: what the solver's own stopping
/// point leaves when two kept lines meet at a slant and it closes in on their
/// crossing rather than landing on it.
const UNTURNED: f64 = 1e-4;

/// A rectangle as the tool lays it: four traits, three right angles, nothing
/// saying which way up.
fn rectangle(corners: [DVec2; 4]) -> (Sketch, [PointId; 4], [SegmentId; 4]) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let points = corners.map(|place| sketch.add_point(place));
    let sides: [SegmentId; 4] =
        std::array::from_fn(|rank| sketch.add_segment(points[rank], points[(rank + 1) % 4]));
    for corner in 0..3 {
        sketch.add_constraint(Constraint::Perpendicular {
            first: sides[corner],
            second: sides[corner + 1],
        });
    }
    (sketch, points, sides)
}

fn upright() -> (Sketch, [PointId; 4], [SegmentId; 4]) {
    rectangle([
        DVec2::new(20.0, 20.0),
        DVec2::new(120.0, 20.0),
        DVec2::new(120.0, 70.0),
        DVec2::new(20.0, 70.0),
    ])
}

fn direction(sketch: &Sketch, side: SegmentId) -> DVec2 {
    let (start, end) = sketch.endpoints(side);
    (end - start).normalize()
}

fn assert_near(found: DVec2, wanted: DVec2, what: &str) {
    assert!(
        found.distance(wanted) < SETTLED,
        "{what}: wanted {wanted}, found {found}"
    );
}

#[test]
fn a_rectangle_pulled_by_a_corner_keeps_its_directions_and_its_opposite_corner() {
    let (mut sketch, [a, b, c, d], sides) = upright();
    let before = sides.map(|side| direction(&sketch, side));

    sketch.settle_around(a, DVec2::new(5.0, 45.0), 1.0);

    assert_near(sketch.point(a), DVec2::new(5.0, 45.0), "the corner pulled");
    assert_near(
        sketch.point(c),
        DVec2::new(120.0, 70.0),
        "the opposite corner",
    );
    assert_near(
        sketch.point(b),
        DVec2::new(120.0, 45.0),
        "the corner after it",
    );
    assert_near(
        sketch.point(d),
        DVec2::new(5.0, 70.0),
        "the corner before it",
    );
    for (rank, side) in sides.iter().enumerate() {
        assert!(
            direction(&sketch, *side).distance(before[rank]) < 1e-6,
            "side {rank} turned to {}",
            direction(&sketch, *side)
        );
    }
}

fn typed(sketch: &mut Sketch, side: SegmentId) {
    let length = sketch.segment_length(side);
    sketch.set_dimension(DimensionTarget::Length(side), length, false);
}

fn turned_by(sketch: &Sketch, side: SegmentId, was: DVec2) -> f64 {
    was.angle_to(direction(sketch, side)).abs()
}

#[test]
fn an_l_pulled_by_one_end_keeps_its_far_end_and_its_corner_slides() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let top = sketch.add_point(DVec2::new(10.0, 60.0));
    let corner = sketch.add_point(DVec2::new(10.0, 10.0));
    let far = sketch.add_point(DVec2::new(90.0, 10.0));
    let up = sketch.add_segment(top, corner);
    let flat = sketch.add_segment(corner, far);
    sketch.add_constraint(Constraint::Perpendicular {
        first: up,
        second: flat,
    });

    sketch.settle_around(top, DVec2::new(-5.0, 70.0), 1.0);

    assert_near(sketch.point(far), DVec2::new(90.0, 10.0), "the far end");
    assert_near(sketch.point(corner), DVec2::new(-5.0, 10.0), "the corner");
}

#[test]
fn an_obtuse_parallelogram_pulled_by_a_corner_keeps_its_opposite_corner() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners = [
        DVec2::new(0.0, 0.0),
        DVec2::new(10.0, 0.0),
        DVec2::new(5.67, 2.5),
        DVec2::new(-4.33, 2.5),
    ]
    .map(|place| sketch.add_point(place + DVec2::splat(50.0)));
    let sides: [SegmentId; 4] =
        std::array::from_fn(|rank| sketch.add_segment(corners[rank], corners[(rank + 1) % 4]));
    for (first, second) in [(0, 2), (1, 3)] {
        sketch.add_constraint(Constraint::Parallel {
            first: sides[first],
            second: sides[second],
        });
    }
    let opposite = sketch.point(corners[2]);
    let was = sides.map(|side| direction(&sketch, side));

    sketch.settle_around(corners[0], DVec2::new(47.0, 49.0), 1.0);

    assert_near(sketch.point(corners[2]), opposite, "the opposite corner");
    for (rank, side) in sides.iter().enumerate() {
        assert!(
            turned_by(&sketch, *side, was[rank]) < UNTURNED,
            "side {rank} turned"
        );
    }
}

#[test]
fn a_triangle_with_two_typed_angles_grows_without_turning() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let p = sketch.add_point(DVec2::new(10.0, 10.0));
    let q = sketch.add_point(DVec2::new(110.0, 10.0));
    let r = sketch.add_point(DVec2::new(60.0, 60.0));
    let sides = [
        sketch.add_segment(p, q),
        sketch.add_segment(q, r),
        sketch.add_segment(r, p),
    ];
    sketch.set_dimension(
        DimensionTarget::Angle {
            first: sides[0],
            second: sides[1],
        },
        45.0,
        false,
    );
    sketch.set_dimension(
        DimensionTarget::Angle {
            first: sides[1],
            second: sides[2],
        },
        90.0,
        false,
    );
    sketch.resolve(1.0);
    let was = sides.map(|side| direction(&sketch, side));
    let size = sketch.segment_length(sides[0]);

    sketch.settle_around(r, DVec2::new(70.0, 80.0), 1.0);

    for (rank, side) in sides.iter().enumerate() {
        assert!(
            turned_by(&sketch, *side, was[rank]) < UNTURNED,
            "side {rank} turned"
        );
    }
    assert!(
        sketch.segment_length(sides[0]) > size + 1.0,
        "and it grew: {}",
        sketch.segment_length(sides[0])
    );
}

#[test]
fn a_free_tail_does_not_take_the_place_of_the_opposite_corner() {
    let (mut sketch, [a, b, c, _], _) = upright();
    let tail = sketch.add_point(DVec2::new(300.0, -100.0));
    sketch.add_segment(b, tail);

    sketch.settle_around(a, DVec2::new(5.0, 45.0), 1.0);

    assert_near(
        sketch.point(c),
        DVec2::new(120.0, 70.0),
        "the opposite corner",
    );
    assert_near(
        sketch.point(tail),
        DVec2::new(300.0, -100.0),
        "the tail's far end",
    );
}

#[test]
fn a_rectangle_held_at_a_corner_keeps_that_corner_and_stretches() {
    let (mut sketch, [a, _, c, _], sides) = rectangle([
        DVec2::new(0.0, 0.0),
        DVec2::new(100.0, 0.0),
        DVec2::new(100.0, 50.0),
        DVec2::new(0.0, 50.0),
    ]);
    sketch.merge_points(Sketch::ORIGIN, a);
    let was = sides.map(|side| direction(&sketch, side));

    sketch.settle_around(c, DVec2::new(130.0, 70.0), 1.0);

    assert_near(sketch.point(Sketch::ORIGIN), DVec2::ZERO, "the corner held");
    assert_near(
        sketch.point(c),
        DVec2::new(130.0, 70.0),
        "the corner pulled",
    );
    for (rank, side) in sides.iter().enumerate() {
        assert!(
            turned_by(&sketch, *side, was[rank]) < UNTURNED,
            "side {rank} turned"
        );
    }
}

#[test]
fn a_circle_dragged_by_its_centre_is_carried_at_its_size() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(40.0, 40.0));
    let circle = sketch.add_circle(centre, 15.0);
    let rim = sketch.add_point(DVec2::new(55.0, 40.0));
    sketch.add_constraint(Constraint::OnCircle { point: rim, circle });

    sketch.settle_around(centre, DVec2::new(70.0, 20.0), 1.0);

    assert!(
        (sketch.circle(circle).radius - 15.0).abs() < SETTLED,
        "the size"
    );
    assert_near(sketch.point(centre), DVec2::new(70.0, 20.0), "the centre");
    let reach = sketch.point(rim).distance(sketch.point(centre));
    assert!(
        (reach - 15.0).abs() < SETTLED,
        "the rim point came along: {reach}"
    );
}

#[test]
fn a_free_trait_of_typed_length_pivots_about_its_other_end() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let stays = sketch.add_point(DVec2::new(10.0, 10.0));
    let end = sketch.add_point(DVec2::new(110.0, 10.0));
    let side = sketch.add_segment(stays, end);
    typed(&mut sketch, side);

    sketch.settle_around(end, DVec2::new(110.0, 60.0), 1.0);

    assert_near(sketch.point(stays), DVec2::new(10.0, 10.0), "the other end");
    let towards = (DVec2::new(110.0, 60.0) - DVec2::new(10.0, 10.0)).normalize();
    assert_near(
        sketch.point(end),
        DVec2::new(10.0, 10.0) + towards * 100.0,
        "the end",
    );
}

#[test]
fn a_rectangle_typed_on_both_sides_pivots_about_its_opposite_corner() {
    let (mut sketch, [a, _, c, _], sides) = upright();
    typed(&mut sketch, sides[0]);
    typed(&mut sketch, sides[1]);
    let diagonal = sketch.point(a).distance(sketch.point(c));

    sketch.settle_around(a, DVec2::new(5.0, 45.0), 1.0);

    assert_near(
        sketch.point(c),
        DVec2::new(120.0, 70.0),
        "the opposite corner",
    );
    let towards = (DVec2::new(5.0, 45.0) - DVec2::new(120.0, 70.0)).normalize();
    assert_near(
        sketch.point(a),
        DVec2::new(120.0, 70.0) + towards * diagonal,
        "the corner pulled, at the distance the sizes allow",
    );
    assert!((sketch.segment_length(sides[0]) - 100.0).abs() < SETTLED);
    assert!((sketch.segment_length(sides[1]) - 50.0).abs() < SETTLED);
}

#[test]
fn a_triangle_of_three_typed_sides_pivots_about_its_far_corner() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let near = sketch.add_point(DVec2::new(0.0, 0.0));
    let middle = sketch.add_point(DVec2::new(40.0, 5.0));
    let far = sketch.add_point(DVec2::new(90.0, 30.0));
    for (from, to) in [(near, middle), (middle, far), (far, near)] {
        let side = sketch.add_segment(from, to);
        typed(&mut sketch, side);
    }
    let reach = sketch.point(near).distance(sketch.point(far));

    sketch.settle_around(near, DVec2::new(0.0, 60.0), 1.0);

    assert_near(sketch.point(far), DVec2::new(90.0, 30.0), "the far corner");
    let towards = (DVec2::new(0.0, 60.0) - DVec2::new(90.0, 30.0)).normalize();
    assert_near(
        sketch.point(near),
        DVec2::new(90.0, 30.0) + towards * reach,
        "the corner pulled",
    );
}

#[test]
fn a_pivot_a_rule_forbids_leaves_the_shape_where_it_is() {
    let (mut sketch, [a, b, c, d], sides) = upright();
    typed(&mut sketch, sides[0]);
    typed(&mut sketch, sides[1]);
    sketch.add_constraint(Constraint::AxisParallel {
        segment: sides[0],
        axis: SketchAxis::U,
    });
    let before = [a, b, c, d].map(|point| sketch.point(point));

    sketch.settle_around(a, DVec2::new(5.0, 45.0), 1.0);

    for (rank, point) in [a, b, c, d].iter().enumerate() {
        assert_near(sketch.point(*point), before[rank], "a corner");
    }
}

#[test]
fn a_rectangle_with_only_its_width_typed_takes_its_height_from_the_cursor() {
    let (mut sketch, [a, b, c, d], sides) = upright();
    typed(&mut sketch, sides[0]);

    sketch.settle_around(a, DVec2::new(5.0, 45.0), 1.0);

    assert_near(sketch.point(a), DVec2::new(20.0, 45.0), "the corner pulled");
    assert_near(
        sketch.point(b),
        DVec2::new(120.0, 45.0),
        "the corner after it",
    );
    assert_near(
        sketch.point(c),
        DVec2::new(120.0, 70.0),
        "the opposite corner",
    );
    assert_near(
        sketch.point(d),
        DVec2::new(20.0, 70.0),
        "the corner before it",
    );
}

#[test]
fn a_leaning_trait_of_typed_length_hung_off_the_origin_pivots_about_it() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let end = sketch.add_point(DVec2::new(80.0, 60.0));
    let side = sketch.add_segment(Sketch::ORIGIN, end);
    typed(&mut sketch, side);

    sketch.settle_around(end, DVec2::new(0.0, 150.0), 1.0);

    assert_near(sketch.point(Sketch::ORIGIN), DVec2::ZERO, "the origin");
    assert_near(
        sketch.point(end),
        DVec2::new(0.0, 100.0),
        "the end, pivoted",
    );
}

#[test]
fn an_arc_end_slides_round_its_circle_and_the_other_end_keeps_its_direction() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(50.0, 50.0));
    let start = sketch.add_point(DVec2::new(90.0, 50.0));
    let end = sketch.add_point(DVec2::new(50.0, 90.0));
    sketch.add_arc(centre, start, end);

    let landing = sketch.slide(start, DVec2::new(110.0, 60.0));
    sketch.settle_around(start, landing, 1.0);

    assert_near(sketch.point(centre), DVec2::new(50.0, 50.0), "the centre");
    assert_near(sketch.point(end), DVec2::new(50.0, 90.0), "the other end");
    let round = sketch.point(start) - DVec2::new(50.0, 50.0);
    assert!(
        (round.length() - 40.0).abs() < SETTLED,
        "the radius stays 40"
    );
    assert!(round.y > 1.0, "and the end went round: {round}");
}

#[test]
fn an_ellipse_with_a_typed_axis_turns_about_its_centre() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(50.0, 20.0));
    let west = sketch.add_point(DVec2::new(20.0, 20.0));
    let east = sketch.add_point(DVec2::new(80.0, 20.0));
    let south = sketch.add_point(DVec2::new(50.0, 10.0));
    let north = sketch.add_point(DVec2::new(50.0, 30.0));
    let id = sketch.add_ellipse(centre, [west, east], [south, north]);
    let first = sketch.ellipses()[id.0].first;
    typed(&mut sketch, first);

    sketch.settle_around(east, DVec2::new(50.0, 80.0), 1.0);

    assert_near(sketch.point(centre), DVec2::new(50.0, 20.0), "the centre");
    assert_near(
        sketch.point(east),
        DVec2::new(50.0, 50.0),
        "the end, turned round",
    );
    assert_near(
        sketch.point(north),
        DVec2::new(40.0, 20.0),
        "the other axis turned with it",
    );
}

#[test]
fn an_axis_end_pulled_twice_as_far_off_its_axis_as_along_it_follows_the_hand() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(50.0, 20.0));
    let west = sketch.add_point(DVec2::new(20.0, 20.0));
    let east = sketch.add_point(DVec2::new(80.0, 20.0));
    let south = sketch.add_point(DVec2::new(50.0, 10.0));
    let north = sketch.add_point(DVec2::new(50.0, 30.0));
    sketch.add_ellipse(centre, [west, east], [south, north]);
    let hand = DVec2::new(75.0, 45.0);

    sketch.settle_around(east, hand, 1.0);

    let eighth = DVec2::from_angle(std::f64::consts::FRAC_PI_4);
    assert_near(sketch.point(centre), DVec2::new(50.0, 20.0), "the centre");
    assert_near(sketch.point(east), hand, "the end, under the hand");
    assert_near(sketch.point(west), DVec2::new(25.0, -5.0), "the other end");
    assert_near(
        sketch.point(north),
        DVec2::new(50.0, 20.0) + eighth.rotate(DVec2::new(0.0, 10.0)),
        "the other axis, turned with it at its length",
    );
}

#[test]
fn an_axis_end_swung_onto_a_place_as_far_out_as_it_stands_ends_there() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(50.0, 20.0));
    let west = sketch.add_point(DVec2::new(37.0, 20.0));
    let east = sketch.add_point(DVec2::new(63.0, 20.0));
    let south = sketch.add_point(DVec2::new(50.0, 15.0));
    let north = sketch.add_point(DVec2::new(50.0, 25.0));
    sketch.add_ellipse(centre, [west, east], [south, north]);

    sketch.settle_around(east, DVec2::new(62.0, 15.0), 1.0);

    assert_near(sketch.point(east), DVec2::new(62.0, 15.0), "the end");
    assert_near(sketch.point(centre), DVec2::new(50.0, 20.0), "the centre");
}

#[test]
fn a_rounded_corner_keeps_its_rectangle_from_going_through_inside_out() {
    let (mut sketch, [a, ..], sides) = upright();
    sketch
        .fillet(sides[1], sides[2], 10.0)
        .expect("the top right corner rounded");

    sketch.settle_around(a, DVec2::new(140.0, 90.0), 1.0);

    assert!(
        direction(&sketch, sides[1]).y > 0.0,
        "the right side still runs up into the rounding: {}",
        direction(&sketch, sides[1])
    );
    assert!(
        direction(&sketch, sides[2]).x < 0.0,
        "the top still runs away from it: {}",
        direction(&sketch, sides[2])
    );
}

#[test]
fn a_hinge_pulled_further_out_of_reach_goes_no_further_back() {
    let mut reached = Vec::new();
    for hand in [-80.0, -100.0, -150.0] {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let tip = sketch.add_point(DVec2::new(0.0, 0.0));
        let knee = sketch.add_point(DVec2::new(50.0, 30.0));
        let foot = sketch.add_point(DVec2::new(100.0, 0.0));
        for (from, to) in [(tip, knee), (knee, foot)] {
            let bar = sketch.add_segment(from, to);
            typed(&mut sketch, bar);
        }
        sketch.settle_around(tip, DVec2::new(hand, 0.0), 1.0);
        reached.push(sketch.point(tip).x);
    }

    for pair in reached.windows(2) {
        assert!(
            pair[1] <= pair[0] + 1e-3,
            "the tip came back as the hand went on: {reached:?}"
        );
    }
}

#[test]
fn with_no_rules_only_the_dragged_point_moves() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners = [
        DVec2::new(0.0, 0.0),
        DVec2::new(40.0, 0.0),
        DVec2::new(40.0, 30.0),
        DVec2::new(0.0, 30.0),
    ]
    .map(|place| sketch.add_point(place + DVec2::splat(10.0)));
    for rank in 0..4 {
        sketch.add_segment(corners[rank], corners[(rank + 1) % 4]);
    }

    sketch.settle_around(corners[0], DVec2::new(-20.0, 5.0), 1.0);

    assert_near(
        sketch.point(corners[0]),
        DVec2::new(-20.0, 5.0),
        "the corner pulled",
    );
    for rank in 1..4 {
        let was = [
            DVec2::new(40.0, 0.0),
            DVec2::new(40.0, 30.0),
            DVec2::new(0.0, 30.0),
        ][rank - 1];
        assert_near(
            sketch.point(corners[rank]),
            was + DVec2::splat(10.0),
            "a corner left alone",
        );
    }
}

#[test]
fn two_typed_bars_hinged_together_bend_to_reach_the_cursor() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let base = sketch.add_point(DVec2::new(0.0, 0.0));
    let elbow = sketch.add_point(DVec2::new(50.0, 0.0));
    let tip = sketch.add_point(DVec2::new(50.0, 40.0));
    for (from, to) in [(base, elbow), (elbow, tip)] {
        let bar = sketch.add_segment(from, to);
        typed(&mut sketch, bar);
    }

    sketch.settle_around(tip, DVec2::new(20.0, 60.0), 1.0);

    assert_near(sketch.point(base), DVec2::ZERO, "the base");
    assert_near(
        sketch.point(tip),
        DVec2::new(20.0, 60.0),
        "the tip, on the cursor",
    );
    assert!((sketch.point(elbow).distance(DVec2::ZERO) - 50.0).abs() < SETTLED);
    assert!((sketch.point(elbow).distance(sketch.point(tip)) - 40.0).abs() < SETTLED);
}

#[test]
fn a_drag_leaves_the_verdict_and_the_freedom_as_they_were() {
    let (mut sketch, [a, ..], sides) = upright();
    typed(&mut sketch, sides[0]);
    sketch.add_constraint(Constraint::Fixed {
        element: Element::Point(a),
    });
    let mut untouched = sketch.clone();

    sketch.settle_around(a, DVec2::new(5.0, 45.0), 1.0);
    untouched.give_back(sketch.shapes_now());

    assert_eq!(
        sketch.freedom(1.0).degrees_of_freedom,
        untouched.freedom(1.0).degrees_of_freedom
    );
    assert_eq!(sketch.settled_points(1.0), untouched.settled_points(1.0));
    assert!(
        sketch.kept_lines().is_empty(),
        "no kept line outlives the drag"
    );
}

#[test]
fn a_pivoting_point_is_pulled_onto_a_grid_point_within_reach() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let stays = sketch.add_point(DVec2::new(10.0, 10.0));
    let end = sketch.add_point(DVec2::new(110.0, 10.0));
    let side = sketch.add_segment(stays, end);
    typed(&mut sketch, side);
    let pull = sketch.pull(end, 1.0);
    let grid = crate::snap::SnapSettings {
        point_reach: 1.0,
        curve_reach: 1.0,
        grid_step: Some(10.0),
        grid_reach: 3.0,
    };

    let near_upright = pull.onto_grid(&sketch, DVec2::new(12.0, 150.0), &grid);
    let off_the_grid = pull.onto_grid(&sketch, DVec2::new(60.0, 150.0), &grid);
    sketch.settle_pulled(&pull, near_upright, 1.0);

    assert_near(
        near_upright,
        DVec2::new(10.0, 110.0),
        "the end, onto the grid",
    );
    assert_near(
        sketch.point(end),
        DVec2::new(10.0, 110.0),
        "and the trait stands upright",
    );
    let towards = (DVec2::new(60.0, 150.0) - DVec2::new(10.0, 10.0)).normalize();
    assert_near(
        off_the_grid,
        DVec2::new(10.0, 10.0) + towards * 100.0,
        "no grid point near",
    );
}

#[test]
fn a_rectangle_pulled_past_its_opposite_corner_comes_out_the_other_way_round() {
    let (mut sketch, [a, b, c, d], sides) = upright();
    let before = sides.map(|side| direction(&sketch, side));

    sketch.settle_around(a, DVec2::new(140.0, 90.0), 1.0);

    assert_near(
        sketch.point(a),
        DVec2::new(140.0, 90.0),
        "the corner pulled",
    );
    assert_near(
        sketch.point(b),
        DVec2::new(120.0, 90.0),
        "the corner after it",
    );
    assert_near(
        sketch.point(c),
        DVec2::new(120.0, 70.0),
        "the opposite corner",
    );
    assert_near(
        sketch.point(d),
        DVec2::new(140.0, 70.0),
        "the corner before it",
    );
    for (rank, side) in sides.iter().enumerate() {
        let now = direction(&sketch, *side);
        assert!(
            now.perp_dot(before[rank]).abs() < UNTURNED,
            "side {rank} lies as it did, only the other way round: {now}"
        );
    }
}

#[test]
fn a_corner_held_to_its_height_by_a_typed_width_goes_through_the_opposite_side() {
    let (mut sketch, [a, b, c, d], sides) = upright();
    typed(&mut sketch, sides[0]);
    let pull = sketch.pull(a, 1.0);

    for height in [60.0, 75.0, 120.0] {
        let mut dragged = sketch.clone();
        dragged.settle_pulled(&pull, DVec2::new(20.0, height), 1.0);
        assert_near(
            dragged.point(a),
            DVec2::new(20.0, height),
            "the corner followed the hand",
        );
        assert_near(dragged.point(b), DVec2::new(120.0, height), "its neighbour");
        assert_near(dragged.point(c), DVec2::new(120.0, 70.0), "the far side");
        assert_near(dragged.point(d), DVec2::new(20.0, 70.0), "the far side");
    }
}

#[test]
fn a_hinge_pulled_out_of_reach_stops_on_the_way_to_the_hand() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let tip = sketch.add_point(DVec2::new(0.0, 0.0));
    let knee = sketch.add_point(DVec2::new(50.0, 30.0));
    let foot = sketch.add_point(DVec2::new(100.0, 0.0));
    for (from, to) in [(tip, knee), (knee, foot)] {
        let bar = sketch.add_segment(from, to);
        typed(&mut sketch, bar);
    }

    sketch.settle_around(tip, DVec2::new(-80.0, 0.0), 1.0);

    let reached = sketch.point(tip);
    assert!(
        reached.x < -15.0 && reached.y.abs() < 1.0,
        "it stopped towards the hand: {reached}"
    );
    assert_near(sketch.point(foot), DVec2::new(100.0, 0.0), "the far end");
}

#[test]
fn an_arc_end_of_typed_radius_slides_all_the_way_round() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(50.0, 50.0));
    let start = sketch.add_point(DVec2::new(80.0, 50.0));
    let end = sketch.add_point(DVec2::new(50.0, 80.0));
    let arc = sketch.add_arc(centre, start, end);
    sketch.set_dimension(DimensionTarget::ArcRadius(arc), 30.0, false);
    sketch.resolve(1.0);
    let pull = sketch.pull(start, 1.0);

    let cursor = DVec2::new(50.0, 50.0) + DVec2::from_angle((-100.0_f64).to_radians()) * 45.0;
    let landing = sketch.slide(start, cursor);
    sketch.settle_pulled(&pull, landing, 1.0);

    let went = (sketch.point(start) - DVec2::new(50.0, 50.0))
        .to_angle()
        .to_degrees();
    assert!((went + 100.0).abs() < 1e-3, "the end went round to {went}°");
}

#[test]
fn a_shape_tied_by_a_distance_to_another_lets_that_one_follow() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let a = sketch.add_point(DVec2::new(20.0, 20.0));
    let b = sketch.add_point(DVec2::new(120.0, 20.0));
    sketch.add_segment(a, b);
    let centre = sketch.add_point(DVec2::new(50.0, 45.0));
    sketch.add_circle(centre, 5.0);
    let apart = sketch.point(a).distance(sketch.point(centre));
    sketch.set_dimension(
        DimensionTarget::Distance {
            from: a,
            to: centre,
        },
        apart,
        false,
    );

    sketch.settle_around(a, DVec2::new(0.0, -20.0), 1.0);

    assert_near(
        sketch.point(a),
        DVec2::new(0.0, -20.0),
        "the point, under the hand",
    );
    assert!((sketch.point(a).distance(sketch.point(centre)) - apart).abs() < SETTLED);
}

#[test]
fn a_point_held_on_a_trait_leaves_the_far_end_whichever_way_the_trait_was_drawn() {
    for backwards in [false, true] {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let near = sketch.add_point(DVec2::new(100.0, 0.0));
        let far = sketch.add_point(DVec2::new(0.0, 0.0));
        let side = match backwards {
            false => sketch.add_segment(far, near),
            true => sketch.add_segment(near, far),
        };
        let held = sketch.add_point(DVec2::new(60.0, 0.0));
        sketch.add_constraint(Constraint::OnSegment {
            point: held,
            segment: side,
        });

        sketch.settle_around(near, DVec2::new(100.0, 30.0), 1.0);

        assert_near(sketch.point(far), DVec2::ZERO, "the far end");
    }
}

#[test]
fn a_trapezoid_pulled_by_a_corner_keeps_its_far_corner() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners = [
        DVec2::new(20.0, 50.0),
        DVec2::new(80.0, 50.0),
        DVec2::new(100.0, 0.0),
        DVec2::new(0.0, 0.0),
    ]
    .map(|place| sketch.add_point(place));
    let sides: [SegmentId; 4] =
        std::array::from_fn(|rank| sketch.add_segment(corners[rank], corners[(rank + 1) % 4]));
    sketch.add_constraint(Constraint::Parallel {
        first: sides[0],
        second: sides[2],
    });

    sketch.settle_around(corners[0], DVec2::new(10.0, 80.0), 1.0);

    assert_near(
        sketch.point(corners[0]),
        DVec2::new(10.0, 80.0),
        "the corner pulled",
    );
    assert_near(
        sketch.point(corners[2]),
        DVec2::new(100.0, 0.0),
        "the far corner",
    );
}

#[test]
fn a_circle_centred_on_a_corner_does_not_take_the_place_of_the_opposite_corner() {
    let (mut sketch, [a, _, c, _], _) = upright();
    sketch.add_circle(c, 8.0);

    sketch.settle_around(a, DVec2::new(5.0, 45.0), 1.0);

    assert_near(sketch.point(a), DVec2::new(5.0, 45.0), "the corner pulled");
    assert_near(
        sketch.point(c),
        DVec2::new(120.0, 70.0),
        "the opposite corner",
    );
}

#[test]
fn a_corner_that_carries_a_circle_stretches_its_rectangle_and_carries_the_circle() {
    let (mut sketch, [a, _, c, _], _) = upright();
    let circle = sketch.add_circle(a, 8.0);

    sketch.settle_around(a, DVec2::new(5.0, 45.0), 1.0);

    assert_near(
        sketch.point(c),
        DVec2::new(120.0, 70.0),
        "the opposite corner",
    );
    assert_near(
        sketch.point(sketch.circle(circle).center),
        DVec2::new(5.0, 45.0),
        "the circle came along",
    );
    assert!(
        (sketch.circle(circle).radius - 8.0).abs() < SETTLED,
        "at its size"
    );
}

#[test]
fn two_shapes_sharing_the_origin_pivot_apart() {
    let (mut sketch, [a, _, c, _], sides) = rectangle([
        DVec2::new(0.0, 0.0),
        DVec2::new(100.0, 0.0),
        DVec2::new(100.0, 50.0),
        DVec2::new(0.0, 50.0),
    ]);
    sketch.merge_points(Sketch::ORIGIN, a);
    typed(&mut sketch, sides[0]);
    typed(&mut sketch, sides[1]);
    let other = sketch.add_point(DVec2::new(-60.0, -40.0));
    sketch.add_segment(Sketch::ORIGIN, other);

    sketch.settle_around(c, DVec2::new(50.0, 120.0), 1.0);

    assert_near(
        sketch.point(other),
        DVec2::new(-60.0, -40.0),
        "the other shape",
    );
    let reach = sketch.point(c).length();
    assert!(
        (reach - 100.0_f64.hypot(50.0)).abs() < 1e-3,
        "pivoted about the origin: {reach}"
    );
}

#[test]
fn a_pivoting_drag_is_taken_towards_the_hand_and_joins_only_what_it_lands_on() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let stays = sketch.add_point(DVec2::new(10.0, 10.0));
    let end = sketch.add_point(DVec2::new(110.0, 10.0));
    let side = sketch.add_segment(stays, end);
    typed(&mut sketch, side);
    let near = sketch.add_point(DVec2::new(12.0, 107.0));
    let pull = sketch.pull(end, 1.0);
    let grid = crate::snap::SnapSettings {
        point_reach: 5.0,
        curve_reach: 5.0,
        grid_step: Some(10.0),
        grid_reach: 3.0,
    };

    let landing = pull.landing(
        &sketch,
        DVec2::new(12.0, 150.0),
        DVec2::new(12.0, 107.0),
        false,
        &grid,
    );
    let mut settled = sketch.clone();
    settled.settle_pulled(&pull, landing, 1.0);

    assert!(pull.turns());
    assert_near(
        landing,
        DVec2::new(10.0, 110.0),
        "towards the hand, onto the grid",
    );
    assert!(pull.arrived(&settled, landing));
    assert_eq!(
        pull.joined_to(&sketch, &settled, landing, 5.0),
        None,
        "near is not on"
    );
    let _ = near;
}

#[test]
fn a_point_that_stopped_short_of_the_hand_is_joined_to_nothing() {
    let (mut sketch, [a, ..], sides) = upright();
    typed(&mut sketch, sides[0]);
    let beside = sketch.add_point(DVec2::new(5.0, 45.0));
    let pull = sketch.pull(a, 1.0);
    let grid = crate::snap::SnapSettings {
        point_reach: 5.0,
        curve_reach: 5.0,
        grid_step: None,
        grid_reach: 0.0,
    };

    let landing = pull.landing(
        &sketch,
        DVec2::new(5.0, 45.0),
        DVec2::new(5.0, 45.0),
        false,
        &grid,
    );
    let mut settled = sketch.clone();
    settled.settle_pulled(&pull, landing, 1.0);

    assert!(!pull.arrived(&settled, landing), "the width held it back");
    assert_eq!(pull.joined_to(&sketch, &settled, landing, 5.0), None);
    let _ = beside;
}

#[test]
fn a_corner_held_at_a_typed_distance_goes_round_towards_the_hand() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let apex = sketch.add_point(DVec2::new(50.0, 80.0));
    let left = sketch.add_point(DVec2::new(10.0, 10.0));
    let right = sketch.add_point(DVec2::new(110.0, 10.0));
    sketch.add_segment(apex, left);
    sketch.add_segment(left, right);
    let typed_side = sketch.add_segment(right, apex);
    typed(&mut sketch, typed_side);
    let reach = sketch.segment_length(typed_side);

    sketch.settle_around(apex, DVec2::new(20.0, 50.0), 1.0);

    let went = sketch.point(apex);
    assert!(
        went.distance(DVec2::new(50.0, 80.0)) > 5.0,
        "it went round towards the hand: {went}"
    );
    assert!(
        (went.distance(sketch.point(right)) - reach).abs() < SETTLED,
        "at its length"
    );
}

#[test]
fn an_l_tied_by_a_collinear_rule_keeps_its_directions() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let a = sketch.add_point(DVec2::new(10.0, 10.0));
    let b = sketch.add_point(DVec2::new(60.0, 10.0));
    let c = sketch.add_point(DVec2::new(80.0, 10.0));
    let d = sketch.add_point(DVec2::new(130.0, 10.0));
    let first = sketch.add_segment(a, b);
    let second = sketch.add_segment(c, d);
    sketch.add_segment(b, c);
    sketch.add_constraint(Constraint::Collinear { first, second });
    let was = [first, second].map(|side| direction(&sketch, side));

    sketch.settle_around(a, DVec2::new(0.0, 30.0), 1.0);

    for (rank, side) in [first, second].iter().enumerate() {
        assert!(
            turned_by(&sketch, *side, was[rank]) < UNTURNED,
            "trait {rank} turned"
        );
    }
}
