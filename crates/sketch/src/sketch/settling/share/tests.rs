//! What sketch · settling/share.rs is held to.
//!
//! Closes #444.
//! - 1: a point held on a circle keeps its angle when the circle's centre is
//!   dragged, when the circle is drawn to another size, and when the shape it
//!   belongs to is pulled —
//!   `a_point_on_a_circle_keeps_its_angle_when_the_centre_is_dragged`,
//!   `a_point_on_a_circle_keeps_its_angle_when_the_circle_is_drawn_to_another_size`,
//!   `a_point_on_a_circle_keeps_its_angle_when_the_shape_it_belongs_to_is_pulled`,
//!   also where a value refuses the centre the hand's place —
//!   `a_point_on_a_circle_whose_centre_a_value_places_keeps_its_angle_when_the_centre_is_dragged`,
//!   and where the centre ends a trait —
//!   `a_point_on_a_circle_whose_centre_ends_a_trait_keeps_its_angle_and_the_circle_its_size`
//! - 2: a point held on a trait keeps its share when either end is pulled,
//!   when the trait is pulled across, and when its shape stretches or turns —
//!   `a_point_on_a_trait_keeps_its_share_when_either_end_is_pulled`,
//!   `a_point_on_a_trait_keeps_its_share_when_the_trait_is_pulled_across`,
//!   `a_point_on_a_side_keeps_its_share_when_its_rectangle_stretches_or_turns`,
//!   `a_point_on_a_trait_off_a_circle_keeps_its_share_when_the_circle_is_drawn_to_another_size`,
//!   also a point a trait leaves square from —
//!   `a_point_a_trait_leaves_square_from_keeps_its_share_of_the_side`
//! - 3: a point held on an arc keeps its share of the sweep when an end is
//!   pulled round, when the arc is drawn to another size or slid round, and
//!   when its centre is dragged —
//!   `a_point_on_an_arc_keeps_its_share_when_an_end_is_pulled_round`,
//!   `a_point_on_an_arc_keeps_its_share_when_the_arc_is_drawn_to_another_size_or_slid_round`,
//!   `a_point_on_an_arc_keeps_its_share_when_its_centre_is_dragged`, a point
//!   slid past the start staying past it —
//!   `a_point_slid_just_behind_an_arc_s_start_stays_behind_it_as_the_arc_opens`
//! - 4: a value that places the point wins over its share —
//!   `a_point_a_value_places_on_its_trait_keeps_that_value`
//! - 5: a point dragged along its support slides, and keeps the share it was
//!   let go of at — `a_point_dragged_along_its_trait_keeps_the_share_it_was_let_go_at`
//! - "Always, unless a value places it": a value typed that moves what holds
//!   the point keeps it at its place too —
//!   `a_point_keeps_its_place_when_a_value_typed_moves_what_holds_it`
//! - 6: #374, #375 and #422's tests stay green — no test: here, the whole
//!   suite is the check; a part replayed rebuilds the same drawing —
//!   `a_trait_end_pulled_with_a_point_held_on_it_is_rebuilt_as_shown` in
//!   cao_part's a_drag_is_replayed_as_it_was_shown.rs
//! - 7: checked by hand in the application — no test: the hand is the check

use glam::DVec2;

use crate::constraints::{Constraint, DimensionTarget};
use crate::plane::WorkPlane;
use crate::resizing::Curved;
use crate::sketch::{PointId, SegmentId, Sketch};
use crate::snap::SnapSettings;

/// Close enough for points the solver placed: it stops at a hundred-thousandth
/// of the drawing's size, and these drawings are about a hundred across.
const SETTLED: f64 = 1e-3;

fn assert_near(found: DVec2, wanted: DVec2, what: &str) {
    assert!(
        found.distance(wanted) < SETTLED,
        "{what}: wanted {wanted}, found {found}"
    );
}

