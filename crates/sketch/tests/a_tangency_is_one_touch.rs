//! A meeting of two curves is decided on the gap between them, not on where
//! the arithmetic lands: within the drawing's tolerance it is one touch, with
//! one vertex, and beyond it a crossing pair or nothing — the same whatever
//! the drawing's position or turn.
//!
//! Closes #516.
//! - a tangency left overlapping or apart by less than `SAME` is one touch,
//!   at every turn of the drawing, for every pair of curve kinds —
//!   `a_graze_within_the_tolerance_is_one_touch_for_every_pair_at_every_turn`
//! - two curves that really cross, farther apart than `SAME`, are two
//!   crossings, at every turn —
//!   `a_real_crossing_is_two_crossings_for_every_pair_at_every_turn`,
//!   `a_small_circle_astride_an_ellipse_crosses_it_twice_wherever_it_stands`,
//!   `a_small_ellipse_astride_a_big_one_crosses_it_twice_wherever_it_stands`
//! - #419's ignored cases are run again and their outcome recorded — no test:
//!   they are `what_random_sketches_found.rs`'s own tests, re-run on this
//!   branch, the record in the pull request

use cao_sketch::{PointId, Sketch, WorkPlane};
use glam::DVec2;

/// Far under the drawing's own tolerance, which comes to about a micron on
/// the hundred-unit drawings laid here.
const GRAZE: f64 = 1e-7;

/// Far over it: a real gap, a real overlap.
const WIDE: f64 = 1.0;

const TURNS: usize = 32;

/// Lays a pair of curves tangent up to `gap` — positive apart, negative
/// overlapping — with every coordinate turned by `spin` first.
type Pair = fn(&mut Sketch, DVec2, f64);

fn point(sketch: &mut Sketch, spin: DVec2, place: DVec2) -> PointId {
    sketch.add_point(spin.rotate(place))
}

fn lay_ellipse(sketch: &mut Sketch, spin: DVec2, centre: DVec2, first: DVec2, second: f64) {
    let across = first.perp().normalize() * second;
    let middle = point(sketch, spin, centre);
    let first = [
        point(sketch, spin, centre + first),
        point(sketch, spin, centre - first),
    ];
    let second = [
        point(sketch, spin, centre + across),
        point(sketch, spin, centre - across),
    ];
    sketch.add_ellipse(middle, first, second);
}

/// A place of the ellipse `x²/a² + y²/b² = 1` and the way straight out of it.
fn on_the_ellipse(a: f64, b: f64, turn: f64) -> (DVec2, DVec2) {
    let place = DVec2::new(a * turn.cos(), b * turn.sin());
    let out = DVec2::new(b * turn.cos(), a * turn.sin()).normalize();
    (place, out)
}

fn a_segment_and_a_circle(sketch: &mut Sketch, spin: DVec2, gap: f64) {
    let centre = point(sketch, spin, DVec2::ZERO);
    sketch.add_circle(centre, 50.0);
    let from = point(sketch, spin, DVec2::new(-60.0, 50.0 + gap));
    let to = point(sketch, spin, DVec2::new(60.0, 50.0 + gap));
    sketch.add_segment(from, to);
}

fn a_segment_and_an_arc(sketch: &mut Sketch, spin: DVec2, gap: f64) {
    let centre = point(sketch, spin, DVec2::ZERO);
    let start = point(sketch, spin, DVec2::new(50.0, 0.0));
    let end = point(sketch, spin, DVec2::new(-50.0, 0.0));
    sketch.add_arc(centre, start, end);
    let from = point(sketch, spin, DVec2::new(-60.0, 50.0 + gap));
    let to = point(sketch, spin, DVec2::new(60.0, 50.0 + gap));
    sketch.add_segment(from, to);
}

