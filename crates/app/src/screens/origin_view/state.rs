use cao_prefs::config::ViewportCorner;
use cao_prefs::{Command, Shortcuts};
use egui::{Align2, Pos2, Rect, vec2};

use crate::screens::SketchContext;
use crate::screens::view_bar::Offered;
use crate::screens::viewport::{ViewportState, cube_rect};
use crate::ui::theme::tokens::SPACING_S;

/// The button, and where it stands against the cube.
#[derive(Clone, Debug, PartialEq)]
pub struct OriginViewButton {
    pub offered: Offered,
    pub pivot: Align2,
    pub at: Pos2,
}

impl OriginViewButton {
    /// Under the cube, or above it when the cube sits in a bottom corner and
    /// leaves no room below. Nothing while a sketch is open: the view belongs
    /// to its plane.
    pub fn of(
        part: &SketchContext<'_>,
        viewport: &ViewportState,
        view: Rect,
        bound: &Shortcuts,
    ) -> Option<Self> {
        if part.editor.active_sketch().is_some() {
            return None;
        }
        let cube = cube_rect(view, &viewport.config);
        let (pivot, at) = match viewport.config.cube_corner {
            ViewportCorner::TopLeft | ViewportCorner::TopRight => (
                Align2::CENTER_TOP,
                cube.center_bottom() + vec2(0.0, SPACING_S),
            ),
            ViewportCorner::BottomLeft | ViewportCorner::BottomRight => (
                Align2::CENTER_BOTTOM,
                cube.center_top() - vec2(0.0, SPACING_S),
            ),
        };
        Some(Self {
            offered: Offered::of(Command::OriginView, None, part, bound),
            pivot,
            at,
        })
    }
}
