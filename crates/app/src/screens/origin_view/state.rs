use cao_prefs::config::ViewportCorner;
use cao_prefs::{Command, Shortcuts};
use egui::{Align2, Pos2, Rect, vec2};

use crate::screens::SketchContext;
use crate::screens::view_bar::Offered;
use crate::screens::viewport::{ViewportState, cube_rect};
use crate::ui::floating_bar::FRAME_INSET;
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
        let below = vec2(0.0, SPACING_S);
        // The button itself lines up with the cube's side; its frame stands out.
        let out = vec2(FRAME_INSET, 0.0);
        let (pivot, at) = match viewport.config.cube_corner {
            ViewportCorner::TopLeft => (Align2::LEFT_TOP, cube.left_bottom() + below - out),
            ViewportCorner::TopRight => (Align2::RIGHT_TOP, cube.right_bottom() + below + out),
            ViewportCorner::BottomLeft => (Align2::LEFT_BOTTOM, cube.left_top() - below - out),
            ViewportCorner::BottomRight => (Align2::RIGHT_BOTTOM, cube.right_top() - below + out),
        };
        Some(Self {
            offered: Offered::of(Command::OriginView, None, part, bound),
            pivot,
            at,
        })
    }
}
