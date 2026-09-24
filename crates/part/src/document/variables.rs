//! Changing the part's variables: the one way in, and everything it refuses
//! before a thing is applied — a name that does not do, a loop, a variable
//! something still uses, a change that would leave a size not holding.

use cao_sketch::DimensionTarget;

use super::PartDocument;
use crate::broken::Broken;
use crate::formula::Formula;
use crate::history::Operation;
use crate::state::PartState;
use crate::variables::{NameProblem, VariableChange, VariableId, Variables};

/// Why a change to the variables was refused. Nothing is applied when it is.
#[derive(Clone, Debug, PartialEq)]
pub enum Refused {
    /// The name does not do.
    Name(NameProblem),
    /// The formula would have the variable lean on itself, round these.
    Loop(Vec<VariableId>),
    /// The variable is still used by these: they are rewritten first.
    InUse(Vec<Use>),
    /// These sizes would no longer hold.
    Breaks(Vec<Broken>),
    /// The variable is not one the part has.
    Gone,
}

/// Something written from a variable.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Use {
    /// Another variable.
    Variable(VariableId),
    /// A value on a drawing.
    Dimension {
        sketch: usize,
        target: DimensionTarget,
    },
    /// A step or a tool, by the number of the operation.
    Operation(u32),
}

impl PartDocument {
    /// The part's variables, as the history in effect leaves them.
    pub fn variables(&self) -> &Variables {
        &self.state.variables
    }

    /// The formula a value on the drawing was written from, with the names
    /// the variables go by now — nothing for a value written as a plain
    /// number.
    pub fn formula_of(&self, sketch: usize, target: DimensionTarget) -> Option<String> {
        let note = self
            .sketches()
            .get(sketch)?
            .dimension_of(target)?
            .written
            .as_deref()?;
        let formula = Formula::from_stored(note)?;
        Some(self.state.variables.written(&formula))
    }

    /// Changes the variables, as one step of the history, and rebuilds the
    /// part from them — or says why not, and changes nothing.
    pub fn change_variable(&mut self, change: VariableChange) -> Result<(), Refused> {
        self.may_change(&change)?;
        let mut trial = self.history.clone();
        trial.push(Operation::Variable(change));
        let before = PartState::rebuild(&self.history).broken;
        let after = PartState::rebuild(&trial);
        let breaks: Vec<Broken> = after
            .broken
            .iter()
            .filter(|broken| !before.contains(broken))
            .copied()
            .collect();
        if !breaks.is_empty() {
            return Err(Refused::Breaks(breaks));
        }
        self.history = trial;
        self.state = after;
        Ok(())
    }

    fn may_change(&self, change: &VariableChange) -> Result<(), Refused> {
        let variables = self.variables();
        let live = |variable: VariableId| variables.live().any(|(found, _)| found == variable);
        match change {
            VariableChange::Added { name, .. } => {
                variables.check_name(name, None).map_err(Refused::Name)
            }
            VariableChange::Edited {
                variable,
                name,
                formula,
            } => {
                if !live(*variable) {
                    return Err(Refused::Gone);
                }
                variables
                    .check_name(name, Some(*variable))
                    .map_err(Refused::Name)?;
                match variables.loop_through(*variable, formula) {
                    Some(round) => Err(Refused::Loop(round)),
                    None => Ok(()),
                }
            }
            VariableChange::Erased { variable } => {
                if !live(*variable) {
                    return Err(Refused::Gone);
                }
                let uses = self.uses_of(*variable);
                match uses.is_empty() {
                    true => Ok(()),
                    false => Err(Refused::InUse(uses)),
                }
            }
        }
    }

    /// Everything the part writes from a variable now: the other variables,
    /// the values on its drawings, and the steps and tools in effect.
    pub fn uses_of(&self, variable: VariableId) -> Vec<Use> {
        let mut uses: Vec<Use> = self
            .variables()
            .live()
            .filter(|(_, other)| other.formula().variables().contains(&variable))
            .map(|(other, _)| Use::Variable(other))
            .collect();
        for (sketch, drawing) in self.sketches().iter().enumerate() {
            for dimension in drawing.dimensions() {
                let leans = dimension
                    .written
                    .as_deref()
                    .and_then(Formula::from_stored)
                    .is_some_and(|formula| formula.variables().contains(&variable));
                if leans {
                    uses.push(Use::Dimension {
                        sketch,
                        target: dimension.target,
                    });
                }
            }
        }
        for (number, operation) in self.history.replay_order() {
            let leans = operation
                .sizes()
                .iter()
                .any(|size| size.variables().contains(&variable));
            if leans {
                uses.push(Use::Operation(number));
            }
        }
        uses
    }
}
