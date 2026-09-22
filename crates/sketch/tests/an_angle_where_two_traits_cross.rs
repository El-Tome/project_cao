//! An angle between two traits that meet without sharing an end: an X, where
//! they cross, and a T, where one ends on the middle of the other.
//!
//! Closes #406.
//! - two crossing traits, clicked one after the other, lay the angle between
//!   them — `two_crossing_traits_clicked_in_turn_lay_the_angle_between_them`,
//!   and it is read in degrees like any angle, never shown as a length —
//!   `an_angle_between_two_traits_is_read_in_degrees` — and clicking the same
//!   two again opens it rather than laying a second one beside it —
//!   `clicking_the_same_two_traits_again_opens_the_angle_already_laid`
//! - a trait ending on the middle of another — a T — does the same —
//!   `a_trait_ending_on_the_middle_of_another_lays_the_angle_too`, opening
//!   only the two ways its stem is drawn —
//!   `a_t_opens_only_the_two_ways_its_stem_is_drawn`
//! - the angle laid, acute or obtuse, is the one on the side the dimension is
//!   placed — `the_angle_laid_is_the_one_on_the_side_the_dimension_is_placed`
//! - typing a value turns the traits to it, and it holds —
//!   `a_typed_angle_turns_the_crossing_traits_to_it_and_holds`,
//!   `a_typed_angle_on_a_t_stays_drawn_once_the_solver_has_moved_it`, and what
//!   hangs off the two traits is turned with them, never bent —
//!   `a_shape_hanging_off_an_angle_between_two_traits_is_turned_not_bent`
//! - the angle at a corner sharing an end is unchanged —
//!   `a_typed_angle_at_a_corner_still_turns_its_two_traits_to_it`
//! - two parallel traits are still refused —
//!   `two_parallel_traits_are_still_refused`
//! - dividing at the crossing afterwards keeps the angle, on the pieces that
//!   make it — `dividing_at_the_crossing_keeps_the_angle_on_the_pieces_that_make_it`,
//!   and cutting a trait anywhere else keeps it on the piece still standing
//!   where the two meet —
//!   `trimming_the_top_off_the_stem_of_a_t_keeps_the_angle_at_its_foot`

use cao_sketch::{
    AnnotationMetrics, Constraint, DimensionMode, DimensionPick, DimensionPicks, DimensionTarget,
    Sketch, Toward, WorkPlane, measure_pick,
};
use glam::DVec2;

const SCALE: f64 = 1.0;

/// How far, in degrees, a solved angle may sit from the value typed. The
/// solver works in small corrections until nothing moves, so what it leaves is
/// close, never exact.
const SETTLED: f64 = 1e-4;

/// The angle between two arms leaving the same place, in degrees.
fn opening(from: DVec2, towards: DVec2, and_towards: DVec2) -> f64 {
    (towards - from)
        .angle_to(and_towards - from)
        .to_degrees()
        .abs()
}

#[test]
fn a_typed_angle_at_a_corner_still_turns_its_two_traits_to_it() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corner = sketch.add_point(DVec2::new(10.0, 10.0));
    let east = sketch.add_point(DVec2::new(40.0, 10.0));
    let north_east = sketch.add_point(DVec2::new(30.0, 35.0));
    let first = sketch.add_segment(corner, east);
    let second = sketch.add_segment(corner, north_east);

    sketch.set_dimension(DimensionTarget::Angle { first, second }, 60.0, false);
    sketch.resolve(SCALE);

    let held = opening(
        sketch.point(corner),
        sketch.point(east),
        sketch.point(north_east),
    );
    assert!(
        (held - 60.0).abs() < SETTLED,
        "the corner turns to the 60° typed, got {held}°",
    );
}

/// Two traits crossing in an X at (20, 20): one rising from the south-west, the
/// other falling from the north-west.
fn an_x() -> (Sketch, [cao_sketch::SegmentId; 2]) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let south_west = sketch.add_point(DVec2::new(10.0, 10.0));
    let north_east = sketch.add_point(DVec2::new(30.0, 30.0));
    let north_west = sketch.add_point(DVec2::new(10.0, 30.0));
    let south_east = sketch.add_point(DVec2::new(30.0, 10.0));
    let rising = sketch.add_segment(south_west, north_east);
    let falling = sketch.add_segment(north_west, south_east);
    (sketch, [rising, falling])
}

