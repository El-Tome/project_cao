//! Which point an operation refers to.
//!
//! Settled when the user clicks and never worked out again on replay, which is
//! the whole reason it is recorded rather than re-derived.

use cao_sketch::{PointId, Support};
use glam::DVec2;
use serde::{Deserialize, Serialize};

/// Which point an operation refers to.
///
/// Resolved when the user clicks, never re-derived on replay: snapping depends
/// on the zoom level at the time, so re-running it later could join different
/// points and rebuild a different drawing.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PointRef {
    Existing(PointId),
    New(DVec2),
    /// A point laid on a curve of the drawing or on an axis of its plane, and
    /// held there. On two of them where they cross, which is what keeps it at
    /// the crossing rather than merely where the crossing then was.
    ///
    /// What it landed on is settled at the click, like the rest of this: the
    /// magnets depend on the zoom at the time, so working it out again on
    /// replay could hold it to something nobody pointed at.
    Held {
        at: DVec2,
        on: Vec<Support>,
    },
}
