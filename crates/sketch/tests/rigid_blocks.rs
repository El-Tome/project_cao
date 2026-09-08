//! A shape the solve is not allowed to bend is carried and turned instead. A
//! turn that only approximates a rotation lengthens every arm a little, in one
//! direction only, so the drift shows up in a long run and in nothing shorter.
//!
//! The angles never repeat the same pair, so a drift that wandered instead of
//! leaning one way would be caught too.

use cao_sketch::{DimensionTarget, Sketch, WorkPlane};
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
