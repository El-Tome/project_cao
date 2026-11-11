//! The point the part is read to, set from the step reader of the Part panel:
//! the canvas's own, never the history's nor saved, so that reading a part
//! undoes nothing and a part always opens read to its end.

use cao_part::PartDocument;
use cao_part::history::StepKind;

use super::ViewportState;
use crate::screens::part_tree::state::key_steps;

/// The box the part is read to, by its rank among the reader's boxes, none
/// while it is read to its end; and how many key steps the part had when it
/// was last looked at.
#[derive(Default)]
pub(super) struct Reading {
    pub(super) point: Option<usize>,
    key_steps: usize,
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
        key_steps(document)
            .enumerate()
            .filter(|(_, (_, _, kind))| *kind == StepKind::Sketch)
            .nth(sketch)
            .is_none_or(|(rank, _)| self.reads_through(rank))
    }

    /// Follows the part: a key step added since it was last looked at brings
    /// it back to its end, where the new step is, until a step can be added
    /// at the reading point (#737). One taken away moves nothing. Says whether
    /// the reading point moved.
    pub(crate) fn keep_reading_up_with(&mut self, document: &PartDocument) -> bool {
        let key_steps = key_steps(document).count();
        let added = key_steps > self.reading.key_steps;
        self.reading.key_steps = key_steps;
        added && self.reading.point.take().is_some()
    }
}
