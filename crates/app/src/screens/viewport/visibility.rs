//! What of the part the canvas shows. A sketch an extrusion or a revolution
//! was made from steps out of the way, unless its eye was lit; one never used
//! stays. A row clicked in the tree is lit where it stands, and the body is
//! drawn invisible, see-through or opaque.

use std::collections::BTreeMap;

use cao_part::history::StepKind;
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
    /// The row the hand is on in the tree, kept apart from `lit` so that
    /// leaving it puts out no light a click left.
    hovered: Option<Points>,
    body: MatterInFront,
}

impl Default for Visibility {
    fn default() -> Self {
        Self {
            eyes: BTreeMap::new(),
            lit: None,
            hovered: None,
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
        self.visibility.hovered = None;
    }

    pub(crate) fn hovered_in_the_tree(&self) -> Option<Points> {
        self.visibility.hovered
    }

    /// Lights the row the hand is on in the tree, as a click on it would.
    pub(crate) fn hover_in_the_tree(&mut self, row: Option<Points>) {
        self.visibility.hovered = row;
    }

    /// Puts the light out and every eye back to its default, once the history
    /// is numbered anew and no sketch is named the way it was.
    pub(crate) fn forget_the_sketches(&mut self) {
        self.visibility.eyes.clear();
        self.light_out();
    }

    pub(crate) fn lit(&self) -> Option<Points> {
        self.visibility.lit
    }

    pub(crate) fn body_mode(&self) -> MatterInFront {
        self.visibility.body
    }

    /// Opaque, see-through, invisible, and round again: the body fades out
    /// a step at a time.
    pub(crate) fn next_body_mode(&mut self) {
        self.visibility.body = match self.visibility.body {
            MatterInFront::Opaque => MatterInFront::SeeThrough,
            MatterInFront::SeeThrough => MatterInFront::Hidden,
            MatterInFront::Hidden => MatterInFront::Opaque,
        };
    }

    /// Each sketch a frame draws, and how.
    ///
    /// The sketch being drawn on, and the one areas are being picked on for a
    /// new extrusion, are drawn whole whatever their eye: what is worked on is
    /// seen. No element is lit in the sketch being drawn on, which has its own
    /// selection; an area, which has none, is.
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
        let here = |row: &Points| {
            row.sketch() == sketch && (!active || matches!(row, Points::Area { .. }))
        };
        let lit = (self.visibility.hovered.filter(here)).or_else(|| self.lit().filter(here));
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
        .filter(|(_, operation)| StepKind::opened_by(operation) == Some(StepKind::Sketch))
        .nth(sketch)?;
    history.number_at(step)
}