/// Clicks two traits with the smart dimension tool, as a user does: the first
/// click takes the trait's length, and the second, made while that length is
/// still being put down, turns it into what the two make together.
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
fn two_crossing_traits_clicked_in_turn_lay_the_angle_between_them() {
    let (sketch, [rising, falling]) = an_x();
    let (on_rising, on_falling) = (DVec2::new(26.0, 26.0), DVec2::new(26.0, 14.0));

    let refined = clicked(&sketch, on_rising, on_falling);
    assert!(
        matches!(
            refined,
            Some(DimensionTarget::AngleBetween { first, second, .. })
                if first == rising && second == falling
        ),
        "two traits that cross have an angle between them, got {refined:?}",
    );

    let forced = clicked_for_an_angle(&sketch, on_rising, on_falling);
    assert!(
        matches!(
            forced,
            DimensionPick::Target(DimensionTarget::AngleBetween { first, second, .. })
                if first == rising && second == falling
        ),
        "and the same with the tool set to angles, got {forced:?}",
    );
}

#[test]
fn a_trait_ending_on_the_middle_of_another_lays_the_angle_too() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let west = sketch.add_point(DVec2::new(10.0, 20.0));
    let east = sketch.add_point(DVec2::new(30.0, 20.0));
    let foot = sketch.add_point(DVec2::new(20.0, 20.0));
    let top = sketch.add_point(DVec2::new(20.0, 40.0));
    let across = sketch.add_segment(west, east);
    let up = sketch.add_segment(foot, top);

    let refined = clicked(&sketch, DVec2::new(14.0, 20.0), DVec2::new(20.0, 34.0));

    assert!(
        matches!(
            refined,
            Some(DimensionTarget::AngleBetween { first, second, .. })
                if first == across && second == up
        ),
        "a trait standing on the middle of another makes an angle with it, got {refined:?}",
    );
}

#[test]
fn two_parallel_traits_are_still_refused() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let low_west = sketch.add_point(DVec2::new(10.0, 10.0));
    let low_east = sketch.add_point(DVec2::new(30.0, 10.0));
    let high_west = sketch.add_point(DVec2::new(10.0, 20.0));
    let high_east = sketch.add_point(DVec2::new(30.0, 20.0));
    sketch.add_segment(low_west, low_east);
    sketch.add_segment(high_west, high_east);
    let (on_low, on_high) = (DVec2::new(20.0, 10.0), DVec2::new(20.0, 20.0));

    let refined = clicked(&sketch, on_low, on_high);
    assert!(
        !matches!(
            refined,
            Some(DimensionTarget::Angle { .. } | DimensionTarget::AngleBetween { .. })
        ),
        "two traits running the same way never meet, and make no angle: {refined:?}",
    );
    assert_eq!(
        clicked_for_an_angle(&sketch, on_low, on_high),
        DimensionPick::TraitsAreParallel,
        "and the tool set to angles says they are parallel",
    );
}

/// Two traits crossing at (20, 20), each thirty degrees off the horizontal, so
/// that the angle opening east and west is sixty degrees and the one opening
/// north and south is a hundred and twenty.
fn a_narrow_x() -> (Sketch, [cao_sketch::SegmentId; 2]) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let crossing = DVec2::new(20.0, 20.0);
    let up = DVec2::from_angle(30.0_f64.to_radians()) * 10.0;
    let down = DVec2::from_angle(-30.0_f64.to_radians()) * 10.0;
    let rising_from = sketch.add_point(crossing - up);
    let rising_to = sketch.add_point(crossing + up);
    let falling_from = sketch.add_point(crossing - down);
    let falling_to = sketch.add_point(crossing + down);
    let rising = sketch.add_segment(rising_from, rising_to);
    let falling = sketch.add_segment(falling_from, falling_to);
    (sketch, [rising, falling])
}

