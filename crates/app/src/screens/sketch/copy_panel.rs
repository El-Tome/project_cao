use cao_sketch::{ChosenAxis, Element, PointId, Selection, ToolState};
use glam::DVec2;

use super::{LiveInput, SketchEditor, Tool};

/// A slot of the panel the mirror and the patterns gather what they need in,
/// top to bottom.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CopySlot {
    #[default]
    Elements,
    /// The mirror's axis, a pattern's centre or direction.
    Target,
    /// The field of that rank among the tool's values.
    Value(usize),
    Validate,
}

/// What fills the target slot: an axis for the mirror and the rectangular
/// pattern, a point of the drawing for the circular one.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CopyTarget {
    Axis(ChosenAxis),
    Centre(PointId),
}

#[derive(Debug, Default)]
pub struct CopyPanel {
    active: CopySlot,
    marked: Option<CopySlot>,
    elements: Vec<Element>,
    target: Option<CopyTarget>,
    values: usize,
    /// The box being pulled across the drawing while Elements is active.
    pub(crate) band: Option<(DVec2, DVec2)>,
}

impl CopyPanel {
    fn opened(tool: Tool, elements: Vec<Element>) -> Self {
        let values = match tool {
            Tool::CircularPattern => 2,
            Tool::RectangularPattern => 4,
            _ => 0,
        };
        let active = match elements.is_empty() {
            true => CopySlot::Elements,
            false => CopySlot::Target,
        };
        Self {
            active,
            elements,
            values,
            ..Self::default()
        }
    }

    pub fn active(&self) -> CopySlot {
        self.active
    }

    pub fn activate(&mut self, slot: CopySlot) {
        self.active = slot;
    }

    pub fn elements(&self) -> &[Element] {
        &self.elements
    }

    /// The slot `Entrée` on Validate found empty, until it is filled.
    pub fn marked(&self) -> Option<CopySlot> {
        self.marked
    }

    pub(crate) fn target(&self) -> Option<CopyTarget> {
        self.target
    }

    pub(crate) fn values(&self) -> usize {
        self.values
    }

    pub(crate) fn toggle(&mut self, element: Element) {
        match self.elements.iter().position(|held| *held == element) {
            Some(rank) => {
                self.elements.remove(rank);
            }
            None => self.elements.push(element),
        }
    }

    /// Adds what a box took that is not held already.
    pub(crate) fn take_in(&mut self, caught: impl IntoIterator<Item = Element>) {
        for element in caught {
            if !self.elements.contains(&element) {
                self.elements.push(element);
            }
        }
    }

    pub(crate) fn name_the_target(&mut self, target: CopyTarget) {
        self.target = Some(target);
        self.move_down();
    }

    /// The slot below the active one becomes active, whether or not it is
    /// filled already.
    pub(crate) fn move_down(&mut self) {
        self.active = match self.active {
            CopySlot::Elements => CopySlot::Target,
            CopySlot::Target if self.values > 0 => CopySlot::Value(0),
            CopySlot::Value(rank) if rank + 1 < self.values => CopySlot::Value(rank + 1),
            _ => CopySlot::Validate,
        };
    }

    pub(crate) fn is_filled(&self, slot: CopySlot, live: &LiveInput) -> bool {
        match slot {
            CopySlot::Elements => !self.elements.is_empty(),
            CopySlot::Target => self.target.is_some(),
            CopySlot::Value(rank) => live.typed(rank).is_some(),
            CopySlot::Validate => true,
        }
    }

    /// Activates and marks the first slot still empty, top to bottom. False
    /// when every slot is filled.
    pub(crate) fn mark_the_first_empty(&mut self, live: &LiveInput) -> bool {
        let slots = [CopySlot::Elements, CopySlot::Target]
            .into_iter()
            .chain((0..self.values).map(CopySlot::Value));
        let Some(empty) = slots.into_iter().find(|slot| !self.is_filled(*slot, live)) else {
            return false;
        };
        self.active = empty;
        self.marked = Some(empty);
        true
    }

    pub(crate) fn unmark_if_filled(&mut self, live: &LiveInput) {
        if self.marked.is_some_and(|slot| self.is_filled(slot, live)) {
            self.marked = None;
        }
    }
}

impl SketchEditor {
    /// The panel of the copy tool in hand, open from the moment it is taken.
    pub fn copy_panel(&self) -> Option<&CopyPanel> {
        self.copy_panel.as_ref()
    }

    pub fn copy_panel_mut(&mut self) -> Option<&mut CopyPanel> {
        self.copy_panel.as_mut()
    }

    pub(crate) fn open_copy_panel(&mut self, elements: Vec<Element>) {
        self.tool_state = ToolState::Copying;
        self.copy_panel = Some(CopyPanel::opened(self.tool, elements));
    }

    /// Closes the panel and lays nothing; the tool stays in hand.
    pub fn cancel_the_copy(&mut self) {
        self.reset_pending();
    }

    /// The selection a tool reached for from the panel starts with: what the
    /// panel held, when that tool works on a selection.
    pub(super) fn what_the_panel_hands_on(&mut self) -> Vec<Selection> {
        let held = self.copy_panel.take().map(|panel| panel.elements);
        held.into_iter().flatten().map(Selection::Element).collect()
    }
}
