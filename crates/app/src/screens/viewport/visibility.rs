//! What of the part the canvas shows. A sketch an extrusion or a revolution
//! was made from steps out of the way, unless its eye was lit; one never used
//! stays. Read to a step, the part shows its sketches as that step left them:
//! only the volumes read count as made from a sketch, and a sketch past the
//! reading point is not drawn, whatever its eye. A row clicked in the tree is
//! lit where it stands, and each volume is drawn invisible, see-through or
//! opaque.

mod shown;

use std::cell::RefCell;
use std::collections::BTreeMap;

use cao_part::history::StepKind;
use cao_part::{Operation, PartDocument, VolumeState};
use cao_sketch::Sketch;

use shown::{Blocked, Replayed};

use super::ViewportState;
use crate::screens::SketchContext;
use crate::screens::part_tree::state::{Points, key_steps};

pub(super) struct Visibility {
    /// The eyes switched by hand, by the number of the operation that created
    /// their sketch: a position is handed to the next sketch once a rewind has
    /// taken this one away.
    eyes: BTreeMap<u32, bool>,
    lit: Option<Points>,
    /// The row the hand is on in the tree, kept apart from `lit` so that
    /// leaving it puts out no light a click left.
    hovered: Option<Points>,
    /// What « Montrer » marked: its row in the tree stands selected, and it
    /// is drawn in the tree's light.
    marked: Option<Marked>,
    /// The volumes set by hand, by the number of the operation that made
    /// them, as the eyes are.
    volumes: BTreeMap<u32, VolumeState>,
    /// The part last shown with the volumes' states, and what it was shown
    /// from.
    shown: RefCell<Option<Replayed>>,
    /// The raw block of the volume marked, and what it was replayed from.
    block: RefCell<Option<Blocked>>,
}

/// One sketch, or the volume made at a step of the history.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Marked {
    Sketch(usize),
    Volume(usize),
}

impl Default for Visibility {
    fn default() -> Self {
        Self {
            eyes: BTreeMap::new(),
            lit: None,
            hovered: None,
            marked: None,
            volumes: BTreeMap::new(),
            shown: RefCell::new(None),
            block: RefCell::new(None),
        }
    }
}

/// How a frame draws one sketch: every piece, or only what the tree lit.
#[derive(Clone, Copy, Default)]
pub(crate) struct Seen {
    pub(crate) hidden: bool,
    /// What a click lit, and what the hand is on.
    pub(crate) lit: [Option<Points>; 2],
    /// Marked whole, every piece of it lit.
    pub(crate) marked: bool,
}

impl Seen {
    pub(crate) fn lights(&self, row: impl Fn(Points) -> bool) -> bool {
        self.lit.into_iter().flatten().any(row)
    }
}

impl ViewportState {
    pub(crate) fn eye_lit(&self, document: &PartDocument, sketch: usize) -> bool {
        let chosen =
            created_by(document, sketch).and_then(|number| self.visibility.eyes.get(&number));
        chosen
            .copied()
            .unwrap_or_else(|| !self.used(document, sketch))
    }

    /// Whether a volume the part is read through was raised or turned from
    /// that sketch.
    fn used(&self, document: &PartDocument, sketch: usize) -> bool {
        key_steps(document)
            .enumerate()
            .any(|(rank, (_, operation, _))| match operation {
                Operation::Extrude { sketch: from, .. }
                | Operation::Revolve { sketch: from, .. } => {
                    *from == sketch && self.reads_through(rank)
                }
                _ => false,
            })
    }

    pub(crate) fn toggle_eye(&mut self, document: &PartDocument, sketch: usize) {
        let lit = self.eye_lit(document, sketch);
        if let Some(number) = created_by(document, sketch) {
            self.visibility.eyes.insert(number, !lit);
        }
    }

    /// Lights a row of the tree on the canvas, or puts it out when it is the
    /// one already lit; what was marked is no longer.
    pub(crate) fn light(&mut self, row: Points) {
        self.visibility.lit = (self.visibility.lit != Some(row)).then_some(row);
        self.visibility.marked = None;
    }

    pub(crate) fn light_out(&mut self) {
        self.visibility.lit = None;
        self.visibility.hovered = None;
        self.visibility.marked = None;
    }

    /// Stands a closed sketch's row selected in the tree and lights the whole
    /// of it, until a light is put out, a row is lit or another is marked.
    pub(crate) fn mark_sketch(&mut self, sketch: usize) {
        self.visibility.marked = Some(Marked::Sketch(sketch));
    }

