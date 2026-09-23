//! Half an ellipse, placed from its two ends and the rise between them.
//!
//! Closes #195.
//! - three clicks lay an arc of ellipse: the two ends give the whole first
//!   axis, and the third click's distance square to them the second half-axis
//!   — `the_two_ends_give_the_whole_first_axis_and_the_rise_the_second`
//! - the curve bulges to the side that third click fell on —
//!   `the_curve_bulges_to_the_side_the_rise_fell_on`
//! - what is laid is the arc alone, with no chord between the two ends —
//!   `nothing_is_drawn_between_the_two_ends`
//! - its handles are points of the drawing: the two clicked ends are the first
//!   axis's own ends, and the curve carries the centre and the axes a trimmed
//!   ellipse carries — `the_two_ends_clicked_are_the_ends_of_the_first_axis`
//! - a placement that would lay nothing lays nothing: the two ends in one
//!   place, or no rise at all — `a_placement_that_lays_nothing_lays_nothing`
//! - placing a whole ellipse still works —
//!   `the_whole_ellipse_is_still_placed_from_its_centre`
//! - every tool still takes the arc — no test: what is laid here is the arc a
//!   cut leaves, the very same `Ellipse` with a stretch taken away, and what
//!   every tool makes of that is held beside the geometry in
//!   what_a_cut_leaves_of_an_ellipse
//! - the ribbon offers the two modes and each asks for its own thing — no test:
//!   it is the ribbon's data and its wording, held in `cao_app` by
//!   `crates/app/src/wording/ellipse/tests.rs` and by the command tests
//! - the first axis's length and angle can be typed at the second click, and
//!   the rise at the third — no test: the fields a tool shows are the
//!   application's, held by `crates/app/src/screens/viewport/render/ellipse`

use cao_sketch::{EllipseId, EllipseMode, PointId, Sketch, WorkPlane, ellipse_from, half_between};
use glam::DVec2;

/// The two ends of a half-ellipse sixty wide, and a rise of twenty above them.
const LEFT: DVec2 = DVec2::new(-30.0, 10.0);
const RIGHT: DVec2 = DVec2::new(30.0, 10.0);
const ABOVE: DVec2 = DVec2::new(5.0, 30.0);
const BELOW: DVec2 = DVec2::new(5.0, -10.0);

/// The half-ellipse those two ends and that rise make, laid the way the tool
/// lays it: the whole curve on its two axes, drawn over the half the rise fell
/// on.
fn a_half_ellipse() -> (Sketch, EllipseId, [PointId; 2]) {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let drawn = ellipse_from(EllipseMode::ByEnds, &[LEFT, RIGHT], ABOVE).expect("an ellipse");
    let across = drawn.second_axis();
    let centre = sketch.add_point(drawn.centre);
    let first = [
        sketch.add_point(drawn.centre - drawn.first),
        sketch.add_point(drawn.centre + drawn.first),
    ];
    let second = [
        sketch.add_point(drawn.centre - across),
        sketch.add_point(drawn.centre + across),
    ];
    let id = sketch.add_ellipse(centre, first, second);
    let [from, to] = half_between(drawn, ABOVE).map(|rank| first[rank]);
    sketch.draw_the_stretch(id, from, to);
    (sketch, id, first)
}

#[test]
fn the_two_ends_clicked_are_the_ends_of_the_first_axis() {
    let (sketch, id, first) = a_half_ellipse();

    let places = first.map(|point| sketch.point(point));
    assert!(places[0].distance(LEFT) < 1e-9, "{}", places[0]);
    assert!(places[1].distance(RIGHT) < 1e-9, "{}", places[1]);
    let run = sketch.ellipse_polyline(id);
    assert!(
        run[0].distance(RIGHT) < 1e-6 && run[run.len() - 1].distance(LEFT) < 1e-6,
        "the curve runs between them: {} to {}",
        run[0],
        run[run.len() - 1],
    );
    let ellipse = sketch.ellipses()[id.0];
    assert_eq!(
        sketch.endpoints(ellipse.first),
        (LEFT, RIGHT),
        "and they are the first axis's own ends, the axes being the ones a \
         trimmed ellipse carries",
    );
}

#[test]
fn nothing_is_drawn_between_the_two_ends() {
    let (sketch, id, first) = a_half_ellipse();
    let axes = sketch.ellipses()[id.0];

    let joined: Vec<_> = sketch
        .live_segments()
        .filter(|(held, segment)| {
            *held != axes.first
                && *held != axes.second
                && [segment.start, segment.end]
                    .iter()
                    .all(|end| first.contains(end))
        })
        .collect();

    assert!(
        joined.is_empty(),
        "the arc is laid alone, and a side is a trait drawn like any other: {joined:?}",
    );
}

#[test]
fn the_two_ends_give_the_whole_first_axis_and_the_rise_the_second() {
    let drawn = ellipse_from(EllipseMode::ByEnds, &[LEFT, RIGHT], ABOVE).expect("an ellipse");

    assert!(
        drawn.centre.distance(DVec2::new(0.0, 10.0)) < 1e-9,
        "the centre is halfway between the two ends: {}",
        drawn.centre,
    );
    assert!(
        (drawn.first.length() - 30.0).abs() < 1e-9,
        "the two ends give the whole axis, so half of it reaches thirty: {}",
        drawn.first.length(),
    );
    assert!(
        (drawn.second - 20.0).abs() < 1e-9,
        "and the rise is the second half-axis, square to the first: {}",
        drawn.second,
    );
}

#[test]
fn the_curve_bulges_to_the_side_the_rise_fell_on() {
    let drawn = ellipse_from(EllipseMode::ByEnds, &[LEFT, RIGHT], ABOVE).expect("an ellipse");

    let [from, to] = half_between(drawn, ABOVE);
    assert_eq!(
        [from, to],
        [1, 0],
        "a rise above runs the stretch from the axis's far end round to its near one",
    );

    let under = ellipse_from(EllipseMode::ByEnds, &[LEFT, RIGHT], BELOW).expect("an ellipse");
    assert_eq!(
        half_between(under, BELOW),
        [0, 1],
        "and a rise below runs it the other way",
    );
}

#[test]
fn a_placement_that_lays_nothing_lays_nothing() {
    assert!(
        ellipse_from(EllipseMode::ByEnds, &[LEFT, LEFT], ABOVE).is_none(),
        "the two ends in one place name no axis",
    );
    assert!(
        ellipse_from(EllipseMode::ByEnds, &[LEFT, RIGHT], DVec2::new(5.0, 10.0)).is_none(),
        "a rise of nothing names no curve",
    );
    assert!(
        ellipse_from(EllipseMode::ByEnds, &[LEFT], ABOVE).is_none(),
        "and one end alone is not a placement",
    );
}

#[test]
fn the_whole_ellipse_is_still_placed_from_its_centre() {
    let drawn = ellipse_from(
        EllipseMode::ByCentre,
        &[DVec2::ZERO, DVec2::new(30.0, 0.0)],
        DVec2::new(0.0, 20.0),
    )
    .expect("an ellipse");

    assert!(drawn.centre.distance(DVec2::ZERO) < 1e-9);
    assert!((drawn.first.length() - 30.0).abs() < 1e-9);
    assert!((drawn.second - 20.0).abs() < 1e-9);
}
