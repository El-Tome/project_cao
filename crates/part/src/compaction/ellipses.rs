//! The ellipses of a sketch, re-emitted as the curves they are pieces of and
//! the cuts that shaped them.
//!
//! An arc of ellipse is not laid as an arc: it is laid as the whole curve and
//! then cut, which is how it came about in the first place. Two pieces of one
//! curve stand on one pair of axes, and only a cut can put them there — laid
//! apart, each would get axes of its own and the two would drift.
//!
//! The cuts come after every point of the drawing is laid, since a cut names
//! the two points its stretch runs between.

use cao_sketch::{Ellipse, EllipseId, PointId, SegmentId, Sketch};

use super::remap::SketchIdMap;
use super::{claim, point_ref, record};
use crate::history::{History, Operation};
use crate::state::PartState;

/// The pieces of one curve: everything standing on the same pair of axes, in
/// the order the curve runs.
struct Curve {
    axes: SegmentId,
    pieces: Vec<(EllipseId, Ellipse)>,
}

/// Every curve the drawing holds, whole or in pieces.
fn curves(sketch: &Sketch) -> Vec<Curve> {
    let mut curves: Vec<Curve> = Vec::new();
    for (id, ellipse) in sketch.live_ellipses() {
        match curves.iter_mut().find(|held| held.axes == ellipse.first) {
            Some(held) => held.pieces.push((id, ellipse)),
            None => curves.push(Curve {
                axes: ellipse.first,
                pieces: vec![(id, ellipse)],
            }),
        }
    }
    for curve in &mut curves {
        curve
            .pieces
            .sort_by(|near, far| opens_at(sketch, near.0).total_cmp(&opens_at(sketch, far.0)));
    }
    curves
}

/// How far round the curve a piece opens, as a fraction of a whole turn.
fn opens_at(sketch: &Sketch, id: EllipseId) -> f64 {
    sketch.ellipse_run(id).0
}

/// Lays every curve the drawing holds, whole, and records where it, its two
/// axes and the five points it stands on landed.
pub(super) fn add_ellipses(
    old_sketch: &Sketch,
    sketch_index: usize,
    map: &mut SketchIdMap,
    new_history: &mut History,
    new_state: &mut PartState,
) {
    for curve in curves(old_sketch) {
        let Some((old_id, oval)) = curve.pieces.first().copied() else {
            continue;
        };
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
                drawn: None,
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
        for (piece, _) in &curve.pieces {
            map.ellipses.insert(*piece, new_id);
        }
    }
}

/// Cuts each curve back to the pieces the drawing has of it, and says which
/// piece each of them became.
pub(super) fn cut_ellipses(
    old_sketch: &Sketch,
    sketch_index: usize,
    map: &mut SketchIdMap,
    new_history: &mut History,
    new_state: &mut PartState,
) {
    for curve in curves(old_sketch) {
        let ends: Vec<(PointId, PointId)> = curve
            .pieces
            .iter()
            .filter_map(|(id, _)| old_sketch.ellipse_ends(*id))
            .collect();
        // A curve nothing cut is laid whole and stays whole.
        if ends.len() != curve.pieces.len() {
            continue;
        }

        // What a cut takes is the stretch between one piece and the next, the
        // way the curve runs.
        for rank in 0..ends.len() {
            // From where this piece closes round to where the next one opens.
            let closes = ends[rank].1;
            let opens = ends[(rank + 1) % ends.len()].0;
            let (Some(from), Some(to)) = (map.points.get(&closes), map.points.get(&opens)) else {
                continue;
            };
            let (from, to) = (*from, *to);
            let Some(ellipse) = piece_holding(&new_state.sketches[sketch_index], from, to) else {
                continue;
            };
            record(
                Operation::TrimEllipse {
                    sketch: sketch_index,
                    ellipse,
                    between: Some((from, to)),
                },
                new_history,
                new_state,
            );
        }

        for (old_piece, _) in &curve.pieces {
            let Some((from, to)) = old_sketch.ellipse_ends(*old_piece) else {
                continue;
            };
            let (Some(from), Some(to)) = (map.points.get(&from), map.points.get(&to)) else {
                continue;
            };
            let new_sketch = &new_state.sketches[sketch_index];
            let found = new_sketch
                .live_ellipses()
                .find(|(id, _)| new_sketch.ellipse_ends(*id) == Some((*from, *to)));
            if let Some((new_id, _)) = found {
                map.ellipses.insert(*old_piece, new_id);
            }
        }
    }
}

/// The piece of a curve a stretch runs inside, which is the one a cut of that
/// stretch names.
fn piece_holding(sketch: &Sketch, from: PointId, to: PointId) -> Option<EllipseId> {
    sketch
        .live_ellipses()
        .map(|(id, _)| id)
        .find(|id| sketch.ellipse_holds_the_stretch(*id, from, to))
}
