//! What of the part the canvas shows. A sketch an extrusion or a revolution
//! was made from steps out of the way, unless its eye was lit; one never used
//! stays. A row clicked in the tree is lit where it stands, and each volume is
//! drawn invisible, see-through or opaque, on its own or with the whole body.

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
    /// The volumes set by hand, by the number of the operation that made
    /// them, as the eyes are.
    volumes: BTreeMap<u32, MatterInFront>,
    /// The last state chosen for the whole body: what a volume left alone
    /// reads, and what the faces no volume made are drawn in.
    whole_body: MatterInFront,
}

impl Default for Visibility {
    fn default() -> Self {
        Self {
            eyes: BTreeMap::new(),
            lit: None,
            hovered: None,
            volumes: BTreeMap::new(),
            whole_body: MatterInFront::Opaque,
        }
    }
}

/// How a frame draws one sketch: every piece, or only what the tree lit.
#[derive(Clone, Copy, Default)]
pub(crate) struct Seen {
    pub(crate) hidden: bool,
    /// What a click lit, and what the hand is on.
    pub(crate) lit: [Option<Points>; 2],
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
}

impl ViewportState {
    /// How the volume made at that step of the history is drawn.
    pub(crate) fn volume_mode(&self, document: &PartDocument, step: usize) -> MatterInFront {
        let chosen = document
            .history
            .number_at(step)
            .and_then(|number| self.visibility.volumes.get(&number));
        chosen.copied().unwrap_or(self.visibility.whole_body)
    }

    pub(crate) fn whole_body_choice(&self) -> MatterInFront {
        self.visibility.whole_body
    }

    /// How each face of the part is drawn: as the volume that made it, worked
    /// out once a frame since asking a step for its faces replays the history.
    pub(crate) fn face_modes(&self, document: &PartDocument) -> FaceModes {
        let made = volumes(document)
            .filter_map(|step| {
                let faces = document.faces_made_by(document.history.number_at(step)?);
                let mode = self.volume_mode(document, step);
                Some(faces.into_iter().map(move |face| (face, mode)))
            })
            .flatten()
            .collect();
        FaceModes {
            made,
            rest: self.whole_body_choice(),
        }
    }

    /// Whether no face of the part can be drawn at all.
    pub(crate) fn shows_nothing(&self) -> bool {
        let hidden = |mode: &MatterInFront| *mode == MatterInFront::Hidden;
        hidden(&self.visibility.whole_body) && self.visibility.volumes.values().all(hidden)
    }

    /// Opaque, see-through, invisible, and round again: the volume made at
    /// that step of the history fades out a step at a time.
    pub(crate) fn next_volume_mode(&mut self, document: &PartDocument, step: usize) {
        let mode = faded(self.volume_mode(document, step));
        if let Some(number) = document.history.number_at(step) {
            self.visibility.volumes.insert(number, mode);
        }
    }

    /// Draws every volume, and the faces none of them made, in that state.
    pub(crate) fn set_whole_body(&mut self, mode: MatterInFront) {
        self.visibility.volumes.clear();
        self.visibility.whole_body = mode;
    }

    /// The state every volume of the history shares, if they share one.
    pub(crate) fn shared_volume_mode(&self, document: &PartDocument) -> Option<MatterInFront> {
        let mut modes = volumes(document).map(|step| self.volume_mode(document, step));
        let first = modes.next().unwrap_or(self.visibility.whole_body);
        modes.all(|mode| mode == first).then_some(first)
    }
}

/// How the faces of the part are drawn, by the number of each.
pub(crate) struct FaceModes {
    made: BTreeMap<usize, MatterInFront>,
    /// What a face no volume made is drawn as.
    rest: MatterInFront,
}

impl FaceModes {
    pub(crate) fn of(&self, face: Option<usize>) -> MatterInFront {
        let made = face.and_then(|face| self.made.get(&face));
        made.copied().unwrap_or(self.rest)
    }

    /// Whether a face is drawn, and so met by a ray.
    pub(crate) fn shows(&self, face: usize) -> bool {
        self.of(Some(face)) != MatterInFront::Hidden
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
        let hidden = !active && !picking && !self.eye_lit(context.document, sketch);
        let here = |row: &Points| {
            row.sketch() == sketch && (!active || matches!(row, Points::Area { .. }))
        };
        let lit = [self.lit(), self.visibility.hovered].map(|row| row.filter(here));
        (!hidden || lit.iter().any(Option::is_some)).then_some(Seen { hidden, lit })
    }
}

fn faded(mode: MatterInFront) -> MatterInFront {
    match mode {
        MatterInFront::Opaque => MatterInFront::SeeThrough,
        MatterInFront::SeeThrough => MatterInFront::Hidden,
        MatterInFront::Hidden => MatterInFront::Opaque,
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

#[cfg(test)]
mod tests;
