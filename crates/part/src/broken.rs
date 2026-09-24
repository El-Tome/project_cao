//! A size that does not hold once the part is rebuilt: what a change to the
//! variables is refused for, so that no size breaks without being seen.

use cao_sketch::DimensionTarget;

use crate::formula::Formula;
use crate::history::Operation;
use crate::state::PartState;
use crate::variables::VariableId;

/// A size that does not hold.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Broken {
    /// A value written on a drawing that the drawing does not take.
    Dimension {
        sketch: usize,
        target: DimensionTarget,
    },
    /// A step or a tool whose size comes to nothing it can use, by the number
    /// of the operation.
    Operation(u32),
    /// A variable that comes to no number: a division by zero.
    Variable(VariableId),
    /// An operation that would name other elements than it did, because
    /// something before it in its sketch lays another number of them — a
    /// pattern counted by a variable, say. By the number of the operation.
    Renumbered(u32),
    /// A sketch that would lose the face of the part it was laid on.
    Adrift(usize),
}

impl PartState {
    /// What a size comes to against the variables as they stand, noting the
    /// operation being replayed as broken when it comes to nothing it can use.
    pub(crate) fn size(&mut self, formula: &Formula, usable: impl Fn(f64) -> bool) -> Option<f64> {
        let value = formula.value(&self.values).filter(|value| usable(*value));
        if value.is_none() {
            self.broke(Broken::Operation(self.replaying));
        }
        value
    }

    pub(crate) fn broke(&mut self, broken: Broken) {
        if !self.broken.contains(&broken) {
            self.broken.push(broken);
        }
    }

    /// A value set again that holds this time: whatever broke on that target
    /// before no longer stands.
    pub(crate) fn held(&mut self, broken: Broken) {
        self.broken.retain(|noted| *noted != broken);
    }

    /// How many of each element the sketch an operation edits holds as the
    /// operation starts — what the ranks it names are counted against.
    pub(crate) fn note_ranks(&mut self, number: u32, operation: &Operation) {
        let Some(drawing) = operation
            .edits()
            .and_then(|sketch| self.sketches.get(sketch))
        else {
            return;
        };
        let held = [
            drawing.points().len(),
            drawing.segments().len(),
            drawing.circles().len(),
            drawing.arcs().len(),
            drawing.ellipses().len(),
        ];
        self.ranks.push((number, held));
    }

    /// What no longer holds here that held in `before`: a size, an operation
    /// that would name other elements than it did, a sketch that would lose
    /// its face. What did not hold already is not held against the change.
    pub(crate) fn broken_since(&self, before: &PartState) -> Vec<Broken> {
        let mut broken: Vec<Broken> = self
            .broken
            .iter()
            .filter(|size| !before.broken.contains(size))
            .copied()
            .collect();
        let was: std::collections::HashMap<u32, [usize; 5]> =
            before.ranks.iter().copied().collect();
        if let Some((number, _)) = self
            .ranks
            .iter()
            .find(|(number, held)| was.get(number).is_some_and(|then| then != held))
        {
            broken.push(Broken::Renumbered(*number));
        }
        broken.extend(
            self.adrift
                .difference(&before.adrift)
                .map(|sketch| Broken::Adrift(*sketch)),
        );
        broken
    }

    /// Every variable of the part that comes to no number.
    pub(crate) fn note_broken_variables(&mut self) {
        let nothing: Vec<VariableId> = self
            .variables
            .live()
            .filter(|(variable, _)| {
                !self
                    .values
                    .get(variable.0)
                    .is_some_and(|value| value.is_finite())
            })
            .map(|(variable, _)| variable)
            .collect();
        for variable in nothing {
            self.broke(Broken::Variable(variable));
        }
    }
}
