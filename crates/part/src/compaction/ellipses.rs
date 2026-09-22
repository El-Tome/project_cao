//! The ellipses of a sketch, re-emitted with the axes they are laid with.

use cao_sketch::{EllipseId, Sketch};

use super::{claim, point_ref, record};
use crate::history::{History, Operation};
use crate::state::PartState;

use super::remap::SketchIdMap;

/// Re-emits every ellipse still drawn, and records where it, its two axes and
/// the points it stands on landed.
pub(super) fn add_ellipses(
    old_sketch: &Sketch,
    sketch_index: usize,
    map: &mut SketchIdMap,
    new_history: &mut History,
    new_state: &mut PartState,
) {
    for (old_id, oval) in old_sketch.live_ellipses() {
        let old_points = old_sketch.ellipse_points(old_id);
        let [center, west, east, south, north] =
            old_points.map(|point| point_ref(point, old_sketch, map));
        let mut next = new_state.sketches[sketch_index].points().len();
        record(
            Operation::AddEllipse {
                sketch: sketch_index,
                center: center.clone(),
                first: [west.clone(), east.clone()],
                second: [south.clone(), north.clone()],
                construction: oval.construction,
            },
            new_history,
            new_state,
        );
        for (old_point, reference) in old_points.iter().zip([center, west, east, south, north]) {
            claim(*old_point, &reference, map, &mut next);
        }
        let new_sketch = &new_state.sketches[sketch_index];
        let new_id = EllipseId(new_sketch.ellipses().len() - 1);
        let laid = new_sketch.ellipses()[new_id.0];
        map.segments.insert(oval.first, laid.first);
        map.segments.insert(oval.second, laid.second);
        map.ellipses.insert(old_id, new_id);
    }
}
