//! What app · screens/viewport/matter.rs is held to.

use cao_sketch::WorkPlane;
use glam::{DVec2, DVec3};

use super::*;
use crate::screens::sketch::SketchPhase;

/// A box straddling the XY plane, from one unit under it to one unit over,
/// raised as the part raises a block rather than laid out face by face.
fn a_body_through_the_plane() -> Body {
    let corners = [
        DVec2::new(-1.0, -1.0),
        DVec2::new(1.0, -1.0),
        DVec2::new(1.0, 1.0),
        DVec2::new(-1.0, 1.0),
    ];
    let triangles = [
        [corners[0], corners[1], corners[2]],
        [corners[0], corners[2], corners[3]],
    ];
    let profile = cao_solid::profile::Profile {
        exact: None,
        sampled: cao_solid::Loop::straight(&corners),
        sampled_holes: Vec::new(),
        triangles: &triangles,
    };
    let frame = cao_solid::profile::Frame {
        origin: DVec3::Z * -1.0,
        u: DVec3::X,
        v: DVec3::Y,
    };
    Body::default()
        .tool_raised(&profile, frame, DVec3::Z * 2.0)
        .expect("the flats never decline")
}

fn reaches_above(body: &Body) -> bool {
    body.triangles().iter().flatten().any(|at| at.z > 1e-9)
}

#[test]
fn nothing_is_cut_away_while_no_sketch_is_being_edited() {
    let editor = SketchEditor::default();
    let body = a_body_through_the_plane();

    let shown = shown_body(&editor, &body, Vec3::new(0.0, 0.0, 10.0));

    assert_eq!(*shown, body);
}

#[test]
fn the_matter_between_the_eye_and_the_sketch_plane_is_not_drawn() {
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    let body = a_body_through_the_plane();

    let shown = shown_body(&editor, &body, Vec3::new(0.0, 0.0, 10.0));

    assert!(!reaches_above(&shown), "the near side is gone");
    assert!(
        shown
            .triangles()
            .iter()
            .flatten()
            .any(|at| (at.z + 1.0).abs() <= 1e-9),
        "and the far side is what one draws against"
    );
}

#[test]
fn closing_the_sketch_puts_the_whole_body_back() {
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    editor.phase = SketchPhase::Idle;
    let body = a_body_through_the_plane();

    let shown = shown_body(&editor, &body, Vec3::new(0.0, 0.0, 10.0));

    assert_eq!(
        *shown, body,
        "the editor keeps its plane so the view can be aligned with it, \
         and keeping the cut with it would leave the part opened for good"
    );
}

#[test]
fn looking_from_under_the_plane_cuts_away_the_other_half() {
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    let body = a_body_through_the_plane();

    let shown = shown_body(&editor, &body, Vec3::new(0.0, 0.0, -10.0));

    assert!(
        shown.triangles().iter().flatten().all(|at| at.z >= -1e-9),
        "what stands in front depends on where one looks from"
    );
}
