//! Rewrites a part's history down to the steps that still describe it.
//!
//! `MovePoint`, `MoveMany`, `MoveDimension` and `MergePoints` say how a shape
//! was reached, not what it is; erased elements and their `EraseMany` leave
//! nothing once the erasing is done. Compaction drops all of that and re-emits
//! each sketch as the points, segments, circles, arcs, ellipses, constraints
//! and dimensions it is rebuilt down to — so a `PointId` never has to be renamed in
//! place, which would silently point a later step at a different piece of the
//! drawing.

use std::collections::HashSet;

use cao_sketch::{ArcId, Area, CircleId, Constraint, PointId, Segment, SegmentId, Sketch};

use crate::history::{History, Operation, PointRef, RevolutionAxis};
use crate::state::PartState;
use remap::{SketchIdMap, remap_area, remap_constraint, remap_target};

mod ellipses;
mod remap;

/// Rewrites the applied part of `history` down to what the part still is.
///
/// The redo tail is dropped along with it: compaction replaces the list
/// rather than acting as one more step of it, so there is nothing to redo back
/// to.
pub fn compact(history: &History) -> History {
    let old_state = PartState::rebuild(history);
    let operations = history.applied_operations();
    let mut new_history = history.following();
    let mut new_state = PartState::default();
    let mut sketch_maps: Vec<SketchIdMap> = Vec::new();

    for operation in operations {
        match operation {
            Operation::CreateSketch { plane, on } => {
                record(
                    Operation::CreateSketch {
                        plane: *plane,
                        on: *on,
                    },
                    &mut new_history,
                    &mut new_state,
                );
                let sketch_index = sketch_maps.len();
                let old_sketch = &old_state.sketches[sketch_index];
                let axis_segments = revolve_axis_segments(sketch_index, operations);
                let map = compact_sketch(
                    old_sketch,
                    sketch_index,
                    &axis_segments,
                    &mut new_history,
                    &mut new_state,
                );
                sketch_maps.push(map);
            }
            Operation::Extrude {
                sketch,
                areas,
                distance,
                mode,
            } => record(
                Operation::Extrude {
                    sketch: *sketch,
                    areas: renamed(areas, &old_state, &sketch_maps, *sketch),
                    distance: *distance,
                    mode: *mode,
                },
                &mut new_history,
                &mut new_state,
            ),
            Operation::Revolve {
                sketch,
                areas,
                axis,
                angle,
                mode,
            } => {
                let axis = match axis {
                    RevolutionAxis::Sketch(sketch_axis) => RevolutionAxis::Sketch(*sketch_axis),
                    RevolutionAxis::Segment(segment) => {
                        RevolutionAxis::Segment(sketch_maps[*sketch].segments[segment])
                    }
                };
                record(
                    Operation::Revolve {
                        sketch: *sketch,
                        areas: renamed(areas, &old_state, &sketch_maps, *sketch),
                        axis,
                        angle: *angle,
                        mode: *mode,
                    },
                    &mut new_history,
                    &mut new_state,
                );
            }
            // Every other operation reshapes a sketch already covered, in full,
            // by the `CreateSketch` branch above.
            _ => {}
        }
    }

    new_history
}

/// Applies one operation to the state being built and records it, so the two
/// never drift apart.
fn record(op: Operation, new_history: &mut History, new_state: &mut PartState) {
    new_state.apply(&op);
    new_history.push(op);
}

