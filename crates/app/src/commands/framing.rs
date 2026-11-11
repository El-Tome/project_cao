use cao_part::PartDocument;
use cao_sketch::WorkPlane;
use glam::DVec3;

use crate::screens;

/// Centre and radius to frame: the sketch if it has anything in it, otherwise a
/// sensible patch of the plane around its origin.
pub(crate) fn sketch_framing(
    doc: &PartDocument,
    sketch: Option<usize>,
    plane: WorkPlane,
) -> (DVec3, f64) {
    let bounds = sketch
        .and_then(|index| doc.sketches().get(index))
        .and_then(|sketch| sketch.bounds());

    match bounds {
        Some((min, max)) if (max - min).length() > 1e-4 => {
            let center = plane.to_world((min + max) * 0.5);
            (center, (max - min).length() * 0.5)
        }
        _ => (plane.origin, screens::viewport::DEFAULT_SKETCH_RADIUS),
    }
}
