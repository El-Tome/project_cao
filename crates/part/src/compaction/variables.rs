//! The variables through a compaction: the live ones laid again, each after
//! those it leans on, and every formula said again in the ranks they land on.

use std::collections::BTreeMap;

use cao_sketch::Dimension;

use super::record;
use crate::formula::Formula;
use crate::history::{History, Operation};
use crate::state::PartState;
use crate::variables::{VariableChange, VariableId, Variables};

/// Where each variable kept landed in the compacted table, and what every
/// variable came to before it.
pub(super) struct Renumbered {
    ranks: BTreeMap<VariableId, VariableId>,
    values: Vec<f64>,
}

impl Renumbered {
    /// A formula said in the new ranks — or, when it leans on a variable that
    /// was not kept, the number it comes to now, which is what it was.
    pub(super) fn formula(&self, formula: &Formula) -> Formula {
        formula
            .renumbered(|old| self.ranks.get(&old).copied())
            .or_else(|| formula.value(&self.values).map(Formula::Number))
            .unwrap_or_else(|| formula.clone())
    }

    /// A value on a drawing as it was written: the formula it remembers, or
    /// the number itself.
    pub(super) fn dimension(&self, dimension: &Dimension) -> Formula {
        match dimension.written.as_deref().and_then(Formula::from_stored) {
            Some(formula) => self.formula(&formula),
            None => Formula::Number(dimension.value),
        }
    }
}

/// Lays the live variables down again, each after those it leans on, so that
/// every step of the compacted history reads a table that holds together.
pub(super) fn compact_variables(
    old: &Variables,
    new_history: &mut History,
    new_state: &mut PartState,
) -> Renumbered {
    let mut order = Vec::new();
    for (variable, _) in old.live() {
        lean_first(old, variable, &mut order, &mut Vec::new());
    }
    let renumbered = Renumbered {
        ranks: order
            .iter()
            .enumerate()
            .map(|(rank, old)| (*old, VariableId(rank)))
            .collect(),
        values: old.values(),
    };
    for variable in order {
        let Some(kept) = old.get(variable) else {
            continue;
        };
        let change = VariableChange::Added {
            name: kept.name().to_string(),
            formula: renumbered.formula(kept.formula()),
        };
        record(Operation::Variable(change), new_history, new_state);
    }
    renumbered
}

/// Puts a live variable in the order after every live one it leans on.
/// `going` holds the way here, so that a loop a file could hold ends rather
/// than turning forever.
fn lean_first(
    table: &Variables,
    variable: VariableId,
    order: &mut Vec<VariableId>,
    going: &mut Vec<VariableId>,
) {
    if !table.is_live(variable) || order.contains(&variable) || going.contains(&variable) {
        return;
    }
    going.push(variable);
    if let Some(found) = table.get(variable) {
        for leaned_on in found.formula().variables() {
            lean_first(table, leaned_on, order, going);
        }
    }
    going.pop();
    order.push(variable);
}