#[test]
fn the_angle_laid_is_the_one_on_the_side_the_dimension_is_placed() {
    let (sketch, [rising, falling]) = a_narrow_x();
    let asked = DimensionTarget::AngleBetween {
        first: rising,
        first_toward: Toward::End,
        second: falling,
        second_toward: Toward::End,
    };

    for (placed, wanted, side) in [
        (DVec2::new(28.0, 20.0), 60.0, "east"),
        (DVec2::new(12.0, 20.0), 60.0, "west"),
        (DVec2::new(20.0, 28.0), 120.0, "north"),
        (DVec2::new(20.0, 12.0), 120.0, "south"),
    ] {
        let laid = sketch.oriented(asked, placed);
        let measured = sketch.opening(laid).expect("the two traits meet");
        assert!(
            (measured - wanted).abs() < 1e-9,
            "put down to the {side}, the angle opening that way is {wanted}°, got {measured}°",
        );
    }
}

#[test]
fn a_typed_angle_turns_the_crossing_traits_to_it_and_holds() {
    for held_at_the_crossing in [false, true] {
        let (mut sketch, [rising, falling]) = a_narrow_x();
        let crossing = held_at_the_crossing.then(|| {
            let crossing = sketch.add_point(DVec2::new(20.0, 20.0));
            for segment in [rising, falling] {
                sketch.add_constraint(Constraint::OnSegment {
                    point: crossing,
                    segment,
                });
            }
            crossing
        });
        let opening_east = DimensionTarget::AngleBetween {
            first: rising,
            first_toward: Toward::End,
            second: falling,
            second_toward: Toward::End,
        };

        for wanted in [45.0, 90.0, 20.0] {
            sketch.set_dimension(opening_east, wanted, false);
            sketch.resolve(SCALE);

            let held = sketch.opening(opening_east).expect("the two traits meet");
            assert!(
                (held - wanted).abs() < SETTLED,
                "the angle opening east turns to the {wanted}° typed, got {held}° \
                 (a point held at the crossing: {held_at_the_crossing})",
            );
            if let Some(crossing) = crossing {
                let place = sketch.point(crossing);
                for segment in [rising, falling] {
                    let (from, to) = sketch.endpoints(segment);
                    let off = (to - from).normalize().perp_dot(place - from).abs();
                    assert!(
                        off < 1e-6,
                        "the point held at the crossing stays on {segment:?} at \
                         {wanted}°, but sits {off} off it",
                    );
                }
            }
        }
    }
}

#[test]
fn dividing_at_the_crossing_keeps_the_angle_on_the_pieces_that_make_it() {
    let (mut sketch, [rising, falling]) = a_narrow_x();
    let (rising_end, falling_end) = (
        sketch.segments()[rising.0].end,
        sketch.segments()[falling.0].end,
    );
    sketch.set_dimension(
        DimensionTarget::AngleBetween {
            first: rising,
            first_toward: Toward::End,
            second: falling,
            second_toward: Toward::End,
        },
        60.0,
        false,
    );

    sketch
        .split(&[rising, falling], &[], DVec2::new(20.0, 20.0))
        .expect("the two traits cross there, with no point laid on it yet");

    let angles: Vec<DimensionTarget> = sketch
        .dimensions()
        .iter()
        .map(|dimension| dimension.target)
        .filter(|target| matches!(target, DimensionTarget::AngleBetween { .. }))
        .collect();
    let [kept] = angles[..] else {
        panic!(
            "the angle was typed, and dividing the traits keeps exactly it: {:?}",
            sketch.dimensions()
        );
    };
    let DimensionTarget::AngleBetween { first, second, .. } = kept else {
        unreachable!("filtered on that kind above");
    };
    let reaches = |piece: cao_sketch::SegmentId, end| {
        let drawn = sketch.segments()[piece.0];
        drawn.start == end || drawn.end == end
    };
    assert!(
        reaches(first, rising_end) && reaches(second, falling_end),
        "the angle opening east is carried by the two pieces that run east from \
         the crossing, not by the two that run west: {kept:?}",
    );
    let measured = sketch.opening(kept).expect("its pieces meet");
    assert!(
        (measured - 60.0).abs() < 1e-9,
        "it still measures sixty degrees, got {measured}°",
    );
}

