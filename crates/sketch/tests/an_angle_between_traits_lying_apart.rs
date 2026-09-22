//! An angle between two traits that do not touch at all: a V whose point is
//! missing, or any two traits drawn apart that do not run the same way.
//!
//! Closes #407.
//! - two traits lying apart and not parallel, clicked one after the other, lay
//!   the angle between their directions —
//!   `two_traits_lying_apart_lay_the_angle_between_their_directions`
//! - two parallel traits are refused, and the tool says « Ces deux traits sont
//!   parallèles » — `two_parallel_traits_are_refused_as_parallel`, also when a
//!   rule holds them parallel and the solver has left them a hair off —
//!   `two_traits_parallel_to_within_what_a_solver_leaves_are_refused_too`
//! - the dimension is drawn between the two traits, around the middle of them,
//!   each end reaching one of the traits, never at the far point where their
//!   lines would cross —
//!   `an_angle_between_two_traits_far_from_where_they_would_cross_is_drawn_between_them`,
//!   and where an end of the arc falls past a trait, a thin line joins it back —
//!   `an_arc_that_ends_past_a_trait_is_joined_back_to_it`
//!   The same holds for traits drawn inwards, towards where their lines cross —
//!   `traits_drawn_towards_where_they_would_cross_have_their_angle_drawn_between_them`
//!   — and after a value typed has swung that far crossing a long way —
//!   `an_angle_between_traits_apart_stays_between_them_when_they_turn`. A
//!   crossing's own arc grows no such line: that is #406's drawing, unchanged —
//!   `a_crossing_s_arc_grows_no_line_past_a_short_trait`
//! - which angle is laid follows the side where the dimension is placed, as in
//!   #406 — `the_angle_between_traits_apart_follows_the_side_it_is_placed`
//! - typing a value turns the traits to it, and it holds —
//!   `a_typed_angle_turns_two_traits_lying_apart_to_it_and_holds`
//! - everything #406 delivers — the angle at a corner, at a crossing, in a T —
//!   is unchanged — no test: none is added here, since the tests of #406 in
//!   `an_angle_where_two_traits_cross.rs` stay in the gate; the one whose
//!   words changed is the parallel pair, now refused for being parallel

use cao_sketch::{
    AnnotationMetrics, DimensionMode, DimensionPick, DimensionPicks, DimensionTarget, SegmentId,
    Sketch, Toward, WorkPlane, measure_pick,
};
use glam::DVec2;

/// A point `distance` out from the origin, `degrees` round from the horizontal.
fn out(degrees: f64, distance: f64) -> DVec2 {
    DVec2::from_angle(degrees.to_radians()) * distance
}

/// A V whose point is missing: two traits heading out from the origin, twenty
/// and sixty degrees round, each starting ten units out so they never meet.
fn a_v_with_its_point_missing() -> (Sketch, [SegmentId; 2]) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let low_from = sketch.add_point(out(20.0, 10.0));
    let low_to = sketch.add_point(out(20.0, 30.0));
    let high_from = sketch.add_point(out(60.0, 10.0));
    let high_to = sketch.add_point(out(60.0, 30.0));
    let low = sketch.add_segment(low_from, low_to);
    let high = sketch.add_segment(high_from, high_to);
    (sketch, [low, high])
}

/// The smart dimension tool's two clicks: the first takes a trait's length, the
/// second — made while it is still being put down — what the two make together.
fn clicked(sketch: &Sketch, first: DVec2, second: DVec2) -> Option<DimensionTarget> {
    let (_, pick) = measure_pick(
        sketch,
        DimensionMode::Auto,
        DimensionPicks::default(),
        first,
        0.5,
    );
    let DimensionPick::Target(length) = pick else {
        panic!("the first click takes the trait it lands on, got {pick:?}");
    };
    sketch.refine(length, second, 0.5)
}

