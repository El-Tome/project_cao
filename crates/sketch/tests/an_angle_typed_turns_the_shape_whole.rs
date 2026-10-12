//! Closes #532.
//! - against an axis, the shape turns whole about its point nearest the origin
//!   (#455), sizes typed or not —
//!   `an_angle_against_an_axis_turns_a_free_rectangle_whole_about_its_corner_nearest_the_origin`,
//!   and a trait alone keeps its length —
//!   `a_trait_alone_keeps_its_length_whatever_angle_is_typed_against_an_axis`;
//!   about the origin or a fixed corner when one holds it —
//!   `a_rectangle_drawn_from_the_origin_turns_whole_about_the_origin`,
//!   `a_rectangle_held_by_a_fixed_corner_turns_whole_about_it`
//! - between two traits, the trait clicked first stays and the other turns,
//!   whole with whatever hangs on it (#451) — at a corner,
//!   `an_angle_at_a_corner_turns_the_rectangle_whole_about_the_corner_and_leaves_the_first_trait`;
//!   apart, `an_angle_between_traits_apart_turns_the_rectangle_whole_and_leaves_the_first_trait`;
//!   in a T, `the_stem_of_a_t_turns_about_its_foot_with_what_hangs_off_it`;
//!   crossing, `the_second_of_two_crossing_traits_turns_about_where_they_cross`
//! - the parallel and perpendicular rules between two traits turn the second
//!   shape whole the same way —
//!   `a_parallel_or_a_perpendicular_to_a_trait_apart_turns_the_rectangle_whole`,
//!   `a_parallel_or_a_perpendicular_at_a_corner_turns_the_rectangle_whole_about_it`,
//!   and the other way round the trait turns alone —
//!   `a_trait_apart_clicked_second_turns_alone_and_leaves_the_rectangle`
//! - never an explosion: either the shape turns whole to the value, or the
//!   value is refused — the angle tests sweep 0° to 180° by five, as far as
//!   the angle takes them: 0° is refused at a corner, and 0° and 180° between
//!   traits apart. Every size is asked to hold, 0° and 180° against an axis
//!   included, where the fold blew a free rectangle up. A shape held at two
//!   places, which cannot turn whole, gives as #445 ranks it, stays no larger
//!   than its holds and never comes out inside out —
//!   `a_rectangle_held_at_two_corners_gives_as_the_order_says_and_never_blows_up`
//!
//! What the issue says must not break, held by tests that were already in the
//! gate and pass unchanged:
//! - 0 refused between two traits sharing a corner — no test: the refusal is
//!   `apply_dimension` in `crates/part/src/dimensioning.rs`, untouched here
//! - 0 and 180 refused between two traits apart, and their message — no test:
//!   `DimensionTarget::takes` is untouched, and
//!   `a_value_that_would_lay_the_traits_parallel_is_refused` stays in the gate
//! - a value refused is still a step that changes nothing, a rule is not — no
//!   test: `apply_dimension` puts the drawing back exactly as before, and
//!   `a_rule_contradicting_a_value_is_refused_and_leaves_no_step` stays
//! - the whole turn of the rules to an axis (#482) — no test: it now goes
//!   through the same whole turn, and `a_trait_square_to_an_axis.rs` passes
//!   unchanged
//! - the tests of #451 and #455, and
//!   `an_angle_typed_turns_a_square_held_by_one_fixed_point` — no test: they
//!   pass unchanged
//!
//! Beside the criteria, what a review of this change found it had to hold: the
//! first trait staying when the turned shape still has to slide
//! (`the_first_trait_stays_when_the_turned_rectangle_still_has_to_slide`), a
//! shape hanging on a trait whose direction is given
//! (`an_angle_against_an_axis_turns_a_side_about_its_foot_on_a_base_held_level`,
//! `an_angle_from_a_free_trait_turns_a_side_about_its_foot_on_a_base_lying_level`),
//! a shape never carrying what it stands on
//! (`a_rectangle_standing_on_a_plate_turns_about_its_foot_and_leaves_the_plate`),
//! and a point on a circle turning with it
//! (`a_trait_ending_on_a_circle_turns_with_the_rectangle_the_circle_is_centred_on`).
//!
//! Each rectangle leans 17°, as the one the issue was measured on.

