//! The part as its volumes' states show it, replayed only when what it is
//! replayed from changes: a frame and a hover read it, and a replay costs far
//! more than either can afford.

use std::collections::BTreeMap;
use std::rc::Rc;

use cao_part::{History, PartDocument, Shown, VolumeState};

use super::super::ViewportState;

/// A part shown, with the history and the states it was replayed from. The
/// history is compared by value: the document keeps no revision of it, and
/// it is all the replay reads.
pub(super) struct Replayed {
    history: History,
    states: BTreeMap<u32, VolumeState>,
    shown: Rc<Shown>,
}

impl ViewportState {
    /// The part as its volumes' states show it, or nothing while every volume
    /// of the document is opaque, when the part shown is the part itself.
    pub(crate) fn part_shown(&self, document: &PartDocument) -> Option<Rc<Shown>> {
        let states = self.volume_states(document);
        if states.is_empty() {
            return None;
        }
        let mut kept = self.visibility.shown.borrow_mut();
        if let Some(replayed) = kept.as_ref()
            && replayed.states == states
            && replayed.history == document.history
        {
            return Some(Rc::clone(&replayed.shown));
        }
        let shown = Rc::new(document.shown(&states));
        *kept = Some(Replayed {
            history: document.history.clone(),
            states,
            shown: Rc::clone(&shown),
        });
        Some(shown)
    }
}
