//! A size that does not hold once the part is rebuilt: what a change to the
//! variables is refused for, so that no size breaks without being seen.

use cao_sketch::DimensionTarget;

use crate::formula::Formula;
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