/// Every segment a `Revolve` of this sketch turns around.
///
/// A revolution already ran against the segment's place at the time, and
/// stays there whatever is drawn afterwards — including erasing that segment,
/// an ordinary cleanup once it has served as an axis. So this can name a
/// segment `old_sketch.live_segments()` no longer does, and compaction has to
/// bring it back rather than leave the axis pointing at nothing.
fn revolve_axis_segments(sketch_index: usize, operations: &[Operation]) -> HashSet<SegmentId> {
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

/// `old_id`'s place in the point already re-emitted, or the `PointRef` that
/// will add it — the id it lands on is only known once the operation carrying
/// it has actually run, since it may be one of several new points a single
/// step adds.
fn point_ref(old_id: PointId, old_sketch: &Sketch, map: &SketchIdMap) -> PointRef {
    match map.points.get(&old_id) {
        Some(&existing) => PointRef::Existing(existing),
        None => PointRef::New(old_sketch.point(old_id)),
    }
}

/// Records that `old_id` landed at the next unclaimed point, if `reference`
/// was a `New` one — an `Existing` reference claimed nothing.
fn claim(old_id: PointId, reference: &PointRef, map: &mut SketchIdMap, next: &mut usize) {
    if let PointRef::New(_) | PointRef::Held { .. } = reference {
        map.points.insert(old_id, PointId(*next));
        *next += 1;
    }
}

/// Re-emits one segment and records where it landed.
fn add_segment(
    old_id: SegmentId,
    segment: Segment,
    old_sketch: &Sketch,
    sketch_index: usize,
    map: &mut SketchIdMap,
    new_history: &mut History,
    new_state: &mut PartState,
) {
    let start = point_ref(segment.start, old_sketch, map);
    let end = point_ref(segment.end, old_sketch, map);
    let mut next = new_state.sketches[sketch_index].points().len();
    record(
        Operation::AddSegment {
            sketch: sketch_index,
            start: start.clone(),
            end: end.clone(),
            construction: segment.construction,
        },
        new_history,
        new_state,
    );
    claim(segment.start, &start, map, &mut next);
    claim(segment.end, &end, map, &mut next);
    let new_id = SegmentId(new_state.sketches[sketch_index].segments().len() - 1);
    map.segments.insert(old_id, new_id);
}

fn compact_sketch(
    old_sketch: &Sketch,
    sketch_index: usize,
    axis_segments: &HashSet<SegmentId>,
    new_history: &mut History,
    new_state: &mut PartState,
) -> SketchIdMap {
    let mut map = SketchIdMap::default();
    map.points.insert(Sketch::ORIGIN, Sketch::ORIGIN);

    // An axis of an ellipse is laid again by the ellipse itself, below.
    let drawn_alone = |(id, _): &(SegmentId, Segment)| old_sketch.ellipse_of_axis(*id).is_none();
    for (old_id, segment) in old_sketch.live_segments().filter(drawn_alone) {
        add_segment(
            old_id,
            segment,
            old_sketch,
            sketch_index,
            &mut map,
            new_history,
            new_state,
        );
    }

    ellipses::add_ellipses(old_sketch, sketch_index, &mut map, new_history, new_state);

    // A revolution around one of these still needs it, even where it is no
    // longer part of the drawing.
    for &old_id in axis_segments {
        if map.segments.contains_key(&old_id) {
            continue;
        }
        if let Some(&segment) = old_sketch.segments().get(old_id.0) {
            add_segment(
                old_id,
                segment,
                old_sketch,
                sketch_index,
                &mut map,
                new_history,
                new_state,
            );
        }
    }

    let mut bundled_rim_points = HashSet::new();
    for (old_id, circle) in old_sketch.live_circles() {
        let center = point_ref(circle.center, old_sketch, &map);
        // A point clicked on the rim while drawing is kept on it by an
        // `OnCircle` constraint added in the very same step, not a separate
        // one — what makes the point a handle the circle can be grabbed and
        // resized by rather than a coincidence that happens to line up.
        let rim_old: Vec<PointId> = old_sketch
            .constraints()
            .iter()
            .filter_map(|constraint| match constraint {
                Constraint::OnCircle {
                    point,
                    circle: held,
                } if *held == old_id => Some(*point),
                _ => None,
            })
            .collect();
        let rim: Vec<PointRef> = rim_old
            .iter()
            .map(|&point| point_ref(point, old_sketch, &map))
            .collect();
        let mut next = new_state.sketches[sketch_index].points().len();
        record(
            Operation::AddCircle {
                sketch: sketch_index,
                center: center.clone(),
                radius: circle.radius,
                rim: rim.clone(),
                construction: circle.construction,
            },
            new_history,
            new_state,
        );
        claim(circle.center, &center, &mut map, &mut next);
        for (&old_point, reference) in rim_old.iter().zip(rim.iter()) {
            claim(old_point, reference, &mut map, &mut next);
            bundled_rim_points.insert((old_point, old_id));
        }
        let new_id = CircleId(new_state.sketches[sketch_index].circles().len() - 1);
        map.circles.insert(old_id, new_id);
    }

    for (old_id, arc) in old_sketch.live_arcs() {
        let center = point_ref(arc.center, old_sketch, &map);
        let start = point_ref(arc.start, old_sketch, &map);
        let end = point_ref(arc.end, old_sketch, &map);
        let mut next = new_state.sketches[sketch_index].points().len();
        record(
            Operation::AddArc {
                sketch: sketch_index,
                center: center.clone(),
                start: start.clone(),
                end: end.clone(),
                construction: arc.construction,
            },
            new_history,
            new_state,
        );
        claim(arc.center, &center, &mut map, &mut next);
        claim(arc.start, &start, &mut map, &mut next);
        claim(arc.end, &end, &mut map, &mut next);
        let new_id = ArcId(new_state.sketches[sketch_index].arcs().len() - 1);
        map.arcs.insert(old_id, new_id);
    }

    // Points drawn on their own, or held only by a constraint (the point of a
    // tangency), are still live but were never reached above.
    let loose: Vec<PointId> = old_sketch
        .live_points()
        .map(|(id, _)| id)
        .filter(|id| !map.points.contains_key(id))
        .collect();
    for old_id in loose {
        record(
            Operation::AddPoint {
                sketch: sketch_index,
                position: old_sketch.point(old_id),
                on: Vec::new(),
            },
            new_history,
            new_state,
        );
        let new_id = PointId(new_state.sketches[sketch_index].points().len() - 1);
        map.points.insert(old_id, new_id);
    }

    for constraint in old_sketch.constraints() {
        if let Constraint::OnCircle { point, circle } = constraint
            && bundled_rim_points.contains(&(*point, *circle))
        {
            continue;
        }
        record(
            Operation::Constrain {
                sketch: sketch_index,
                constraint: remap_constraint(*constraint, &map),
            },
            new_history,
            new_state,
        );
    }

    for dimension in old_sketch.dimensions() {
        record(
            Operation::SetDimension {
                sketch: sketch_index,
                target: remap_target(dimension.target, &map),
                value: dimension.value,
                placement: dimension.offset,
            },
            new_history,
            new_state,
        );
    }

    map
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
fn renamed(areas: &[Area], old: &PartState, maps: &[SketchIdMap], sketch: usize) -> Vec<Area> {
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

#[cfg(test)]
mod tests;
