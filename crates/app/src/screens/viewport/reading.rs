//! The point the part is read to, set from the step reader of the Part panel:
//! the canvas's own, never the history's nor saved, so that reading a part
//! undoes nothing and a part always opens read to its end.

use cao_part::history::{Operation, StepKind};
use cao_part::{Outcome, PartDocument};

use super::ViewportState;

/// The box the part is read to, by its rank among the reader's boxes, none
/// while it is read to its end; the boxes as the canvas last saw them, each
/// by the number of the operation opening its step, which is what the point
/// follows its box by; and whether a step put in has moved it since.
#[derive(Default)]
pub(super) struct Reading {
    pub(super) point: Option<usize>,
    boxes: Vec<u32>,
    moved: bool,
}

impl ViewportState {
    /// Reads the part to the `rank`th of the step reader's `boxes`, that one
    /// included; the last box, or any past it, reads it to its end, as unread.
    pub(crate) fn read_to(&mut self, rank: usize, boxes: usize) {
        if boxes == 0 {
            return;
        }
        self.reading.point = (rank + 1 < boxes).then_some(rank);
    }

    /// ◀: reads the part to the box before the one it is read to, among
    /// `boxes`, from the last while it is read whole; never before the first.
    pub(crate) fn read_previous(&mut self, boxes: usize) {
        let Some(last) = boxes.checked_sub(1) else {
            return;
        };
        let at = self.reading.point.map_or(last, |at| at.min(last));
        self.reading.point = Some(at.saturating_sub(1));
    }

    /// ▶: reads the part to the box after the one it is read to, among
    /// `boxes`; reaching the last reads it whole.
    pub(crate) fn read_next(&mut self, boxes: usize) {
        let (Some(at), Some(last)) = (self.reading.point, boxes.checked_sub(1)) else {
            return;
        };
        self.reading.point = (at + 1 < last).then_some(at + 1);
    }

    /// Whether the part is read through the `rank`th box of the step reader.
    pub(crate) fn reads_through(&self, rank: usize) -> bool {
        self.reading.point.is_none_or(|last| rank <= last)
    }

    /// Whether the part is read through the step that opened that sketch.
    pub(crate) fn reads_through_sketch(&self, document: &PartDocument, sketch: usize) -> bool {
        document
            .history
            .key_step_of_sketch(sketch)
            .is_none_or(|rank| self.reads_through(rank))
    }

    /// The sketches the part is read through: the ones a step can stand on,
    /// whatever lies past the box read to.
    pub(crate) fn sketches_read(&self, document: &PartDocument) -> Vec<usize> {
        (0..document.sketches().len())
            .filter(|sketch| self.reads_through_sketch(document, *sketch))
            .collect()
    }

    /// Records an operation made on the part as it is read. One that opens a
    /// key step while the part is read to a box goes in right after that box,
    /// and the part is read to the new one, the rest still unread — a step of
    /// matter whose sketch stands past that box going right after its sketch
    /// instead, never before it. A sketch laid on a kept section's cut goes
    /// at the end, after the section, and the part is read whole. Anything
    /// else, or anything made while the part is read whole, is recorded as it
    /// always is.
    pub(crate) fn record_where_read(
        &mut self,
        document: &mut PartDocument,
        operation: Operation,
    ) -> Option<Outcome> {
        let Some(read) = self.reading.point else {
            return document.apply(operation);
        };
        if matches!(operation, Operation::CreateSketchOnSection { .. }) {
            self.reading.point = None;
            self.reading.moved = true;
            return document.apply(operation);
        }
        let point = match &operation {
            Operation::Extrude { sketch, .. } | Operation::Revolve { sketch, .. } => {
                let own = document.history.key_step_of_sketch(*sketch);
                own.map_or(read, |own| own.max(read))
            }
            _ => read,
        };
        let opens_a_box = StepKind::opened_by(&operation).is_some_and(StepKind::is_key);
        let outcome = document.insert_after(point, operation);
        if opens_a_box {
            self.reading.point = Some(point + 1);
            self.reading.boxes = boxes_of(document);
            self.reading.moved = true;
        }
        outcome
    }

    /// Follows the part as steps are taken back and brought back: the point
    /// stays on its box whatever goes or comes before or after it, and goes to
    /// the box before when its own goes — the part read whole when none is
    /// left before it. Says whether the point moved since this was last asked,
    /// a step put in included: the panel, drawn before, has to catch up.
    pub(crate) fn keep_reading_up_with(&mut self, document: &PartDocument) -> bool {
        let moved = std::mem::take(&mut self.reading.moved);
        let boxes = boxes_of(document);
        if boxes == self.reading.boxes {
            return moved;
        }
        let before = std::mem::replace(&mut self.reading.boxes, boxes);
        // A point set before the canvas first looked was set against the boxes
        // the reader shows now, and has nothing to follow.
        let Some(point) = self.reading.point.filter(|_| !before.is_empty()) else {
            return moved;
        };
        let followed = before[..before.len().min(point + 1)]
            .iter()
            .rev()
            .find_map(|number| self.reading.boxes.iter().position(|now| now == number));
        self.reading.point = followed;
        moved || followed != Some(point)
    }
}

/// The reader's boxes, in the order of the part, each by the number of the
/// operation opening its step.
fn boxes_of(document: &PartDocument) -> Vec<u32> {
    let history = &document.history;
    history
        .key_steps()
        .filter_map(|(_, opened_at)| history.number_at(opened_at))
        .collect()
}
