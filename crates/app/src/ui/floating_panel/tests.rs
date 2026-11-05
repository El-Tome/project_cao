//! What app · ui/floating_panel.rs is held to.
//!
//! Closes #321.
//! - a position that no longer fits the view is brought back inside it —
//!   `a_panel_left_outside_a_smaller_view_is_brought_back_inside_it`,
//!   `a_panel_left_above_and_left_of_the_view_comes_back_to_its_corner`
//! - a position that still fits is where the panel comes back —
//!   `a_panel_that_fits_stays_where_it_was_left`
//! - a panel larger than the view keeps its title in reach, to be dragged —
//!   `a_panel_larger_than_the_view_keeps_its_title_in_reach`

use super::*;

const VIEW: egui::Rect = egui::Rect {
    min: egui::pos2(300.0, 40.0),
    max: egui::pos2(1100.0, 640.0),
};
const PANEL: egui::Vec2 = egui::vec2(200.0, 150.0);

fn lands_on(kept: egui::Pos2, expected: egui::Pos2) {
    assert!(
        (kept - expected).length() <= 1e-3,
        "the panel lands at {kept:?}, expected {expected:?}"
    );
}

#[test]
fn a_panel_that_fits_stays_where_it_was_left() {
    let left_at = egui::pos2(500.0, 200.0);

    lands_on(kept_inside(left_at, PANEL, VIEW), left_at);
}

#[test]
fn a_panel_left_outside_a_smaller_view_is_brought_back_inside_it() {
    let left_on_a_wider_screen = egui::pos2(1700.0, 900.0);

    lands_on(
        kept_inside(left_on_a_wider_screen, PANEL, VIEW),
        egui::pos2(900.0, 490.0),
    );
}

#[test]
fn a_panel_left_above_and_left_of_the_view_comes_back_to_its_corner() {
    lands_on(kept_inside(egui::pos2(-50.0, -20.0), PANEL, VIEW), VIEW.min);
}

#[test]
fn a_panel_larger_than_the_view_keeps_its_title_in_reach() {
    let too_large = egui::vec2(1000.0, 900.0);

    lands_on(
        kept_inside(egui::pos2(700.0, 300.0), too_large, VIEW),
        VIEW.min,
    );
}
