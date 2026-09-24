//! What the panel of variables holds between frames: the row being edited,
//! the row that adds a variable, and what went wrong with the last thing
//! asked of it.

use cao_part::{
    Broken, Formula, PartDocument, Refused, Unusable, VariableChange, VariableId, Variables,
};
use cao_sketch::DimensionTarget;

/// What the panel refused, named for the view to say.
#[derive(Clone, Debug, PartialEq)]
pub enum Problem {
    /// The formula typed does not read.
    Formula(Unusable),
    /// The part refused the change.
    Refused(Refused),
}

/// One variable as its row shows it.
#[derive(Clone, Debug, PartialEq)]
pub struct Row {
    pub variable: VariableId,
    pub name: String,
    /// The formula, with the names the variables go by now.
    pub formula: String,
    /// What it comes to — nothing when it comes to no number.
    pub value: Option<f64>,
}

/// A row being typed into, not yet asked of the part.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Typed {
    pub name: String,
    pub formula: String,
}

/// The panel of variables, between frames.
#[derive(Default)]
pub struct VariablesPanel {
    /// The one row being typed into, and what its two fields hold.
    editing: Option<(VariableId, Typed)>,
    /// The row at the foot of the table, which adds a variable.
    pub adding: Typed,
    problem: Option<Problem>,
}

impl VariablesPanel {
    /// Every variable the part has now, in the order they were made.
    pub fn rows(&self, variables: &Variables) -> Vec<Row> {
        let values = variables.values();
        variables
            .live()
            .map(|(variable, found)| Row {
                variable,
                name: found.name().to_string(),
                formula: variables.written(found.formula()),
                value: values
                    .get(variable.0)
                    .copied()
                    .filter(|value| value.is_finite()),
            })
            .collect()
    }

    /// What a row's two fields show: what is being typed into it, or what the
    /// table holds.
    pub fn texts(&self, row: &Row) -> Typed {
        match &self.editing {
            Some((variable, typed)) if *variable == row.variable => typed.clone(),
            _ => Typed {
                name: row.name.clone(),
                formula: row.formula.clone(),
            },
        }
    }

    /// Takes what was typed into a row. Typing into another row drops what
    /// the one before held: only one is edited at a time.
    pub fn typed_into(&mut self, row: &Row, name: String, formula: String) {
        self.editing = Some((row.variable, Typed { name, formula }));
    }

    pub fn is_editing(&self, variable: VariableId) -> bool {
        self.editing
            .as_ref()
            .is_some_and(|(editing, _)| *editing == variable)
    }

    /// Asks the part for what the row being edited says. Returns true when the
    /// part changed; a row committed as it was records nothing.
    pub fn commit(&mut self, document: &mut PartDocument) -> bool {
        let Some((variable, typed)) = self.editing.take() else {
            return false;
        };
        let formula = match read(document.variables(), &typed.formula) {
            Ok(formula) => formula,
            Err(problem) => return self.refused(problem, Some((variable, typed))),
        };
        let name = typed.name.trim().to_string();
        let unchanged = document
            .variables()
            .get(variable)
            .is_some_and(|found| found.name() == name && *found.formula() == formula);
        if unchanged {
            self.problem = None;
            return false;
        }
        let change = VariableChange::Edited {
            variable,
            name,
            formula,
        };
        match document.change_variable(change) {
            Ok(()) => self.done(),
            Err(refused) => self.refused(Problem::Refused(refused), Some((variable, typed))),
        }
    }

    /// Drops what was typed into the row being edited.
    pub fn cancel(&mut self) {
        self.editing = None;
        self.problem = None;
    }

    /// Asks the part for the variable the foot of the table names.
    pub fn add(&mut self, document: &mut PartDocument) -> bool {
        let formula = match read(document.variables(), &self.adding.formula) {
            Ok(formula) => formula,
            Err(problem) => return self.refused(problem, None),
        };
        let change = VariableChange::Added {
            name: self.adding.name.trim().to_string(),
            formula,
        };
        match document.change_variable(change) {
            Ok(()) => {
                self.adding = Typed::default();
                self.done()
            }
            Err(refused) => self.refused(Problem::Refused(refused), None),
        }
    }

    pub fn erase(&mut self, document: &mut PartDocument, variable: VariableId) -> bool {
        match document.change_variable(VariableChange::Erased { variable }) {
            Ok(()) => {
                self.editing = None;
                self.done()
            }
            Err(refused) => self.refused(Problem::Refused(refused), None),
        }
    }

    /// What went wrong with the last thing asked, until something else is.
    pub fn problem(&self) -> Option<&Problem> {
        self.problem.as_ref()
    }

    /// The values on a drawing the last refusal said a change would break,
    /// for the drawing to show.
    pub fn breaking(&self) -> Vec<(usize, DimensionTarget)> {
        match &self.problem {
            Some(Problem::Refused(Refused::Breaks(broken))) => broken
                .iter()
                .filter_map(|broken| match broken {
                    Broken::Dimension { sketch, target } => Some((*sketch, *target)),
                    _ => None,
                })
                .collect(),
            _ => Vec::new(),
        }
    }

    fn done(&mut self) -> bool {
        self.problem = None;
        true
    }

    /// Keeps what was typed, so it can be fixed where it was typed.
    fn refused(&mut self, problem: Problem, editing: Option<(VariableId, Typed)>) -> bool {
        self.problem = Some(problem);
        if editing.is_some() {
            self.editing = editing;
        }
        false
    }
}

fn read(variables: &Variables, text: &str) -> Result<Formula, Problem> {
    variables
        .read(text)
        .map_err(|wrong| Problem::Formula(Unusable::Unreadable(wrong)))
}

#[cfg(test)]
mod tests;
