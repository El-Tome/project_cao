//! A sketch compaction leaves as it was drawn.
//!
//! Compacting re-emits a sketch as the elements it is rebuilt down to, and a
//! tool that laid copies or cut corners becomes the elements it laid. That is
//! right for a number and wrong for a formula: a pattern counted by a variable
//! would become so many copies that no longer follow it. Such a sketch is laid
//! down again operation by operation, every formula said in the ranks the
//! compacted table gives its variables.

use cao_sketch::{ArcId, CircleId, EllipseId, PointId, SegmentId, Sketch};

use super::record;
use super::remap::SketchIdMap;
use super::variables::Renumbered;
use crate::formula::Formula;
use crate::history::{ChamferAsked, History, Operation, RepeatsAsked, StepKind};
use crate::state::PartState;

/// Whether a sketch holds a tool written from the variables — a pattern, a
/// chamfer, a fillet — which flattening would take the formula from.
pub(super) fn holds_a_tool_written_from_variables(history: &History, sketch: usize) -> bool {
    operations_of(history, sketch).iter().any(|(_, operation)| {
        operation
            .sizes()
            .iter()
            .any(|size| !size.variables().is_empty())
    })
}

/// Lays the sketch down again as it was drawn, and says where everything
/// landed: where it was.
pub(super) fn keep_as_drawn(
    history: &History,
    sketch: usize,
    old: &PartState,
    renumbered: &Renumbered,
    new_history: &mut History,
    new_state: &mut PartState,
) -> SketchIdMap {
    for (number, operation) in operations_of(history, sketch) {
        let again = said_again(operation, number, &mut 0, old, renumbered);
        record(again, new_history, new_state);
    }
    old.sketches
        .get(sketch)
        .map(where_it_was)
        .unwrap_or_default()
}

/// The operations of one sketch in effect, in the order they replay, the
/// opening one left out: it has been laid already.
fn operations_of(history: &History, sketch: usize) -> Vec<(u32, &Operation)> {
    let mut opened = 0;
    let mut current = None;
    let mut held = Vec::new();
    for (number, operation) in history.replay_order() {
        match StepKind::opened_by(operation) {
            Some(StepKind::Sketch) => {
                current = Some(opened);
                opened += 1;
                continue;
            }
            Some(_) => current = None,
            None => {}
        }
        if current == Some(sketch) {
            held.push((number, operation));
        }
    }
    held
}

/// The same operation, its formulas said in the new ranks. A value taken away
/// or typed again since is laid as the number it came to then: it follows
/// nothing any more, and in the compacted history every variable comes first.
/// `setting` counts the values the operation sets, as the replay counts them.
fn said_again(
    operation: &Operation,
    number: u32,
    setting: &mut u32,
    old: &PartState,
    renumbered: &Renumbered,
) -> Operation {
    let say = |formula: &Formula| renumbered.formula(formula);
    match operation {
        Operation::Gesture(done) => Operation::Gesture(
            done.iter()
                .map(|one| said_again(one, number, setting, old, renumbered))
                .collect(),
        ),
        Operation::SetDimension {
            sketch,
            target,
            value,
            placement,
        } => {
            let set = (number, *setting);
            *setting += 1;
            Operation::SetDimension {
                sketch: *sketch,
                target: *target,
                value: match old.replay.superseded.get(&set) {
                    Some(then) => value
                        .value(then)
                        .map_or_else(|| say(value), Formula::Number),
                    None => say(value),
                },
                placement: *placement,
            }
        }
        Operation::Chamfer {
            sketch,
            corners,
            mode,
        } => Operation::Chamfer {
            sketch: *sketch,
            corners: corners.clone(),
            mode: match mode {
                ChamferAsked::Equal(reach) => ChamferAsked::Equal(say(reach)),
                ChamferAsked::Angled { along, degrees } => ChamferAsked::Angled {
                    along: say(along),
                    degrees: say(degrees),
                },
                ChamferAsked::Sided { first, second } => ChamferAsked::Sided {
                    first: say(first),
                    second: say(second),
                },
            },
        },
        Operation::Fillet {
            sketch,
            corners,
            radius,
        } => Operation::Fillet {
            sketch: *sketch,
            corners: corners.clone(),
            radius: say(radius),
        },
        Operation::CircularPattern {
            sketch,
            elements,
            centre,
            degrees,
            count,
        } => Operation::CircularPattern {
            sketch: *sketch,
            elements: elements.clone(),
            centre: *centre,
            degrees: say(degrees),
            count: say(count),
        },
        Operation::RectangularPattern {
            sketch,
            elements,
            direction,
            along,
            across,
        } => {
            let run = |run: &RepeatsAsked| RepeatsAsked {
                step: say(&run.step),
                count: say(&run.count),
            };
            Operation::RectangularPattern {
                sketch: *sketch,
                elements: elements.clone(),
                direction: *direction,
                along: run(along),
                across: run(across),
            }
        }
        other => other.clone(),
    }
}

/// A sketch laid down again as it was drawn keeps every rank it had.
fn where_it_was(sketch: &Sketch) -> SketchIdMap {
    SketchIdMap {
        points: (0..sketch.points().len())
            .map(|rank| (PointId(rank), PointId(rank)))
            .collect(),
        segments: (0..sketch.segments().len())
            .map(|rank| (SegmentId(rank), SegmentId(rank)))
            .collect(),
        circles: (0..sketch.circles().len())
            .map(|rank| (CircleId(rank), CircleId(rank)))
            .collect(),
        arcs: (0..sketch.arcs().len())
            .map(|rank| (ArcId(rank), ArcId(rank)))
            .collect(),
        ellipses: (0..sketch.ellipses().len())
            .map(|rank| (EllipseId(rank), EllipseId(rank)))
            .collect(),
    }
}