fn no_grid() -> SnapSettings {
    SnapSettings {
        point_reach: 1.0,
        curve_reach: 1.0,
        grid_step: None,
        grid_reach: 0.0,
    }
}

fn at(degrees: f64) -> DVec2 {
    DVec2::from_angle(degrees.to_radians())
}

/// A circle of radius 30 about (50, 50), and a point held on it at 40°.
fn a_circle() -> (Sketch, PointId, crate::sketch::CircleId, PointId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(50.0, 50.0));
    let circle = sketch.add_circle(centre, 30.0);
    let held = sketch.add_point(DVec2::new(50.0, 50.0) + at(40.0) * 30.0);
    sketch.add_constraint(Constraint::OnCircle {
        point: held,
        circle,
    });
    (sketch, centre, circle, held)
}

/// A trait from the origin of the page to (100, 0), and a point held on it
/// at 70 of it.
fn a_trait() -> (Sketch, [PointId; 2], SegmentId, PointId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(10.0, 10.0));
    let end = sketch.add_point(DVec2::new(110.0, 10.0));
    let line = sketch.add_segment(start, end);
    let held = sketch.add_point(DVec2::new(80.0, 10.0));
    sketch.add_constraint(Constraint::OnSegment {
        point: held,
        segment: line,
    });
    (sketch, [start, end], line, held)
}

/// A quarter arc of radius 40 about (50, 50), from (90, 50) round to (50, 90),
/// and a point held on it halfway round.
fn an_arc() -> (Sketch, [PointId; 3], crate::arc::ArcId, PointId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(50.0, 50.0));
    let start = sketch.add_point(DVec2::new(90.0, 50.0));
    let end = sketch.add_point(DVec2::new(50.0, 90.0));
    let arc = sketch.add_arc(centre, start, end);
    let held = sketch.add_point(DVec2::new(50.0, 50.0) + at(45.0) * 40.0);
    sketch.add_constraint(Constraint::OnArc { point: held, arc });
    (sketch, [centre, start, end], arc, held)
}

/// A rectangle as the tool lays it, 100 wide and 50 high, its corners
/// counter-clockwise from (20, 20).
fn a_rectangle() -> (Sketch, [PointId; 4], [SegmentId; 4]) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corners = [
        DVec2::new(20.0, 20.0),
        DVec2::new(120.0, 20.0),
        DVec2::new(120.0, 70.0),
        DVec2::new(20.0, 70.0),
    ]
    .map(|place| sketch.add_point(place));
    let sides: [SegmentId; 4] =
        std::array::from_fn(|rank| sketch.add_segment(corners[rank], corners[(rank + 1) % 4]));
    for corner in 0..3 {
        sketch.add_constraint(Constraint::Perpendicular {
            first: sides[corner],
            second: sides[corner + 1],
        });
    }
    (sketch, corners, sides)
}

#[test]
fn a_point_on_a_circle_keeps_its_angle_when_the_centre_is_dragged() {
    let (mut sketch, centre, _, held) = a_circle();

    sketch.settle_around(centre, DVec2::new(61.0, 55.0), 1.0);

    assert_near(
        sketch.point(held),
        DVec2::new(61.0, 55.0) + at(40.0) * 30.0,
        "the point, at 40° still",
    );
}

#[test]
fn a_point_on_a_circle_keeps_its_angle_when_the_circle_is_drawn_to_another_size() {
    let (mut sketch, _, circle, held) = a_circle();

    sketch.resize(Curved::Circle(circle), 45.0, 1.0);

    assert_near(
        sketch.point(held),
        DVec2::new(50.0, 50.0) + at(40.0) * 45.0,
        "the point, at 40° on the bigger circle",
    );
}