fn a_segment_and_an_ellipse(sketch: &mut Sketch, spin: DVec2, gap: f64) {
    lay_ellipse(sketch, spin, DVec2::ZERO, DVec2::new(40.0, 0.0), 20.0);
    let (place, out) = on_the_ellipse(40.0, 20.0, 0.8);
    let along = out.perp();
    let from = point(sketch, spin, place + out * gap - along * 30.0);
    let to = point(sketch, spin, place + out * gap + along * 30.0);
    sketch.add_segment(from, to);
}

fn two_circles_meeting_outside(sketch: &mut Sketch, spin: DVec2, gap: f64) {
    let near = point(sketch, spin, DVec2::ZERO);
    sketch.add_circle(near, 30.0);
    let far = point(sketch, spin, DVec2::new(60.0 + gap, 0.0));
    sketch.add_circle(far, 30.0);
}

fn two_circles_meeting_inside(sketch: &mut Sketch, spin: DVec2, gap: f64) {
    let near = point(sketch, spin, DVec2::ZERO);
    sketch.add_circle(near, 50.0);
    let far = point(sketch, spin, DVec2::new(20.0 - gap, 0.0));
    sketch.add_circle(far, 30.0);
}

fn an_arc_and_a_circle(sketch: &mut Sketch, spin: DVec2, gap: f64) {
    let centre = point(sketch, spin, DVec2::ZERO);
    let start = point(sketch, spin, DVec2::new(50.0, 0.0));
    let end = point(sketch, spin, DVec2::new(-50.0, 0.0));
    sketch.add_arc(centre, start, end);
    let far = point(sketch, spin, DVec2::new(0.0, 80.0 + gap));
    sketch.add_circle(far, 30.0);
}

fn two_arcs_meeting(sketch: &mut Sketch, spin: DVec2, gap: f64) {
    let centre = point(sketch, spin, DVec2::ZERO);
    let start = point(sketch, spin, DVec2::new(50.0, 0.0));
    let end = point(sketch, spin, DVec2::new(-50.0, 0.0));
    sketch.add_arc(centre, start, end);
    let high = DVec2::new(0.0, 80.0 + gap);
    let centre = point(sketch, spin, high);
    let start = point(sketch, spin, high + DVec2::new(-30.0, 0.0));
    let end = point(sketch, spin, high + DVec2::new(30.0, 0.0));
    sketch.add_arc(centre, start, end);
}

fn a_circle_and_an_ellipse(sketch: &mut Sketch, spin: DVec2, gap: f64) {
    lay_ellipse(sketch, spin, DVec2::ZERO, DVec2::new(40.0, 0.0), 20.0);
    let (place, out) = on_the_ellipse(40.0, 20.0, 0.8);
    let far = point(sketch, spin, place + out * (30.0 + gap));
    sketch.add_circle(far, 30.0);
}

fn an_arc_and_an_ellipse(sketch: &mut Sketch, spin: DVec2, gap: f64) {
    lay_ellipse(sketch, spin, DVec2::ZERO, DVec2::new(40.0, 0.0), 20.0);
    let (place, out) = on_the_ellipse(40.0, 20.0, 0.8);
    let middle = place + out * (30.0 + gap);
    let toward = (-out).to_angle();
    let centre = point(sketch, spin, middle);
    let start = point(
        sketch,
        spin,
        middle + DVec2::from_angle(toward - 1.0) * 30.0,
    );
    let end = point(
        sketch,
        spin,
        middle + DVec2::from_angle(toward + 1.0) * 30.0,
    );
    sketch.add_arc(centre, start, end);
}

fn two_ellipses_meeting(sketch: &mut Sketch, spin: DVec2, gap: f64) {
    lay_ellipse(sketch, spin, DVec2::ZERO, DVec2::new(40.0, 0.0), 20.0);
    let (place, out) = on_the_ellipse(40.0, 20.0, 0.8);
    let (held, facing) = on_the_ellipse(25.0, 15.0, 0.6);
    let lean = (-out).to_angle() - facing.to_angle();
    let spun = DVec2::from_angle(lean);
    let centre = place + out * gap - spun.rotate(held);
    lay_ellipse(
        sketch,
        spin,
        centre,
        spun.rotate(DVec2::new(25.0, 0.0)),
        15.0,
    );
}

