//! Points drawn in one place, taken by the graph as the one vertex they are.

use glam::DVec2;

use super::off_by;

/// The vertex each of the drawing's own points stands on: the first point
/// drawn within reach of it, as `vertex_for` picks one for a crossing.
///
/// A rectangle's two generated corners are always new points, and one landing
/// where the drawing already has a point would otherwise be a second vertex
/// in one place, which the curves meeting there never share. The drawing keeps
/// both; only the graph takes them as one, as it does a crossing.
///
/// One step and no further: points each within reach of the next, followed
/// along, would join two that stand well apart.
pub(super) fn welded(places: &[DVec2]) -> Vec<usize> {
    let mut across: Vec<usize> = (0..places.len()).collect();
    across.sort_by(|left, right| places[*left].x.total_cmp(&places[*right].x));
    let mut vertex: Vec<usize> = (0..places.len()).collect();
    for (rank, point) in across.iter().copied().enumerate() {
        let (place, off) = (places[point], off_by(places[point]));
        let before = across[..rank].iter().rev();
        let after = across[rank + 1..].iter();
        let near = |other: &&usize| (place.x - places[**other].x).abs() < off;
        vertex[point] = before
            .take_while(near)
            .chain(after.take_while(near))
            .copied()
            .filter(|other| place.distance(places[*other]) < off)
            .fold(point, usize::min);
    }
    vertex
}