use cao_sketch::{
    Along, AxisToward, Constraint, DimensionTarget, Element, LaidFrom, LengthOutcome, PointId,
    SegmentId, Sketch, SketchAxis, Toward, WorkPlane,
};
use glam::DVec2;

const SCALE: f64 = 1.0;
/// What the solver promises of a value it had to reach by moving points: a
/// hundred-thousandth of the drawing's size, a hundred units across here.
const SETTLED: f64 = 1e-3;

/// Every value from 0° to 180° by five, the three a fold sat at among them.
fn every_value() -> Vec<f64> {
    (0..=36).map(|step| f64::from(step) * 5.0).collect()
}

/// The same but for 0° and 180°, which lay two traits parallel: an angle
/// between two traits that share no end refuses them (#407).
fn every_opening() -> Vec<f64> {
    (1..36).map(|step| f64::from(step) * 5.0).collect()
}

/// How the tool lays a rectangle: four traits, three right angles, nothing
/// saying which way up. This one is 100 by 50, leans 17° and starts at
/// `corner`; `sizes` types its width and height as well.
fn leaning_rectangle(corner: DVec2, sizes: bool) -> (Sketch, [PointId; 4], [SegmentId; 4]) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let along = DVec2::from_angle(17.0_f64.to_radians());
    let across = along.perp();
    let corners = [
        corner,
        corner + along * 100.0,
        corner + along * 100.0 + across * 50.0,
        corner + across * 50.0,
    ];
    let points = corners.map(|place| sketch.add_point(place));
    let sides: [SegmentId; 4] =
        std::array::from_fn(|rank| sketch.add_segment(points[rank], points[(rank + 1) % 4]));
    for rank in 0..3 {
        sketch.add_constraint(Constraint::Perpendicular {
            first: sides[rank],
            second: sides[rank + 1],
        });
    }
    if sizes {
        sketch.set_dimension(DimensionTarget::Length(sides[0]), 100.0, false);
        sketch.set_dimension(DimensionTarget::Length(sides[1]), 50.0, false);
    }
    (sketch, points, sides)
}

fn direction(sketch: &Sketch, side: SegmentId) -> DVec2 {
    let drawn = sketch.segments()[side.0];
    (sketch.point(drawn.end) - sketch.point(drawn.start)).normalize()
}

/// Types `value` into the dimension on `target` and lets the drawing land it.
fn typed(sketch: &mut Sketch, target: DimensionTarget, value: f64) -> LengthOutcome {
    sketch.set_dimension(target, value, false);
    sketch.land_value(target, SCALE)
}

/// The rectangle turned whole: its sides keep their lengths and their right
/// angles, and `stays` is where it was.
fn assert_turned_whole(
    sketch: &Sketch,
    sides: [SegmentId; 4],
    stays: PointId,
    at: DVec2,
    case: &str,
) {
    for (rank, side) in sides.iter().enumerate() {
        let wanted = if rank % 2 == 0 { 100.0 } else { 50.0 };
        let length = sketch.segment_length(*side);
        assert!(
            (length - wanted).abs() < SETTLED,
            "{case}: side {rank} went from {wanted} to {length}"
        );
        let next = sides[(rank + 1) % 4];
        let corner = direction(sketch, *side).dot(direction(sketch, next));
        assert!(
            corner.abs() < SETTLED,
            "{case}: corner {rank} is no longer square"
        );
    }
    assert!(
        sketch.point(stays).distance(at) < SETTLED,
        "{case}: the corner it turns about moved to {}",
        sketch.point(stays)
    );
}

