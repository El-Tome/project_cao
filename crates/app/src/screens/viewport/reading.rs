//! The point the part is read to, set from the step reader of the Part panel:
//! the canvas's own, never the history's nor saved, so that reading a part
//! undoes nothing and a part always opens read to its end.

use super::ViewportState;

impl ViewportState {
    /// Reads the part to the `rank`th box of the step reader, that one
    /// included; the last box, or any past it, reads it whole.
    pub(crate) fn read_to(&mut self, rank: usize) {
        self.reading_point = Some(rank);
    }

    /// ◀: reads the part to the box before the one it is read to, among
    /// `boxes`, from the last while it is read whole; never before the first.
    pub(crate) fn read_previous(&mut self, boxes: usize) {
        let Some(last) = boxes.checked_sub(1) else {
            return;
        };
        let at = self.reading_point.map_or(last, |at| at.min(last));
        self.reading_point = Some(at.saturating_sub(1));
    }

    /// ▶: reads the part to the box after the one it is read to, among
    /// `boxes`; reaching the last reads it whole.
    pub(crate) fn read_next(&mut self, boxes: usize) {
        let (Some(at), Some(last)) = (self.reading_point, boxes.checked_sub(1)) else {
            return;
        };
        self.reading_point = (at + 1 < last).then_some(at + 1);
    }

    /// Whether the part is read through the `rank`th box of the step reader.
    pub(crate) fn reads_through(&self, rank: usize) -> bool {
        self.reading_point.is_none_or(|last| rank <= last)
    }
}
