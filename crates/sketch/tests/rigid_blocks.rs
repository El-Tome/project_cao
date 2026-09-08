//! A shape the solve is not allowed to bend is carried and turned instead. A
//! turn that only approximates a rotation lengthens every arm a little, in one
//! direction only, so the drift shows up in a long run and in nothing shorter.
//!
//! The angles never repeat the same pair, so a drift that wandered instead of
//! leaning one way would be caught too.

use cao_sketch::{Constraint, DimensionTarget, Element, LengthOutcome, Sketch, WorkPlane};
use glam::DVec2;

const SCALE: f64 = 1.0;

#[test]
fn a_shape_turned_four_hundred_times_is_the_same_size_as_when_it_started() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let o = Sketch::ORIGIN;
    let a = sketch.add_point(DVec2::new(40.0, 0.0));
    let corner = sketch.add_point(DVec2::new(80.0, 0.0));
    let c = sketch.add_point(DVec2::new(80.0, 40.0));
    let d = sketch.add_point(DVec2::new(120.0, 60.0));
    sketch.add_segment(o, a);
    let first = sketch.add_segment(a, corner);
    let second = sketch.add_segment(corner, c);
    sketch.add_segment(c, d);

    sketch.set_dimension(DimensionTarget::Angle { first, second }, 90.0, false);
    sketch.resolve(SCALE);
    let side = sketch.point(o).distance(sketch.point(a));

    for turn in 0..400 {
        let angle = 30.0 + f64::from((turn * 37) % 91);
        sketch.set_dimension(DimensionTarget::Angle { first, second }, angle, false);
        sketch.resolve(SCALE);
    }

    let now = sketch.point(o).distance(sketch.point(a));
    assert!(
        (now - side).abs() < 1e-9 * side,
        "the side no dimension holds went from {side} to {now}"
    );
}

/// The shape hanging off the corner is held by nothing, so every solve carries
/// it whole. What comes out of a settle is what the weld put back, never what a
/// sweep left half corrected.
#[test]
fn a_shape_the_solve_only_carries_comes_out_as_it_went_in() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let right = sketch.add_point(DVec2::new(60.0, 0.0));
    let corner = sketch.add_point(DVec2::new(60.0, 40.0));
    let left = sketch.add_point(DVec2::new(0.0, 40.0));
    let base = sketch.add_segment(Sketch::ORIGIN, right);
    let side = sketch.add_segment(right, corner);
    let top = sketch.add_segment(corner, left);
    sketch.add_segment(left, Sketch::ORIGIN);

    let one = sketch.add_point(DVec2::new(100.0, 10.0));
    let two = sketch.add_point(DVec2::new(130.0, 30.0));
    let three = sketch.add_point(DVec2::new(100.0, 50.0));
    sketch.add_segment(corner, one);
    sketch.add_segment(one, two);
    sketch.add_segment(two, three);
    sketch.add_segment(three, corner);

    sketch.set_dimension(DimensionTarget::Length(base), 60.0, false);
    sketch.set_dimension(DimensionTarget::Length(top), 60.0, false);
    sketch.set_dimension(DimensionTarget::Length(side), 40.0, false);
    sketch.resolve(SCALE);

    let carried = [(one, two), (two, three), (three, one)];
    let was: Vec<f64> = carried
        .iter()
        .map(|(from, to)| sketch.point(*from).distance(sketch.point(*to)))
        .collect();

    for height in [55.0, 32.0, 71.0, 40.0] {
        sketch.set_dimension(DimensionTarget::Length(side), height, false);
        sketch.resolve(SCALE);
    }

    for (rank, (from, to)) in carried.iter().enumerate() {
        let now = sketch.point(*from).distance(sketch.point(*to));
        assert!(
            (now - was[rank]).abs() < 1e-12 * was[rank],
            "side {rank} of the carried shape went from {} to {now}",
            was[rank],
        );
    }
}

/// A chain fixed at both ends: the two outer arms are each free to turn about
/// their own anchor, and the elbow between them is cut from both. Its two
/// ends are hinges owned by two different blocks, neither of which answers
/// for the other's turn — a shape no single rotation can keep, since the two
/// arms are free to turn to angles that leave the elbow's own corners further
/// apart than it is long.
///
/// Measured: with the weld in place, the drawing notices it cannot keep the
/// elbow and settles the whole chain around it instead, the elbow's own
/// traits growing to at most 2.5 times their drawn length. Without the weld
/// the block-preserving pass reports itself satisfied while the elbow alone
/// stretches past 3.5 times — the far arm kept exactly rigid throughout, so
/// nothing else gives any sign that the elbow was ever bent.
#[test]
fn a_trait_hinged_to_two_other_blocks_settles_instead_of_multiplying_in_length() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let o = Sketch::ORIGIN;
    let a = sketch.add_point(DVec2::new(40.0, 5.0));
    let b = sketch.add_point(DVec2::new(70.0, 45.0));
    let m = sketch.add_point(DVec2::new(85.0, 55.0));
    let c = sketch.add_point(DVec2::new(95.0, 70.0));
    let e = sketch.add_point(DVec2::new(130.0, 80.0));
    let far = sketch.add_point(DVec2::new(160.0, 120.0));

    sketch.add_segment(o, a);
    let first = sketch.add_segment(a, b);
    let elbow_in = sketch.add_segment(b, m);
    let elbow_out = sketch.add_segment(m, c);
    let second = sketch.add_segment(c, e);
    sketch.add_segment(e, far);

    sketch.add_constraint(Constraint::Fixed {
        element: Element::Point(far),
    });
    sketch.add_constraint(Constraint::Perpendicular {
        first,
        second: elbow_in,
    });
    sketch.add_constraint(Constraint::Perpendicular {
        first: elbow_out,
        second,
    });

    let bm = sketch.point(b).distance(sketch.point(m));
    let mc = sketch.point(m).distance(sketch.point(c));
    assert_eq!(sketch.resolve(SCALE), LengthOutcome::Exact);

    let now_bm = sketch.point(b).distance(sketch.point(m));
    let now_mc = sketch.point(m).distance(sketch.point(c));
    assert!(
        now_bm < bm * 2.5 && now_mc < mc * 2.5,
        "the elbow multiplied in length rather than the drawing settling around it: \
         ({bm}, {mc}) then ({now_bm}, {now_mc})",
    );
}
