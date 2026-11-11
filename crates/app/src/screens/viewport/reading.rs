//! The point the part is read to, set from the step reader of the Part panel:
//! the canvas's own, never the history's nor saved, so that reading a part
//! undoes nothing and a part always opens read to its end.

use cao_part::history::{Operation, StepKind};
use cao_part::{Outcome, PartDocument};

use super::ViewportState;

/// The box the part is read to, by its rank among the reader's boxes, none
/// while it is read to its end; and whether a step put in has moved it since
/// the canvas last looked.
#[derive(Default)]
pub(super) struct Reading {
    pub(super) point: Option<usize>,
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

    /// Records an operation made on the part as it is read. One that opens a
    /// key step while the part is read to a box goes in right after that box,
    /// and the part is read to the new one, the rest still unread; anything
    /// else, or anything made while the part is read whole, is recorded as it
    /// always is.
    pub(crate) fn record_where_read(
        &mut self,
        document: &mut PartDocument,
        operation: Operation,
    ) -> Option<Outcome> {
        let Some(point) = self.reading.point else {
            return document.apply(operation);
        };
        let opens_a_box = StepKind::opened_by(&operation).is_some_and(StepKind::is_key);
        let outcome = document.insert_after(point, operation);
        if opens_a_box {
            self.reading.point = Some(point + 1);
            self.reading.moved = true;
        }
        outcome
    }

    /// Whether a step put in has moved the reading point since this was last
    /// asked: the panel, drawn before the gesture, has to catch up.
    pub(crate) fn reading_moved(&mut self) -> bool {
        std::mem::take(&mut self.reading.moved)
    }
}