    /// The same for the volume made at that step of the history, its raw block
    /// lit.
    pub(crate) fn mark_volume(&mut self, step: usize) {
        self.visibility.marked = Some(Marked::Volume(step));
    }

    pub(crate) fn marked_sketch(&self) -> Option<usize> {
        match self.visibility.marked {
            Some(Marked::Sketch(sketch)) => Some(sketch),
            _ => None,
        }
    }

    pub(crate) fn marked_volume(&self) -> Option<usize> {
        match self.visibility.marked {
            Some(Marked::Volume(step)) => Some(step),
            _ => None,
        }
    }

    pub(crate) fn hovered_in_the_tree(&self) -> Option<Points> {
        self.visibility.hovered
    }

    /// Lights the row the hand is on in the tree, as a click on it would.
    pub(crate) fn hover_in_the_tree(&mut self, row: Option<Points>) {
        self.visibility.hovered = row;
    }

    /// Puts the light out, every eye back to its default and every volume
    /// back to opaque, once the history is numbered anew and no operation is
    /// named the way it was.
    pub(crate) fn forget_the_numbers(&mut self) {
        self.visibility.eyes.clear();
        self.visibility.volumes.clear();
        self.light_out();
    }

    pub(crate) fn lit(&self) -> Option<Points> {
        self.visibility.lit
    }
}

impl ViewportState {
    /// How the volume made at that step of the history is drawn: opaque until
    /// its visibility button is clicked.
    pub(crate) fn volume_mode(&self, document: &PartDocument, step: usize) -> VolumeState {
        let chosen = document
            .history
            .number_at(step)
            .and_then(|number| self.visibility.volumes.get(&number));
        chosen.copied().unwrap_or(VolumeState::Opaque)
    }

    /// Opaque, see-through, invisible, and round again: the volume made at
    /// that step of the history fades out a step at a time.
    pub(crate) fn next_volume_mode(&mut self, document: &PartDocument, step: usize) {
        let mode = faded(self.volume_mode(document, step));
        if let Some(number) = document.history.number_at(step) {
            self.visibility.volumes.insert(number, mode);
        }
    }

    /// Whether every volume of the document is opaque.
    pub(crate) fn all_opaque(&self, document: &PartDocument) -> bool {
        volumes(document).all(|step| self.volume_mode(document, step) == VolumeState::Opaque)
    }

    /// « Tout rendre opaque »: every volume back to opaque.
    pub(crate) fn make_all_opaque(&mut self) {
        self.visibility.volumes.clear();
    }

    /// Every volume of the document not shown opaque, by the number of its
    /// operation.
    fn volume_states(&self, document: &PartDocument) -> BTreeMap<u32, VolumeState> {
        volumes(document)
            .filter_map(|step| {
                let number = document.history.number_at(step)?;
                let state = self.volume_mode(document, step);
                (state != VolumeState::Opaque).then_some((number, state))
            })
            .collect()
    }
}

impl ViewportState {
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
        let read = self.reads_through_sketch(context.document, sketch);
        let hidden = !active && !picking && !(read && self.eye_lit(context.document, sketch));
        let here = |row: &Points| {
            row.sketch() == sketch && (!active || matches!(row, Points::Area { .. }))
        };
        let lit = [self.lit(), self.visibility.hovered].map(|row| row.filter(here));
        let marked = !active && self.marked_sketch() == Some(sketch);
        let shown = !hidden || marked || lit.iter().any(Option::is_some);
        shown.then_some(Seen {
            hidden,
            lit,
            marked,
        })
    }
}

fn faded(mode: VolumeState) -> VolumeState {
    match mode {
        VolumeState::Opaque => VolumeState::SeeThrough,
        VolumeState::SeeThrough => VolumeState::Invisible,
        VolumeState::Invisible => VolumeState::Opaque,
    }
}

/// Where the volumes stand in the history: its extrusions and revolutions.
fn volumes(document: &PartDocument) -> impl Iterator<Item = usize> + '_ {
    let operations = document.history.applied_operations().iter().enumerate();
    operations
        .filter(|(_, operation)| {
            matches!(
                operation,
                Operation::Extrude { .. } | Operation::Revolve { .. }
            )
        })
        .map(|(step, _)| step)
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

#[cfg(test)]
mod tests;
