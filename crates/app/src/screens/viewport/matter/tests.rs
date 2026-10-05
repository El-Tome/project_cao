//! What app · screens/viewport/matter.rs is held to.
//!
//! Closes #526.
//! - the section view works on an exact body: what stands between the eye and
//!   the sketch plane is not drawn, and the rest is —
//!   `an_exact_body_is_drawn_behind_the_sketch_plane_only`

use cao_sketch::WorkPlane;
use cao_solid::profile::{Contour, Frame, Profile};
use glam::{DVec2, DVec3};

use super::*;
use crate::screens::sketch::SketchPhase;

/// A box straddling the XY plane, from one unit under it to one unit over,
/// raised by the exact kernel as the part raises a block rather than laid out
/// face by face.
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
    let profile = Profile {
        exact: Some((Contour::straight(corners.to_vec()), Vec::new())),
        sampled: cao_solid::Loop::straight(&corners),
        sampled_holes: Vec::new(),
        triangles: &triangles,
    };
    let frame = Frame {
        origin: DVec3::Z * -1.0,
        u: DVec3::X,
        v: DVec3::Y,
    };
    Body::default()
        .tool_raised(&profile, frame, DVec3::Z * 2.0)
        .expect("the exact kernel raises a lone profile")
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

/// A disc 10 across standing on its axis, from one unit under the XY plane to
/// one unit over, raised by the exact kernel.
fn a_disc_through_the_plane() -> Body {
    let points: Vec<DVec2> = (0..48)
        .map(|step| DVec2::from_angle(std::f64::consts::TAU * step as f64 / 48.0) * 5.0)
        .collect();
    let curves = vec![Some(0); points.len()];
    let triangles: Vec<[DVec2; 3]> = (1..points.len() - 1)
        .map(|index| [points[0], points[index], points[index + 1]])
        .collect();
    let profile = Profile {
        exact: Some((Contour::circle(DVec2::ZERO, 5.0), Vec::new())),
        sampled: cao_solid::Loop {
            points: &points,
            curves: &curves,
        },
        sampled_holes: Vec::new(),
        triangles: &triangles,
    };
    let frame = Frame {
        origin: DVec3::Z * -1.0,
        u: DVec3::X,
        v: DVec3::Y,
    };
    Body::default()
        .tool_raised(&profile, frame, DVec3::Z * 2.0)
        .expect("the exact kernel raises a lone profile")
}

#[test]
fn an_exact_body_is_drawn_behind_the_sketch_plane_only() {
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    let body = a_disc_through_the_plane();

    let shown = shown_body(&editor, &body, Vec3::new(0.0, 0.0, 10.0));
    let corners: Vec<DVec3> = shown.triangles().into_iter().flatten().collect();

    assert!(!corners.is_empty(), "the far half is drawn");
    assert!(!reaches_above(&shown), "the near half is gone");
    assert!(
        corners.iter().any(|at| (at.z + 1.0).abs() <= 1e-9),
        "the far end of the disc is drawn",
    );
    assert!(
        corners
            .iter()
            .any(|at| at.z.abs() <= 1e-9 && (at.truncate().length() - 5.0).abs() <= 1e-9),
        "the wall is drawn right up to the plane",
    );
    assert!(
        corners
            .iter()
            .all(|at| at.truncate().length() <= 5.0 + 1e-9),
        "nothing is drawn outside the true cylinder",
    );
}
