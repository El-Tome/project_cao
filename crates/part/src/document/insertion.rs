use crate::document::PartDocument;
use crate::history::{Operation, StepKind};
use crate::outcome::Outcome;

impl PartDocument {
    /// Records an operation made while the part is read through its
    /// `key_step`th key step in effect, counted as [`Self::shown_through`]
    /// counts them: one that opens a key step puts it in right after that
    /// one, and the steps after it replay on top. Anything else — a stroke in
    /// a sketch, a section — is recorded as [`Self::apply`] records it.
    pub fn insert_after(&mut self, key_step: usize, operation: Operation) -> Option<Outcome> {
        let Some(kind) = StepKind::opened_by(&operation).filter(|kind| kind.is_key()) else {
            return self.apply(operation);
        };
        self.history.insert_after(key_step, kind, operation);
        self.rebuild();
        None
    }
}
