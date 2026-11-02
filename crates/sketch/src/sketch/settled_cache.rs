//! The last reading of which points can no longer move, kept against a print
//! of the drawing it was read from.

use super::Sketch;

impl Sketch {
    /// The verdict remembered for this print, if it is the one still standing.
    ///
    /// The cell never leaves this method: a borrow held across a reading would
    /// meet the one taken to record the answer, and that is a panic.
    pub(crate) fn settled_read_from(&self, print: u64) -> Option<Vec<bool>> {
        match self.settled.borrow().as_ref() {
            Some((was, verdict)) if *was == print => Some(verdict.clone()),
            _ => None,
        }
    }

    pub(crate) fn remember_settled(&self, print: u64, verdict: Vec<bool>) {
        *self.settled.borrow_mut() = Some((print, verdict));
    }

    #[cfg(test)]
    pub(crate) fn forget_what_is_settled(&self) {
        *self.settled.borrow_mut() = None;
    }
}