/// The same two clicks with the tool forced to measure an angle.
fn clicked_for_an_angle(sketch: &Sketch, first: DVec2, second: DVec2) -> DimensionPick {
    let (picks, _) = measure_pick(
        sketch,
        DimensionMode::Angle,
        DimensionPicks::default(),
        first,
        0.5,
    );
    measure_pick(sketch, DimensionMode::Angle, picks, second, 0.5).1
}

#[test]
fn two_traits_lying_apart_lay_the_angle_between_their_directions() {
    let (sketch, [low, high]) = a_v_with_its_point_missing();
    let (on_low, on_high) = (out(20.0, 20.0), out(60.0, 20.0));

    let refined = clicked(&sketch, on_low, on_high);
    let Some(target @ DimensionTarget::AngleBetween { first, second, .. }) = refined else {
        panic!("two traits that do not touch still make an angle, got {refined:?}");
    };
    assert_eq!(
        (first, second),
        (low, high),
        "between the two traits clicked"
    );
    let measured = sketch.opening(target).expect("they run different ways");
    assert!(
        (measured - 40.0).abs() < 1e-9,
        "the angle between their directions, forty degrees, got {measured}°",
    );

    assert!(
        matches!(
            clicked_for_an_angle(&sketch, on_low, on_high),
            DimensionPick::Target(DimensionTarget::AngleBetween { .. })
        ),
        "and the same with the tool set to angles",
    );
}

#[test]
fn two_parallel_traits_are_refused_as_parallel() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let low_from = sketch.add_point(DVec2::new(10.0, 10.0));
    let low_to = sketch.add_point(DVec2::new(30.0, 20.0));
    let high_from = sketch.add_point(DVec2::new(10.0, 30.0));
    let high_to = sketch.add_point(DVec2::new(30.0, 40.0));
    sketch.add_segment(low_from, low_to);
    sketch.add_segment(high_from, high_to);

    assert_eq!(
        clicked_for_an_angle(&sketch, DVec2::new(20.0, 15.0), DVec2::new(20.0, 35.0)),
        DimensionPick::TraitsAreParallel,
        "two traits running the same way make no angle, and that is the reason given",
    );
}

/// Two traits running almost the same way, a hundred units and more out along
/// lines that only cross back at the origin — the drawing a user sees has no
/// crossing anywhere near it.
fn two_traits_nearly_parallel() -> (Sketch, [SegmentId; 2]) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let low_from = sketch.add_point(out(0.0, 100.0));
    let low_to = sketch.add_point(out(0.0, 140.0));
    let high_from = sketch.add_point(out(8.0, 100.0));
    let high_to = sketch.add_point(out(8.0, 140.0));
    let low = sketch.add_segment(low_from, low_to);
    let high = sketch.add_segment(high_from, high_to);
    (sketch, [low, high])
}

/// Annotations at a zoom where a pixel is a twentieth of a unit, so that what
/// stands off a line by a few pixels stands off it by less than a unit.
fn zoomed_in() -> AnnotationMetrics {
    AnnotationMetrics {
        offset_pixels: 20.0,
        arrow_pixels: 8.0,
        arc_pixels: 30.0,
        pixel: 0.05,
        nudge: DVec2::ZERO,
    }
}

#[test]
fn an_angle_between_two_traits_far_from_where_they_would_cross_is_drawn_between_them() {
    let (mut sketch, [low, high]) = two_traits_nearly_parallel();
    let opening = DimensionTarget::AngleBetween {
        first: low,
        first_toward: Toward::End,
        second: high,
        second_toward: Toward::End,
    };
    sketch.set_dimension(opening, 8.0, false);

    let drawn = sketch
        .place(opening, zoomed_in())
        .expect("the angle is drawn");

    let nearest = drawn
        .shape
        .iter()
        .flat_map(|(from, to)| [*from, *to])
        .map(|place| place.length())
        .fold(f64::MAX, f64::min);
    assert!(
        nearest > 90.0,
        "nothing is drawn out where the two lines would cross, at the origin; the \
         nearest stroke is {nearest} from it",
    );
    let text = drawn.text_at.length();
    assert!(
        (100.0..141.0).contains(&text),
        "the value sits alongside the two traits, between 100 and 140 out, got {text}",
    );
    for (segment, name) in [(low, "lower"), (high, "upper")] {
        let (from, to) = sketch.endpoints(segment);
        let reached = drawn.shape.iter().flat_map(|(a, b)| [*a, *b]).any(|place| {
            let along = (place - from).dot(to - from) / (to - from).length_squared();
            let off = (to - from).normalize().perp_dot(place - from).abs();
            off < 1e-6 && (0.01..0.99).contains(&along)
        });
        assert!(
            reached,
            "one end of the arc lands on the {name} trait itself, not merely a line \
             joined back to one of its ends",
        );
    }
}

