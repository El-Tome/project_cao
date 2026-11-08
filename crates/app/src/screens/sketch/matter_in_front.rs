//! How the open sketch shows the matter standing in front of its plane:
//! stepped round the three states by the ribbon's button, or set to one of
//! them by the view bar's menu.

use cao_render::MatterInFront;

use super::SketchEditor;

impl SketchEditor {
    pub fn next_matter_in_front(&mut self) {
        self.matter_in_front = self.matter_in_front.next();
    }

    // Only its tests call it until the view bar's menu is drawn.
    #[cfg_attr(not(test), expect(dead_code))]
    pub fn set_matter_in_front(&mut self, state: MatterInFront) {
        self.matter_in_front = state;
    }
}
