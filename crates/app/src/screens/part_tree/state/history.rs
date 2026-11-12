//! The history at the bottom of the Part panel: one flat line per action,
//! numbered as the list reads. A section is a way of looking at the part, not
//! an action on it, and takes no line.
//!
//! A sketch and a volume go by the names their rows above give them — a
//! sketch by the number the part gave it, a volume ranked over every step,
//! undone ones included; any other action keeps the name the history has
//! always given it.

use cao_part::feature::Feature;
use cao_part::history::{Operation, PointRef};
use cao_part::{Formula, PartDocument, Variables};

use super::numbered;
use super::volumes::{mode_key, volume_name};
use crate::lang::Catalogue;
use crate::wording::{self, dimension, formula};

/// One action of the history.
#[derive(Clone, Debug, PartialEq)]
pub struct HistoryLine {
    pub number: usize,
    pub name: String,
    pub detail: String,
    /// Taken back by an undo or a rewind, and still there to be redone.
    pub undone: bool,
    /// The part stands just after this action, the sections kept right after
    /// it included.
    pub current: bool,
    /// How many operations stand applied once the part is put back to just
    /// after this action, the sections kept right after it still in effect.
    pub back_to: usize,
    /// The sketch this action started, which a sketch's line offers to reopen;
    /// none once undone, when there is no sketch left to reopen.
    pub sketch: Option<usize>,
    /// What a pattern laying a full turn short of its count says of itself.
    pub capped: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct History {
    /// How many actions the list holds, undone ones included.
    pub actions: String,
    pub lines: Vec<HistoryLine>,
}

impl History {
    pub fn of(document: &PartDocument, lang: &Catalogue) -> Self {
        let variables = document.variables();
        let applied = document.history.applied();
        let (mut sketches, mut volumes) = (0, 0);
        let mut lines = Vec::new();
        for (step, operation) in document.history.operations().iter().enumerate() {
            if operation.is_about_a_section() {
                continue;
            }
            let started = matches!(
                operation,
                Operation::CreateSketch { .. } | Operation::CreateSketchOnSection { .. }
            );
            let (name, detail) = match operation {
                Operation::CreateSketch { plane, on } => {
                    sketches += 1;
                    let laid = wording::history::laid_on(lang, plane, on);
                    (wording::sketch::name(lang, document, sketches - 1), laid)
                }
                Operation::CreateSketchOnSection { section, .. } => {
                    sketches += 1;
                    let cut = numbered(lang, "part_tree.section", section + 1);
                    (wording::sketch::name(lang, document, sketches - 1), cut)
                }
                Operation::Extrude { .. } | Operation::Revolve { .. } => {
                    volumes += 1;
                    let name = volume_name(lang, operation, volumes).unwrap_or_default();
                    (name, matter(lang, variables, operation))
                }
                Operation::Blend { size, .. } => {
                    let name = wording::history::label(lang, variables, operation);
                    (name, blended(lang, document, step, size))
                }
                _ => drawn(lang, document, operation),
            };
            let ends = Feature::line_ends(&document.history, step);
            lines.push(HistoryLine {
                number: lines.len() + 1,
                name,
                detail,
                undone: step >= applied,
                current: step < applied && applied <= ends,
                back_to: applied.clamp(step + 1, ends),
                sketch: (started && step < applied).then(|| sketches - 1),
                capped: document
                    .history
                    .number_at(step)
                    .and_then(|number| document.capped_at(number))
                    .map(|capped| wording::pattern::step_capped(lang, capped)),
            });
        }
        let actions = counted(lang, lines.len());
        Self { actions, lines }
    }

    /// What the folded header reads, « N actions », worked out without
    /// naming a single line; nothing while nothing was done to the part.
    pub fn actions_of(document: &PartDocument, lang: &Catalogue) -> Option<String> {
        let count = document
            .history
            .operations()
            .iter()
            .filter(|operation| !operation.is_about_a_section())
            .count();
        (count > 0).then(|| counted(lang, count))
    }
}

fn counted(lang: &Catalogue, count: usize) -> String {
    let key = match count {
        1 => "part_tree.history.actions_one",
        _ => "part_tree.history.actions_many",
    };
    lang.t_with(key, &[("count", &count.to_string())])
}

/// « ajout de matière · 10 mm »: what a step of matter does, then how far.
fn matter(lang: &Catalogue, variables: &Variables, operation: &Operation) -> String {
    let (key, reach, mode) = match operation {
        Operation::Extrude { distance, mode, .. } => ("part_tree.history.raised", distance, mode),
        Operation::Revolve { angle, mode, .. } => ("part_tree.history.swept", angle, mode),
        _ => return String::new(),
    };
    let reach = formula::sized(lang, variables, reach, |at| at.to_string());
    lang.t_with(
        key,
        &[("mode", &lang.t(mode_key(*mode))), ("reach", &reach)],
    )
}

/// « 2 mm » for an edge chamfered or rounded as asked; both values when the
/// faces along it gave less.
fn blended(lang: &Catalogue, document: &PartDocument, step: usize, size: &Formula) -> String {
    let reduced = document
        .history
        .number_at(step)
        .and_then(|number| document.reduced_at(number));
    match reduced {
        Some(reduced) => lang.t_with(
            "part_tree.history.blend_reduced",
            &[
                ("asked", &dimension::short(reduced.asked)),
                ("taken", &dimension::short(reduced.taken)),
            ],
        ),
        None => lang.t_with(
            "part_tree.history.blend_size",
            &[(
                "size",
                &formula::sized(lang, document.variables(), size, dimension::short),
            )],
        ),
    }
}

/// The name and the detail of an action inside a sketch: its size, for a
/// rectangle or a circle, and what the history has always said otherwise.
fn drawn(lang: &Catalogue, document: &PartDocument, operation: &Operation) -> (String, String) {
    let name = wording::history::label(lang, document.variables(), operation);
    let size = match operation {
        Operation::AddRectangle {
            corner: PointRef::New(corner),
            opposite: PointRef::New(opposite),
            ..
        } => {
            let span = (*opposite - *corner).abs();
            Some(lang.t_with(
                "part_tree.history.rectangle",
                &[
                    ("width", &dimension::short(span.x)),
                    ("height", &dimension::short(span.y)),
                ],
            ))
        }
        Operation::AddCircle { radius, .. } => Some(lang.t_with(
            "part_tree.history.circle",
            &[("diameter", &dimension::short(radius * 2.0))],
        )),
        _ => None,
    };
    let detail = size.unwrap_or_else(|| wording::history::detail_in(lang, document, operation));
    (name, detail)
}