/// A T whose bar leans, so that where the stem stands on it is no round number
/// and the crossing of the two lines is only ever found to within rounding.
///
/// These particular angles put that crossing a few parts in 10¹⁶ *before* the
/// stem's foot rather than on it — one T in five does, over a sweep of them.
fn a_leaning_t() -> (Sketch, [cao_sketch::SegmentId; 2], [cao_sketch::PointId; 2]) {
    // Written as the sums they came out of a sweep as, not as their decimal
    // results: one bit of difference and the crossing lands on the foot.
    let k = 7.0;
    let mut sketch = Sketch::new(WorkPlane::XY);
    let along = DVec2::from_angle(0.01 + k * 0.0137);
    let foot_at = DVec2::new(20.0 + k * 0.013, 20.0 - k * 0.007);
    let from = sketch.add_point(foot_at - along * 13.7);
    let to = sketch.add_point(foot_at + along * 9.1);
    let bar = sketch.add_segment(from, to);
    let rise = DVec2::from_angle(0.2 + k * 0.0091) * 17.3;
    let foot = sketch.add_point(foot_at);
    let top = sketch.add_point(foot_at + rise);
    let stem = sketch.add_segment(foot, top);
    sketch.add_constraint(Constraint::OnSegment {
        point: foot,
        segment: bar,
    });
    let low = sketch.add_point(foot_at + rise * 0.6);
    let high = sketch.add_point(foot_at + rise * 0.8);
    (sketch, [bar, stem], [low, high])
}

#[test]
fn trimming_the_top_off_the_stem_of_a_t_keeps_the_angle_at_its_foot() {
    let (mut sketch, [bar, stem], [low, high]) = a_leaning_t();
    let opening = DimensionTarget::AngleBetween {
        first: bar,
        first_toward: Toward::End,
        second: stem,
        second_toward: Toward::End,
    };
    sketch.set_dimension(opening, sketch.opening(opening).expect("they meet"), false);

    sketch
        .trim(stem, low, high)
        .expect("a stretch of the stem to take away");

    assert!(
        sketch
            .dimensions()
            .iter()
            .any(|dimension| matches!(dimension.target, DimensionTarget::AngleBetween { .. })),
        "the piece of the stem still standing on the bar keeps the angle: {:?}",
        sketch.dimensions(),
    );
}

#[test]
fn an_angle_between_two_traits_is_read_in_degrees() {
    let (_, [rising, falling]) = an_x();

    assert!(
        DimensionTarget::AngleBetween {
            first: rising,
            first_toward: Toward::End,
            second: falling,
            second_toward: Toward::Start,
        }
        .is_angle(),
        "an angle is written with a degree sign and typed in degrees, not millimetres",
    );
}

/// How big an annotation is drawn, in pixels, at one sketch unit a pixel.
fn metrics() -> AnnotationMetrics {
    AnnotationMetrics {
        offset_pixels: 20.0,
        arrow_pixels: 8.0,
        arc_pixels: 30.0,
        pixel: 1.0,
        nudge: DVec2::ZERO,
    }
}

#[test]
fn a_typed_angle_on_a_t_stays_drawn_once_the_solver_has_moved_it() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let west = sketch.add_point(DVec2::new(10.0, 20.0));
    let east = sketch.add_point(DVec2::new(30.0, 20.0));
    let foot = sketch.add_point(DVec2::new(20.0, 20.0));
    let top = sketch.add_point(DVec2::new(20.0, 40.0));
    let across = sketch.add_segment(west, east);
    let up = sketch.add_segment(foot, top);
    sketch.add_constraint(Constraint::OnSegment {
        point: foot,
        segment: across,
    });
    let opening = DimensionTarget::AngleBetween {
        first: across,
        first_toward: Toward::End,
        second: up,
        second_toward: Toward::End,
    };

    for wanted in [60.0, 120.0, 45.0] {
        sketch.set_dimension(opening, wanted, false);
        sketch.resolve(SCALE);

        let held = sketch.opening(opening).expect("the two traits are there");
        assert!(
            (held - wanted).abs() < SETTLED,
            "the T turns to the {wanted}° typed, got {held}°",
        );
        assert!(
            sketch.place(opening, metrics()).is_some(),
            "at {wanted}° the angle still drives the T, so it has to stay on the screen",
        );
    }
}

