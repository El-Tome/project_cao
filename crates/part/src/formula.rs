//! A size as the user wrote it: a number, or a small calculation over the
//! part's variables — the four operations, parentheses, numbers and names.
//!
//! A formula names a variable by its rank in the part's table rather than by
//! its name, so renaming a variable renames it in every formula that uses it,
//! and a name given again after an erasing never answers for the one before.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::variables::VariableId;

mod reading;
mod writing;

use reading::Naming;
pub use reading::{goes_on_a_name, starts_a_name};

/// A number, or a calculation over the part's variables.
#[derive(Clone, Debug, PartialEq)]
pub enum Formula {
    Number(f64),
    Variable(VariableId),
    Negative(Box<Formula>),
    Combined(Operator, Box<Formula>, Box<Formula>),
}

/// One of the four operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operator {
    fn signed(sign: char) -> Option<Self> {
        match sign {
            '+' => Some(Self::Add),
            '-' => Some(Self::Subtract),
            '*' => Some(Self::Multiply),
            '/' => Some(Self::Divide),
            _ => None,
        }
    }

    fn sign(self) -> char {
        match self {
            Self::Add => '+',
            Self::Subtract => '-',
            Self::Multiply => '*',
            Self::Divide => '/',
        }
    }
}

impl Formula {
    /// Reads what the user typed. `names` says which variable a name is.
    pub fn read(
        text: &str,
        names: impl Fn(&str) -> Option<VariableId>,
    ) -> Result<Self, Unreadable> {
        reading::read(text, Naming::ByName(names))
    }

    /// Writes the formula out the way the user reads it, each variable under
    /// the name `name` gives it.
    pub fn written(&self, name: impl Fn(VariableId) -> String) -> String {
        writing::written(self, &name)
    }

    /// Writes the formula out the way a part file keeps it, each variable by
    /// its rank: the name can change, the rank cannot.
    pub fn stored(&self) -> String {
        self.written(|variable| format!("#{}", variable.0))
    }

    /// Reads back what [`Formula::stored`] wrote.
    pub fn from_stored(text: &str) -> Option<Self> {
        reading::read(text, Naming::<fn(&str) -> Option<VariableId>>::ByRank).ok()
    }

    /// What the formula comes to as a count: a whole number, and never below
    /// zero. A hair off a whole number is that number — a count worked out from
    /// variables lands there, not on it.
    pub fn whole(&self, values: &[f64]) -> Option<usize> {
        let value = self.value(values)?;
        let rounded = value.round();
        ((value - rounded).abs() < 1e-6 && rounded >= 0.0).then_some(rounded as usize)
    }

    /// What a value on a drawing keeps of the formula it was written from: its
    /// stored form, or nothing for a plain number, which the value says on
    /// its own.
    pub fn note(&self) -> Option<String> {
        self.as_number().is_none().then(|| self.stored())
    }

    /// The number it is, when it is a plain number rather than a calculation.
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Self::Number(number) => Some(*number),
            _ => None,
        }
    }

    /// The same size the other way round — a plain number stays one, and a
    /// size turned twice is the size it was.
    pub fn negated(self) -> Self {
        match self {
            Self::Number(number) => Self::Number(-number),
            Self::Negative(inside) => *inside,
            written => Self::Negative(Box::new(written)),
        }
    }

    /// The same size taken so many times over — a plain number stays one.
    pub fn times(self, factor: f64) -> Self {
        match self {
            Self::Number(number) => Self::Number(number * factor),
            written => Self::Combined(
                Operator::Multiply,
                Box::new(written),
                Box::new(Self::Number(factor)),
            ),
        }
    }

    /// The same formula with every variable said by another rank — nothing
    /// when one of them has none to go by.
    pub fn renumbered(&self, rank: impl Fn(VariableId) -> Option<VariableId>) -> Option<Self> {
        self.renumbered_by(&rank)
    }

    fn renumbered_by(&self, rank: &impl Fn(VariableId) -> Option<VariableId>) -> Option<Self> {
        Some(match self {
            Self::Number(number) => Self::Number(*number),
            Self::Variable(variable) => Self::Variable(rank(*variable)?),
            Self::Negative(inside) => Self::Negative(Box::new(inside.renumbered_by(rank)?)),
            Self::Combined(operator, left, right) => Self::Combined(
                *operator,
                Box::new(left.renumbered_by(rank)?),
                Box::new(right.renumbered_by(rank)?),
            ),
        })
    }

    /// The variables it is written from, each once, in the order they are
    /// read.
    pub fn variables(&self) -> Vec<VariableId> {
        let mut found = Vec::new();
        self.gather(&mut found);
        found
    }

    fn gather(&self, found: &mut Vec<VariableId>) {
        match self {
            Self::Number(_) => {}
            Self::Variable(variable) => {
                if !found.contains(variable) {
                    found.push(*variable);
                }
            }
            Self::Negative(inside) => inside.gather(found),
            Self::Combined(_, left, right) => {
                left.gather(found);
                right.gather(found);
            }
        }
    }

    /// What the formula comes to, given what every variable comes to.
    ///
    /// Nothing when it does not come to a number at all — a division by zero,
    /// or a variable that comes to nothing itself.
    pub fn value(&self, values: &[f64]) -> Option<f64> {
        let value = match self {
            Self::Number(number) => *number,
            Self::Variable(variable) => *values.get(variable.0)?,
            Self::Negative(inside) => -inside.value(values)?,
            Self::Combined(operator, left, right) => {
                let (left, right) = (left.value(values)?, right.value(values)?);
                match operator {
                    Operator::Add => left + right,
                    Operator::Subtract => left - right,
                    Operator::Multiply => left * right,
                    Operator::Divide => left / right,
                }
            }
        };
        value.is_finite().then_some(value)
    }
}

impl From<f64> for Formula {
    fn from(number: f64) -> Self {
        Self::Number(number)
    }
}

/// What a part file holds of a formula: a plain number as the number it is,
/// anything else as the text [`Formula::stored`] writes.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Kept {
    Number(f64),
    Written(String),
}

impl Serialize for Formula {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Number(number) => Kept::Number(*number),
            _ => Kept::Written(self.stored()),
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Formula {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match Kept::deserialize(deserializer)? {
            Kept::Number(number) => Ok(Self::Number(number)),
            Kept::Written(text) => {
                Self::from_stored(&text).ok_or_else(|| serde::de::Error::custom(Garbled(text)))
            }
        }
    }
}

/// A formula a part file holds that cannot be read back.
#[derive(Debug, thiserror::Error)]
#[error("a part file holds a formula that does not read: {0}")]
struct Garbled(String);

/// Why a formula does not read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Unreadable {
    /// There is nothing to read.
    Empty,
    /// A name the part has no variable for.
    UnknownName(String),
    /// A character no formula is written with.
    StrayCharacter(char),
    /// A value was wanted and this stood there instead — nothing at all when
    /// the formula ends before it.
    MissingValue(Option<char>),
    /// Two values side by side, with no operation between them.
    MissingOperator,
    /// A parenthesis opened and never closed.
    Unclosed,
    /// A parenthesis closed that was never opened.
    Unopened,
}

#[cfg(test)]
mod tests;
