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

    /// Whether the part is read through the `rank`th box of the step reader.
    pub(crate) fn reads_through(&self, rank: usize) -> bool {
        self.reading_point.is_none_or(|last| rank <= last)
    }
}
