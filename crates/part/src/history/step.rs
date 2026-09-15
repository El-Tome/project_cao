//! One major step of a design: what it is, what it is raised from, and the
//! operations recorded under it.

use serde::{Deserialize, Serialize};

use super::Operation;

/// What kind of major step it is.
///
/// Written out by name, so a kind added later — a drilling, a section view —
/// costs no change of format: a part written before that kind existed names
/// only the kinds it knew, and goes on reading.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StepKind {
    Sketch,
    Extrusion,
    Revolution,
}

impl StepKind {
    /// The step this operation opens, when it opens one. Every other operation
    /// is recorded under the step already open.
    pub(crate) fn opened_by(operation: &Operation) -> Option<Self> {
        match operation {
            Operation::CreateSketch { .. } => Some(Self::Sketch),
            Operation::Extrude { .. } => Some(Self::Extrusion),
            Operation::Revolve { .. } => Some(Self::Revolution),
            _ => None,
        }
    }

    /// What the step's folder is called, before its number.
    pub(crate) fn folder(self) -> &'static str {
        match self {
            Self::Sketch => "sketch",
            Self::Extrusion => "extrusion",
            Self::Revolution => "revolution",
        }
    }
}

/// One major step of the design, as the index of a part file holds it.
///
/// What the step is made of is not in here: the index says what each step is
/// and what it hangs on, and the operations live in the step's own folder.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Step {
    number: u32,
    kind: StepKind,
    /// The step this one is raised from — the sketch an extrusion lifts, and
    /// later the face a sketch is drawn on.
    #[serde(default)]
    raised_from: Option<u32>,
    /// The numbers of the operations recorded under it, in the order they
    /// were done.
    operations: Vec<u32>,
}

impl Step {
    pub(crate) fn opened(number: u32, kind: StepKind, raised_from: Option<u32>) -> Self {
        Self {
            number,
            kind,
            raised_from,
            operations: Vec::new(),
        }
    }

    /// Handed out once and never again, even after the step is undone. It is
    /// what names the folder: a rank would move as soon as a step before it
    /// went, and every folder after the cut would have to be renamed.
    pub fn number(&self) -> u32 {
        self.number
    }

    pub fn kind(&self) -> StepKind {
        self.kind
    }

    pub fn raised_from(&self) -> Option<u32> {
        self.raised_from
    }

    pub fn operations(&self) -> &[u32] {
        &self.operations
    }

    /// How many operations are recorded under it.
    pub fn len(&self) -> usize {
        self.operations.len()
    }

    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// Where the step's operations live in the archive.
    pub(crate) fn folder(&self) -> String {
        format!("design/{}-{}/steps.json", self.kind.folder(), self.number)
    }

    pub(crate) fn record(&mut self, operation: u32) {
        self.operations.push(operation);
    }

    pub(crate) fn keep(&mut self, operations: usize) {
        self.operations.truncate(operations);
    }
}
