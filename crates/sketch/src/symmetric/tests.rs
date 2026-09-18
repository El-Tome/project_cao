//! What sketch · symmetric.rs is held to.

use super::*;
use crate::plane::WorkPlane;

#[test]
fn the_first_click_only_remembers_the_middle() {
    let sketch = Sketch::new(WorkPlane::XY);
    let cursor = DVec2::new(12.0, 34.0);

    let click = symmetric_click(&sketch, None, cursor, 1.0, LockedInput::default(), 1.0);

    assert_eq!(click, SymmetricClick::Started(ChainAnchor::Pending(cursor)));
}

#[test]
fn the_second_click_draws_a_segment_growing_from_the_middle() {
    let sketch = Sketch::new(WorkPlane::XY);
    let middle = ChainAnchor::Pending(DVec2::new(10.0, 0.0));

    let click = symmetric_click(
        &sketch,
        Some(middle),
        DVec2::new(40.0, 0.0),
        1.0,
        LockedInput::default(),
        1.0,
    );

    assert_eq!(
        click,
        SymmetricClick::Drew {
            middle,
            end: ChainAnchor::Pending(DVec2::new(40.0, 0.0)),
        },
    );
}

#[test]
fn clicking_back_onto_the_middle_draws_nothing() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let middle = sketch.add_point(DVec2::new(5.0, 5.0));

    let click = symmetric_click(
        &sketch,
        Some(ChainAnchor::Point(middle)),
        DVec2::new(5.0, 5.0),
        1.0,
        LockedInput::default(),
        1.0,
    );

    assert_eq!(click, SymmetricClick::Ignored);
}
