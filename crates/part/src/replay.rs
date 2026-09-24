//! Replaying a history into a part, and what the replay notes on the way: how
//! many elements each operation laid, which steps were raised from each
//! sketch, and when a value written from the variables left the drawing.
//!
//! A value that leaves the drawing — its dimension erased, the trait it
//! measured erased, dropped by a cut, typed again and taken — stops following
//! the variables from that moment: replayed against the table as it stands
//! now, the shape it once set would go on moving with a variable nothing on
//! the drawing shows. Which values went, and when, is only known once the part
//! has been replayed; when the variables changed since, the history is
//! replayed a second time with each of them worked out against the table as
//! it stood when it went.

use std::collections::{HashMap, HashSet};

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
    /// The part a history comes to, replayed once — or twice, when a value
    /// written from the variables left the drawing before they last changed.
    pub fn rebuild(history: &History) -> Self {
        let once = Self::replayed(history, HashMap::new());
        let changes = history.variable_changes().len();
        let then: HashMap<SetBy, Vec<f64>> = once
            .replay
            .gone
            .iter()
            .filter_map(|(set, at)| {
                let before = history.variable_changes_before(*at);
                (before.len() < changes).then(|| {
                    let mut table = Variables::default();
                    for change in before {
                        table.change(change);
                    }
                    (*set, table.values())
                })
            })
            .collect();
        if then.is_empty() {
            return once;
        }
        Self::replayed(history, then)
    }

    fn replayed(history: &History, superseded: HashMap<SetBy, Vec<f64>>) -> Self {
        let mut state = Self::default();
        state.replay.superseded = superseded;
        state.read_variables(history);
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
