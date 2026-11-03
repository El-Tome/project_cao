//! What app · screens/viewport/matter.rs is held to.
//!
//! Closes #526.
//! - the section view works on an exact body: what stands between the eye and
//!   the sketch plane is not drawn, and the rest is —
//!   `an_exact_body_is_drawn_behind_the_sketch_plane_only`
//!
//! Closes #319.
//! - only in the plane view: the matter between the camera and the sketch
//!   plane is treated only while the view sits on the sketch plane, reached by
//!   starting the sketch or by the re-align button —
//!   `the_matter_between_the_eye_and_the_sketch_plane_is_not_drawn`
//! - as soon as the camera is orbited out of it, the whole part is drawn again,
//!   as usual; no angle tolerance —
//!   `orbiting_out_of_the_plane_view_draws_the_whole_part_again`
//! - the plane view is the view sitting on the sketch plane, not any face of
//!   the cube — `a_cube_face_with_no_sketch_open_cuts_nothing`,
//!   `a_view_sitting_on_another_plane_than_the_sketch_cuts_nothing`
//! - three ways to show that matter, each click moving to the next state —
//!   `each_click_moves_the_matter_in_front_to_the_next_state`
//! - Hidden: the matter in front of the plane is not drawn —
//!   `hidden_draws_nothing_in_front_of_the_plane`
//! - See-through: it is drawn translucent, and the sketch shows through it —
//!   `see_through_draws_the_matter_in_front_translucent`
//! - Opaque: it is drawn as usual — `opaque_draws_the_matter_in_front_as_usual`
//! - the button is shown only while a sketch is open, next to the re-align
//!   button — no test: here; it is a ribbon command, held under its own
//!   `Closes #319` in `crates/app/src/screens/ribbon/state/tests.rs`
//! - what lies behind the plane is always drawn as usual —
//!   `what_lies_behind_the_plane_is_drawn_in_every_state`
//! - every sketch opens on Hidden, and Hidden stays the default —
//!   `every_sketch_opens_on_hidden`
//! - the choice lasts as long as that sketch is open, and is not remembered for
//!   the next one — `the_state_chosen_is_not_carried_to_the_next_sketch`
//! - closing the sketch draws the whole part again —
//!   `closing_the_sketch_puts_the_whole_body_back`
//! - clicks always reach the sketch plane, whatever the state, picking and
//!   snapping on it included — no test: here; a click is met with the plane and
//!   never with the body, held by a click through opaque matter in
//!   `crates/app/tests/drawing.rs` under its own `Closes #319`
//! - the part's picture in the explorer carries no cut — no test: `picture::of`
//!   reads `document.body()` and is handed neither the editor nor the view, so
//!   there is nothing it could cut with

use cao_render::CubeFace;
use cao_render::camera::CubeZone;
use cao_sketch::WorkPlane;
use cao_solid::profile::{Contour, Frame, Profile};
use glam::{DVec2, DVec3, Vec2};

use super::*;
use crate::screens::viewport::{ViewMode, ViewportState};

const ABOVE: Vec3 = Vec3::new(0.0, 0.0, 10.0);

/// The view a sketch on `plane` lands in when it is started.
fn sitting_on(plane: WorkPlane) -> ViewMode {
    let mut view = ViewportState::default();
    view.look_at_plane(plane, DVec3::ZERO, 5.0);
    view.mode()
}

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

    let shown = shown_body(&editor, sitting_on(WorkPlane::XY), &body, ABOVE);

    assert_eq!(*shown, body);
}

#[test]
fn the_matter_between_the_eye_and_the_sketch_plane_is_not_drawn() {
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    let body = a_body_through_the_plane();

    let shown = shown_body(&editor, sitting_on(WorkPlane::XY), &body, ABOVE);

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
    editor.close();
    let body = a_body_through_the_plane();

    let shown = shown_body(&editor, sitting_on(WorkPlane::XY), &body, ABOVE);

    assert_eq!(
        *shown, body,
        "the view still sits on the plane the sketch was closed on, \
         and keeping the cut with it would leave the part opened for good"
    );
}

#[test]
fn looking_from_under_the_plane_cuts_away_the_other_half() {
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    let body = a_body_through_the_plane();

    let shown = shown_body(&editor, sitting_on(WorkPlane::XY), &body, -ABOVE);

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

    let shown = shown_body(&editor, sitting_on(WorkPlane::XY), &body, ABOVE);
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

#[test]
fn orbiting_out_of_the_plane_view_draws_the_whole_part_again() {
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    let mut view = ViewportState::default();
    view.look_at_plane(WorkPlane::XY, DVec3::ZERO, 5.0);
    view.orbit(Vec2::new(1.0, 0.0), 0.01);
    let body = a_body_through_the_plane();

    let shown = shown_body(&editor, view.mode(), &body, ABOVE);

    assert_eq!(
        *shown, body,
        "orbited off the sketch plane, the part is still drawn opened, and reads as broken",
    );
}

#[test]
fn a_cube_face_with_no_sketch_open_cuts_nothing() {
    let editor = SketchEditor::default();
    let mut view = ViewportState::default();
    view.snap_to_zone(CubeZone::Face(CubeFace::PlusZ));
    let body = a_body_through_the_plane();

    let shown = shown_body(&editor, view.mode(), &body, ABOVE);

    assert_eq!(
        *shown, body,
        "a face of the cube lands on a plane with its grid, and no sketch is there to cut for",
    );
}

#[test]
fn a_view_sitting_on_another_plane_than_the_sketch_cuts_nothing() {
    let mut editor = SketchEditor::default();
    editor.begin_editing(0, WorkPlane::XY);
    let mut view = ViewportState::default();
    view.snap_to_zone(CubeZone::Face(CubeFace::MinusY));
    let body = a_body_through_the_plane();

    let shown = shown_body(&editor, view.mode(), &body, Vec3::new(0.0, -10.0, 1.0));

    assert_eq!(
        *shown, body,
        "seen from the front, the sketch plane is edge-on, and cutting along it halves the part",
    );
}