fn assert_reads(sketch: &Sketch, target: DimensionTarget, value: f64, case: &str) {
    let read = sketch.opening(target).expect("the angle is there");
    assert!(
        (read - value).abs() < SETTLED,
        "{case}: the angle reads {read}°"
    );
}

fn against_the_horizontal(segment: SegmentId) -> DimensionTarget {
    DimensionTarget::AxisAngle {
        segment,
        segment_toward: Toward::End,
        axis: SketchAxis::U,
        axis_toward: AxisToward::Positive,
    }
}

#[test]
fn an_angle_against_an_axis_turns_a_free_rectangle_whole_about_its_corner_nearest_the_origin() {
    let corner = DVec2::new(40.0, 30.0);
    for sizes in [true, false] {
        for value in every_value() {
            let case = format!("{value}°, sizes typed: {sizes}");
            let (mut sketch, points, sides) = leaning_rectangle(corner, sizes);
            let target = against_the_horizontal(sides[0]);

            assert_eq!(
                typed(&mut sketch, target, value),
                LengthOutcome::Exact,
                "{case}"
            );

            assert_reads(&sketch, target, value, &case);
            assert_turned_whole(&sketch, sides, points[0], corner, &case);
        }
    }
}

#[test]
fn a_trait_alone_keeps_its_length_whatever_angle_is_typed_against_an_axis() {
    let near = DVec2::new(30.0, 20.0);
    for value in every_value() {
        let mut sketch = Sketch::new(WorkPlane::XY);
        let start = sketch.add_point(near);
        let end = sketch.add_point(near + DVec2::from_angle(0.4) * 89.4);
        let alone = sketch.add_segment(start, end);
        let target = against_the_horizontal(alone);

        assert_eq!(
            typed(&mut sketch, target, value),
            LengthOutcome::Exact,
            "{value}°"
        );

        assert_reads(&sketch, target, value, &format!("{value}°"));
        let length = sketch.segment_length(alone);
        assert!(
            (length - 89.4).abs() < SETTLED,
            "{value}°: the trait went from 89.4 long to {length}"
        );
        assert!(
            sketch.point(start).distance(near) < SETTLED,
            "{value}°: the end nearest the origin moved to {}",
            sketch.point(start)
        );
    }
}

fn assert_still(sketch: &Sketch, points: &[PointId], places: &[DVec2], case: &str) {
    for (point, place) in points.iter().zip(places) {
        assert!(
            sketch.point(*point).distance(*place) < SETTLED,
            "{case}: {point:?} moved from {place} to {}",
            sketch.point(*point)
        );
    }
}

#[test]
fn an_angle_at_a_corner_turns_the_rectangle_whole_about_the_corner_and_leaves_the_first_trait() {
    let corner = DVec2::new(40.0, 30.0);
    let far = DVec2::new(-40.0, 30.0);
    for sizes in [true, false] {
        for value in every_value().into_iter().filter(|value| *value > 0.0) {
            let case = format!("{value}°, sizes typed: {sizes}");
            let (mut sketch, points, sides) = leaning_rectangle(corner, sizes);
            let away = sketch.add_point(far);
            let level = sketch.add_segment(away, points[0]);
            let target = DimensionTarget::Angle {
                first: level,
                first_along: Along::Prolongation,
                second: sides[0],
                second_along: Along::Trait,
            };

            assert_eq!(
                typed(&mut sketch, target, value),
                LengthOutcome::Exact,
                "{case}"
            );

            assert_reads(&sketch, target, value, &case);
            assert_still(&sketch, &[away, points[0]], &[far, corner], &case);
            assert_turned_whole(&sketch, sides, points[0], corner, &case);
        }
    }
}

