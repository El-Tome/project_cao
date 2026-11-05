//! What of the part the canvas shows. A sketch an extrusion or a revolution
//! was made from steps out of the way, unless its eye was lit; one never used
//! stays. A row clicked in the tree is lit where it stands, and the body is
//! drawn invisible, see-through or opaque.

use std::collections::BTreeMap;

use cao_part::{Operation, PartDocument};
use cao_render::MatterInFront;
use cao_sketch::Sketch;

use super::ViewportState;
use crate::screens::SketchContext;
use crate::screens::part_tree::state::Points;

pub(super) struct Visibility {
    /// The eyes switched by hand, by the number of the operation that created
    /// their sketch: a position is handed to the next sketch once a rewind has
    /// taken this one away.
    eyes: BTreeMap<u32, bool>,
    lit: Option<Points>,
    body: MatterInFront,
}

impl Default for Visibility {
    fn default() -> Self {
        Self {
            eyes: BTreeMap::new(),
            lit: None,
            body: MatterInFront::Opaque,
        }
    }
}

/// How a frame draws one sketch: every piece, or only what the tree lit.
#[derive(Clone, Copy, Default)]
pub(crate) struct Seen {
    pub(crate) hidden: bool,
    pub(crate) lit: Option<Points>,
}

impl ViewportState {
    pub(crate) fn eye_lit(&self, document: &PartDocument, sketch: usize) -> bool {
        let chosen =
            created_by(document, sketch).and_then(|number| self.visibility.eyes.get(&number));
        chosen.copied().unwrap_or_else(|| !used(document, sketch))
    }

    pub(crate) fn toggle_eye(&mut self, document: &PartDocument, sketch: usize) {
        let lit = self.eye_lit(document, sketch);
        if let Some(number) = created_by(document, sketch) {
            self.visibility.eyes.insert(number, !lit);
        }
    }

    /// Lights a row of the tree on the canvas, or puts it out when it is the
    /// one already lit.
    pub(crate) fn light(&mut self, row: Points) {
        self.visibility.lit = (self.visibility.lit != Some(row)).then_some(row);
    }

    pub(crate) fn light_out(&mut self) {
        self.visibility.lit = None;
    }

    pub(crate) fn lit(&self) -> Option<Points> {
        self.visibility.lit
    }

    pub(crate) fn body_mode(&self) -> MatterInFront {
        self.visibility.body
    }

    pub(crate) fn next_body_mode(&mut self) {
        self.visibility.body = self.visibility.body.next();
    }

    /// Each sketch a frame draws, and how.
    ///
    /// The sketch being drawn on, and the one areas are being picked on for a
    /// new extrusion, are drawn whole whatever their eye: what is worked on is
    /// seen. Nothing is lit in the sketch being drawn on, which has its own
    /// selection.
    pub(crate) fn sketches_seen<'a>(
        &'a self,
        context: &'a SketchContext<'_>,
    ) -> impl Iterator<Item = (usize, &'a Sketch, Seen)> + 'a {
        let sketches = context.document.sketches().iter().enumerate();
        sketches.filter_map(|(index, sketch)| Some((index, sketch, self.seen(context, index)?)))
    }

    fn seen(&self, context: &SketchContext<'_>, sketch: usize) -> Option<Seen> {
        let active = context.editor.active_sketch() == Some(sketch);
        let picking = context.extrusion.is_active() && context.extrusion.sketch == Some(sketch);
        let hidden = !active && !picking && !self.eye_lit(context.document, sketch);
        let lit = self.lit().filter(|row| !active && row.sketch() == sketch);
        (!hidden || lit.is_some()).then_some(Seen { hidden, lit })
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

fn created_by(document: &PartDocument, sketch: usize) -> Option<u32> {
    let history = &document.history;
    let (step, _) = history
        .applied_operations()
        .iter()
        .enumerate()
        .filter(|(_, operation)| matches!(operation, Operation::CreateSketch { .. }))
        .nth(sketch)?;
    history.number_at(step)
}
