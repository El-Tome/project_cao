//! The panel the part's variables are edited in, beside the history.

mod state;
mod view;

use cao_part::{PartDocument, Variables};

pub use state::{Named, VariablesPanel};
pub use view::VariablesAction;

use crate::lang::Catalogue;
use crate::ui::formula_field::{Naming, Offer};

/// What makes a name, as the formula reader has it: what a field completes
/// is what a formula reads.
pub const NAMING: Naming = Naming {
    starts: cao_part::formula::starts_a_name,
    goes_on: cao_part::formula::goes_on_a_name,
};

/// The part's variables as a field offers them to complete a name typed:
/// in the order the panel shows them, each with what it comes to.
pub fn offered(variables: &Variables) -> Vec<Offer> {
    let values = variables.values();
    variables
        .live()
        .map(|(variable, found)| Offer {
            name: found.name().to_string(),
            beside: values
                .get(variable.0)
                .filter(|value| value.is_finite())
                .map(|value| shown(*value))
                .unwrap_or_default(),
        })
        .collect()
}

/// What a variable comes to, to the ten-thousandth and no further.
pub(crate) fn shown(value: f64) -> String {
    let text = format!("{value:.4}");
    text.trim_end_matches('0').trim_end_matches('.').to_string()
}

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
