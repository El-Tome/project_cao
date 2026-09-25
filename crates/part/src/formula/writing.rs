//! Writing a formula back out, with only the parentheses it needs to read as
//! the same formula again.

use crate::variables::VariableId;

use super::{Formula, Operator};

pub(super) fn written(formula: &Formula, name: &impl Fn(VariableId) -> String) -> String {
    match formula {
        Formula::Number(number) => number.to_string(),
        Formula::Variable(variable) => name(*variable),
        Formula::Negative(inside) => format!("-{}", wrapped(inside, inside.binds() < SINGLE, name)),
        Formula::Combined(operator, left, right) => {
            let binds = operator.binds();
            format!(
                "{} {} {}",
                wrapped(left, left.binds() < binds, name),
                operator.sign(),
                // A second term of the same strength is wrapped too: without
                // the parentheses `a - (b - c)` would read back as `a - b - c`.
                wrapped(right, right.binds() <= binds, name),
            )
        }
    }
}

fn wrapped(formula: &Formula, wrap: bool, name: &impl Fn(VariableId) -> String) -> String {
    match wrap {
        true => format!("({})", written(formula, name)),
        false => written(formula, name),
    }
}

/// How tightly a piece of a formula holds together, weakest first.
const SUM: u8 = 1;
const PRODUCT: u8 = 2;
const SIGNED: u8 = 3;
const SINGLE: u8 = 4;

impl Operator {
    fn binds(self) -> u8 {
        match self {
            Self::Add | Self::Subtract => SUM,
            Self::Multiply | Self::Divide => PRODUCT,
        }
    }
}

impl Formula {
    fn binds(&self) -> u8 {
        match self {
            Self::Number(number) if *number < 0.0 => SIGNED,
            Self::Number(_) | Self::Variable(_) => SINGLE,
            Self::Negative(_) => SIGNED,
            Self::Combined(operator, ..) => operator.binds(),
        }
    }
}