const PAIRS: [(&str, Pair); 10] = [
    ("a segment and a circle", a_segment_and_a_circle),
    ("a segment and an arc", a_segment_and_an_arc),
    ("a segment and an ellipse", a_segment_and_an_ellipse),
    ("two circles meeting outside", two_circles_meeting_outside),
    ("two circles meeting inside", two_circles_meeting_inside),
    ("an arc and a circle", an_arc_and_a_circle),
    ("two arcs meeting", two_arcs_meeting),
    ("a circle and an ellipse", a_circle_and_an_ellipse),
    ("an arc and an ellipse", an_arc_and_an_ellipse),
    ("two ellipses meeting", two_ellipses_meeting),
];

fn crossings_at_every_turn(gap: f64, expected: usize, verdict: &str) {
    for (label, lay) in PAIRS {
        for step in 0..TURNS {
            let spin = DVec2::from_angle(0.1 + step as f64 / TURNS as f64 * std::f64::consts::TAU);
            let mut sketch = Sketch::new(WorkPlane::XY);
            lay(&mut sketch, spin, gap);
            let found = sketch.crossings();
            assert_eq!(
                found.len(),
                expected,
                "{label}, {gap} apart, turned {step}/{TURNS}: {verdict}, got {found:?}",
            );
        }
    }
}

#[test]
fn a_graze_within_the_tolerance_is_one_touch_for_every_pair_at_every_turn() {
    crossings_at_every_turn(GRAZE, 1, "one touch");
    crossings_at_every_turn(-GRAZE, 1, "one touch");
}

#[test]
fn a_real_crossing_is_two_crossings_for_every_pair_at_every_turn() {
    crossings_at_every_turn(-WIDE, 2, "two crossings");
}

#[test]
fn curves_farther_apart_than_the_tolerance_meet_nowhere() {
    crossings_at_every_turn(WIDE, 0, "no meeting");
}

#[test]
fn a_small_circle_astride_an_ellipse_crosses_it_twice_wherever_it_stands() {
    for step in 0..64 {
        let turn = 0.013 + step as f64 / 64.0 * std::f64::consts::TAU;
        let mut sketch = Sketch::new(WorkPlane::XY);
        lay_ellipse(
            &mut sketch,
            DVec2::X,
            DVec2::ZERO,
            DVec2::new(40.0, 0.0),
            20.0,
        );
        let (place, out) = on_the_ellipse(40.0, 20.0, turn);
        let centre = sketch.add_point(place + out * 0.05);
        sketch.add_circle(centre, 0.3);

        let found = sketch.crossings();
        assert_eq!(
            found.len(),
            2,
            "standing at {turn}: both crossings, got {found:?}",
        );
        assert_eq!(
            sketch.regions().len(),
            3,
            "standing at {turn}: a lens and what is left of each",
        );
    }
}

#[test]
fn a_small_ellipse_astride_a_big_one_crosses_it_twice_wherever_it_stands() {
    for step in 0..64 {
        let turn = 0.013 + step as f64 / 64.0 * std::f64::consts::TAU;
        let mut sketch = Sketch::new(WorkPlane::XY);
        lay_ellipse(
            &mut sketch,
            DVec2::X,
            DVec2::ZERO,
            DVec2::new(40.0, 0.0),
            20.0,
        );
        let (place, out) = on_the_ellipse(40.0, 20.0, turn);
        let centre = place + out * 0.03;
        lay_ellipse(&mut sketch, DVec2::X, centre, out.perp() * 0.35, 0.2);

        let found = sketch.crossings();
        assert_eq!(
            found.len(),
            2,
            "standing at {turn}: both crossings, got {found:?}",
        );
        assert_eq!(
            sketch.regions().len(),
            3,
            "standing at {turn}: a lens and what is left of each",
        );
    }
}
