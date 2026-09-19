//! What a click on a place means, as a point of the drawing.

use cao_part::PointRef;
use glam::DVec2;

use crate::screens::viewport::SketchContext;

/// Which point a click on this place means.
///
/// A point already drawn there first, so clicking back onto a corner joins
/// the geometry instead of laying a second point on top of it. Then a corner
/// of the part, which holds the point and carries it along. Failing both, a
/// new point where the click fell.
pub(crate) fn point_ref_at(
    context: &SketchContext<'_>,
    index: usize,
    position: DVec2,
    snap: f64,
) -> PointRef {
    if let Some(point) = context.document.sketches()[index].nearest_point(position, snap) {
        return PointRef::Existing(point);
    }
    // A corner of the part comes next: a point dropped on one is held there
    // and travels with it, which a point merely placed at the same figures
    // never would.
    match nearest_corner(context, index, position, snap) {
        Some((at, faces)) => PointRef::OnCorner { at, faces },
        None => PointRef::New(position),
    }
}

/// The corner of the part nearest this place on the drawing's own plane.
fn nearest_corner(
    context: &SketchContext<'_>,
    index: usize,
    position: DVec2,
    snap: f64,
) -> Option<(DVec2, Vec<usize>)> {
    context
        .document
        .corners_on(index)
        .into_iter()
        .filter(|(at, _)| at.distance(position) <= snap)
        .min_by(|a, b| a.0.distance(position).total_cmp(&b.0.distance(position)))
}
