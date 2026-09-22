//! What the trim preview names as going: the rules and values a click would
//! really take, asked of the pieces the cut leaves rather than of the whole
//! drawing.
//!
//! Closes #402.
//! - a point held on a piece the cut keeps is not shown as going —
//!   `a_point_held_on_the_piece_that_stays_is_not_said_to_be_going`
//! - an angle a piece inherits is not shown as going, whichever of the two
//!   traits the cut falls on — `an_angle_a_piece_keeps_is_not_said_to_be_going_whichever_arm_is_cut`,
//!   and, the same fault met from the other side, one an earlier cut already
//!   handed over is not shown as going by every cut after it —
//!   `an_angle_an_earlier_cut_handed_over_is_not_said_to_be_going_by_the_next_one`
//! - a value or rule the cut genuinely takes is shown as going even when the
//!   drawing carries another of the same shape elsewhere —
//!   `a_distance_the_cut_takes_is_said_to_be_going_though_another_edge_is_measured_too`,
//!   `a_reach_the_cut_takes_is_said_to_be_going_though_another_arc_is_measured_too`
//! - the candidates the reckoning walks are the pieces the cut returned, not
//!   every live curve —
//!   `a_distance_the_cut_takes_is_said_to_be_going_though_another_edge_is_measured_too`,
//!   `a_reach_the_cut_takes_is_said_to_be_going_though_another_arc_is_measured_too`:
//!   each fails the moment a curve the cut never touched is offered as a piece
//! - the preview for a trait and for an arc (#286) still says what it said in
//!   every case that was already right — no test: none was added here, since
//!   the tests of #286 live beside the module and stay in the gate unchanged;
//!   the ones that hold this are `a_rule_a_piece_inherits_is_not_said_to_be_going`,
//!   `a_value_a_piece_inherits_is_not_said_to_be_going`,
//!   `a_reach_a_piece_of_the_arc_keeps_is_not_said_to_be_going_and_a_sweep_is`
//!   and `a_point_held_on_the_stretch_that_goes_loses_what_held_it`

use cao_sketch::{Constraint, DimensionTarget, PointId, SegmentId, Sketch, WorkPlane};
use glam::DVec2;

const REACH: f64 = 10.0;

fn a_trait_with_two_points_on_it() -> (Sketch, SegmentId, [PointId; 4]) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(0.0, 1.0));
    let end = sketch.add_point(DVec2::new(10.0, 1.0));
    let segment = sketch.add_segment(start, end);
    let near = sketch.add_point(DVec2::new(3.0, 1.0));
    let far = sketch.add_point(DVec2::new(7.0, 1.0));
    (sketch, segment, [start, near, far, end])
}

#[test]
fn a_point_held_on_the_piece_that_stays_is_not_said_to_be_going() {
    let (mut sketch, segment, [_, near, far, _]) = a_trait_with_two_points_on_it();
    let beside_the_cut = sketch.add_point(DVec2::new(1.0, 1.0));
    sketch.add_constraint(Constraint::OnSegment {
        point: beside_the_cut,
        segment,
    });

    let going = sketch
        .trim_takes(segment, near, far)
        .expect("a cut that can be made");

    assert!(
        going.rules.is_empty(),
        "the point sits on the piece the cut keeps, and stays held by it: {:?}",
        going.rules,
    );
}

/// Two traits leaving one corner square to each other, the angle between them
/// typed, and two points sitting on each arm so either can be cut away from the
/// corner. The horizontal arm is laid first, so it holds the lower rank.
fn a_square_corner_with_its_angle_typed() -> (Sketch, [SegmentId; 2], [[PointId; 2]; 2]) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corner = sketch.add_point(DVec2::new(1.0, 1.0));
    let east = sketch.add_point(DVec2::new(11.0, 1.0));
    let north = sketch.add_point(DVec2::new(1.0, 11.0));
    let across = sketch.add_segment(corner, east);
    let up = sketch.add_segment(corner, north);
    sketch.set_dimension(
        DimensionTarget::Angle {
            first: across,
            second: up,
        },
        90.0,
        false,
    );
    let on_across = [
        sketch.add_point(DVec2::new(5.0, 1.0)),
        sketch.add_point(DVec2::new(8.0, 1.0)),
    ];
    let on_up = [
        sketch.add_point(DVec2::new(1.0, 5.0)),
        sketch.add_point(DVec2::new(1.0, 8.0)),
    ];
    (sketch, [across, up], [on_across, on_up])
}