#[test]
fn an_arc_that_ends_past_a_trait_is_joined_back_to_it() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let low_from = sketch.add_point(out(0.0, 100.0));
    let low_to = sketch.add_point(out(0.0, 140.0));
    let high_from = sketch.add_point(out(8.0, 150.0));
    let high_to = sketch.add_point(out(8.0, 190.0));
    let low = sketch.add_segment(low_from, low_to);
    let high = sketch.add_segment(high_from, high_to);
    let opening = DimensionTarget::AngleBetween {
        first: low,
        first_toward: Toward::End,
        second: high,
        second_toward: Toward::End,
    };
    sketch.set_dimension(opening, 8.0, false);

    let drawn = sketch
        .place(opening, zoomed_in())
        .expect("the angle is drawn");

    for (end, name) in [
        (out(0.0, 140.0), "the lower trait's far end"),
        (out(8.0, 150.0), "the upper trait's near end"),
    ] {
        assert!(
            drawn
                .shape
                .iter()
                .any(|(from, to)| from.distance(end) < 1e-9 || to.distance(end) < 1e-9),
            "the two traits never run side by side, so the arc falls past each; a \
             line joins it back to {name}",
        );
    }
}

#[test]
fn the_angle_between_traits_apart_follows_the_side_it_is_placed() {
    let (sketch, [low, high]) = a_v_with_its_point_missing();
    let asked = DimensionTarget::AngleBetween {
        first: low,
        first_toward: Toward::End,
        second: high,
        second_toward: Toward::End,
    };

    for (placed, wanted, side) in [
        (out(40.0, 20.0), 40.0, "inside the V"),
        (
            out(220.0, 20.0),
            40.0,
            "across from it, past where the lines cross",
        ),
        (
            out(130.0, 20.0),
            140.0,
            "beside it, where one arm turns back",
        ),
    ] {
        let measured = sketch
            .opening(sketch.oriented(asked, placed))
            .expect("they run different ways");
        assert!(
            (measured - wanted).abs() < 1e-9,
            "put down {side}, the angle opening that way is {wanted}°, got {measured}°",
        );
    }
}

#[test]
fn a_typed_angle_turns_two_traits_lying_apart_to_it_and_holds() {
    let (mut sketch, [low, high]) = a_v_with_its_point_missing();
    let opening = DimensionTarget::AngleBetween {
        first: low,
        first_toward: Toward::End,
        second: high,
        second_toward: Toward::End,
    };

    for wanted in [25.0, 70.0, 55.0] {
        sketch.set_dimension(opening, wanted, false);
        sketch.resolve(1.0);

        let held = sketch.opening(opening).expect("they run different ways");
        assert!(
            (held - wanted).abs() < 1e-4,
            "the two traits turn to the {wanted}° typed, got {held}°",
        );
    }
}