#[test]
fn a_point_on_a_circle_keeps_its_angle_when_the_shape_it_belongs_to_is_pulled() {
    let (mut sketch, [_, b, c, _], _) = a_rectangle();
    let circle = sketch.add_circle(c, 10.0);
    let held = sketch.add_point(DVec2::new(120.0, 70.0) + at(40.0) * 10.0);
    sketch.add_constraint(Constraint::OnCircle {
        point: held,
        circle,
    });

    sketch.settle_around(b, DVec2::new(140.0, 20.0), 1.0);

    let centre = sketch.point(c);
    assert_near(
        centre,
        DVec2::new(140.0, 70.0),
        "the corner carrying the circle",
    );
    assert_near(
        sketch.point(held),
        centre + at(40.0) * 10.0,
        "the point, at 40°",
    );
}

#[test]
fn a_point_on_a_trait_keeps_its_share_when_either_end_is_pulled() {
    let (mut sketch, [start, end], _, held) = a_trait();
    sketch.settle_around(end, DVec2::new(160.0, 30.0), 1.0);
    let wanted = sketch.point(start).lerp(sketch.point(end), 0.7);
    assert_near(
        sketch.point(held),
        wanted,
        "at 70 of the trait, its end pulled",
    );

    let (mut sketch, [start, end], _, held) = a_trait();
    sketch.settle_around(start, DVec2::new(-40.0, -10.0), 1.0);
    let wanted = sketch.point(start).lerp(sketch.point(end), 0.7);
    assert_near(
        sketch.point(held),
        wanted,
        "at 70 of the trait, its start pulled",
    );
}

#[test]
fn a_point_on_a_trait_keeps_its_share_when_the_trait_is_pulled_across() {
    let (mut sketch, corners, sides) = a_rectangle();
    let held = sketch.add_point(DVec2::new(50.0, 70.0));
    sketch.add_constraint(Constraint::OnSegment {
        point: held,
        segment: sides[2],
    });

    sketch.move_side(sides[3], DVec2::new(-30.0, 0.0), 1.0);

    let wanted = sketch.point(corners[2]).lerp(sketch.point(corners[3]), 0.7);
    assert_near(
        sketch.point(held),
        wanted,
        "at 70 of the top, the rectangle wider",
    );
}

#[test]
fn a_point_on_a_side_keeps_its_share_when_its_rectangle_stretches_or_turns() {
    let (mut sketch, [_, b, c, _], sides) = a_rectangle();
    let held = sketch.add_point(DVec2::new(120.0, 55.0));
    sketch.add_constraint(Constraint::OnSegment {
        point: held,
        segment: sides[1],
    });

    sketch.settle_around(c, DVec2::new(140.0, 90.0), 1.0);
    let wanted = sketch.point(b).lerp(sketch.point(c), 0.7);
    assert_near(sketch.point(held), wanted, "at 70 of the stretched side");

    let drag = sketch.side_drag(
        sides[0],
        DVec2::new(70.0, 20.0),
        DVec2::new(85.0, 19.0),
        &no_grid(),
        1.0,
    );
    let crate::pulling::SideDrag::Along(turn) = drag else {
        panic!("sliding the bottom along turns: {drag:?}");
    };
    sketch.turn_shape(&turn.points, turn.about, turn.angle, 1.0);
    let wanted = sketch.point(b).lerp(sketch.point(c), 0.7);
    assert_near(sketch.point(held), wanted, "at 70 of the turned side");
}

#[test]
fn a_point_on_an_arc_keeps_its_share_when_an_end_is_pulled_round() {
    let (mut sketch, [_, _, end], _, held) = an_arc();

    sketch.settle_around(end, DVec2::new(10.0, 50.0), 1.0);

    assert_near(
        sketch.point(held),
        DVec2::new(50.0, 90.0),
        "halfway round the half circle it opened to",
    );
}

