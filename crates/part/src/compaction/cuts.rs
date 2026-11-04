//! The fillets and chamfers of a sketch, laid again on the traits re-emitted.

use cao_sketch::{Chamfer, Constraint, Corner, CornerCutId, CutShape, CutSide, Sketch, Support};

use super::record;
use super::remap::SketchIdMap;
use crate::history::{ChamferAsked, CutChange, History, Operation};
use crate::state::PartState;

/// Lays every cut still standing again, one operation for each group laid
/// together so the corners keep changing together, its values written in
/// millimetres as the user would have typed them — then switches each cut
/// whose construction differs from its group's first.
pub(super) fn add_cuts(
    old_sketch: &Sketch,
    sketch_index: usize,
    map: &mut SketchIdMap,
    new_history: &mut History,
    new_state: &mut PartState,
) {
    let mut groups: Vec<usize> = Vec::new();
    for (_, cut) in old_sketch.live_corner_cuts() {
        if !groups.contains(&cut.group) {
            groups.push(cut.group);
        }
    }
    for group in groups {
        let laid: Vec<(CornerCutId, cao_sketch::CornerCut)> = old_sketch
            .live_corner_cuts()
            .filter(|(_, cut)| cut.group == group)
            .collect();
        let Some((_, first)) = laid.first().copied() else {
            continue;
        };
        let corners = laid
            .iter()
            .filter_map(|(id, _)| match old_sketch.corner_named(*id)? {
                Corner::Between(one, other) => {
                    Some(Corner::Between(side_now(one, map)?, side_now(other, map)?))
                }
                Corner::At(point) => map.points.get(&point).copied().map(Corner::At),
            })
            .collect();
        let before = new_state.sketches[sketch_index].corner_cuts().len();
        let scale = new_state.scale();
        let mm = |units: f64| units * scale;
        record(
            match first.shape {
                CutShape::Fillet { radius } => Operation::Fillet {
                    sketch: sketch_index,
                    corners,
                    radius: mm(radius).into(),
                    construction: first.construction,
                },
                CutShape::Chamfer(chamfer) => Operation::Chamfer {
                    sketch: sketch_index,
                    corners,
                    mode: ChamferAsked::from(match chamfer {
                        Chamfer::Equal(reach) => Chamfer::Equal(mm(reach)),
                        Chamfer::Sided { first, second } => Chamfer::Sided {
                            first: mm(first),
                            second: mm(second),
                        },
                        Chamfer::Angled { along, degrees } => Chamfer::Angled {
                            along: mm(along),
                            degrees,
                        },
                    }),
                    construction: first.construction,
                },
            },
            new_history,
            new_state,
        );
        for (rank, (old_id, cut)) in laid.iter().enumerate() {
            let now = CornerCutId(before + rank);
            map.cuts.insert(*old_id, now);
            // Construction is a cut's own since #618: one switched apart from
            // its group is switched again after the group is laid.
            if cut.construction != first.construction {
                record(
                    Operation::CutChange(CutChange::Construction {
                        sketch: sketch_index,
                        cut: now,
                        construction: cut.construction,
                    }),
                    new_history,
                    new_state,
                );
            }
        }
    }
}

/// A cut's side, in the numbers the re-emitted sketch uses.
fn side_now(side: CutSide, map: &SketchIdMap) -> Option<CutSide> {
    match side {
        CutSide::Trait(id) => map.segments.get(&id).copied().map(CutSide::Trait),
        CutSide::Arc(id) => map.arcs.get(&id).copied().map(CutSide::Arc),
        CutSide::ArcOfEllipse(id) => map.ellipses.get(&id).copied().map(CutSide::ArcOfEllipse),
    }
}

/// Whether a rule hangs something on a cut: laid once the cuts are, since it
/// names one.
fn hung(rule: Constraint) -> bool {
    rule.hung_on_a_cut().is_some()
}

/// The rules of a drawing that name no cut, which can be laid before them.
pub(super) fn laid_before_the_cuts(sketch: &Sketch) -> impl Iterator<Item = &Constraint> {
    sketch.constraints().iter().filter(|rule| !hung(**rule))
}

/// Lays again every rule that hung something on a cut, the cuts it names
/// given the numbers they were laid again under. One naming a cut no longer
/// standing, which was not laid again, holds nothing and is left out.
pub(super) fn hang_again(
    old_sketch: &Sketch,
    sketch_index: usize,
    map: &SketchIdMap,
    new_history: &mut History,
    new_state: &mut PartState,
) {
    let laid_again = |rule: &&Constraint| {
        let what_hangs_is_there = match **rule {
            Constraint::CutTangent { segment, .. } => map.segments.contains_key(&segment),
            rule => {
                Support::held_by(rule).is_some_and(|(point, _)| map.points.contains_key(&point))
            }
        };
        what_hangs_is_there
            && rule
                .hung_on_a_cut()
                .is_some_and(|cut| map.cuts.contains_key(&cut))
    };
    for rule in old_sketch.constraints().iter().filter(laid_again) {
        record(
            Operation::Constrain {
                sketch: sketch_index,
                constraint: super::remap::remap_constraint(super::remap::without_order(*rule), map),
            },
            new_history,
            new_state,
        );
    }
}
