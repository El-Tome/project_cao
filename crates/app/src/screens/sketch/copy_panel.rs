use cao_sketch::{ChosenAxis, Element, ToolState};

use super::{SketchEditor, Tool};

/// A slot of the panel the mirror and the patterns gather what they need in,
/// top to bottom.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CopySlot {
    #[default]
    Elements,
    /// The mirror's axis, a pattern's centre or direction.
    Target,
    Value(usize),
    Validate,
}

#[derive(Debug, Default)]
pub struct CopyPanel {
    active: CopySlot,
    elements: Vec<Element>,
    axis: Option<ChosenAxis>,
    values: usize,
}

impl CopyPanel {
    fn opened(tool: Tool, elements: Vec<Element>) -> Self {
        let values = match tool {
            Tool::CircularPattern => 2,
            Tool::RectangularPattern => 4,
            _ => 0,
        };
        Self {
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

    pub(crate) fn axis(&self) -> Option<ChosenAxis> {
        self.axis
    }

    pub(crate) fn toggle(&mut self, element: Element) {
        match self.elements.iter().position(|held| *held == element) {
            Some(rank) => {
                self.elements.remove(rank);
            }
            None => self.elements.push(element),
        }
    }

    pub(crate) fn first_empty(&self) -> Option<CopySlot> {
        match (self.elements.is_empty(), self.axis) {
            (true, _) => Some(CopySlot::Elements),
            (false, None) => Some(CopySlot::Target),
            (false, Some(_)) => None,
        }
    }

    pub(crate) fn name_the_axis(&mut self, axis: ChosenAxis) {
        self.axis = Some(axis);
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
        self.tool_state = ToolState::Copying {
            held: Vec::new(),
            naming_the_target: false,
        };
        self.copy_panel = Some(CopyPanel::opened(self.tool, elements));
    }

    /// A copy tool state set up without a panel still holds its own elements.
    pub(crate) fn copied(&self) -> &[Element] {
        match (&self.copy_panel, &self.tool_state) {
            (Some(panel), _) => panel.elements(),
            (None, ToolState::Copying { held, .. }) => held,
            _ => &[],
        }
    }
}