/// A level trait drawn apart from the rectangle, below it and to its left.
fn a_level_trait_apart(sketch: &mut Sketch) -> (SegmentId, [PointId; 2], [DVec2; 2]) {
    let places = [DVec2::new(-100.0, -40.0), DVec2::new(-20.0, -40.0)];
    let ends = places.map(|place| sketch.add_point(place));
    (sketch.add_segment(ends[0], ends[1]), ends, places)
}

#[test]
fn an_angle_between_traits_apart_turns_the_rectangle_whole_and_leaves_the_first_trait() {
    let corner = DVec2::new(40.0, 30.0);
    for sizes in [true, false] {
        for value in every_opening() {
            let case = format!("{value}°, sizes typed: {sizes}");
            let (mut sketch, points, sides) = leaning_rectangle(corner, sizes);
            let (level, ends, places) = a_level_trait_apart(&mut sketch);
            let target = DimensionTarget::AngleBetween {
                first: level,
                first_toward: Toward::End,
                second: sides[0],
                second_toward: Toward::End,
            };

            assert_eq!(
                typed(&mut sketch, target, value),
                LengthOutcome::Exact,
                "{case}"
            );

            assert_reads(&sketch, target, value, &case);
            assert_still(&sketch, &ends, &places, &case);
            assert_turned_whole(&sketch, sides, points[0], corner, &case);
        }
    }
}

fn parallel_or_perpendicular(first: SegmentId, second: SegmentId, square: bool) -> Constraint {
    match square {
        true => Constraint::Perpendicular { first, second },
        false => Constraint::Parallel { first, second },
    }
}

#[test]
fn a_parallel_or_a_perpendicular_to_a_trait_apart_turns_the_rectangle_whole() {
    let corner = DVec2::new(40.0, 30.0);
    for sizes in [true, false] {
        for square in [false, true] {
            let case = format!("square: {square}, sizes typed: {sizes}");
            let (mut sketch, points, sides) = leaning_rectangle(corner, sizes);
            let (level, ends, places) = a_level_trait_apart(&mut sketch);

            let rule = parallel_or_perpendicular(level, sides[0], square);
            assert_eq!(sketch.lay_rule(rule, SCALE), LengthOutcome::Exact, "{case}");

            let across = direction(&sketch, level).perp_dot(direction(&sketch, sides[0]));
            let off = if square {
                1.0 - across.abs()
            } else {
                across.abs()
            };
            assert!(off < SETTLED, "{case}: the rule does not hold");
            assert_still(&sketch, &ends, &places, &case);
            assert_turned_whole(&sketch, sides, points[0], corner, &case);
        }
    }
}

#[test]
fn a_parallel_or_a_perpendicular_at_a_corner_turns_the_rectangle_whole_about_it() {
    let corner = DVec2::new(40.0, 30.0);
    let far = DVec2::new(-40.0, 30.0);
    for sizes in [true, false] {
        for square in [false, true] {
            let case = format!("square: {square}, sizes typed: {sizes}");
            let (mut sketch, points, sides) = leaning_rectangle(corner, sizes);
            let away = sketch.add_point(far);
            let level = sketch.add_segment(away, points[0]);

            let rule = parallel_or_perpendicular(level, sides[0], square);
            assert_eq!(sketch.lay_rule(rule, SCALE), LengthOutcome::Exact, "{case}");

            assert_still(&sketch, &[away, points[0]], &[far, corner], &case);
            assert_turned_whole(&sketch, sides, points[0], corner, &case);
        }
    }
}

