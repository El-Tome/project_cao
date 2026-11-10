//! The part as it is shown once some of its volumes are made see-through or
//! invisible: the history replayed with each volume's state. A volume is its
//! raw solid — the block its step raised or removed, as extruded or turned —
//! whatever the steps after it did to the part.
//!
//! An opaque step is applied as it always is. An invisible step is not
//! applied at all: an addition brings no matter, a cut removes none. A
//! see-through addition brings no matter to the opaque part and its raw block
//! is a ghost; a see-through cut is applied, and the raw block it removes is a
//! ghost. A step left out still counts past the numbers it would have given
//! its faces, as a declined step does, so that the steps after it are raised
//! as they are in the part. A sketch takes the plane it has in the part: one
//! laid on a face of a volume left out, or whose face the part shown outlines
//! otherwise, stays where it is in the part. What a face of the part shown
//! answers to is read by where it lies, not by its number.

use std::collections::BTreeMap;

use cao_sketch::WorkPlane;
use cao_solid::Body;

use crate::document::PartDocument;
use crate::history::ExtrusionMode;
use crate::state::PartState;

/// How one volume of the part is shown.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VolumeState {
    Opaque,
    SeeThrough,
    Invisible,
}

/// The part shown: the opaque matter, and the raw block of every volume shown
/// see-through, in the order the steps replay.
#[derive(Clone, Debug)]
pub struct Shown {
    pub body: Body,
    pub ghosts: Vec<Ghost>,
}

/// The raw block of a volume shown see-through, by the number of its step.
#[derive(Clone, Debug)]
pub struct Ghost {
    pub step: u32,
    pub body: Body,
}

/// What a replay of the part shown carries besides the part: each volume's
/// state, by the number of its step, the plane each sketch has in the part,
/// and the ghosts laid so far. Empty for the part itself.
#[derive(Clone, Debug, Default)]
pub(crate) struct Showing {
    states: BTreeMap<u32, VolumeState>,
    planes: Vec<WorkPlane>,
    ghosts: Vec<Ghost>,
}

impl PartDocument {
    /// The part as it is shown with these states, a volume with none opaque.
    /// Replayed apart: the part, its warnings and its cache are left alone.
    pub fn shown(&self, states: &BTreeMap<u32, VolumeState>) -> Shown {
        let states: BTreeMap<u32, VolumeState> = states
            .iter()
            .filter(|(_, state)| **state != VolumeState::Opaque)
            .map(|(step, state)| (*step, *state))
            .collect();
        if states.is_empty() {
            return Shown {
                body: self.body().clone(),
                ghosts: Vec::new(),
            };
        }
        let replayed = PartState::rebuild_showing(
            &self.history,
            Showing {
                states,
                planes: self.sketches().iter().map(|sketch| sketch.plane).collect(),
                ghosts: Vec::new(),
            },
        );
        Shown {
            body: replayed.body,
            ghosts: replayed.showing.ghosts,
        }
    }
}

impl PartState {
    /// The plane the sketch of that rank has in the part, for the part shown
    /// to lay it on: a sketch on a face or a section of the part stays where
    /// it is in the part, whatever the part shown makes of what it stands on.
    /// Nothing for the part itself.
    pub(crate) fn plane_in_the_part(&self, sketch: usize) -> Option<WorkPlane> {
        self.showing.planes.get(sketch).copied()
    }

    /// Whether the step of matter being replayed is to be applied, with its
    /// tool: the part itself applies every one, the part shown leaves out an
    /// invisible step and a see-through addition, and keeps the tool of a
    /// see-through step as its ghost.
    pub(crate) fn shows(&mut self, tool: &Body, mode: ExtrusionMode) -> bool {
        let step = self.replaying;
        match self.showing.states.get(&step) {
            None | Some(VolumeState::Opaque) => true,
            Some(VolumeState::Invisible) => false,
            Some(VolumeState::SeeThrough) => {
                if !tool.is_empty() {
                    let body = tool.clone();
                    self.showing.ghosts.push(Ghost { step, body });
                }
                mode == ExtrusionMode::Cut
            }
        }
    }
}
