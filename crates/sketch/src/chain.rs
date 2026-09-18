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
mod tests;