#[test]
fn a_trait_apart_clicked_second_turns_alone_and_leaves_the_rectangle() {
    let corner = DVec2::new(40.0, 30.0);
    for sizes in [true, false] {
        for square in [false, true] {
            let case = format!("square: {square}, sizes typed: {sizes}");
            let (mut sketch, points, sides) = leaning_rectangle(corner, sizes);
            let places: Vec<DVec2> = points.iter().map(|point| sketch.point(*point)).collect();
            let (level, ends, _) = a_level_trait_apart(&mut sketch);

            let rule = parallel_or_perpendicular(sides[0], level, square);
            assert_eq!(sketch.lay_rule(rule, SCALE), LengthOutcome::Exact, "{case}");

            assert_still(&sketch, &points, &places, &case);
            let length = sketch.segment_length(level);
            assert!(
                (length - 80.0).abs() < SETTLED,
                "{case}: the trait went from 80 long to {length}"
            );
            assert!(
                sketch.point(ends[1]).distance(DVec2::new(-20.0, -40.0)) < SETTLED,
                "{case}: the end nearest the origin moved to {}",
                sketch.point(ends[1])
            );
        }
    }
}

#[test]
fn the_stem_of_a_t_turns_about_its_foot_with_what_hangs_off_it() {
    let foot_at = DVec2::new(20.0, 20.0);
    for value in every_opening() {
        let case = format!("{value}°");
        let mut sketch = Sketch::new(WorkPlane::XY);
        let bar_places = [DVec2::new(-30.0, 20.0), DVec2::new(70.0, 20.0)];
        let bar_ends = bar_places.map(|place| sketch.add_point(place));
        let bar = sketch.add_segment(bar_ends[0], bar_ends[1]);
        let foot = sketch.add_point(foot_at);
        let top = sketch.add_point(foot_at + DVec2::from_angle(1.0) * 40.0);
        let tip = sketch.add_point(sketch.point(top) + DVec2::new(25.0, 0.0));
        let stem = sketch.add_segment(foot, top);
        let hanging = sketch.add_segment(top, tip);
        sketch.add_constraint(Constraint::OnSegment {
            point: foot,
            segment: bar,
            from: LaidFrom::Nowhere,
        });
        let target = DimensionTarget::AngleBetween {
            first: bar,
            first_toward: Toward::End,
            second: stem,
            second_toward: Toward::End,
        };

        assert_eq!(
            typed(&mut sketch, target, value),
            LengthOutcome::Exact,
            "{case}"
        );

        assert_reads(&sketch, target, value, &case);
        assert_still(&sketch, &bar_ends, &bar_places, &case);
        assert_still(&sketch, &[foot], &[foot_at], &case);
        for (segment, wanted) in [(stem, 40.0), (hanging, 25.0)] {
            let length = sketch.segment_length(segment);
            assert!(
                (length - wanted).abs() < SETTLED,
                "{case}: {segment:?} went from {wanted} long to {length}"
            );
        }
        let bend = direction(&sketch, stem).angle_to(direction(&sketch, hanging));
        assert!(
            (bend + 1.0).abs() < SETTLED,
            "{case}: what hangs off the stem bent to {bend} rad from it"
        );
    }
}

#[test]
fn the_second_of_two_crossing_traits_turns_about_where_they_cross() {
    for value in every_opening() {
        let case = format!("{value}°");
        let mut sketch = Sketch::new(WorkPlane::XY);
        let places = [
            DVec2::new(0.0, 10.0),
            DVec2::new(60.0, 50.0),
            DVec2::new(10.0, 60.0),
            DVec2::new(50.0, 0.0),
        ];
        let points = places.map(|place| sketch.add_point(place));
        let rising = sketch.add_segment(points[0], points[1]);
        let falling = sketch.add_segment(points[2], points[3]);
        let crossing = sketch
            .where_lines_cross(rising, falling)
            .expect("the two traits cross");
        let length = sketch.segment_length(falling);
        let target = DimensionTarget::AngleBetween {
            first: rising,
            first_toward: Toward::End,
            second: falling,
            second_toward: Toward::End,
        };

        assert_eq!(
            typed(&mut sketch, target, value),
            LengthOutcome::Exact,
            "{case}"
        );

        assert_reads(&sketch, target, value, &case);
        assert_still(&sketch, &points[..2], &places[..2], &case);
        let now = sketch
            .where_lines_cross(rising, falling)
            .expect("the two traits still cross");
        assert!(
            now.distance(crossing) < SETTLED,
            "{case}: they crossed at {crossing}, now at {now}"
        );
        let turned = sketch.segment_length(falling);
        assert!(
            (turned - length).abs() < SETTLED,
            "{case}: the trait turned went from {length} long to {turned}"
        );
    }
}

