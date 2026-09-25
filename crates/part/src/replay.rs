//! Replaying a history into a part, and what the replay notes on the way: how
//! many elements each operation laid, which steps were raised from each
//! sketch, and when a value written from the variables left the drawing.
//!
//! A value that leaves the drawing — its dimension erased, the trait it
//! measured erased, dropped by a cut, typed again and taken — stops following
//! the variables from that moment: replayed against the table as it stands
//! now, the shape it once set would go on moving with a variable nothing on
//! the drawing shows. Which values went, and when, is only known once the part
//! has been replayed; the history is then replayed again with each of them
//! worked out against the table as it stood when it went — when that comes to
//! another number than the table as it stands.
//!
//! A value the table as it stands cannot give is refused where it is set, and
//! so is never seen leaving. Before anything is decided, it is tried against
//! the table it was written with: that shows whether it leaves, and where.

use std::collections::{HashMap, HashSet};

use crate::formula::Formula;
use crate::history::{History, Operation};
use crate::state::PartState;
use crate::variables::Variables;

/// Which value a replay set: the number of the operation, and the rank of the
/// value among those the operation set — a gesture sets several.
pub(crate) type SetBy = (u32, u32);

/// What a replay notes on the way.
#[derive(Clone, Debug, Default)]
pub(crate) struct Replay {
    /// How many values the operation being replayed has set so far.
    setting: u32,
    /// Every operation that edits a sketch, in the order they replayed.
    pub(crate) laid: Vec<Laid>,
    /// Every step of matter, by number, with the sketch it was raised from.
    pub(crate) raised: Vec<(u32, usize)>,
    /// Every value set from a calculation, with the calculation.
    written: HashMap<SetBy, Formula>,
    /// Those of them the drawing refused.
    refused: HashSet<SetBy>,
    /// The values written from the variables that left the drawing, with the
    /// number of the operation they went at.
    gone: HashMap<SetBy, u32>,
    /// What the variables came to, for each value that went, as it went.
    pub(crate) superseded: HashMap<SetBy, Vec<f64>>,
}

/// How many of each element — points, traits, circles, arcs, ellipses — a
/// sketch held before and after one operation that edits it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Laid {
    pub(crate) number: u32,
    pub(crate) sketch: usize,
    before: [usize; 5],
    after: [usize; 5],
}

impl Laid {
    /// How many of each element the operation laid.
    pub(crate) fn count(&self) -> [usize; 5] {
        std::array::from_fn(|kind| self.after[kind].saturating_sub(self.before[kind]))
    }
}

/// What a sketch holds at one moment of a replay: how many of each element,
/// and which values written from the variables stand on it.
struct Held {
    elements: [usize; 5],
    set: HashSet<SetBy>,
}

impl PartState {
    /// The part a history comes to: replayed once, and again when a value
    /// written from the variables left the drawing under other ones.
    pub fn rebuild(history: &History) -> Self {
        let once = Self::replayed(history, HashMap::new());
        let tried = (!once.replay.refused.is_empty()).then(|| {
            let mut tried = once.frozen(history);
            for set in &once.replay.refused {
                tried
                    .entry(*set)
                    .or_insert_with(|| table_before(history, set.0));
            }
            Self::replayed(history, tried)
        });
        let frozen = tried.as_ref().unwrap_or(&once).frozen(history);
        if frozen.is_empty() {
            return once;
        }
        Self::replayed(history, frozen)
    }

    /// Every value that left the drawing, with the table as it stood when it
    /// went — kept only where that table gives it another number than the
    /// table as it stands. The same calculation over the same numbers gives
    /// the very same number, so any difference at all is a drawing that would
    /// move.
    fn frozen(&self, history: &History) -> HashMap<SetBy, Vec<f64>> {
        self.replay
            .gone
            .iter()
            .filter_map(|(set, at)| {
                let then = table_before(history, *at);
                let moves = self
                    .replay
                    .written
                    .get(set)
                    .is_none_or(|formula| formula.value(&then) != formula.value(&self.values));
                moves.then_some((*set, then))
            })
            .collect()
    }

    fn replayed(history: &History, superseded: HashMap<SetBy, Vec<f64>>) -> Self {
        let mut state = Self::default();
        state.replay.superseded = superseded;
        state.read_variables(history);
        if history.first_size_is_off_the_drawing() {
            state.fix_a_unit_at_a_millimetre();
        }
        // Step by step, each step whole — not in the order things were typed.
        // A corner of a sketch dragged long after an extrusion was raised from
        // it is played with that sketch, so the extrusion is raised again.
        for (number, operation) in history.replay_order() {
            state.replaying = number;
            state.replay.setting = 0;
            let before = operation
                .edits()
                .and_then(|sketch| Some((sketch, state.held_by(sketch)?)));
            state.apply(operation);
            state.note(number, operation, before);
        }
        // What is applied from here on is applied live, not replayed.
        state.replaying = 0;
        state.note_broken_variables();
        state
    }

    /// Which value the replay sets next, and what the variables come to for
    /// it: as they stood when it went, or as they stand.
    pub(crate) fn next_value_set(&mut self) -> (SetBy, &[f64]) {
        let set = (self.replaying, self.replay.setting);
        self.replay.setting += 1;
        let values = self.replay.superseded.get(&set).unwrap_or(&self.values);
        (set, values)
    }

    /// Notes a value the replay set from a calculation, and whether the
    /// drawing took it.
    pub(crate) fn note_value_set(&mut self, set: SetBy, written: &Formula, held: bool) {
        if self.replaying == 0 || written.as_number().is_some() {
            return;
        }
        self.replay.written.insert(set, written.clone());
        if !held {
            self.replay.refused.insert(set);
        }
    }

    fn held_by(&self, sketch: usize) -> Option<Held> {
        let drawing = self.sketches.get(sketch)?;
        Some(Held {
            elements: [
                drawing.points().len(),
                drawing.segments().len(),
                drawing.circles().len(),
                drawing.arcs().len(),
                drawing.ellipses().len(),
            ],
            set: drawing
                .dimensions()
                .iter()
                .filter_map(|value| value.set_by)
                .collect(),
        })
    }

    fn note(&mut self, number: u32, operation: &Operation, before: Option<(usize, Held)>) {
        if let Operation::Extrude { sketch, .. } | Operation::Revolve { sketch, .. } = operation {
            self.replay.raised.push((number, *sketch));
        }
        let Some((sketch, before)) = before else {
            return;
        };
        let Some(after) = self.held_by(sketch) else {
            return;
        };
        self.replay.laid.push(Laid {
            number,
            sketch,
            before: before.elements,
            after: after.elements,
        });
        for went in before.set.difference(&after.set) {
            self.replay.gone.entry(*went).or_insert(number);
        }
    }
}

/// The variables as the changes made before the operation of that number
/// leave them.
fn table_before(history: &History, number: u32) -> Vec<f64> {
    let mut table = Variables::default();
    for change in history.variable_changes_before(number) {
        table.change(change);
    }
    table.values()
}
