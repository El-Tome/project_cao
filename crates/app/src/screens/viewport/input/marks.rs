//! A rule taken by its mark, the way a dimension is taken by its value.

use cao_sketch::{Constraint, Selection, Sketch};
use glam::DVec2;

use crate::screens::viewport::render::{mark_under, placed_marks};
use crate::screens::viewport::{ViewportState, to_ndc};

/// The marks as the painter writes them this frame, on screen and on the
/// sketch's plane.
pub(super) struct Marks {
    on_screen: Vec<(Constraint, egui::Pos2)>,
    on_the_plane: Vec<(Constraint, DVec2)>,
}

impl Marks {
    pub(super) fn seen(state: &ViewportState, rect: egui::Rect, sketch: &Sketch) -> Self {
        let aspect = rect.width() / rect.height().max(1.0);
        let on_screen = placed_marks(sketch, state.camera.view_projection(aspect), rect);
        let on_the_plane = on_screen
            .iter()
            .filter_map(|(rule, at)| {
                let (origin, direction) = state.camera.ray(to_ndc(*at, rect), aspect);
                let place = sketch
                    .plane
                    .ray_intersection(origin.as_dvec3(), direction.as_dvec3())?;
                Some((*rule, place))
            })
            .collect();
        Self {
            on_screen,
            on_the_plane,
        }
    }

    /// The rule whose mark is under this place on screen.
    pub(super) fn under(&self, pointer: egui::Pos2) -> Option<Selection> {
        mark_under(&self.on_screen, pointer).map(Selection::Rule)
    }

    /// The rules with a mark inside a box pulled on the sketch's plane, each
    /// once however many of its marks fall inside.
    pub(super) fn inside(&self, from: DVec2, to: DVec2) -> Vec<Selection> {
        let (low, high) = (from.min(to), from.max(to));
        let mut caught: Vec<Selection> = Vec::new();
        for (rule, at) in &self.on_the_plane {
            let rule = Selection::Rule(*rule);
            if at.cmpge(low).all() && at.cmple(high).all() && !caught.contains(&rule) {
                caught.push(rule);
            }
        }
        caught
    }
}

#[cfg(test)]
mod tests;
