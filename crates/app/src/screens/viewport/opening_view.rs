//! The view a part opens with, asked for again.

use cao_render::ViewTransition;

use super::state::{ViewMode, ViewportState};

impl ViewportState {
    /// Turns to the angles a new part opens with and frames the whole part;
    /// with no volume to frame, comes back to the very camera a new part opens
    /// with. Those angles are oblique, so the view lies on no plane any more.
    pub(crate) fn turn_to_the_opening_view(&mut self, body: &cao_solid::Body) {
        let opening = Self::default().camera;
        match body.bounds() {
            Some(bounds) => self.fit(bounds),
            None => {
                let (yaw, pitch) = (self.camera.yaw(), self.camera.pitch());
                self.camera = opening;
                self.camera.set_view_angles(yaw, pitch);
            }
        }
        self.transition = Some(ViewTransition::to_angles(
            &self.camera,
            opening.yaw(),
            opening.pitch(),
        ));
        self.mode = ViewMode::Free;
    }
}
