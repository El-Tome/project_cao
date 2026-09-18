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
        };
        assert!(
            copied,
            "a {} was handed to duplicate and no copy came back: {made:?}",
            name_of(element),
        );
    }
}
