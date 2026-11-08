//! Where the view shows a point of the drawing, for the driver tests that
//! click it there: through the camera and the canvas of the last frame drawn.
#![cfg(feature = "test-support")]

use glam::{DVec2, DVec3};

use super::CaoApp;
use crate::screens::{OpenPart, Screen};

impl CaoApp {
    /// Where a point of the world lands on screen: nothing while no part is
    /// drawn, or for a point behind the camera.
    pub fn where_the_view_shows(&self, world: DVec3) -> Option<egui::Pos2> {
        self.part_opened()?.viewport.where_it_shows(world)
    }

    /// The same for a point of the sketch being drawn, in that sketch's own
    /// coordinates: nothing while no sketch is.
    pub fn where_the_view_shows_the_sketch_point(&self, point: DVec2) -> Option<egui::Pos2> {
        let part = self.part_opened()?;
        let sketch = part.doc.sketches().get(part.editor.active_sketch()?)?;
        part.viewport.where_it_shows(sketch.plane.to_world(point))
    }

    fn part_opened(&self) -> Option<&OpenPart> {
        match &self.screen {
            Screen::PartOpened(part) => Some(part),
            Screen::StartMenu => None,
        }
    }
}
