//! What the part says of the sections it keeps, and of the drawings laid on
//! them.

use super::PartDocument;
use crate::history::{Operation, StepKind};
use crate::sections::SavedSection;

impl PartDocument {
    /// The sections the part keeps, in the order they were made, each found
    /// on the part as it stands now.
    pub fn sections(&self) -> Vec<SavedSection> {
        self.state.sections_standing()
    }

    /// The kept section a drawing was laid on, when it was laid on one.
    pub fn section_under(&self, sketch: usize) -> Option<usize> {
        match self.sketch_openings().nth(sketch)? {
            Operation::CreateSketchOnSection { section, .. } => Some(*section),
            _ => None,
        }
    }

    /// Whether a drawing stands on that section, which deleting it would leave
    /// following nothing more.
    pub fn is_stood_on(&self, section: usize) -> bool {
        self.sketch_openings().any(|opening| {
            matches!(opening, Operation::CreateSketchOnSection { section: under, .. } if *under == section)
        })
    }

    fn sketch_openings(&self) -> impl Iterator<Item = &Operation> {
        self.history
            .applied_operations()
            .iter()
            .filter(|operation| StepKind::opened_by(operation) == Some(StepKind::Sketch))
    }
}
