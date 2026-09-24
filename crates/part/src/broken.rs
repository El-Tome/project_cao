//! A size that does not hold once the part is rebuilt: what a change to the
//! variables is refused for, so that no size breaks without being seen.

use std::collections::HashMap;

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
    /// An operation that would lay another number of elements — a pattern
    /// counted by a variable, say — while another follows it in its sketch or
    /// is raised from that sketch, and names elements by their rank. By the
    /// numbers of the two operations.
    Renumbered { changed: u32, followed_by: u32 },
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

    /// What no longer holds here that held in `before`: a size, an operation
    /// that would lay another number of elements under what follows it, a
    /// sketch that would lose its face. What did not hold already is not held
    /// against the change.
    pub(crate) fn broken_since(&self, before: &PartState) -> Vec<Broken> {
        let mut broken: Vec<Broken> = self
            .broken
            .iter()
            .filter(|size| !before.broken.contains(size))
            .copied()
            .collect();
        broken.extend(self.renumbered_since(before));
        broken.extend(
            self.adrift
                .difference(&before.adrift)
                .map(|sketch| Broken::Adrift(*sketch)),
        );
        broken
    }

    /// The first operation that lays another number of elements than it did
    /// in `before`, when anything follows it: whatever comes after it in its
    /// sketch, or is raised from that sketch, names elements by their rank,
    /// and the ranks after it would all move. One the change breaks is named
    /// for that already — and only that one: a chamfer short of a corner still
    /// cuts the others, and one short of a corner before the change is not
    /// named again.
    fn renumbered_since(&self, before: &PartState) -> Option<Broken> {
        let named = |number: u32| {
            let broken = Broken::Operation(number);
            self.broken.contains(&broken) && !before.broken.contains(&broken)
        };
        let was: HashMap<u32, [usize; 5]> = before
            .replay
            .laid
            .iter()
            .map(|laid| (laid.number, laid.count()))
            .collect();
        let laid = &self.replay.laid;
        laid.iter().enumerate().find_map(|(at, changed)| {
            if was
                .get(&changed.number)
                .is_none_or(|count| *count == changed.count())
                || named(changed.number)
            {
                return None;
            }
            let next_in_its_sketch = laid[at + 1..]
                .iter()
                .find(|later| later.sketch == changed.sketch)
                .map(|later| later.number);
            let raised_from_it = || {
                self.replay
                    .raised
                    .iter()
                    .find(|(_, sketch)| *sketch == changed.sketch)
                    .map(|(number, _)| *number)
            };
            let followed_by = next_in_its_sketch.or_else(raised_from_it)?;
            Some(Broken::Renumbered {
                changed: changed.number,
                followed_by,
            })
        })
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
