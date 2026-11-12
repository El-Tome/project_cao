//! The panel an edge of the part is chamfered or rounded from: which of the
//! two, the edge chosen, and the size typed.

use cao_part::history::{BlendShape, Operation};
use cao_part::{Formula, Outcome, PartDocument, Variables};
use cao_prefs::Command;
use cao_solid::{Declined, EdgeName};

/// What the edge panel remembers between frames. Closed while `shape` is
/// nothing.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BlendState {
    pub shape: Option<BlendShape>,
    pub edge: Option<EdgeName>,
    /// The distance or the radius typed, in millimetres.
    pub size_input: String,
    /// Why the kernel declined the last « Appliquer », until another edge
    /// is chosen.
    pub declined: Option<Declined>,
}

impl BlendState {
    /// Opens the panel on a chamfer or a fillet, or turns the one open into
    /// the other, keeping the edge chosen and the size typed.
    pub fn offer(&mut self, shape: BlendShape) {
        if !self.is_active() {
            self.size_input = "1".to_string();
        }
        self.shape = Some(shape);
        self.declined = None;
    }

    pub fn choose(&mut self, edge: EdgeName) {
        self.edge = Some(edge);
        self.declined = None;
    }

    /// The command a pressed chord stands for: the one apply chord applies
    /// the blend while the edge panel is open, the extrusion otherwise.
    pub fn answering(&self, command: Command) -> Command {
        match command {
            Command::ExtrusionApply if self.is_active() => Command::BlendApply,
            other => other,
        }
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

/// Chamfers or rounds the chosen edge and closes the panel. An edge the kernel
/// declines records nothing and leaves the panel open, saying why. True when
/// the part changed.
pub fn apply_blend(document: &mut PartDocument, blend: &mut BlendState) -> bool {
    let (Some(shape), Some(edge), Some((size, _))) = (
        blend.shape,
        blend.edge.clone(),
        blend.size(document.variables()),
    ) else {
        return false;
    };
    if let Some(Outcome::BlendDeclined(reason)) =
        document.apply(Operation::Blend { edge, shape, size })
    {
        blend.declined = Some(reason);
        return false;
    }
    blend.close();
    true
}
