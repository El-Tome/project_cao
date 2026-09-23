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
//! - its handles are points of the drawing, the two clicked ends being the
//!   first axis's own ends — `the_two_ends_clicked_are_the_ends_of_the_first_axis`
//!   — and moving one reshapes the curve as it does on a whole ellipse —
//!   `an_end_dragged_reshapes_the_half_and_it_still_runs_between_the_ends`
//! - a placement that would lay nothing lays nothing: the two ends in one
//!   place, or no rise at all — `a_placement_that_lays_nothing_lays_nothing`
//! - the second axis stops at the rise rather than crossing to where nothing is
//!   drawn, and the drawing holds it there —
//!   `the_second_axis_stops_at_the_rise_and_the_solver_keeps_it_there`. The
//!   list first said the opposite — that a half carries the axes a trimmed
//!   ellipse carries; it was decided the other way while the branch was open,
//!   and the issue says so.
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

use cao_sketch::{EllipseId, EllipseMode, PointId, Rise, Sketch, WorkPlane, ellipse_from, rise_of};
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
    // The tool lays the second axis from the centre out to the rise, not across
    // the whole curve: half of it would stand where nothing is drawn.
    let second = [centre, sketch.add_point(drawn.centre + across)];
    let id = sketch.add_ellipse(centre, first, second);
    let [from, to] = rise_of(drawn, ABOVE).between().map(|rank| first[rank]);
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

    assert_eq!(rise_of(drawn, ABOVE), Rise::Along);
    assert_eq!(
        rise_of(drawn, ABOVE).between(),
        [1, 0],
        "a rise above runs the stretch from the axis's far end round to its near one",
    );
    assert!(
        rise_of(drawn, ABOVE).reach(drawn).y > 0.0,
        "and the second axis reaches up to it",
    );

    let under = ellipse_from(EllipseMode::ByEnds, &[LEFT, RIGHT], BELOW).expect("an ellipse");
    assert_eq!(rise_of(under, BELOW), Rise::Against);
    assert_eq!(
        rise_of(under, BELOW).between(),
        [0, 1],
        "and a rise below runs it the other way",
    );
    assert!(rise_of(under, BELOW).reach(under).y < 0.0);
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

#[test]
fn the_second_axis_stops_at_the_rise_and_the_solver_keeps_it_there() {
    let (mut sketch, id, _) = a_half_ellipse();
    let axes = sketch.ellipses()[id.0];

    let (from, to) = sketch.endpoints(axes.second);
    assert!(
        from.distance(DVec2::new(0.0, 10.0)) < 1e-9,
        "it starts at the centre: {from}",
    );
    assert!(
        to.distance(DVec2::new(0.0, 30.0)) < 1e-9,
        "and stops at the rise: {to}",
    );

    sketch.resolve(1.0);

    let drawn = sketch.ellipse_draft(id);
    assert!(
        (drawn.second - 20.0).abs() < 1e-6,
        "the curve still rises twenty, an axis read from the centre out: {}",
        drawn.second,
    );
    let (from, to) = sketch.endpoints(axes.second);
    assert!(
        from.distance(to) > 19.0,
        "and the drawing did not collapse the axis onto its centre: {from} to {to}",
    );
}

#[test]
fn an_end_dragged_reshapes_the_half_and_it_still_runs_between_the_ends() {
    let (mut sketch, id, first) = a_half_ellipse();
    let was = sketch.ellipse_draft(id).first.length();

    sketch.settle_around(first[1], DVec2::new(50.0, 30.0), 1.0);

    let drawn = sketch.ellipse_draft(id);
    assert!(
        drawn.first.length() > was + 5.0,
        "the axis stretched to the end that moved: {} was {was}",
        drawn.first.length(),
    );
    let places = sketch.ellipse_polyline(id);
    let (from, to) = (places[0], places[places.len() - 1]);
    assert!(
        from.distance(sketch.point(first[1])) < 1e-6 && to.distance(sketch.point(first[0])) < 1e-6,
        "and the half still runs between the two ends: {from} to {to}",
    );
    assert!(
        (drawn.second - 20.0).abs() < 1e-6,
        "the rise is untouched, the second axis standing square on the centre: {}",
        drawn.second,
    );
}
