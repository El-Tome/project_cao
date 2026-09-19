//! How far a drawing reaches.

use super::*;

#[test]
fn bounds_cover_every_point() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    sketch.add_point(DVec2::new(-3.0, 7.0));
    sketch.add_point(DVec2::new(12.0, -1.0));
    let (min, max) = sketch.bounds().expect("some points");
    // The origin is a point like any other as far as framing goes.
    assert_eq!(min, DVec2::new(-3.0, -1.0));
    assert_eq!(max, DVec2::new(12.0, 7.0));
}