#[test]
fn a_rectangle_drawn_from_the_origin_turns_whole_about_the_origin() {
    for sizes in [true, false] {
        for value in every_value() {
            let case = format!("{value}°, sizes typed: {sizes}");
            let (mut sketch, points, sides) = leaning_rectangle(DVec2::ZERO, sizes);
            sketch.merge_points(Sketch::ORIGIN, points[0]);
            let target = against_the_horizontal(sides[0]);

            assert_eq!(
                typed(&mut sketch, target, value),
                LengthOutcome::Exact,
                "{case}"
            );

            assert_reads(&sketch, target, value, &case);
            assert_turned_whole(&sketch, sides, Sketch::ORIGIN, DVec2::ZERO, &case);
        }
    }
}

#[test]
fn a_rectangle_held_by_a_fixed_corner_turns_whole_about_it() {
    let corner = DVec2::new(40.0, 30.0);
    for sizes in [true, false] {
        for value in every_value() {
            let case = format!("{value}°, sizes typed: {sizes}");
            let (mut sketch, points, sides) = leaning_rectangle(corner, sizes);
            let fixed = sketch.point(points[2]);
            sketch.add_constraint(Constraint::Fixed {
                element: Element::Point(points[2]),
            });
            let target = against_the_horizontal(sides[0]);

            assert_eq!(
                typed(&mut sketch, target, value),
                LengthOutcome::Exact,
                "{case}"
            );

            assert_reads(&sketch, target, value, &case);
            assert_turned_whole(&sketch, sides, points[2], fixed, &case);
        }
    }
}

#[test]
fn a_rectangle_held_at_two_corners_gives_as_the_order_says_and_never_blows_up() {
    for value in every_opening() {
        let case = format!("{value}°");
        let (mut sketch, points, sides) = leaning_rectangle(DVec2::ZERO, false);
        sketch.merge_points(Sketch::ORIGIN, points[0]);
        let fixed = sketch.point(points[2]);
        sketch.add_constraint(Constraint::Fixed {
            element: Element::Point(points[2]),
        });
        let target = against_the_horizontal(sides[0]);

        assert_eq!(
            typed(&mut sketch, target, value),
            LengthOutcome::Exact,
            "{case}"
        );

        assert_reads(&sketch, target, value, &case);
        let diagonal = fixed.length();
        for (rank, side) in sides.iter().enumerate() {
            let length = sketch.segment_length(*side);
            assert!(
                length < diagonal + SETTLED,
                "{case}: side {rank} grew to {length}, past the {diagonal} between the holds"
            );
            let next = sides[(rank + 1) % 4];
            let corner = direction(&sketch, *side).dot(direction(&sketch, next));
            assert!(
                corner.abs() < SETTLED,
                "{case}: corner {rank} is no longer square"
            );
        }
        let way_round = direction(&sketch, sides[0]).perp_dot(direction(&sketch, sides[1]));
        assert!(
            way_round > 0.0,
            "{case}: the rectangle came out inside out between its holds"
        );
    }
}

