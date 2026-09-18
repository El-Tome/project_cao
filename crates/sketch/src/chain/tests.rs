//! What sketch · chain.rs is held to.

use super::*;
use crate::plane::WorkPlane;

#[test]
fn the_first_click_of_a_chain_records_nothing() {
    let sketch = Sketch::new(WorkPlane::XY);
    let cursor = DVec2::new(12.0, 34.0);

    let click = chain_click(
        &sketch,
        None,
        None,
        cursor,
        1.0,
        LockedInput::default(),
        1.0,
    );

    assert_eq!(
        click,
        ChainClick::Started(ChainAnchor::Pending(cursor)),
        "nothing exists yet for the click to join, so the place is only remembered",
    );
}

#[test]
fn the_first_click_of_a_chain_joins_a_point_already_there() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let existing = sketch.add_point(DVec2::new(12.0, 34.0));

    let click = chain_click(
        &sketch,
        None,
        None,
        DVec2::new(12.1, 34.1),
        1.0,
        LockedInput::default(),
        1.0,
    );

    assert_eq!(click, ChainClick::Started(ChainAnchor::Point(existing)));
}

#[test]
fn the_second_click_of_a_chain_draws_one_segment_and_moves_the_anchor() {
    let sketch = Sketch::new(WorkPlane::XY);
    let anchor = ChainAnchor::Pending(DVec2::ZERO);
    let cursor = DVec2::new(40.0, 0.0);

    let click = chain_click(
        &sketch,
        Some(anchor),
        None,
        cursor,
        1.0,
        LockedInput::default(),
        1.0,
    );

    match click {
        ChainClick::Drew { start, end, .. } => {
            assert_eq!(start, anchor, "the segment starts where the chain was");
            assert_eq!(
                end,
                ChainAnchor::Pending(cursor),
                "the chain now carries on from where the click landed"
            );
        }
        other => panic!("expected a segment to be drawn, got {other:?}"),
    }
}

#[test]
fn clicking_back_onto_the_chains_own_start_draws_nothing() {
    let mut sketch = Sketch::new(WorkPlane::XY);
    let start = sketch.add_point(DVec2::new(5.0, 5.0));
    let anchor = ChainAnchor::Point(start);

    let click = chain_click(
        &sketch,
        Some(anchor),
        None,
        DVec2::new(5.0, 5.0),
        1.0,
        LockedInput::default(),
        1.0,
    );

    assert_eq!(
        click,
        ChainClick::Ignored,
        "a segment back onto its own start would have no length: nothing to draw"
    );
}
