//! The steps of matter through a compaction: the areas they stand on and the
//! axis they turn about, said in the numbers the re-emitted sketch uses.

use std::collections::HashSet;

use cao_sketch::{Area, SegmentId};

use super::remap::{SketchIdMap, remap_area};
use crate::history::{Operation, RevolutionAxis};
use crate::state::PartState;

/// Every segment a `Revolve` of this sketch turns around.
///
/// A revolution already ran against the segment's place at the time, and
/// stays there whatever is drawn afterwards — including erasing that segment,
/// an ordinary cleanup once it has served as an axis. So this can name a
/// segment `old_sketch.live_segments()` no longer does, and compaction has to
/// bring it back rather than leave the axis pointing at nothing.
pub(super) fn revolve_axis_segments(
    sketch_index: usize,
    operations: &[Operation],
) -> HashSet<SegmentId> {
    operations
        .iter()
        .filter_map(|operation| match operation {
            Operation::Revolve {
                sketch,
                axis: RevolutionAxis::Segment(segment),
                ..
            } if *sketch == sketch_index => Some(*segment),
            _ => None,
        })
        .collect()
}

/// An extrusion's areas, said in the numbers the re-emitted sketch uses.
///
/// A name is read against the drawing **as the old history leaves it** before
/// it is translated: it was written when the area was clicked, and the curves
/// it named may have been chamfered, rounded, trimmed or divided since.
/// Compaction drops those cuts and re-emits what they left, so a name still
/// speaking of what they took out would name nothing at all — which is how a
/// part came out of compaction with its matter gone.
///
/// A name the drawing no longer answers to is carried over as it stands: it
/// was already lost, and it stays lost.
pub(super) fn renamed(
    areas: &[Area],
    old: &PartState,
    maps: &[SketchIdMap],
    sketch: usize,
) -> Vec<Area> {
    let (Some(map), Some(drawing)) = (maps.get(sketch), old.sketches.get(sketch)) else {
        return areas.to_vec();
    };
    let regions = drawing.regions();
    areas
        .iter()
        .map(|area| {
            old.area_rank(sketch, area, &regions)
                .map(|rank| Area::of(&regions[rank], area.inside))
                .and_then(|now| remap_area(&now, map))
                .unwrap_or_else(|| area.clone())
        })
        .collect()
}
