//! The panel the part's variables are edited in, beside the history.

mod state;
mod view;

use cao_part::PartDocument;

pub use state::{Named, VariablesPanel};
pub use view::VariablesAction;

use crate::lang::Catalogue;

/// What the panel did this frame.
#[derive(Default)]
pub struct Asked {
    /// Whether the part changed.
    pub changed: bool,
    /// What a refusal named, to be shown where it stands.
    pub named: Named,
}

/// Puts the panel on screen and carries out what was asked of it.
pub fn run(
    panel: &mut VariablesPanel,
    ui: &mut egui::Ui,
    document: &mut PartDocument,
    lang: &Catalogue,
) -> Asked {
    let changed = match view::panel(ui, panel, document, lang) {
        VariablesAction::Commit => panel.commit(document),
        VariablesAction::Add => panel.add(document),
        VariablesAction::Erase(variable) => panel.erase(document, variable),
        VariablesAction::Cancel => {
            panel.cancel();
            false
        }
        VariablesAction::None => return Asked::default(),
    };
    Asked {
        changed,
        named: panel.named(),
    }
}