#[test]
fn two_traits_parallel_to_within_what_a_solver_leaves_are_refused_too() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let low_from = sketch.add_point(DVec2::new(10.0, 10.0));
    let low_to = sketch.add_point(DVec2::new(30.0, 10.0));
    let high_from = sketch.add_point(DVec2::new(10.0, 30.0));
    let high_to = sketch.add_point(DVec2::new(10.0, 30.0) + DVec2::from_angle(1e-6) * 20.0);
    sketch.add_segment(low_from, low_to);
    sketch.add_segment(high_from, high_to);

    assert_eq!(
        clicked_for_an_angle(&sketch, DVec2::new(20.0, 10.0), DVec2::new(20.0, 30.0)),
        DimensionPick::TraitsAreParallel,
        "a millionth of a radian is what a rule holding two traits parallel leaves \
         once solved; an angle laid there would fight that rule",
    );
}

#[test]
fn traits_drawn_towards_where_they_would_cross_have_their_angle_drawn_between_them() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let low_from = sketch.add_point(out(20.0, 30.0));
    let low_to = sketch.add_point(out(20.0, 10.0));
    let high_from = sketch.add_point(out(60.0, 30.0));
    let high_to = sketch.add_point(out(60.0, 10.0));
    sketch.add_segment(low_from, low_to);
    sketch.add_segment(high_from, high_to);

    let picked =
        clicked(&sketch, out(20.0, 20.0), out(60.0, 20.0)).expect("the two traits make an angle");
    let drawn = sketch
        .place(picked, zoomed_in())
        .expect("the angle is drawn");

    let heading = drawn.text_at.to_angle().to_degrees();
    assert!(
        (20.0..60.0).contains(&heading) && (10.0..31.0).contains(&drawn.text_at.length()),
        "the value sits inside the V, between its two traits — not mirrored past \
         where their lines cross — got {} at {heading}°",
        drawn.text_at.length(),
    );
}

#[test]
fn an_angle_between_traits_apart_stays_between_them_when_they_turn() {
    let (mut sketch, [low, high]) = two_traits_nearly_parallel();
    let opening = DimensionTarget::AngleBetween {
        first: low,
        first_toward: Toward::End,
        second: high,
        second_toward: Toward::End,
    };
    sketch.set_dimension(opening, 8.0, false);
    let put_down = sketch.place(opening, zoomed_in()).expect("drawn").offset;
    sketch.offset_dimension(opening, put_down);

    sketch.set_dimension(opening, 12.0, false);
    sketch.resolve(1.0);
    let drawn = sketch.place(opening, zoomed_in()).expect("still drawn");

    for (segment, name) in [(low, "lower"), (high, "upper")] {
        let (from, to) = sketch.endpoints(segment);
        let middle = (from + to) * 0.5;
        assert!(
            drawn.text_at.distance(middle) < 30.0,
            "turned to twelve degrees, the value still sits alongside the {name} \
             trait, not wherever their far crossing swung off to: {} from its middle",
            drawn.text_at.distance(middle),
        );
    }
}

#[test]
fn a_crossing_s_arc_grows_no_line_past_a_short_trait() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let short_from = sketch.add_point(DVec2::new(19.0, 19.0));
    let short_to = sketch.add_point(DVec2::new(21.0, 21.0));
    let long_from = sketch.add_point(DVec2::new(0.0, 40.0));
    let long_to = sketch.add_point(DVec2::new(40.0, 0.0));
    let short = sketch.add_segment(short_from, short_to);
    let long = sketch.add_segment(long_from, long_to);
    let opening = DimensionTarget::AngleBetween {
        first: short,
        first_toward: Toward::End,
        second: long,
        second_toward: Toward::End,
    };
    sketch.set_dimension(opening, 90.0, false);
    let far_out = AnnotationMetrics {
        pixel: 1.0,
        ..zoomed_in()
    };

    let drawn = sketch.place(opening, far_out).expect("the angle is drawn");

    for end in [DVec2::new(19.0, 19.0), DVec2::new(21.0, 21.0)] {
        assert!(
            !drawn
                .shape
                .iter()
                .any(|(from, to)| from.distance(end) < 1e-9 || to.distance(end) < 1e-9),
            "an X's arc is drawn as it was before traits apart were measured: no line \
             runs out from the short trait's end at {end}",
        );
    }
}
