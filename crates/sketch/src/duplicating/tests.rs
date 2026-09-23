//! What sketch · duplicating.rs is held to.

use super::*;
use crate::element::kinds::{name_of, one_of_every_kind};
use crate::plane::WorkPlane;

#[test]
fn a_copy_answers_for_one_of_every_kind() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let drawn = one_of_every_kind(&mut sketch);

    let made = sketch.duplicate(&drawn, |at| at + DVec2::new(100.0, 0.0));

    for element in &drawn {
        let copied = match element {
            Element::Point(_) => !made.points.is_empty(),
            Element::Segment(_) => !made.segments.is_empty(),
            Element::Circle(_) => !made.circles.is_empty(),
            Element::Arc(_) => !made.arcs.is_empty(),
            Element::Ellipse(_) => !made.ellipses.is_empty(),
        };
        assert!(
            copied,
            "a {} was handed to duplicate and no copy came back: {made:?}",
            name_of(element),
        );
    }
}

#[test]
fn an_ellipse_taken_with_its_axes_is_laid_once() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let centre = sketch.add_point(DVec2::new(0.0, 0.0));
    let west = sketch.add_point(DVec2::new(-10.0, 0.0));
    let east = sketch.add_point(DVec2::new(10.0, 0.0));
    let south = sketch.add_point(DVec2::new(0.0, -4.0));
    let north = sketch.add_point(DVec2::new(0.0, 4.0));
    let oval = sketch.add_ellipse(centre, [west, east], [south, north]);
    let held = sketch.ellipses()[oval.0];
    let segments_before = sketch.segments().len();

    let made = sketch.duplicate(
        &[
            Element::Ellipse(oval),
            Element::Segment(held.first),
            Element::Segment(held.second),
        ],
        |at| at + DVec2::new(0.0, 50.0),
    );

    assert_eq!(made.ellipses.len(), 1);
    assert_eq!(
        made.segments.len(),
        2,
        "the axes come with the ellipse, and are laid once: {made:?}",
    );
    assert_eq!(sketch.segments().len(), segments_before + 2);
    assert_eq!(made.points.len(), 5);
}
