//! The history at the bottom of the Part panel: one flat line per action,
//! numbered as the list reads. A section is a way of looking at the part, not
//! an action on it, and takes no line.

use cao_part::PartDocument;

use crate::lang::Catalogue;
use crate::wording;

/// One action of the history.
#[derive(Clone, Debug, PartialEq)]
pub struct HistoryLine {
    pub number: usize,
    pub name: String,
    pub detail: String,
    /// Taken back by an undo or a rewind, and still there to be redone.
    pub undone: bool,
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
        let lines: Vec<HistoryLine> = document
            .history
            .operations()
            .iter()
            .enumerate()
            .filter(|(_, operation)| !operation.is_about_a_section())
            .enumerate()
            .map(|(shown, (step, operation))| HistoryLine {
                number: shown + 1,
                name: wording::history::label(lang, variables, operation),
                detail: wording::history::detail(lang, variables, operation),
                undone: step >= applied,
            })
            .collect();
        let key = match lines.len() {
            1 => "part_tree.history.actions_one",
            _ => "part_tree.history.actions_many",
        };
        let actions = lang.t_with(key, &[("count", &lines.len().to_string())]);
        Self { actions, lines }
    }
}