#[test]
fn a_t_opens_only_the_two_ways_its_stem_is_drawn() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let west = sketch.add_point(DVec2::new(10.0, 20.0));
    let east = sketch.add_point(DVec2::new(30.0, 20.0));
    let foot = sketch.add_point(DVec2::new(20.0, 20.0));
    let top =
        sketch.add_point(DVec2::new(20.0, 20.0) + DVec2::from_angle(60.0_f64.to_radians()) * 15.0);
    let across = sketch.add_segment(west, east);
    let stem = sketch.add_segment(foot, top);
    let asked = DimensionTarget::AngleBetween {
        first: across,
        first_toward: Toward::End,
        second: stem,
        second_toward: Toward::End,
    };

    for (placed, wanted, side) in [
        (DVec2::new(28.0, 24.0), 60.0, "east, above the bar"),
        (
            DVec2::new(28.0, 12.0),
            60.0,
            "east, below the bar where no stem is drawn",
        ),
        (DVec2::new(12.0, 24.0), 120.0, "west, above the bar"),
        (
            DVec2::new(12.0, 12.0),
            120.0,
            "west, below the bar where no stem is drawn",
        ),
    ] {
        let laid = sketch.oriented(asked, placed);
        let measured = sketch.opening(laid).expect("the two traits are there");
        assert!(
            (measured - wanted).abs() < 1e-9,
            "put down {side}, the angle reads between the bar and the stem as it is \
             drawn — {wanted}° — got {measured}°",
        );
    }
}

#[test]
fn clicking_the_same_two_traits_again_opens_the_angle_already_laid() {
    let (mut sketch, [rising, falling]) = a_narrow_x();
    let laid = DimensionTarget::AngleBetween {
        first: rising,
        first_toward: Toward::Start,
        second: falling,
        second_toward: Toward::End,
    };
    sketch.set_dimension(laid, 120.0, false);

    let crossing = DVec2::new(20.0, 20.0);
    let on_rising = crossing + DVec2::from_angle(30.0_f64.to_radians()) * 6.0;
    let on_falling = crossing + DVec2::from_angle(-30.0_f64.to_radians()) * 6.0;
    assert_eq!(
        clicked(&sketch, on_rising, on_falling),
        Some(laid),
        "the two traits already carry an angle, and clicking them again reaches it",
    );
    for placed in [DVec2::new(28.0, 20.0), DVec2::new(20.0, 28.0)] {
        assert_eq!(
            sketch.oriented(laid, placed),
            laid,
            "put down anywhere, it stays the angle already laid rather than a second \
             one beside it",
        );
    }
}

/// What dividing a crossing leaves: two traits sharing the point they were cut
/// at, the angle between them named by its arms rather than by a corner, and a
/// shape hanging off each that nothing holds.
#[test]
fn a_shape_hanging_off_an_angle_between_two_traits_is_turned_not_bent() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let o = Sketch::ORIGIN;
    let a = sketch.add_point(DVec2::new(40.0, 0.0));
    let crossing = sketch.add_point(DVec2::new(80.0, 0.0));
    let c = sketch.add_point(DVec2::new(80.0, 40.0));
    let d = sketch.add_point(DVec2::new(120.0, 60.0));
    sketch.add_segment(o, a);
    let first = sketch.add_segment(a, crossing);
    let second = sketch.add_segment(crossing, c);
    sketch.add_segment(c, d);
    let opening = DimensionTarget::AngleBetween {
        first,
        first_toward: Toward::Start,
        second,
        second_toward: Toward::End,
    };

    sketch.set_dimension(opening, 90.0, false);
    sketch.resolve(SCALE);
    let side = sketch.point(o).distance(sketch.point(a));

    for turn in 0..400 {
        let angle = 30.0 + f64::from((turn * 37) % 91);
        sketch.set_dimension(opening, angle, false);
        sketch.resolve(SCALE);
    }

    let now = sketch.point(o).distance(sketch.point(a));
    assert!(
        (now - side).abs() < 1e-9 * side,
        "the side no dimension holds went from {side} to {now}: the angle bent the \
         shape instead of turning it",
    );
}
