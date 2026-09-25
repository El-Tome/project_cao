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
//!   `an_obtuse_parallelogram_pulled_by_a_corner_keeps_its_opposite_corner`,
//!   `a_triangle_with_two_typed_angles_grows_without_turning`
//! - 3: what stays is the point holding the shape, else the centre of the
//!   curve the point is on, else the farthest point along the traits tied by
//!   direction; a curve's centre still carries its curve —
//!   `a_rectangle_held_at_a_corner_keeps_that_corner_and_stretches`,
//!   `a_free_tail_does_not_take_the_place_of_the_opposite_corner`,
//!   `a_circle_dragged_by_its_centre_is_carried_at_its_size`
//! - 4: a shape that cannot stretch at all pivots about that point, the point
//!   pulled stopping where its sizes allow; a turn a rule forbids leaves it
//!   where it is — `a_free_trait_of_typed_length_pivots_about_its_other_end`,
//!   `a_rectangle_typed_on_both_sides_pivots_about_its_opposite_corner`,
//!   `a_triangle_of_three_typed_sides_pivots_about_its_far_corner`,
//!   `a_pivot_a_rule_forbids_leaves_the_shape_where_it_is`
//! - 5: a shape that can stretch one way only follows the hand that way and no
//!   further — `a_rectangle_with_only_its_width_typed_takes_its_height_from_the_cursor`
//! - 6: a trait of typed length whose other end is held still pivots about it —
//!   `a_leaning_trait_of_typed_length_hung_off_the_origin_pivots_about_it`
//! - 7: a point on a curve leaves the curve's centre in place: an arc's end
//!   slides round, its other end keeping its direction —
//!   `an_arc_end_slides_round_its_circle_and_the_other_end_keeps_its_direction`;
//!   an ellipse's axis end lengthens its axis without turning it — no test:
//!   here, `an_axis_end_dragged_alone_leaves_the_centre_where_it_was` in
//!   ellipse/tests.rs says it; an ellipse whose axis is typed turns about its
//!   centre — `an_ellipse_with_a_typed_axis_turns_about_its_centre`
//! - 8: a trait held by nothing stretches freely —
//!   `with_no_rules_only_the_dragged_point_moves`, and traits held by lengths
//!   alone still bend — `two_typed_bars_hinged_together_bend_to_reach_the_cursor`
//! - 19: the guard that keeps the way up when a value is typed stays — no
//!   test: here, `a_rectangle_does_not_turn_when_one_of_its_sides_changes` in
//!   sketch/tests/values.rs holds it, untouched
//! - 18: a shape pivoting brings the dragged point onto a grid point within
//!   reach — `a_pivoting_point_is_pulled_onto_a_grid_point_within_reach`
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
