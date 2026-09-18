//! What sketch · arc.rs is held to.

use super::*;
use crate::constraints::DimensionTarget;
use crate::plane::WorkPlane;

const TOLERANCE: f64 = 1e-9;

/// What being an arc means, whatever the solver did to get there: the two
/// ends the same reach from the centre, wherever the centre ended up.
fn assert_round(sketch: &Sketch, arc: ArcId) {
    let centre = sketch.point(sketch.arc(arc).center);
    let near = centre.distance(sketch.point(sketch.arc(arc).start));
    let far = centre.distance(sketch.point(sketch.arc(arc).end));
    assert!(
        (far - near).abs() < 1e-6 * near.max(1.0),
        "one end {near} from the centre, the other {far}",
    );
}

fn quarter() -> (Sketch, ArcId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::ZERO);
    let east = sketch.add_point(DVec2::new(10.0, 0.0));
    let north = sketch.add_point(DVec2::new(0.0, 10.0));
    let arc = sketch.add_arc(centre, east, north);
    (sketch, arc)
}

#[test]
fn an_arc_runs_counter_clockwise_from_its_start_to_its_end() {
    let (sketch, arc) = quarter();
    let sweep = sketch.arc_sweep(arc);

    assert!(
        (sweep - std::f64::consts::FRAC_PI_2).abs() < TOLERANCE,
        "a quarter turn expected, got {sweep}",
    );
}

#[test]
fn a_guide_arc_says_it_is_one_and_an_ordinary_arc_does_not() {
    let (mut sketch, drawn) = quarter();
    let arc = sketch.arc(drawn);
    let guide = sketch.add_construction_arc(arc.center, arc.start, arc.end);

    assert!(!sketch.arc(drawn).construction);
    assert!(sketch.arc(guide).construction);
}

#[test]
fn erasing_a_point_an_arc_leans_on_takes_the_arc_with_it() {
    for which in 0..3 {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let centre = sketch.add_point(DVec2::ZERO);
        let east = sketch.add_point(DVec2::new(10.0, 0.0));
        let north = sketch.add_point(DVec2::new(0.0, 10.0));
        let arc = sketch.add_arc(centre, east, north);
        assert_eq!(sketch.live_arcs().count(), 1);

        sketch.erase(Element::Point([centre, east, north][which]));

        assert!(
            sketch.is_erased_arc(arc),
            "point {which} left the arc behind"
        );
        assert_eq!(sketch.live_arcs().count(), 0);
    }
}

#[test]
fn an_arc_stays_round_when_the_drawing_pulls_one_of_its_ends_out() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let east = sketch.add_point(DVec2::new(10.0, 0.0));
    let north = sketch.add_point(DVec2::new(0.0, 10.0));
    let arc = sketch.add_arc(Sketch::ORIGIN, east, north);

    sketch.set_dimension(
        DimensionTarget::Distance {
            from: Sketch::ORIGIN,
            to: east,
        },
        20.0,
        false,
    );
    sketch.resolve(1.0);

    let start = sketch.point(east).length();
    let end = sketch.point(north).length();
    assert!(
        (start - 20.0).abs() < 1e-2,
        "the end asked for is at {start}"
    );
    assert!(
        (end - start).abs() < 1e-2,
        "the far end stayed at {end} while the near one went to {start}",
    );
    assert!((sketch.arc_radius(arc) - 20.0).abs() < 1e-2);
}

#[test]
fn an_arc_stays_round_when_one_of_its_ends_is_dragged() {
    let (mut sketch, arc) = quarter();
    let dragged = sketch.arc(arc).end;
    let dropped_at = DVec2::new(-3.0, 17.0);
    sketch.settle_around(dragged, dropped_at, 1.0);

    assert!(
        sketch.point(dragged).distance(dropped_at) < 1e-6,
        "the end let go of the cursor, at {:?}",
        sketch.point(dragged),
    );
    assert_round(&sketch, arc);
}

#[test]
fn a_point_that_is_nobodys_centre_carries_no_end_with_it() {
    let (sketch, arc) = quarter();
    let end = sketch.arc(arc).end;

    assert_eq!(sketch.arc_ends_around(end), Vec::new());
}

#[test]
fn dragging_the_centre_of_an_arc_would_carry_both_of_its_ends() {
    let (sketch, arc) = quarter();
    let drawn = sketch.arc(arc);

    let mut ends = sketch.arc_ends_around(drawn.center);
    ends.sort_by_key(|point| point.0);
    let mut expected = [drawn.start, drawn.end];
    expected.sort_by_key(|point| point.0);
    assert_eq!(ends, expected);
}

#[test]
fn an_erased_arc_carries_nothing_from_its_old_centre_any_more() {
    let (mut sketch, arc) = quarter();
    let centre = sketch.arc(arc).center;
    sketch.erase(Element::Arc(arc));

    assert_eq!(sketch.arc_ends_around(centre), Vec::new());
}

#[test]
fn an_erased_arc_leaves_the_points_it_leaned_on_alone() {
    let (mut sketch, arc) = quarter();
    sketch.erase(Element::Arc(arc));

    assert!(sketch.is_erased_arc(arc));
    assert_eq!(sketch.live_points().count(), 4);
}

#[test]
fn every_place_along_an_arc_is_the_same_reach_from_its_centre() {
    let (sketch, arc) = quarter();
    let places = sketch.arc_polyline(arc);

    assert!(places.len() >= 3, "a quarter turn is not two straight bits");
    for place in &places {
        let reach = place.length();
        assert!(
            (reach - 10.0).abs() < TOLERANCE,
            "reach {reach} at {place:?}"
        );
    }
    assert!(places.first().unwrap().distance(DVec2::new(10.0, 0.0)) < TOLERANCE);
    assert!(places.last().unwrap().distance(DVec2::new(0.0, 10.0)) < TOLERANCE);
}

#[test]
fn a_longer_arc_is_cut_into_more_pieces_than_a_shorter_one() {
    let (sketch, quarter) = quarter();
    let mut wider = Sketch::new(WorkPlane::XY);
    let centre = wider.add_point(DVec2::ZERO);
    let east = wider.add_point(DVec2::new(10.0, 0.0));
    let west = wider.add_point(DVec2::new(-10.0, 0.0));
    let half = wider.add_arc(centre, east, west);

    assert!(wider.arc_steps(half) > sketch.arc_steps(quarter));
}

#[test]
fn the_same_two_ends_the_other_way_round_are_the_rest_of_the_circle() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::ZERO);
    let east = sketch.add_point(DVec2::new(10.0, 0.0));
    let north = sketch.add_point(DVec2::new(0.0, 10.0));
    let long_way = sketch.add_arc(centre, north, east);
    let sweep = sketch.arc_sweep(long_way);

    assert!(
        (sweep - 3.0 * std::f64::consts::FRAC_PI_2).abs() < TOLERANCE,
        "three quarters expected, got {sweep}",
    );
}
