//! One major step of a design: what kind it is, and the operations recorded
//! under it.
//!
//! What the step stands on — the plane a sketch is drawn on, the sketch an
//! extrusion lifts — is not in here. It is already in the operation that opens
//! the step; where it goes in the file is `document::design`'s business.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use super::Operation;

/// What kind of major step it is.
///
/// Written out by name in the index, so a kind added later — a drilling, a
/// section — costs no change of format: a part written before that kind
/// existed names only the kinds it knew.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StepKind {
    Sketch,
    Extrusion,
    Revolution,
    /// A section kept in the part: a way of looking, which raises nothing.
    Section,
}

impl StepKind {
    /// The step this operation opens, when it opens one. Every other operation
    /// is recorded under the step already open.
    pub fn opened_by(operation: &Operation) -> Option<Self> {
        match operation {
            Operation::CreateSketch { .. } | Operation::CreateSketchOnSection { .. } => {
                Some(Self::Sketch)
            }
            Operation::Extrude { .. } => Some(Self::Extrusion),
            Operation::Revolve { .. } => Some(Self::Revolution),
            Operation::SaveSection(_) => Some(Self::Section),
            _ => None,
        }
    }

    /// What the step's folder is called, before its rank.
    pub(crate) fn folder(self) -> &'static str {
        match self {
            Self::Sketch => "sketch",
            Self::Extrusion => "extrusion",
            Self::Revolution => "revolution",
            Self::Section => "section",
        }
    }

    /// Whether a step of this kind is a key step of the part — a sketch, an
    /// extrusion, a revolution — which a section, raising nothing, is not.
    pub(crate) fn is_key(self) -> bool {
        self != Self::Section
    }

    /// Which of these steps an operation that opens none is recorded under:
    /// the sketch it edits, or the section it changes, by rank in its kind;
    /// the step of matter it changes, by the number of the operation that
    /// opened it.
    pub(crate) fn owner(steps: &[Step], operation: &Operation) -> Option<usize> {
        if let Operation::EditMatter { raised_by, .. } = operation {
            return steps
                .iter()
                .position(|step| step.operations().first() == Some(raised_by));
        }
        let (kind, rank) = match (operation.edits(), operation.edits_section()) {
            (Some(sketch), _) => (Self::Sketch, sketch),
            (None, Some(section)) => (Self::Section, section),
            (None, None) => return None,
        };
        kind.in_the_order_made(steps).get(rank).map(|(_, at)| *at)
    }

    /// The steps of this kind among `steps`, each as the number of the
    /// operation that opened it and its place, in the order they were made.
    ///
    /// That is the rank a sketch or a section is named by, and not its rank in
    /// the order of the part: a step put in among the others would otherwise
    /// take the name of the one it lands before, and every operation recorded
    /// on that one would replay on it. A step nothing has opened yet is none of
    /// them.
    pub(crate) fn in_the_order_made(self, steps: &[Step]) -> Vec<(u32, usize)> {
        let mut made: Vec<(u32, usize)> = steps
            .iter()
            .enumerate()
            .filter(|(_, step)| step.kind == self)
            .filter_map(|(at, step)| Some((*step.operations.first()?, at)))
            .collect();
        made.sort_unstable();
        made
    }
}

/// One major step of the design, as the index of a part file holds it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Step {
    kind: StepKind,
    /// The numbers of the operations recorded under it, in the order they
    /// were done. Never reused, so undo can walk them back in the order the
    /// user acted whatever the tree comes to look like.
    operations: Vec<u32>,
}

impl Step {
    pub(crate) fn opened(kind: StepKind) -> Self {
        Self {
            kind,
            operations: Vec::new(),
        }
    }

    /// A step as the index of a file names it.
    pub(crate) fn listed(kind: StepKind, operations: Vec<u32>) -> Self {
        Self { kind, operations }
    }

    pub fn kind(&self) -> StepKind {
        self.kind
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

    pub(crate) fn record(&mut self, operation: u32) {
        self.operations.push(operation);
    }

    /// Keeps only the operations still in the list, whatever their order.
    pub(crate) fn keep_only(&mut self, left: &BTreeSet<u32>) {
        self.operations.retain(|number| left.contains(number));
    }
}
