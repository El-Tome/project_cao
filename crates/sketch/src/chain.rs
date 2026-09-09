//! One click of the line tool.

use glam::DVec2;

use crate::aim::{ChainAnchor, LockedInput};
use crate::sketch::{SegmentId, Sketch};

/// What one click of the line tool does, given where the chain currently
/// stands.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ChainClick {
    /// The first click of a chain: nothing is drawn yet, only remembered.
    Started(ChainAnchor),
    /// The same place clicked twice in a row: a segment with no length would
    /// be a stray click, not a drawing.
    Ignored,
    /// A segment can be drawn from `start` to `end`; the chain now carries on
    /// from `end`.
    Drew {
        start: ChainAnchor,
        end: ChainAnchor,
        aimed: crate::aim::Aim,
    },
}

/// One click of the line tool: starts a chain, draws the next segment of one
/// already begun, or is ignored.
pub fn chain_click(
    sketch: &Sketch,
    anchor: Option<ChainAnchor>,
    previous: Option<SegmentId>,
    cursor: DVec2,
    snap: f64,
    locked: LockedInput,
    scale: f64,
) -> ChainClick {
    let Some(anchor) = anchor else {
        let end = match sketch.nearest_point(cursor, snap) {
            Some(id) => ChainAnchor::Point(id),
            None => ChainAnchor::Pending(cursor),
        };
        return ChainClick::Started(end);
    };

    let aimed = sketch.aim(anchor, previous, cursor, locked, scale);
    let end = match sketch.nearest_point(aimed.position, snap) {
        // A value typed is a decision; joining a point that happens to be
        // near would quietly give the line another length.
        Some(id) if locked.first.is_none() && locked.second.is_none() => ChainAnchor::Point(id),
        _ => ChainAnchor::Pending(aimed.position),
    };

    if anchor == end {
        return ChainClick::Ignored;
    }
    ChainClick::Drew {
        start: anchor,
        end,
        aimed,
    }
}

#[cfg(test)]
mod tests {
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
}