#[test]
fn a_point_on_an_arc_keeps_its_share_when_the_arc_is_drawn_to_another_size_or_slid_round() {
    let (mut sketch, _, arc, held) = an_arc();
    sketch.resize(Curved::Arc(arc), 60.0, 1.0);
    assert_near(
        sketch.point(held),
        DVec2::new(50.0, 50.0) + at(45.0) * 60.0,
        "halfway round, further out",
    );

    let (mut sketch, _, arc, held) = an_arc();
    let about = DVec2::new(50.0, 50.0);
    let cursor = about + DVec2::from_angle(1.1) * 44.0;
    let drag = sketch.curve_drag(
        Curved::Arc(arc),
        about + DVec2::from_angle(0.8) * 40.0,
        cursor,
        cursor,
        &no_grid(),
        1.0,
    );
    let crate::pulling::CurveDrag::Along {
        turn,
        reach: Some(reach),
    } = drag
    else {
        panic!("sliding round turns and draws out: {drag:?}");
    };
    sketch.turn_shape(&turn.points, turn.about, turn.angle, 1.0);
    sketch.resize_in_place(Curved::Arc(arc), reach, 1.0);
    assert_near(
        sketch.point(held),
        about + DVec2::from_angle(45f64.to_radians() + turn.angle) * reach,
        "halfway round, turned and further out",
    );
}

#[test]
fn a_point_on_an_arc_keeps_its_share_when_its_centre_is_dragged() {
    let (mut sketch, [centre, start, end], _, held) = an_arc();
    let by = DVec2::new(15.0, -5.0);

    sketch.settle_around_all(
        &[centre, start, end].map(|point| (point, sketch.point(point) + by)),
        1.0,
    );

    assert_near(
        sketch.point(held),
        DVec2::new(50.0, 50.0) + by + at(45.0) * 40.0,
        "halfway round, carried",
    );
}

#[test]
fn a_point_a_value_places_on_its_trait_keeps_that_value() {
    let (mut sketch, [start, end], _, held) = a_trait();
    sketch.set_dimension(
        DimensionTarget::Distance {
            from: start,
            to: held,
        },
        70.0,
        false,
    );

    sketch.settle_around(end, DVec2::new(160.0, 10.0), 1.0);

    assert_near(
        sketch.point(held),
        DVec2::new(80.0, 10.0),
        "70 from the start, as typed",
    );
}

#[test]
fn a_point_dragged_along_its_trait_keeps_the_share_it_was_let_go_at() {
    let (mut sketch, [start, end], _, held) = a_trait();
    sketch.settle_around(held, DVec2::new(40.0, 10.0), 1.0);
    assert_near(sketch.point(held), DVec2::new(40.0, 10.0), "slid along");

    sketch.settle_around(end, DVec2::new(210.0, 10.0), 1.0);

    let wanted = sketch.point(start).lerp(sketch.point(end), 0.3);
    assert_near(sketch.point(held), wanted, "at the 30 it was let go at");
}

#[test]
fn a_point_on_a_circle_whose_centre_a_value_places_keeps_its_angle_when_the_centre_is_dragged() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(60.0, 80.0));
    let circle = sketch.add_circle(centre, 20.0);
    let held = sketch.add_point(DVec2::new(60.0, 80.0) + at(40.0) * 20.0);
    sketch.add_constraint(Constraint::OnCircle {
        point: held,
        circle,
    });
    sketch.set_dimension(
        DimensionTarget::Distance {
            from: Sketch::ORIGIN,
            to: centre,
        },
        100.0,
        false,
    );
    sketch.resolve(1.0);

    sketch.settle_around(centre, DVec2::new(95.0, 50.0), 1.0);

    let now = sketch.point(centre);
    assert!(
        (now.length() - 100.0).abs() < SETTLED,
        "the value held the centre"
    );
    assert_near(
        sketch.point(held),
        now + at(40.0) * sketch.circle(circle).radius,
        "the point, at 40° still",
    );
}

