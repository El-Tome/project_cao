//! The panel an edge of the part is chamfered or rounded from: which of the
//! two, the edge chosen, and the size typed.

use cao_part::history::BlendShape;
use cao_part::{Formula, Variables};
use cao_solid::EdgeName;

/// What the edge panel remembers between frames. Closed while `shape` is
/// nothing.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BlendState {
    pub shape: Option<BlendShape>,
    pub edge: Option<EdgeName>,
    /// The distance or the radius typed, in millimetres.
    pub size_input: String,
}

impl BlendState {
    /// Opens the panel on a chamfer or a fillet, or turns the one open into
    /// the other, keeping the edge chosen and the size typed.
    pub fn offer(&mut self, shape: BlendShape) {
        if !self.is_active() {
            self.size_input = "1".to_string();
        }
        self.shape = Some(shape);
    }

    pub fn is_active(&self) -> bool {
        self.shape.is_some()
    }

    pub fn close(&mut self) {
        *self = Self::default();
    }

    /// The size typed, as written and as what it comes to, when it is one an
    /// edge can be given.
    pub fn size(&self, variables: &Variables) -> Option<(Formula, f64)> {
        variables
            .size_of(&self.size_input)
            .ok()
            .filter(|(_, value)| *value > 1e-6)
    }

    /// Whether there is enough to apply: an edge chosen, and a usable size.
    pub fn is_ready(&self, variables: &Variables) -> bool {
        self.edge.is_some() && self.size(variables).is_some()
    }
}
