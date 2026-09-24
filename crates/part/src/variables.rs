//! The part's variables: named values every sketch and every step of the part
//! can write a size from.

use serde::{Deserialize, Serialize};

use crate::formula::{Formula, Unreadable};
use crate::formula::{goes_on_a_name, starts_a_name};

/// A variable, by its rank in the part's table.
///
/// A rank rather than a name: an erased variable keeps its place, so what was
/// written from it still reads the value it had, and a name given again never
/// answers for the one before.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct VariableId(pub usize);

/// One change to the table, as the history records it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum VariableChange {
    /// A variable named and written for the first time. It takes the next rank.
    Added { name: String, formula: Formula },
    /// A variable renamed or written again, both at once — the way its row is
    /// edited.
    Edited {
        variable: VariableId,
        name: String,
        formula: Formula,
    },
    /// A variable taken out of the table. Its rank stays taken, so what was
    /// written from it before still reads the value it last had.
    Erased { variable: VariableId },
}

/// One variable of the part.
#[derive(Clone, Debug, PartialEq)]
pub struct Variable {
    name: String,
    formula: Formula,
    erased: bool,
}

impl Variable {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn formula(&self) -> &Formula {
        &self.formula
    }
}

/// Why a size typed as text cannot be used.
#[derive(Clone, Debug, PartialEq)]
pub enum Unusable {
    /// It does not read as a formula.
    Unreadable(Unreadable),
    /// It reads, and comes to no number: a division by zero.
    NoNumber,
}

/// Why a variable may not go by a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NameProblem {
    Empty,
    /// A digit opens a number, not a name.
    OpensOnADigit,
    /// A character that is neither a letter, a digit nor `_`.
    NotAllowed(char),
    /// Another variable of the part already goes by it.
    Taken,
}

/// Every variable the part has had, in the order they were made.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Variables {
    all: Vec<Variable>,
}

impl Variables {
    /// The variables the part has now, in the order they were made.
    pub fn live(&self) -> impl Iterator<Item = (VariableId, &Variable)> {
        self.all
            .iter()
            .enumerate()
            .filter(|(_, variable)| !variable.erased)
            .map(|(rank, variable)| (VariableId(rank), variable))
    }

    /// Whether the part has this variable now.
    pub fn is_live(&self, variable: VariableId) -> bool {
        self.get(variable).is_some_and(|found| !found.erased)
    }

    /// Any variable the part has had, erased ones included.
    pub fn get(&self, variable: VariableId) -> Option<&Variable> {
        self.all.get(variable.0)
    }

    /// The variable going by this name now. An erased one goes by none.
    pub fn named(&self, name: &str) -> Option<VariableId> {
        self.live()
            .find(|(_, variable)| variable.name == name)
            .map(|(variable, _)| variable)
    }

    /// A formula as the user reads it, with the names the variables go by now.
    pub fn written(&self, formula: &Formula) -> String {
        formula.written(|variable| {
            self.get(variable)
                .map(|found| found.name.clone())
                .unwrap_or_else(|| format!("#{}", variable.0))
        })
    }

    /// Reads what the user typed against the names the table has now.
    pub fn read(&self, text: &str) -> Result<Formula, Unreadable> {
        Formula::read(text, |name| self.named(name))
    }

    /// Reads a size typed as text and works it out: the formula it was
    /// written as, and the number that comes to now.
    pub fn size_of(&self, text: &str) -> Result<(Formula, f64), Unusable> {
        let formula = self.read(text).map_err(Unusable::Unreadable)?;
        let value = formula.value(&self.values()).ok_or(Unusable::NoNumber)?;
        Ok((formula, value))
    }

    /// What every variable comes to, by rank — not a number where it comes to
    /// none: a division by zero, or a loop a file written by another hand
    /// could hold.
    pub fn values(&self) -> Vec<f64> {
        let mut values = vec![None; self.all.len()];
        for rank in 0..self.all.len() {
            self.work_out(VariableId(rank), &mut values, &mut Vec::new());
        }
        values
            .into_iter()
            .map(|value| value.unwrap_or(f64::NAN))
            .collect()
    }

    /// Works a variable out after everything it leans on, by rank. `going`
    /// is the run of variables being worked out on the way here: meeting one
    /// of them again is a loop, and a loop comes to nothing.
    fn work_out(
        &self,
        variable: VariableId,
        values: &mut [Option<f64>],
        going: &mut Vec<VariableId>,
    ) {
        if values[variable.0].is_some() || going.contains(&variable) {
            return;
        }
        let Some(formula) = self.all.get(variable.0).map(|found| &found.formula) else {
            return;
        };
        going.push(variable);
        for leaned_on in formula.variables() {
            if leaned_on.0 < values.len() {
                self.work_out(leaned_on, values, going);
            }
        }
        going.pop();
        let known: Vec<f64> = values
            .iter()
            .map(|value| value.unwrap_or(f64::NAN))
            .collect();
        values[variable.0] = Some(formula.value(&known).unwrap_or(f64::NAN));
    }

    /// Whether a variable may go by this name. `renaming` is the variable
    /// whose row is being edited, which may of course keep its own.
    pub fn check_name(&self, name: &str, renaming: Option<VariableId>) -> Result<(), NameProblem> {
        let mut characters = name.chars();
        let Some(first) = characters.next() else {
            return Err(NameProblem::Empty);
        };
        if first.is_ascii_digit() {
            return Err(NameProblem::OpensOnADigit);
        }
        if let Some(stray) = name.chars().find(|character| !goes_on_a_name(*character)) {
            return Err(NameProblem::NotAllowed(stray));
        }
        if !starts_a_name(first) {
            return Err(NameProblem::NotAllowed(first));
        }
        match self.named(name) {
            Some(taken) if Some(taken) != renaming => Err(NameProblem::Taken),
            _ => Ok(()),
        }
    }

    /// The loop writing `formula` into `variable` would close, from `variable`
    /// round to the last one before it comes back — nothing when it closes
    /// none.
    pub fn loop_through(&self, variable: VariableId, formula: &Formula) -> Option<Vec<VariableId>> {
        let mut path = vec![variable];
        self.comes_back(variable, formula, &mut path)
            .then_some(path)
    }

    /// Whether something `formula` leans on leads back to `to`, `path` holding
    /// the way there when it does.
    fn comes_back(&self, to: VariableId, formula: &Formula, path: &mut Vec<VariableId>) -> bool {
        for leaned_on in formula.variables() {
            if leaned_on == to {
                return true;
            }
            if path.contains(&leaned_on) {
                continue;
            }
            let Some(next) = self.get(leaned_on) else {
                continue;
            };
            path.push(leaned_on);
            if self.comes_back(to, &next.formula, path) {
                return true;
            }
            path.pop();
        }
        false
    }

    /// Plays one change of the history onto the table.
    pub fn change(&mut self, change: &VariableChange) {
        match change {
            VariableChange::Added { name, formula } => self.all.push(Variable {
                name: name.clone(),
                formula: formula.clone(),
                erased: false,
            }),
            VariableChange::Edited {
                variable,
                name,
                formula,
            } => {
                if let Some(edited) = self.all.get_mut(variable.0) {
                    edited.name = name.clone();
                    edited.formula = formula.clone();
                }
            }
            VariableChange::Erased { variable } => {
                if let Some(erased) = self.all.get_mut(variable.0) {
                    erased.erased = true;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests;
