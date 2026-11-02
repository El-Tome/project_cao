//! The fillets and chamfers of a sketch, laid again on the traits re-emitted.

use cao_sketch::{Chamfer, Corner, CornerCutId, CutShape, Sketch};

use super::record;
use super::remap::SketchIdMap;
use crate::history::{ChamferAsked, History, Operation};
use crate::state::PartState;

/// Lays every cut still standing again, one operation for each group laid
/// together so the corners keep changing together, its values written in
/// millimetres as the user would have typed them.
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
            .filter_map(|(_, cut)| {
                let [one, other] = cut.sides.map(|side| map.segments.get(&side).copied());
                Some(Corner::Between(one?, other?))
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
        for (rank, (old_id, _)) in laid.iter().enumerate() {
            map.cuts.insert(*old_id, CornerCutId(before + rank));
        }
    }
}
