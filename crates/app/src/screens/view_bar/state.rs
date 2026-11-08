use cao_render::MatterInFront;

use crate::lang::Catalogue;
use crate::screens::sketch::SketchEditor;
use crate::wording::matter;

/// The bar's « Matière devant »: what it reads, whether it opens, and the
/// three states it offers in its menu.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatterInFrontMenu {
    pub label: String,
    pub enabled: bool,
    pub choices: Vec<MatterInFrontChoice>,
}

/// One state the menu offers, marked when the open sketch is in it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatterInFrontChoice {
    pub state: MatterInFront,
    pub label: String,
    pub current: bool,
}

impl MatterInFrontMenu {
    /// Greyed with no sketch open, as the ribbon's button is: there is no
    /// plane to show the matter in front of.
    pub fn of(editor: &SketchEditor, lang: &Catalogue) -> Self {
        let now = editor.matter_in_front;
        let choices = [
            MatterInFront::Hidden,
            MatterInFront::SeeThrough,
            MatterInFront::Opaque,
        ]
        .into_iter()
        .map(|state| MatterInFrontChoice {
            state,
            label: matter::state(lang, state),
            current: state == now,
        })
        .collect();
        Self {
            label: matter::in_front(lang, now),
            enabled: editor.active_sketch().is_some(),
            choices,
        }
    }
}

#[cfg(test)]
mod tests;
