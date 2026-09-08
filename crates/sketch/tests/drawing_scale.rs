//! The same drawing, in units a million times larger, is the same drawing. What
//! the solver refuses to divide by is written as an absolute figure in places,
//! and a gradient measured in radians per unit shrinks with the units: a big
//! enough drawing has its angles read as flat and left where they are, with
//! nothing said about it.

use cao_sketch::{DimensionTarget, Sketch, WorkPlane};
use glam::DVec2;

const SCALE: f64 = 1.0;
const WANTED: f64 = 60.0;

/// A corner of two traits, told to stand at `WANTED`, in a drawing `size` units
/// across. Answers with the angle it actually holds afterwards.
fn corner_of_a_drawing(size: f64) -> f64 {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corner = sketch.add_point(DVec2::new(size, 0.0));
    let far = sketch.add_point(DVec2::new(size, size * 0.5));
    let first = sketch.add_segment(Sketch::ORIGIN, corner);
    let second = sketch.add_segment(corner, far);

    sketch.set_dimension(DimensionTarget::Angle { first, second }, WANTED, false);
    sketch.resolve(SCALE);

    let one = sketch.point(corner) - sketch.point(Sketch::ORIGIN);
    let other = sketch.point(far) - sketch.point(corner);
    one.angle_to(other).to_degrees().abs()
}

#[test]
fn an_angle_is_held_however_large_the_units_are() {
    let held = corner_of_a_drawing(1.0);

    for size in [1e3, 1e5, 1e7, 1e9] {
        let far_out = corner_of_a_drawing(size);
        assert!(
            (far_out - held).abs() < 1e-3,
            "a drawing {size:e} units across holds {far_out}° where the same \
             one at unit size holds {held}°",
        );
    }
}
