//! The step reader, the card above the history in the Part panel: one box per
//! key step the part stands on — a sketch or a volume. A section raises
//! nothing and is no step.

use std::collections::HashMap;

use cao_part::PartDocument;
use cao_part::history::{ExtrusionMode, Operation, StepKind};

use super::volumes::volume_name;
use super::{PartTree, sketch_name};
use crate::lang::Catalogue;
use crate::screens::viewport::ViewportState;
use crate::ui::badge::Level;
use crate::wording::part_tree::{self as said, Problem};

/// One key step, named as its row in the tree is.
#[derive(Clone, Debug, PartialEq)]
pub struct KeyStep {
    pub kind: StepKind,
    /// Whether a volume adds matter or takes it away, as its row in the tree
    /// says; nothing for a sketch.
    pub mode: Option<ExtrusionMode>,
    pub name: String,
    /// Played: at or before the reading point.
    pub applied: bool,
    /// The box read to: the last played, the last of all while the part is
    /// read to its end.
    pub current: bool,
    /// The level of the gravest alert of its own step, as its row in the
    /// tree wears it, wherever the part is read to.
    pub alert: Option<Level>,
}

/// The line under the row: the step the part is read to, with its problem
/// when it has one and whether the steps after it are hidden, then how many
/// boxes are played out of all of them.
#[derive(Clone, Debug, PartialEq)]
pub struct ReadingLine {
    pub text: String,
    /// The level whose colour it is written in, the alert of that step.
    pub alert: Option<Level>,
    pub counter: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct StepReader {
    pub boxes: Vec<KeyStep>,
    /// One between each two boxes, applied when the box it leads to is.
    pub arrows: Vec<bool>,
    /// Nothing while there is no box.
    pub line: Option<ReadingLine>,
    /// Each level a box can wear, with its word; empty while there is no box.
    pub legend: Vec<(Level, String)>,
}

impl StepReader {
    /// The part read to the point the canvas reads it to.
    pub fn seen(document: &PartDocument, viewport: &ViewportState, lang: &Catalogue) -> Self {
        Self::read(document, lang, |rank| viewport.reads_through(rank))
    }

    /// The part read to its end.
    #[cfg_attr(not(test), expect(dead_code))]
    pub fn of(document: &PartDocument, lang: &Catalogue) -> Self {
        Self::read(document, lang, |_| true)
    }

    fn read(document: &PartDocument, lang: &Catalogue, played: impl Fn(usize) -> bool) -> Self {
        let tree = PartTree::of(document, lang);
        let problems = problems(&tree);
        let modes: HashMap<usize, ExtrusionMode> = tree
            .bodies
            .iter()
            .map(|body| (body.step, body.mode))
            .collect();
        let (mut sketches, mut volumes) = (0, 0);
        let mut steps: Vec<(KeyStep, Option<Problem>)> = key_steps(document)
            .filter_map(|(step, operation, kind)| {
                let name = match kind {
                    StepKind::Sketch => {
                        sketches += 1;
                        sketch_name(lang, sketches)
                    }
                    StepKind::Extrusion | StepKind::Revolution => {
                        volumes += 1;
                        volume_name(lang, operation, volumes)?
                    }
                    StepKind::Section => return None,
                };
                let mode = modes.get(&step).copied();
                Some((kind, mode, name, problems.get(&step).copied()))
            })
            .enumerate()
            .map(|(rank, (kind, mode, name, problem))| {
                let applied = played(rank);
                let alert = problem.map(Problem::level);
                let step = KeyStep {
                    kind,
                    mode,
                    name,
                    applied,
                    current: false,
                    alert,
                };
                (step, problem)
            })
            .collect();
        let played = steps.iter().filter(|(step, _)| step.applied).count();
        if let Some((step, _)) = played.checked_sub(1).and_then(|last| steps.get_mut(last)) {
            step.current = true;
        }
        let line = line(lang, &steps);
        let boxes: Vec<KeyStep> = steps.into_iter().map(|(step, _)| step).collect();
        let arrows = boxes.iter().skip(1).map(|step| step.applied).collect();
        let legend = match boxes.is_empty() {
            true => Vec::new(),
            false => legend(lang),
        };
        Self {
            boxes,
            arrows,
            line,
            legend,
        }
    }
}

/// The key steps in effect, in the part's order — each sketch and each
/// volume, a section raising nothing — with the place of the operation that
/// opens each among the operations applied.
pub fn key_steps(document: &PartDocument) -> impl Iterator<Item = (usize, &Operation, StepKind)> {
    let applied = document.history.applied_operations().iter().enumerate();
    applied.filter_map(|(step, operation)| {
        let kind = StepKind::opened_by(operation).filter(|kind| *kind != StepKind::Section)?;
        Some((step, operation, kind))
    })
}

/// The problem of each badge of the tree's sketches and volumes, by the step
/// in the design the row stands for.
fn problems(tree: &PartTree) -> HashMap<usize, Problem> {
    let sketches = tree.sketches.iter().map(|drawn| (drawn.step, drawn.badge));
    let volumes = tree.bodies.iter().map(|body| (body.step, body.badge));
    sketches
        .chain(volumes)
        .filter_map(|(step, badge)| Some((step, badge?)))
        .collect()
}

/// The line under the row, read off the box read to.
fn line(lang: &Catalogue, steps: &[(KeyStep, Option<Problem>)]) -> Option<ReadingLine> {
    let played = steps.iter().filter(|(step, _)| step.applied).count();
    let (step, problem) = steps.iter().find(|(step, _)| step.current)?;
    let named = match problem {
        Some(problem) => lang.t_with(
            "part_tree.reader.alerted",
            &[
                ("name", &step.name),
                ("problem", &said::badge(lang, *problem)),
            ],
        ),
        None => step.name.clone(),
    };
    let text = match played < steps.len() {
        true => lang.t_with("part_tree.reader.hidden", &[("line", &named)]),
        false => named,
    };
    let counter = lang.t_with(
        "part_tree.reader.counter",
        &[
            ("played", &played.to_string()),
            ("total", &steps.len().to_string()),
        ],
    );
    Some(ReadingLine {
        text,
        alert: step.alert,
        counter,
    })
}

fn legend(lang: &Catalogue) -> Vec<(Level, String)> {
    vec![
        (Level::Warning, lang.t("part_tree.reader.legend.warning")),
        (Level::Error, lang.t("part_tree.reader.legend.error")),
    ]
}
