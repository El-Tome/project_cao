//! What app · screens/viewport/cube_labels.rs is held to.

use super::*;

fn axis_aligned_square(half_side: f32) -> (egui::Vec2, egui::Vec2) {
    (egui::vec2(half_side, 0.0), egui::vec2(0.0, half_side))
}

#[test]
fn a_box_smaller_than_the_face_needs_no_shrinking() {
    let (a, b) = axis_aligned_square(50.0);
    let scale = inscribed_scale(a, b, egui::vec2(40.0, 10.0));
    assert!(scale >= 1.0);
}

#[test]
fn a_box_wider_than_the_face_is_shrunk_to_fit_exactly() {
    let (a, b) = axis_aligned_square(50.0);
    let scale = inscribed_scale(a, b, egui::vec2(100.0, 10.0));
    assert!((scale - 0.5).abs() < 1e-4);
}

#[test]
fn a_face_seen_nearly_edge_on_shrinks_the_box_towards_nothing() {
    let a = egui::vec2(50.0, 0.0);
    let b = egui::vec2(0.0, 1.0);
    let scale = inscribed_scale(a, b, egui::vec2(40.0, 10.0));
    assert!(scale < 0.2);
}

#[test]
fn a_face_turned_to_a_diamond_still_holds_an_axis_aligned_box() {
    let a = egui::vec2(50.0, 50.0);
    let b = egui::vec2(50.0, -50.0);
    let scale = inscribed_scale(a, b, egui::vec2(20.0, 20.0));
    assert!((scale - 2.5).abs() < 1e-4);
}

#[test]
fn fitting_font_size_drops_the_label_when_nothing_readable_fits() {
    let result = fitting_font_size(
        egui::vec2(2.0, 0.0),
        egui::vec2(0.0, 2.0),
        egui::vec2(40.0, 10.0),
        100.0,
        20.0,
        8.0,
    );
    assert_eq!(result, None);
}

#[test]
fn fitting_font_size_keeps_the_preferred_size_when_it_already_fits() {
    let result = fitting_font_size(
        egui::vec2(200.0, 0.0),
        egui::vec2(0.0, 200.0),
        egui::vec2(40.0, 10.0),
        100.0,
        20.0,
        8.0,
    );
    assert_eq!(result, Some(20.0));
}