#[test]
fn a_point_on_a_trait_off_a_circle_keeps_its_share_when_the_circle_is_drawn_to_another_size() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(50.0, 50.0));
    let circle = sketch.add_circle(centre, 30.0);
    let rim = sketch.add_point(DVec2::new(80.0, 50.0));
    sketch.add_constraint(Constraint::OnCircle { point: rim, circle });
    let far = sketch.add_point(DVec2::new(140.0, 50.0));
    let line = sketch.add_segment(rim, far);
    let held = sketch.add_point(DVec2::new(110.0, 50.0));
    sketch.add_constraint(Constraint::OnSegment {
        point: held,
        segment: line,
    });

    sketch.resize(Curved::Circle(circle), 45.0, 1.0);

    let wanted = sketch.point(rim).lerp(sketch.point(far), 0.5);
    assert_near(sketch.point(held), wanted, "halfway along the trait still");
}

#[test]
fn a_point_slid_just_behind_an_arc_s_start_stays_behind_it_as_the_arc_opens() {
    let (mut sketch, [_, _, end], _, held) = an_arc();
    sketch.settle_around(held, DVec2::new(50.0, 50.0) + at(-15.0) * 40.0, 1.0);

    sketch.settle_around(end, DVec2::new(50.0, 50.0) + at(95.0) * 40.0, 1.0);

    let angle = (sketch.point(held) - DVec2::new(50.0, 50.0))
        .to_angle()
        .to_degrees();
    assert!(
        (-17.0..-14.0).contains(&angle),
        "still a sixth of the sweep behind the start: {angle}°"
    );
}

#[test]
fn a_point_a_trait_leaves_square_from_keeps_its_share_of_the_side() {
    let (mut sketch, [_, b, c, d], sides) = a_rectangle();
    let foot = sketch.add_point(DVec2::new(90.0, 70.0));
    sketch.add_constraint(Constraint::OnSegment {
        point: foot,
        segment: sides[2],
    });
    let tip = sketch.add_point(DVec2::new(90.0, 110.0));
    let square = sketch.add_segment(foot, tip);
    sketch.add_constraint(Constraint::Perpendicular {
        first: sides[2],
        second: square,
    });

    sketch.settle_around(b, DVec2::new(170.0, 20.0), 1.0);

    let wanted = sketch.point(c).lerp(sketch.point(d), 0.3);
    assert_near(
        sketch.point(foot),
        wanted,
        "at 30 of the top from its start",
    );
}

#[test]
fn a_point_on_a_circle_whose_centre_ends_a_trait_keeps_its_angle_and_the_circle_its_size() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let tail = sketch.add_point(DVec2::new(30.0, 50.0));
    let centre = sketch.add_point(DVec2::new(50.0, 50.0));
    sketch.add_segment(tail, centre);
    let circle = sketch.add_circle(centre, 30.0);
    let held = sketch.add_point(DVec2::new(50.0, 50.0) + at(40.0) * 30.0);
    sketch.add_constraint(Constraint::OnCircle {
        point: held,
        circle,
    });

    sketch.settle_around(centre, DVec2::new(65.0, 65.0), 1.0);

    assert!(
        (sketch.circle(circle).radius - 30.0).abs() < SETTLED,
        "the circle kept its size: {}",
        sketch.circle(circle).radius
    );
    assert_near(
        sketch.point(held),
        sketch.point(centre) + at(40.0) * 30.0,
        "the point, at 40°",
    );
}

#[test]
fn a_point_keeps_its_place_when_a_value_typed_moves_what_holds_it() {
    let (mut sketch, [start, end], line, held) = a_trait();
    sketch.set_dimension(DimensionTarget::Length(line), 150.0, false);

    sketch.resolve_keeping_places(1.0);

    let wanted = sketch.point(start).lerp(sketch.point(end), 0.7);
    assert_near(sketch.point(held), wanted, "at 70 of the longer trait");

    let (mut sketch, centre, _, held) = a_circle();
    sketch.set_dimension(
        DimensionTarget::Distance {
            from: Sketch::ORIGIN,
            to: centre,
        },
        110.0,
        false,
    );

    sketch.resolve_keeping_places(1.0);

    assert_near(
        sketch.point(held),
        sketch.point(centre) + at(40.0) * 30.0,
        "at 40° about the centre the value moved",
    );
}