#[test]
fn the_first_trait_stays_when_the_turned_rectangle_still_has_to_slide() {
    let corner = DVec2::new(40.0, 30.0);
    for value in [10.0, 40.0, 75.0] {
        let case = format!("{value}°");
        let (mut sketch, points, sides) = leaning_rectangle(corner, true);
        let height = DimensionTarget::Projected {
            from: Sketch::ORIGIN,
            to: points[3],
            axis: SketchAxis::V,
        };
        let held_at = sketch.point(points[3]).y;
        sketch.set_dimension(height, held_at, false);
        let (level, ends, places) = a_level_trait_apart(&mut sketch);
        let target = DimensionTarget::AngleBetween {
            first: level,
            first_toward: Toward::End,
            second: sides[0],
            second_toward: Toward::End,
        };

        assert_eq!(
            typed(&mut sketch, target, value),
            LengthOutcome::Exact,
            "{case}"
        );

        assert_reads(&sketch, target, value, &case);
        assert_still(&sketch, &ends, &places, &case);
        let now = sketch.point(points[3]).y;
        assert!(
            (now - held_at).abs() < SETTLED,
            "{case}: the corner whose height is typed went from {held_at} to {now}"
        );
        let stays = sketch.point(points[0]);
        assert_turned_whole(&sketch, sides, points[0], stays, &case);
    }
}

/// A profile from the origin: a base 100 long, then a side 50 long rising at
/// 30° from its far end. `level` makes the base « Horizontale » by a rule.
fn a_profile(level: bool) -> (Sketch, PointId, PointId, SegmentId, SegmentId) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let foot = sketch.add_point(DVec2::new(100.0, 0.0));
    let top =
        sketch.add_point(DVec2::new(100.0, 0.0) + DVec2::from_angle(30.0_f64.to_radians()) * 50.0);
    let base = sketch.add_segment(Sketch::ORIGIN, foot);
    let side = sketch.add_segment(foot, top);
    if level {
        sketch.add_constraint(Constraint::AxisParallel {
            segment: base,
            axis: SketchAxis::U,
        });
    }
    (sketch, foot, top, base, side)
}

fn assert_turned_about_its_foot(
    sketch: &Sketch,
    foot: PointId,
    base: SegmentId,
    side: SegmentId,
    case: &str,
) {
    assert!(
        sketch.point(foot).distance(DVec2::new(100.0, 0.0)) < SETTLED,
        "{case}: the foot of the side moved to {}",
        sketch.point(foot)
    );
    for (segment, wanted) in [(base, 100.0), (side, 50.0)] {
        let length = sketch.segment_length(segment);
        assert!(
            (length - wanted).abs() < SETTLED,
            "{case}: {segment:?} went from {wanted} long to {length}"
        );
    }
}

#[test]
fn an_angle_against_an_axis_turns_a_side_about_its_foot_on_a_base_held_level() {
    for value in every_value() {
        let case = format!("{value}°");
        let (mut sketch, foot, _, base, side) = a_profile(true);
        let target = against_the_horizontal(side);

        assert_eq!(
            typed(&mut sketch, target, value),
            LengthOutcome::Exact,
            "{case}"
        );

        assert_reads(&sketch, target, value, &case);
        assert_turned_about_its_foot(&sketch, foot, base, side, &case);
    }
}

#[test]
fn an_angle_from_a_free_trait_turns_a_side_about_its_foot_on_a_base_lying_level() {
    for value in [20.0, 70.0, 110.0] {
        let case = format!("{value}°");
        let (mut sketch, foot, _, base, side) = a_profile(false);
        let (level, ends, places) = a_level_trait_apart(&mut sketch);
        let target = DimensionTarget::AngleBetween {
            first: level,
            first_toward: Toward::End,
            second: side,
            second_toward: Toward::End,
        };

        assert_eq!(
            typed(&mut sketch, target, value),
            LengthOutcome::Exact,
            "{case}"
        );

        assert_reads(&sketch, target, value, &case);
        assert_still(&sketch, &ends, &places, &case);
        assert_turned_about_its_foot(&sketch, foot, base, side, &case);
    }
}

