//! The part as its volumes' states show it, and the raw block of the volume
//! marked, each replayed only when what it is replayed from changes: a frame
//! and a hover read them, and a replay costs far more than either can afford.

use std::collections::BTreeMap;
use std::rc::Rc;

use cao_part::{History, PartDocument, Shown, VolumeState};
use cao_solid::Body;

use super::super::ViewportState;

/// A part shown, with the history and the states it was replayed from. The
/// history is compared by value: the document keeps no revision of it, and
/// it is all the replay reads.
pub(super) struct Replayed {
    history: History,
    states: BTreeMap<u32, VolumeState>,
    shown: Rc<Shown>,
}

/// The raw block of the volume made by the step of that number, with the
/// history it was replayed from: nothing when that step raises nothing.
pub(super) struct Blocked {
    history: History,
    number: u32,
    block: Option<Rc<Body>>,
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

    /// The raw block of the volume marked: the matter its step raises or
    /// removes, the block it shows see-through, which for a step the kernel
    /// declined stands where it would have been joined to the part.
    pub(crate) fn marked_block(&self, document: &PartDocument) -> Option<Rc<Body>> {
        let number = document.history.number_at(self.marked_volume()?)?;
        let mut kept = self.visibility.block.borrow_mut();
        if let Some(blocked) = kept.as_ref()
            && blocked.number == number
            && blocked.history == document.history
        {
            return blocked.block.clone();
        }
        let see_through = BTreeMap::from([(number, VolumeState::SeeThrough)]);
        let block = document
            .shown(&see_through)
            .ghosts
            .into_iter()
            .find(|ghost| ghost.step == number)
            .map(|ghost| Rc::new(ghost.body));
        *kept = Some(Blocked {
            history: document.history.clone(),
            number,
            block: block.clone(),
        });
        block
    }
}
