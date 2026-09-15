use std::f64::consts::TAU;

use glam::DVec2;

use super::*;
use crate::axis::ChosenAxis;
use crate::constraints::SketchAxis;
use crate::plane::WorkPlane;
use crate::sketch::SegmentId;

const TOLERANCE: f64 = 1e-9;

/// A short trait standing three units east of a centre point.
fn a_trait_beside_a_centre() -> (Sketch, PointId, Vec<Element>) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(1.0, 1.0));
    let near = sketch.add_point(DVec2::new(4.0, 1.0));
    let far = sketch.add_point(DVec2::new(5.0, 1.0));
    let side = Element::Segment(sketch.add_segment(near, far));
    (sketch, centre, vec![side])
}

#[test]
fn a_circular_pattern_lays_one_copy_short_of_the_count_asked() {
    let (mut sketch, centre, held) = a_trait_beside_a_centre();

    let made = sketch
        .pattern_around(&held, centre, 90.0, 4)
        .expect("a centre to turn about");

    assert_eq!(
        made.segments.len(),
        3,
        "the original is the first of the four, so three are laid"
    );
    assert_eq!(sketch.live_segments().count(), 4);
}

#[test]
fn every_copy_stands_one_step_further_round_than_the_last() {
    let (mut sketch, centre, held) = a_trait_beside_a_centre();

    let made = sketch
        .pattern_around(&held, centre, 90.0, 4)
        .expect("a centre to turn about");

    let at = sketch.point(centre);
    let mut angles: Vec<f64> = made
        .segments
        .iter()
        .map(|id| {
            let (from, _) = sketch.endpoints(*id);
            (from - at).to_angle().rem_euclid(TAU).to_degrees()
        })
        .collect();
    angles.sort_by(f64::total_cmp);
    for (got, should) in angles.iter().zip([90.0, 180.0, 270.0]) {
        assert!(
            (got - should).abs() <= 1e-9,
            "a copy stands at {got}°, wanted {should}°"
        );
    }
}

#[test]
fn a_copy_keeps_its_distance_from_the_centre_and_its_own_size() {
    let (mut sketch, centre, held) = a_trait_beside_a_centre();

    let made = sketch
        .pattern_around(&held, centre, 90.0, 4)
        .expect("a centre to turn about");

    let at = sketch.point(centre);
    for id in &made.segments {
        let (from, to) = sketch.endpoints(*id);
        assert!((from.distance(at) - 3.0).abs() <= TOLERANCE);
        assert!((to.distance(at) - 4.0).abs() <= TOLERANCE);
        assert!((from.distance(to) - 1.0).abs() <= TOLERANCE);
    }
}

#[test]
fn a_pattern_of_one_lays_nothing_at_all() {
    let (mut sketch, centre, held) = a_trait_beside_a_centre();

    assert_eq!(sketch.pattern_around(&held, centre, 90.0, 1), None);
    assert_eq!(sketch.live_segments().count(), 1);
}

#[test]
fn a_pattern_turning_by_nothing_would_pile_its_copies_up_and_is_refused() {
    let (mut sketch, centre, held) = a_trait_beside_a_centre();

    assert_eq!(sketch.pattern_around(&held, centre, 0.0, 4), None);
    assert_eq!(sketch.live_segments().count(), 1);
}

#[test]
fn a_centre_the_drawing_does_not_have_turns_nothing() {
    let (mut sketch, _, held) = a_trait_beside_a_centre();

    assert_eq!(sketch.pattern_around(&held, PointId(99), 90.0, 4), None);
}

#[test]
fn a_pattern_turns_a_curve_without_turning_it_over() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::ZERO);
    let arc_centre = sketch.add_point(DVec2::new(10.0, 0.0));
    let start = sketch.add_point(DVec2::new(12.0, 0.0));
    let end = sketch.add_point(DVec2::new(10.0, 2.0));
    let arc = sketch.add_arc(arc_centre, start, end);
    let sweep = sketch.arc_sweep(arc);

    let made = sketch
        .pattern_around(&[Element::Arc(arc)], centre, 120.0, 3)
        .expect("a centre to turn about");

    for copy in &made.arcs {
        assert!(
            (sketch.arc_sweep(*copy) - sweep).abs() <= 1e-9,
            "a turn is not a reflection, so the ends stay where they were: got {}",
            sketch.arc_sweep(*copy)
        );
    }
}

/// A trait a unit long, lying on the U axis with its near end at the origin.
fn a_trait_on_the_axis() -> (Sketch, Vec<Element>) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let near = sketch.add_point(DVec2::ZERO);
    let far = sketch.add_point(DVec2::new(1.0, 0.0));
    let side = Element::Segment(sketch.add_segment(near, far));
    (sketch, vec![side])
}

#[test]
fn a_rectangular_pattern_fills_the_grid_but_for_the_original() {
    let (mut sketch, held) = a_trait_on_the_axis();

    let made = sketch
        .pattern_along(
            &held,
            ChosenAxis::Sketch(SketchAxis::U),
            Repeats {
                step: 10.0,
                count: 3,
            },
            Repeats {
                step: 5.0,
                count: 2,
            },
        )
        .expect("a direction to run along");

    assert_eq!(
        made.segments.len(),
        5,
        "the original is one of the six, so five are laid"
    );
    assert_eq!(sketch.live_segments().count(), 6);
}

