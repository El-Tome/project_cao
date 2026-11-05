//! Which sketches the canvas draws. One an extrusion or a revolution was made
//! from steps out of the way, unless its eye was lit; one never used stays.

use std::collections::BTreeMap;

use cao_part::{Operation, PartDocument};

use super::ViewportState;
use crate::screens::SketchContext;

/// The eyes the user switched by hand; every other sketch keeps its default.
#[derive(Default)]
pub(super) struct Eyes(BTreeMap<usize, bool>);

impl ViewportState {
    pub(crate) fn eye_lit(&self, document: &PartDocument, sketch: usize) -> bool {
        let chosen = self.eyes.0.get(&sketch).copied();
        chosen.unwrap_or_else(|| !used(document, sketch))
    }

    pub(crate) fn toggle_eye(&mut self, document: &PartDocument, sketch: usize) {
        let lit = self.eye_lit(document, sketch);
        self.eyes.0.insert(sketch, !lit);
    }

    /// The sketch being drawn on, and the one areas are being picked on for a
    /// new extrusion, are drawn whatever their eye: what is worked on is seen.
    pub(super) fn draws_sketch(&self, context: &SketchContext<'_>, sketch: usize) -> bool {
        context.editor.active_sketch() == Some(sketch)
            || (context.extrusion.is_active() && context.extrusion.sketch == Some(sketch))
            || self.eye_lit(context.document, sketch)
    }
}

fn used(document: &PartDocument, sketch: usize) -> bool {
    document
        .history
        .applied_operations()
        .iter()
        .any(|operation| match operation {
            Operation::Extrude { sketch: from, .. } | Operation::Revolve { sketch: from, .. } => {
                *from == sketch
            }
            _ => false,
        })
}
