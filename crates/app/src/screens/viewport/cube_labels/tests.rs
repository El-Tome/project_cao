//! What app · screens/viewport/cube_labels.rs is held to.

use super::*;

#[test]
fn a_box_smaller_than_the_face_needs_no_shrinking() {
    let scale = inscribed_scale(50.0, egui::vec2(40.0, 10.0));
    assert!(scale >= 1.0);
}

#[test]
fn a_box_wider_than_the_face_is_shrunk_to_fit_exactly() {
    let scale = inscribed_scale(50.0, egui::vec2(100.0, 10.0));
    assert!((scale - 0.5).abs() < 1e-4);
}

#[test]
fn fitting_font_size_drops_the_label_when_nothing_readable_fits() {
    let result = fitting_font_size(2.0, egui::vec2(40.0, 10.0), 100.0, 20.0, 8.0);
    assert_eq!(result, None);
}

#[test]
fn fitting_font_size_keeps_the_preferred_size_when_it_already_fits() {
    let result = fitting_font_size(200.0, egui::vec2(40.0, 10.0), 100.0, 20.0, 8.0);
    assert_eq!(result, Some(20.0));
}