#[test]
fn every_copy_stands_a_whole_number_of_steps_along_and_across() {
    let (mut sketch, held) = a_trait_on_the_axis();

    let made = sketch
        .pattern_along(
            &held,
            ChosenAxis::Sketch(SketchAxis::U),
            Repeats {
                step: 10.0,
                count: 3,
            },
            Repeats {
                step: 5.0,
                count: 2,
            },
        )
        .expect("a direction to run along");

    let mut places: Vec<DVec2> = made
        .segments
        .iter()
        .map(|id| sketch.endpoints(*id).0)
        .collect();
    places.sort_by(|one, other| one.y.total_cmp(&other.y).then(one.x.total_cmp(&other.x)));
    let wanted = [
        DVec2::new(10.0, 0.0),
        DVec2::new(20.0, 0.0),
        DVec2::new(0.0, 5.0),
        DVec2::new(10.0, 5.0),
        DVec2::new(20.0, 5.0),
    ];
    for (got, should) in places.iter().zip(wanted) {
        assert!(
            got.distance(should) <= TOLERANCE,
            "a copy starts at {got}, wanted {should}"
        );
    }
}

#[test]
fn a_trait_of_the_drawing_leans_the_whole_grid() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let low = sketch.add_point(DVec2::ZERO);
    let high = sketch.add_point(DVec2::new(1.0, 1.0));
    let diagonal = sketch.add_segment(low, high);
    let held = sketch.add_point(DVec2::new(5.0, 0.0));
    let step = 2.0_f64.sqrt();

    let made = sketch
        .pattern_along(
            &[Element::Point(held)],
            ChosenAxis::Trait(diagonal),
            Repeats { step, count: 2 },
            Repeats { step, count: 2 },
        )
        .expect("a direction to run along");

    let mut places: Vec<DVec2> = made.points.iter().map(|id| sketch.point(*id)).collect();
    places.sort_by(|one, other| one.y.total_cmp(&other.y).then(one.x.total_cmp(&other.x)));
    let wanted = [
        DVec2::new(4.0, 1.0),
        DVec2::new(6.0, 1.0),
        DVec2::new(5.0, 2.0),
    ];
    for (got, should) in places.iter().zip(wanted) {
        assert!(
            got.distance(should) <= TOLERANCE,
            "a copy stands at {got}, wanted {should}"
        );
    }
}

#[test]
fn a_grid_of_one_by_one_lays_nothing_at_all() {
    let (mut sketch, held) = a_trait_on_the_axis();
    let once = Repeats {
        step: 10.0,
        count: 1,
    };

    assert_eq!(
        sketch.pattern_along(&held, ChosenAxis::Sketch(SketchAxis::U), once, once),
        None
    );
    assert_eq!(sketch.live_segments().count(), 1);
}

#[test]
fn a_grid_stepping_by_nothing_would_pile_its_copies_up_and_is_refused() {
    let (mut sketch, held) = a_trait_on_the_axis();

    assert_eq!(
        sketch.pattern_along(
            &held,
            ChosenAxis::Sketch(SketchAxis::U),
            Repeats {
                step: 0.0,
                count: 3,
            },
            Repeats {
                step: 5.0,
                count: 2,
            },
        ),
        None
    );
    assert_eq!(sketch.live_segments().count(), 1);
}

#[test]
fn a_single_row_still_fills_the_direction_it_has() {
    let (mut sketch, held) = a_trait_on_the_axis();

    let made = sketch
        .pattern_along(
            &held,
            ChosenAxis::Sketch(SketchAxis::U),
            Repeats {
                step: 10.0,
                count: 4,
            },
            Repeats {
                step: 5.0,
                count: 1,
            },
        )
        .expect("a direction to run along");

    assert_eq!(made.segments.len(), 3);
}

#[test]
fn a_count_of_none_still_leaves_the_row_the_original_stands_in() {
    let (mut sketch, held) = a_trait_on_the_axis();

    let made = sketch
        .pattern_along(
            &held,
            ChosenAxis::Sketch(SketchAxis::U),
            Repeats {
                step: 10.0,
                count: 3,
            },
            Repeats {
                step: 5.0,
                count: 0,
            },
        )
        .expect("a direction to run along");

    assert_eq!(made.segments.len(), 2);
}

#[test]
fn a_direction_the_drawing_does_not_have_lays_nothing() {
    let (mut sketch, held) = a_trait_on_the_axis();
    let twice = Repeats {
        step: 10.0,
        count: 2,
    };

    assert_eq!(
        sketch.pattern_along(&held, ChosenAxis::Trait(SegmentId(99)), twice, twice),
        None
    );
}

#[test]
fn the_centre_a_pattern_turns_about_is_not_repeated_onto_itself() {
    let (mut sketch, centre, mut held) = a_trait_beside_a_centre();
    held.push(Element::Point(centre));

    let made = sketch
        .pattern_around(&held, centre, 90.0, 4)
        .expect("a centre to turn about");

    assert_eq!(
        made.points.len(),
        6,
        "a turn leaves its centre where it is, so only the two ends of the trait are laid"
    );
    let at = sketch.point(centre);
    assert!(
        made.points
            .iter()
            .all(|id| sketch.point(*id).distance(at) > TOLERANCE),
        "no copy stands on the centre it turned about"
    );
}
