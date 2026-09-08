use cao_sketch::{DimensionTarget, Element, PointId, Sketch, SketchAxis, WorkPlane};
use glam::DVec2;

const SCALE: f64 = 1.0;

/// The origin and one point held to it by a distance along each axis: nothing
/// in it can move any more.
fn pinned_pair() -> Sketch {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let corner = sketch.add_point(DVec2::new(3.0, 4.0));
    for axis in [SketchAxis::U, SketchAxis::V] {
        sketch.set_dimension(
            DimensionTarget::Projected {
                from: PointId(0),
                to: corner,
                axis,
            },
            30.0,
            false,
        );
    }
    sketch
}

#[test]
fn an_erased_point_says_nothing_about_the_rest_of_the_drawing() {
    let mut sketch = pinned_pair();
    let before = sketch.settled_points(SCALE);
    let freedom = sketch.freedom(SCALE);

    let orphan = sketch.add_point(DVec2::new(50.0, 50.0));
    sketch.erase(Element::Point(orphan));

    let after = sketch.settled_points(SCALE);

    assert_eq!(
        after[..before.len()],
        before[..],
        "the points still drawn keep the verdict they had",
    );
    assert_eq!(
        sketch.freedom(SCALE).degrees_of_freedom,
        freedom.degrees_of_freedom,
        "an erased point adds no freedom",
    );
    assert!(
        after[orphan.0],
        "an erased point cannot move, so it is not what is left to do",
    );
}

#[test]
fn a_point_still_drawn_is_the_one_that_reads_as_free() {
    let mut sketch = pinned_pair();
    let orphan = sketch.add_point(DVec2::new(50.0, 50.0));

    let settled = sketch.settled_points(SCALE);

    assert!(!settled[orphan.0], "nothing holds it, so it can still move");
    assert_eq!(sketch.freedom(SCALE).degrees_of_freedom, 2);
}