#[test]
fn a_rectangle_standing_on_a_plate_turns_about_its_foot_and_leaves_the_plate() {
    for square in [false, true] {
        let case = format!("square: {square}");
        let mut sketch = Sketch::new(WorkPlane::XY);
        let along = DVec2::from_angle(10.0_f64.to_radians());
        let plate_places = [
            along * 120.0,
            along * 120.0 + along.perp() * 80.0,
            along.perp() * 80.0,
        ];
        let plate_points = plate_places.map(|place| sketch.add_point(place));
        let corners = [
            Sketch::ORIGIN,
            plate_points[0],
            plate_points[1],
            plate_points[2],
        ];
        let plate: [SegmentId; 4] =
            std::array::from_fn(|rank| sketch.add_segment(corners[rank], corners[(rank + 1) % 4]));
        let foot_at = plate_places[2] + (plate_places[1] - plate_places[2]) * 0.4;
        let small_along = DVec2::from_angle(35.0_f64.to_radians());
        let small_places = [
            foot_at,
            foot_at + small_along * 30.0,
            foot_at + small_along * 30.0 + small_along.perp() * 20.0,
            foot_at + small_along.perp() * 20.0,
        ];
        let small_points = small_places.map(|place| sketch.add_point(place));
        let small: [SegmentId; 4] = std::array::from_fn(|rank| {
            sketch.add_segment(small_points[rank], small_points[(rank + 1) % 4])
        });
        for rank in 0..3 {
            sketch.add_constraint(Constraint::Perpendicular {
                first: small[rank],
                second: small[rank + 1],
            });
        }
        sketch.add_constraint(Constraint::OnSegment {
            point: small_points[0],
            segment: plate[2],
            from: LaidFrom::Nowhere,
        });
        let (level, ends, places) = a_level_trait_apart(&mut sketch);

        let rule = parallel_or_perpendicular(level, small[0], square);
        assert_eq!(sketch.lay_rule(rule, SCALE), LengthOutcome::Exact, "{case}");

        assert_still(&sketch, &ends, &places, &case);
        assert_still(&sketch, &plate_points, &plate_places, &case);
        assert_still(&sketch, &small_points[..1], &small_places[..1], &case);
        for (rank, side) in small.iter().enumerate() {
            let wanted = if rank % 2 == 0 { 30.0 } else { 20.0 };
            let length = sketch.segment_length(*side);
            assert!(
                (length - wanted).abs() < SETTLED,
                "{case}: side {rank} of the small rectangle went from {wanted} to {length}"
            );
        }
    }
}

#[test]
fn a_trait_ending_on_a_circle_turns_with_the_rectangle_the_circle_is_centred_on() {
    let corner = DVec2::new(40.0, 30.0);
    for value in [5.0, 40.0, 100.0] {
        let case = format!("{value}°");
        let (mut sketch, points, sides) = leaning_rectangle(corner, false);
        let round = sketch.add_circle(points[2], 20.0);
        let on_it = sketch.add_point(sketch.point(points[2]) + DVec2::new(20.0, 0.0));
        sketch.add_constraint(Constraint::OnCircle {
            point: on_it,
            circle: round,
            from: LaidFrom::Nowhere,
        });
        let spoke = sketch.add_segment(points[1], on_it);
        let length = sketch.segment_length(spoke);
        let bend = direction(&sketch, sides[0]).angle_to(direction(&sketch, spoke));
        let target = against_the_horizontal(sides[0]);

        assert_eq!(
            typed(&mut sketch, target, value),
            LengthOutcome::Exact,
            "{case}"
        );

        assert_turned_whole(&sketch, sides, points[0], corner, &case);
        let now = sketch.segment_length(spoke);
        assert!(
            (now - length).abs() < SETTLED,
            "{case}: the trait ending on the circle went from {length} long to {now}"
        );
        let bent = direction(&sketch, sides[0]).angle_to(direction(&sketch, spoke));
        assert!(
            (bent - bend).abs() < SETTLED,
            "{case}: the trait ending on the circle turned from {bend} rad off the rectangle to {bent}"
        );
    }
}