#[test]
fn an_angle_a_piece_keeps_is_not_said_to_be_going_whichever_arm_is_cut() {
    let (sketch, arms, stops) = a_square_corner_with_its_angle_typed();

    for (arm, [near, far]) in arms.into_iter().zip(stops) {
        let going = sketch
            .trim_takes(arm, near, far)
            .expect("a cut that can be made");

        assert!(
            going.values.is_empty(),
            "the piece left at the corner keeps the angle, cutting {arm:?}: {:?}",
            going.values,
        );
    }
}

#[test]
fn a_distance_the_cut_takes_is_said_to_be_going_though_another_edge_is_measured_too() {
    let (mut sketch, segment, [_, near, far, _]) = a_trait_with_two_points_on_it();
    let low = sketch.add_point(DVec2::new(20.0, 0.0));
    let high = sketch.add_point(DVec2::new(20.0, 10.0));
    let elsewhere = sketch.add_segment(low, high);
    let above = sketch.add_point(DVec2::new(5.0, 5.0));
    let to_the_cut = DimensionTarget::PointToSegment {
        point: above,
        segment,
    };
    sketch.set_dimension(to_the_cut, 4.0, false);
    sketch.set_dimension(
        DimensionTarget::PointToSegment {
            point: above,
            segment: elsewhere,
        },
        15.0,
        false,
    );

    let going = sketch
        .trim_takes(segment, near, far)
        .expect("a cut that can be made");

    assert_eq!(
        going.values,
        vec![to_the_cut],
        "the foot falls in the stretch that goes, so the distance goes with it — \
         the one to the other edge answers for nothing here",
    );
}

#[test]
fn a_reach_the_cut_takes_is_said_to_be_going_though_another_arc_is_measured_too() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let east = sketch.add_point(DVec2::new(REACH, 0.0));
    let west = sketch.add_point(DVec2::new(-REACH, 0.0));
    let cut = sketch.add_arc(Sketch::ORIGIN, east, west);
    let other_centre = sketch.add_point(DVec2::new(50.0, 0.0));
    let other_east = sketch.add_point(DVec2::new(50.0 + REACH, 0.0));
    let other_west = sketch.add_point(DVec2::new(50.0 - REACH, 0.0));
    let elsewhere = sketch.add_arc(other_centre, other_east, other_west);
    sketch.set_dimension(DimensionTarget::ArcRadius(cut), REACH, false);
    sketch.set_dimension(DimensionTarget::ArcRadius(elsewhere), REACH, false);

    let going = sketch
        .arc_trim_takes(cut, east, west)
        .expect("a cut that can be made");

    assert_eq!(
        going.values,
        vec![DimensionTarget::ArcRadius(cut)],
        "the whole curve goes and its reach with it — the other arc's answers \
         for nothing here",
    );
}

#[test]
fn an_angle_an_earlier_cut_handed_over_is_not_said_to_be_going_by_the_next_one() {
    let (mut sketch, [across, _], [[near, far], _]) = a_square_corner_with_its_angle_typed();
    sketch
        .trim(across, near, far)
        .expect("the earlier cut, which hands the angle over to a piece");
    let left = sketch.add_point(DVec2::new(30.0, 30.0));
    let right = sketch.add_point(DVec2::new(40.0, 30.0));
    let unrelated = sketch.add_segment(left, right);
    let first = sketch.add_point(DVec2::new(33.0, 30.0));
    let second = sketch.add_point(DVec2::new(37.0, 30.0));

    let going = sketch
        .trim_takes(unrelated, first, second)
        .expect("a cut that can be made");

    assert!(
        going.values.is_empty(),
        "the angle is nowhere near this cut, whatever order it was stored in: {:?}",
        going.values,
    );
}
