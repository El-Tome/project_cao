//! One click of the symmetric line tool.

use glam::DVec2;

use crate::aim::{ChainAnchor, LockedInput};
use crate::sketch::Sketch;

/// What one click of the symmetric line tool does, given where its middle
/// currently stands.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SymmetricClick {
    /// The first click: nothing is drawn yet, only the middle remembered.
    Started(ChainAnchor),
    /// The far end landed back on the middle: a segment with no length would
    /// be a stray click, not a drawing.
    Ignored,
    /// A segment can be drawn from `middle`'s mirror image to `end`.
    Drew {
        middle: ChainAnchor,
        end: ChainAnchor,
    },
}

/// One click of the symmetric line tool: starts a fresh middle, or draws the
/// segment growing equally on both sides of the one already placed.
pub fn symmetric_click(
    sketch: &Sketch,
    middle: Option<ChainAnchor>,
    cursor: DVec2,
    snap: f64,
    locked: LockedInput,
    scale: f64,
) -> SymmetricClick {
    let Some(middle) = middle else {
        let anchor = match sketch.nearest_point(cursor, snap) {
            Some(id) => ChainAnchor::Point(id),
            None => ChainAnchor::Pending(cursor),
        };
        return SymmetricClick::Started(anchor);
    };
    let Some(middle_position) = sketch.anchor_position(middle) else {
        return SymmetricClick::Ignored;
    };

    let (start, end) = sketch.symmetric_ends(middle_position, cursor, locked, scale);
    let end = match sketch.nearest_point(end, snap) {
        Some(id) if locked.first.is_none() && locked.second.is_none() => ChainAnchor::Point(id),
        _ => ChainAnchor::Pending(end),
    };

    if middle_position.distance(start) < 1e-6 || middle == end {
        return SymmetricClick::Ignored;
    }
    SymmetricClick::Drew { middle, end }
}

#[cfg(test)]
mod tests;
