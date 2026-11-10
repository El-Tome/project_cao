//! What the part offers to draw on, face by face.
//!
//! Closes #358.
//! - the origin is a corner of the face itself, not of the piece the ray met —
//!   `a_drawing_is_read_from_a_corner_of_the_whole_face`
//! - a face with corners at its bounds spans no negative size from there —
//!   `a_drawing_is_read_from_a_corner_of_the_whole_face`. A round face has no
//!   corner at the bottom-left of what it spans, so the nearest real one
//!   answers and a sliver of it stays behind: a corner of the part beats a
//!   point that is on no corner at all.
//! - which way up it reads, and the base planes — no test: the rule is
//!   `WorkPlane::from_face`, held by the tests beside it in `cao_sketch`
//!
//! Closes #192.
//! - a sketch on a curved face is refused, and the click starts nothing —
//!   `a_curved_wall_offers_no_plane_to_draw_on`
//! - a flat face answers exactly as before —
//!   `a_flat_end_still_offers_its_plane`
//! - a curved wall is one face made of many pieces, and a cut leaves it one —
//!   no test: held in `cao_solid`, where the faces are made and carried, by
//!   `a_curve_raises_one_wall_however_finely_it_was_sampled` and
//!   `a_cut_across_a_face_leaves_it_one_face`
//!
//! Closes #526.
//! - picking reads the exact kernel's body: a click anywhere on a cylinder's
//!   wall lands on its one curved face, with the true cylinder's normal —
//!   `a_click_on_an_exact_cylinders_wall_lands_on_its_one_curved_face`
//!
//! Closes #571.
//! - a click on an elliptic wall reads its true normal, and lands on its one
//!   curved face — `a_click_on_an_exact_elliptic_wall_lands_on_its_one_curved_face`
//!
//! Closes #193 — the whole transcription is in
//! `crates/app/src/screens/viewport/render/tests.rs`; these bullets are the
//! ones a click on the part answers.
//! - a flat face of the part clicked becomes the cutting plane, a curved one
//!   is refused — `a_flat_face_clicked_becomes_the_cutting_plane`,
//!   `a_curved_face_clicked_is_no_cutting_plane`
//! - the section picked starts a sketch on the cutting plane, and starting a
//!   sketch turns the section view off —
//!   `a_click_on_the_section_starts_a_sketch_on_the_cutting_plane`,
//!   `starting_a_sketch_on_a_plane_of_the_origin_turns_the_section_view_off`;
//!   the history reads it as laid on a plane off the origin held by no face,
//!   not on a face of the part —
//!   `a_sketch_started_on_the_section_reads_as_laid_on_an_offset_plane_in_the_history`
//! - found in review: a face of the part picked through the section is
//!   anchored to the part's own face, and the face lit under the cursor is
//!   that same face —
//!   `a_face_picked_through_an_inverted_section_is_anchored_to_the_part_s_own_face`
//! - found in review: the cut is told from a face by the faces the part does
//!   not have, not by a distance — a part far from the origin keeps its cut,
//!   `the_cut_of_a_part_far_from_the_origin_is_still_the_section`, and the face
//!   a section was laid on stays a face of the part,
//!   `the_face_a_section_was_laid_on_is_still_a_face_of_the_part`
//! - found in review: only the primary button pulls the handle; a drag that
//!   orbits does not — `only_the_primary_button_pulls_the_handle`; and the
//!   section takes no gesture while a sketch is edited —
//!   `the_section_takes_no_gesture_while_a_sketch_is_edited`
//!
//! Closes #348 — the whole transcription is in
//! `crates/app/src/screens/viewport/render/tests.rs`; these bullets are the
//! ones a click on the part answers.
//! - found in review: a face of a see-through body is offered to draw on, and
//!   anchored, rather than the plane of the origin behind it —
//!   `a_face_of_a_see_through_body_is_offered_to_draw_on`
//! - found in review: nothing of an invisible body is offered, neither to draw
//!   on nor to cut along — `an_invisible_body_offers_nothing_of_itself_to_draw_on`,
//!   `an_invisible_body_offers_no_face_to_cut_along`
//!
//! Closes #734 — the whole transcription is in
//! `crates/app/src/screens/part_tree/state/tests.rs`; these bullets are the
//! ones a click on the part answers.
//! - a face can still be picked to start a sketch on a see-through or opaque
//!   volume — `a_face_of_a_see_through_volume_is_offered_to_draw_on_beside_an_opaque_one`
//! - found while laying the frame: an invisible volume is not drawn, so none
//!   of its faces is offered, and a ray goes through it to the face behind —
//!   `a_hidden_volume_offers_none_of_its_faces_and_the_face_behind_it_is_offered`
//! - found in review: under a section that parts a face of a hidden volume in
//!   two and numbers a piece anew, no piece of it is offered to draw on —
//!   `a_hidden_volume_the_section_parts_offers_no_piece_of_itself_to_draw_on` —
//!   nor taken as a cutting face —
//!   `a_hidden_volume_the_section_parts_offers_no_piece_of_itself_to_cut_along`
//! - found in review: a face is picked by where the ray meets it, not by the
//!   number the part shown gives it — with a block hidden, each piece a slot
//!   leaves of the other's top anchors on that piece of the part —
//!   `with_one_block_hidden_each_piece_of_the_other_s_slotted_top_anchors_on_it`;
//!   a see-through cut's ghost, where the part has no face, lets the ray go
//!   on to the block behind it and offers nothing of its own —
//!   `a_see_through_cut_s_ghost_lets_the_ray_go_on_where_the_part_has_no_face`;
//!   and the top an invisible cut fills in, where the part has no face,
//!   offers nothing either —
//!   `the_top_an_invisible_cut_fills_in_offers_nothing_where_the_part_has_none`
//! - found in review: a curved face met stops the ray there and refuses, as a
//!   curved wall always has, under a section and with a volume left out alike,
//!   rather than letting it through to a flat face behind —
//!   `a_curved_wall_under_a_section_still_refuses_rather_than_letting_the_ray_through`,
//!   `a_curved_wall_of_a_part_shown_still_refuses_rather_than_letting_the_ray_through`
//! - found in review: a section laid from a face the part has nothing of — a
//!   see-through cut's ghost, cut by the section — is kept on no face of the
//!   part — `a_section_laid_on_the_cut_of_a_ghost_follows_no_face_of_the_part`

use glam::{DVec2, DVec3};

use super::*;
use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::SketchEditor;
use crate::screens::viewport::matter::shown;
use crate::screens::viewport::section::Section;
use cao_part::{PartDocument, VolumeState};
use cao_render::OrbitCamera;
use cao_solid::profile::{Contour, Run};

/// Which way is up on screen once the view has swung onto a face — the same
/// answer `once_facing` gives, without a viewport to ask.
fn facing(normal: DVec3) -> DVec3 {
    let mut camera = OrbitCamera::default();
    let (yaw, pitch) = view_angles_towards(normal.as_vec3());
    camera.set_view_angles(yaw, pitch);
    camera.up().as_dvec3()
}

/// A disc pushed along its own normal by the exact kernel: flat ends, and
/// one curved wall drawn as many triangles.
fn cylinder(radius: f64, height: f64) -> Body {
    let points: Vec<DVec2> = (0..48)
        .map(|step| {
            let angle = std::f64::consts::TAU * step as f64 / 48.0;
            DVec2::new(radius * angle.cos(), radius * angle.sin())
        })
        .collect();
    let curves = vec![Some(0); points.len()];
    let triangles: Vec<[DVec2; 3]> = (1..points.len() - 1)
        .map(|index| [points[0], points[index], points[index + 1]])
        .collect();
    let profile = cao_solid::profile::Profile {
        exact: Some((Contour::circle(DVec2::ZERO, radius), Vec::new())),
        sampled: cao_solid::Loop {
            points: &points,
            curves: &curves,
        },
        sampled_holes: Vec::new(),
        triangles: &triangles,
    };
    let frame = cao_solid::profile::Frame {
        origin: DVec3::Z * 0.0,
        u: DVec3::X,
        v: DVec3::Y,
    };
    Body::default()
        .tool_raised(&profile, frame, DVec3::Z * height)
        .expect("the exact kernel raises a lone profile")
}

/// A block raised by the exact kernel, whose top is one flat face drawn as
/// two triangles.
fn block(width: f64, depth: f64, height: f64) -> Body {
    let points = vec![
        DVec2::ZERO,
        DVec2::new(width, 0.0),
        DVec2::new(width, depth),
        DVec2::new(0.0, depth),
    ];
    let triangles = vec![
        [points[0], points[1], points[2]],
        [points[0], points[2], points[3]],
    ];
    let profile = cao_solid::profile::Profile {
        exact: Some((
            Contour::rectangle(DVec2::ZERO, DVec2::new(width, depth)),
            Vec::new(),
        )),
        sampled: cao_solid::Loop::straight(&points),
        sampled_holes: Vec::new(),
        triangles: &triangles,
    };
    let frame = cao_solid::profile::Frame {
        origin: DVec3::Z * 0.0,
        u: DVec3::X,
        v: DVec3::Y,
    };
    Body::default()
        .tool_raised(&profile, frame, DVec3::Z * height)
        .expect("the exact kernel raises a lone profile")
}

#[test]
fn a_curved_wall_offers_no_plane_to_draw_on() {
    let body = cylinder(10.0, 20.0);

    let offered = what_the_part_offers(&body, DVec3::new(40.0, 0.0, 10.0), DVec3::NEG_X, facing)
        .expect("the wall is there to be hit");

    assert!(
        matches!(offered, PlaneChoice::Curved(_)),
        "the wall handed back {offered:?}",
    );
    assert!(
        offered.plane().is_none(),
        "a refusal offers no plane, so the click starts nothing",
    );
}

#[test]
fn a_flat_end_still_offers_its_plane() {
    let body = cylinder(10.0, 20.0);

    let offered = what_the_part_offers(&body, DVec3::new(0.0, 0.0, 50.0), DVec3::NEG_Z, facing)
        .expect("the top is there to be hit");

    let plane = offered.plane().expect("a flat face offers its plane");
    assert!(
        plane.normal().dot(DVec3::Z).abs() > 0.99,
        "the top of a cylinder faces up, not {:?}",
        plane.normal(),
    );
}

/// A block's top, which is stored as two triangles: read from the corner of
/// the whole face, the top spans nothing but positive sizes. Read from the
/// triangle the ray happened to meet, half of it would sit behind the origin.
#[test]
fn a_drawing_is_read_from_a_corner_of_the_whole_face() {
    let body = block(30.0, 20.0, 12.0);

    let offered = what_the_part_offers(&body, DVec3::new(11.0, 7.0, 50.0), DVec3::NEG_Z, facing)
        .expect("the top is there to be hit");
    let PlaneChoice::Face { plane, face } = offered else {
        panic!("a flat end offers its plane, not {offered:?}");
    };

    let corners: Vec<DVec3> = body.plane_of(face).expect("the top is a face").corners;
    assert!(
        corners
            .iter()
            .any(|corner| corner.distance(plane.origin) < 1e-9),
        "the origin sits at {:?}, which is no corner of the face",
        plane.origin,
    );
    for corner in &corners {
        let local = plane.to_local(*corner);
        assert!(
            local.x >= -1e-9 && local.y >= -1e-9,
            "{corner:?} sits at {local:?}, behind the corner the face is read from",
        );
    }
}

#[test]
fn a_click_on_an_exact_cylinders_wall_lands_on_its_one_curved_face() {
    let body = cylinder(10.0, 20.0);
    let mut walls = Vec::new();

    for step in 0..12 {
        let angle = (30.0 * step as f64 + 7.0).to_radians();
        let outwards = DVec3::new(angle.cos(), angle.sin(), 0.0);
        let origin = outwards * 40.0 + DVec3::Z * 10.0;

        let offered = what_the_part_offers(&body, origin, -outwards, facing)
            .expect("the wall is there to be hit");
        let PlaneChoice::Curved(face) = offered else {
            panic!("the wall at {step} offered {offered:?}");
        };
        walls.push(face);
        let hit = body.ray_hit(origin, -outwards).expect("the wall is hit");
        assert!(
            hit.normal.distance(outwards) < 1e-9,
            "the wall at {step} faces {:?}, not straight out of the axis",
            hit.normal,
        );
    }

    assert!(
        walls.iter().all(|face| *face == walls[0]),
        "the wall answered to several faces: {walls:?}",
    );
}

/// An ellipse of half-axes `a` along X and `b` pushed along its normal by
/// the exact kernel, as the drawing hands it: one run of a whole turn. The
/// flats are handed nothing, which is why a body that fell back to them
/// would be empty.
fn elliptic_cylinder(a: f64, b: f64, height: f64) -> Body {
    let profile = cao_solid::profile::Profile {
        exact: Some((
            Contour {
                corners: vec![DVec2::X * a],
                runs: vec![Run::Oval {
                    center: DVec2::ZERO,
                    first: DVec2::X * a,
                    second: b,
                    turn: std::f64::consts::TAU,
                }],
            },
            Vec::new(),
        )),
        sampled: cao_solid::Loop::straight(&[]),
        sampled_holes: Vec::new(),
        triangles: &[],
    };
    let frame = cao_solid::profile::Frame {
        origin: DVec3::ZERO,
        u: DVec3::X,
        v: DVec3::Y,
    };
    Body::default()
        .tool_raised(&profile, frame, DVec3::Z * height)
        .expect("the exact kernel raises a lone ellipse")
}

#[test]
fn a_click_on_an_exact_elliptic_wall_lands_on_its_one_curved_face() {
    let (a, b) = (15.0, 6.0);
    let body = elliptic_cylinder(a, b, 20.0);
    let mut walls = Vec::new();

    for step in 0..12 {
        let angle = (30.0 * step as f64 + 7.0).to_radians();
        let outwards = DVec3::new(angle.cos(), angle.sin(), 0.0);
        let origin = outwards * 40.0 + DVec3::Z * 10.0;

        let offered = what_the_part_offers(&body, origin, -outwards, facing)
            .expect("the wall is there to be hit");
        let PlaneChoice::Curved(face) = offered else {
            panic!("the wall at {step} offered {offered:?}");
        };
        walls.push(face);
        let hit = body.ray_hit(origin, -outwards).expect("the wall is hit");
        let at = origin - outwards * hit.distance;
        let true_normal = DVec3::new(at.x / (a * a), at.y / (b * b), 0.0).normalize();
        assert!(
            hit.normal.distance(true_normal) < 1e-9,
            "the wall at {step} faces {:?}, not {true_normal:?}, the way the \
             ellipse through where it was hit does",
            hit.normal,
        );
    }

    assert!(
        walls.iter().all(|face| *face == walls[0]),
        "the wall answered to several faces: {walls:?}",
    );
}

/// A section on, waiting for a face to cut along, XY at mid-height before one
/// is clicked.
fn waiting_for_a_face() -> ViewportState {
    let mut section = Section::new(WorkPlane::XY);
    section.offset = 2.0;
    section.wait_for_a_face();
    let mut state = ViewportState::default();
    state.section = Some(section);
    state
}

#[test]
fn a_flat_face_clicked_becomes_the_cutting_plane() {
    let body = block(10.0, 10.0, 4.0);
    let mut state = waiting_for_a_face();

    let taken = choose_a_cutting_face(
        &mut state,
        &body,
        true,
        DVec3::new(50.0, 5.0, 1.0),
        DVec3::NEG_X,
    );

    let section = state.section.as_ref().expect("the section stays on");
    assert!(taken, "a click on the side of the block takes nothing");
    assert!(
        section.plane.normal().distance(DVec3::X) < 1e-9,
        "the side facing +X is cut along a plane facing {:?}, not outwards",
        section.plane.normal(),
    );
    assert!(
        (section.plane.origin.x - 10.0).abs() < 1e-9 && section.offset.abs() < 1e-9,
        "the cut does not start on the face clicked: the plane passes x = {} and the \
         distance reads {}",
        section.plane.origin.x,
        section.offset,
    );
    assert!(
        !section.is_waiting_for_a_face(),
        "a face taken, the panel still waits for one",
    );
}

#[test]
fn a_curved_face_clicked_is_no_cutting_plane() {
    let body = cylinder(10.0, 20.0);
    let mut state = waiting_for_a_face();

    let taken = choose_a_cutting_face(
        &mut state,
        &body,
        true,
        DVec3::new(40.0, 0.0, 10.0),
        DVec3::NEG_X,
    );

    let section = state.section.as_ref().expect("the section stays on");
    assert!(!taken, "a click on a curved wall takes it as a plane");
    assert_eq!(
        section.plane,
        WorkPlane::XY,
        "a click on a curved wall moved the cut"
    );
    assert!(
        section.is_waiting_for_a_face(),
        "a curved wall refused, the panel no longer waits for a face",
    );
}

/// A part holding a block 10 by 10 from 0 to 4 over XY, with nothing open.
fn a_part_holding_a_block() -> PartDocument {
    a_part_holding_a_block_from(DVec2::ZERO)
}

/// The same block, its corner at `corner` on XY.
fn a_part_holding_a_block_from(corner: DVec2) -> PartDocument {
    let mut document = PartDocument::new("part", chrono::Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    raise(
        &mut document,
        0,
        &[(corner, corner + DVec2::splat(10.0))],
        4.0,
    );
    document
}

/// Rectangles drawn on the sketch of rank `sketch`, raised `height` along its
/// normal.
fn raise(document: &mut PartDocument, sketch: usize, rectangles: &[(DVec2, DVec2)], height: f64) {
    use cao_part::{ExtrusionMode, PointRef};

    for &(corner, opposite) in rectangles {
        document.apply(Operation::AddRectangle {
            sketch,
            corner: PointRef::New(corner),
            opposite: PointRef::New(opposite),
            construction: false,
        });
    }
    let inside: Vec<DVec2> = rectangles
        .iter()
        .map(|(corner, opposite)| (*corner + *opposite) / 2.0)
        .collect();
    document.apply(Operation::Extrude {
        sketch,
        areas: document.areas_at(sketch, &inside),
        distance: height.into(),
        mode: ExtrusionMode::Add,
    });
}

/// A U standing on XY: a base 10 by 10 from 0 to 2, and two walls from 2 to
/// 6, one over x from 0 to 2, the other over x from 8 to 10.
fn a_part_shaped_as_a_u() -> PartDocument {
    let mut document = PartDocument::new("part", chrono::Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    raise(&mut document, 0, &[(DVec2::ZERO, DVec2::splat(10.0))], 2.0);
    document.apply(Operation::CreateSketch {
        plane: WorkPlane {
            origin: DVec3::Z * 2.0,
            ..WorkPlane::XY
        },
        on: None,
    });
    raise(
        &mut document,
        1,
        &[
            (DVec2::ZERO, DVec2::new(2.0, 10.0)),
            (DVec2::new(8.0, 0.0), DVec2::new(10.0, 10.0)),
        ],
        4.0,
    );
    let (_, high) = document.body().bounds().expect("the U holds matter");
    assert!(
        (high.z - 6.0).abs() < 1e-9,
        "the walls of the U stand to {}",
        high.z
    );
    document
}

/// What happened once a plane was chosen for a new sketch.
struct Started {
    state: ViewportState,
    document: PartDocument,
    editor: SketchEditor,
    offered: Option<PlaneChoice>,
    started: bool,
}

/// The cursor brought over a place, then a click there, on `document` seen
/// through `section`.
fn a_sketch_started(
    mut document: PartDocument,
    section: Section,
    origin: DVec3,
    direction: DVec3,
) -> Started {
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    editor.start_choosing_plane(&lang);
    let mut state = ViewportState::default();
    state.section = Some(section);

    let (offered, started) = {
        let mut context = SketchContext {
            document: &mut document,
            editor: &mut editor,
            extrusion: &mut extrusion,
            lang: &lang,
        };
        choose_a_plane(&mut state, &mut context, false, origin, direction);
        let offered = context.editor.hovered_plane;
        let started = choose_a_plane(&mut state, &mut context, true, origin, direction);
        (offered, started)
    };
    Started {
        state,
        document,
        editor,
        offered,
        started,
    }
}

/// The same on the block cut through XY at mid-height, its upper half hidden.
fn a_sketch_started_through_a_section(origin: DVec3, direction: DVec3) -> Started {
    let mut section = Section::new(WorkPlane::XY);
    section.offset = 2.0;
    a_sketch_started(a_part_holding_a_block(), section, origin, direction)
}

#[test]
fn a_click_on_the_section_starts_a_sketch_on_the_cutting_plane() {
    let Started {
        state,
        document,
        editor,
        offered,
        started,
    } = a_sketch_started_through_a_section(DVec3::new(5.0, 5.0, 50.0), DVec3::NEG_Z);

    assert!(
        matches!(offered, Some(PlaneChoice::Section(_))),
        "the section is offered as {offered:?}, not as the cutting plane",
    );
    assert!(started, "a click on the section starts no sketch");
    let Some(Operation::CreateSketch { plane, on, .. }) = document.history.operations().last()
    else {
        panic!("the click recorded no sketch");
    };
    assert_eq!(
        *on, None,
        "the sketch is anchored to a face of the part, not laid on the cutting plane",
    );
    for local in [DVec2::ZERO, DVec2::new(3.0, 4.0)] {
        let at = plane.to_world(local);
        assert!(
            (at.z - 2.0).abs() < 1e-9,
            "the sketch is drawn through {at}, off the cutting plane at mid-height",
        );
    }
    assert_eq!(
        editor.plane,
        Some(*plane),
        "the sketch opened is on another plane"
    );
    assert!(
        state.section.is_none(),
        "a sketch started, the section view is still on over the sketch's own cut",
    );
}

#[test]
fn starting_a_sketch_on_a_plane_of_the_origin_turns_the_section_view_off() {
    let Started {
        state,
        offered,
        started,
        ..
    } = a_sketch_started_through_a_section(DVec3::new(-30.0, -30.0, 50.0), DVec3::NEG_Z);

    assert!(
        matches!(offered, Some(PlaneChoice::Origin(_))),
        "beside the block, the click landed on {offered:?}",
    );
    assert!(started, "a click on XY beside the block starts no sketch");
    assert!(
        state.section.is_none(),
        "a sketch started, the section view is still on over the sketch's own cut",
    );
}

/// The face of `document`'s own body the sketch it last started is anchored
/// to, read back from what was recorded.
fn anchored_face(document: &PartDocument) -> usize {
    match document.history.operations().last() {
        Some(Operation::CreateSketch {
            on: Some(FaceAnchor { face, .. }),
            ..
        }) => *face,
        other => panic!("the click recorded no sketch anchored to a face: {other:?}"),
    }
}

/// Whether `face` of the document's own body lies flat at `height`, over x
/// from `from` to `to`.
fn lies_flat_at(document: &PartDocument, face: usize, height: f64, (from, to): (f64, f64)) -> bool {
    let corners: Vec<DVec3> = document.body().triangles_of(face).flatten().collect();
    !corners.is_empty()
        && corners
            .iter()
            .all(|at| (at.z - height).abs() < 1e-9 && at.x >= from - 1e-9 && at.x <= to + 1e-9)
}

/// Whether `face` of the document's own body is the U's whole front, the
/// face standing at y = 0 from the ground to the tops of both walls.
fn is_the_front_of_the_u(document: &PartDocument, face: usize) -> bool {
    let corners: Vec<DVec3> = document.body().triangles_of(face).flatten().collect();
    let reaches = |at: DVec3| corners.iter().any(|corner| corner.distance(at) < 1e-9);
    !corners.is_empty()
        && corners.iter().all(|corner| corner.y.abs() < 1e-9)
        && reaches(DVec3::ZERO)
        && reaches(DVec3::new(10.0, 0.0, 6.0))
}

#[test]
fn a_face_picked_through_an_inverted_section_is_anchored_to_the_part_s_own_face() {
    let mut section = Section::new(WorkPlane::XY);
    section.offset = 4.0;
    section.invert();

    let Started {
        document, offered, ..
    } = a_sketch_started(
        a_part_shaped_as_a_u(),
        section,
        DVec3::new(9.0, -50.0, 5.0),
        DVec3::Y,
    );

    let Some(PlaneChoice::Face { face: lit, .. }) = offered else {
        panic!("the front of the right wall is offered as {offered:?}");
    };
    assert!(
        is_the_front_of_the_u(&document, lit),
        "the face lit under the cursor, {lit}, is not the U's front in the part: the cut \
         split that face in two and numbered the right half anew",
    );
    let anchored = anchored_face(&document);
    assert!(
        is_the_front_of_the_u(&document, anchored),
        "the sketch is anchored to face {anchored}, which the part does not have as the \
         U's front it was started on",
    );
}

#[test]
fn the_cut_of_a_part_far_from_the_origin_is_still_the_section() {
    let mut section = Section::new(WorkPlane::XY);
    section.offset = 2.0;
    section.invert();

    let Started {
        offered, started, ..
    } = a_sketch_started(
        a_part_holding_a_block_from(DVec2::splat(500.0)),
        section,
        DVec3::new(505.0, 505.0, -50.0),
        DVec3::Z,
    );

    assert!(
        matches!(offered, Some(PlaneChoice::Section(_))),
        "the cut of a block 500 away, its side in front kept, is offered as {offered:?}",
    );
    assert!(started, "a click on that cut starts no sketch");
}

#[test]
fn the_face_a_section_was_laid_on_is_still_a_face_of_the_part() {
    let mut state = ViewportState::default();
    let mut section = Section::new(WorkPlane::XY);
    section.wait_for_a_face();
    state.section = Some(section);
    let document = a_part_holding_a_block();
    assert!(
        choose_a_cutting_face(
            &mut state,
            document.body(),
            true,
            DVec3::new(5.0, 5.0, 50.0),
            DVec3::NEG_Z,
        ),
        "the top of the block is not taken to cut along",
    );
    let on_the_top = state.section.take().expect("the section stays on");

    let Started {
        document, offered, ..
    } = a_sketch_started(
        document,
        on_the_top,
        DVec3::new(5.0, 5.0, 50.0),
        DVec3::NEG_Z,
    );

    assert!(
        matches!(offered, Some(PlaneChoice::Face { .. })),
        "the top of the block, which the section was laid on, is offered as {offered:?}",
    );
    let anchored = anchored_face(&document);
    assert!(
        lies_flat_at(&document, anchored, 4.0, (0.0, 10.0)),
        "the sketch is anchored to face {anchored}, not to the top of the block",
    );
}

#[test]
fn a_sketch_started_on_the_section_reads_as_laid_on_an_offset_plane_in_the_history() {
    let Started { document, .. } =
        a_sketch_started_through_a_section(DVec3::new(5.0, 5.0, 50.0), DVec3::NEG_Z);

    let recorded = document
        .history
        .operations()
        .last()
        .expect("the click recorded a step");
    let said = crate::wording::history::label(
        &Catalogue::french(),
        &cao_part::Variables::default(),
        recorded,
    );

    assert_eq!(said, "Esquisse — Plan décalé");
}

const SCREEN: egui::Rect = egui::Rect {
    min: egui::Pos2::ZERO,
    max: egui::pos2(1280.0, 800.0),
};

fn on_the_screen(state: &ViewportState, at: DVec3) -> egui::Pos2 {
    let view_projection = state
        .camera
        .view_projection(SCREEN.width() / SCREEN.height());
    let clip = view_projection * at.as_vec3().extend(1.0);
    let ndc = clip.truncate() / clip.w;
    egui::pos2(
        SCREEN.center().x + ndc.x * SCREEN.width() * 0.5,
        SCREEN.center().y - ndc.y * SCREEN.height() * 0.5,
    )
}

fn pressed(
    at: egui::Pos2,
    button: egui::PointerButton,
    modifiers: egui::Modifiers,
    pressed: bool,
) -> Vec<egui::Event> {
    vec![egui::Event::PointerButton {
        pos: at,
        button,
        pressed,
        modifiers,
    }]
}

/// The block cut through XY at mid-height, its handle taken with `button`
/// and `modifiers` and dragged thirty pixels up the screen, a sketch being
/// edited when `sketching`: the distance after, and whether the section took
/// any frame of the gesture.
fn the_handle_dragged(
    button: egui::PointerButton,
    modifiers: egui::Modifiers,
    sketching: bool,
) -> (f64, bool) {
    let mut document = a_part_holding_a_block();
    let mut editor = SketchEditor::default();
    if sketching {
        editor.begin_editing(0, WorkPlane::XY);
    }
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let mut section = Section::new(WorkPlane::XY);
    section.offset = 2.0;
    let handle = section.handle(document.body());
    let mut state = ViewportState::default();
    state.section = Some(section);
    let at = on_the_screen(&state, handle);
    let up = |pixels: f32| at - egui::vec2(0.0, pixels);
    let context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    let egui = egui::Context::default();
    let mut took = false;
    for events in [
        vec![],
        vec![
            egui::Event::ModifiersChanged(modifiers),
            egui::Event::PointerMoved(at),
        ],
        pressed(at, button, modifiers, true),
        vec![egui::Event::PointerMoved(up(15.0))],
        vec![egui::Event::PointerMoved(up(30.0))],
        pressed(up(30.0), button, modifiers, false),
    ] {
        egui.begin_pass(egui::RawInput {
            screen_rect: Some(SCREEN),
            events,
            ..Default::default()
        });
        egui::Area::new(egui::Id::new("the canvas"))
            .fixed_pos(SCREEN.min)
            .show(&egui, |ui| {
                ui.set_min_size(SCREEN.size());
                let response = ui.interact(
                    SCREEN,
                    egui::Id::new("the canvas's surface"),
                    egui::Sense::click_and_drag(),
                );
                let scale = ViewScale::of(&state.camera, SCREEN, 1.0, &state.config, 1.0);
                took |= point_the_section(ui, &mut state, &response, (SCREEN, scale), &context);
            });
        egui.end_pass().textures_delta.clear();
    }
    let offset = state.section.as_ref().expect("the section stays on").offset;
    (offset, took)
}

#[test]
fn only_the_primary_button_pulls_the_handle() {
    let (pulled, _) =
        the_handle_dragged(egui::PointerButton::Primary, egui::Modifiers::NONE, false);
    let (middle, middle_taken) =
        the_handle_dragged(egui::PointerButton::Middle, egui::Modifiers::NONE, false);
    let (orbit, orbit_taken) =
        the_handle_dragged(egui::PointerButton::Primary, egui::Modifiers::ALT, false);

    assert!(
        (pulled - 2.0).abs() > 0.1,
        "the handle dragged with the primary button left the distance at {pulled}",
    );
    assert!(
        (middle - 2.0).abs() < 1e-9 && !middle_taken,
        "a middle-button drag over the handle, which orbits, moved the cut to {middle}",
    );
    assert!(
        (orbit - 2.0).abs() < 1e-9 && !orbit_taken,
        "an Alt drag over the handle, which orbits, moved the cut to {orbit}",
    );
}

#[test]
fn the_section_takes_no_gesture_while_a_sketch_is_edited() {
    let (offset, taken) =
        the_handle_dragged(egui::PointerButton::Primary, egui::Modifiers::NONE, true);

    assert!(
        !taken && (offset - 2.0).abs() < 1e-9,
        "with a sketch being edited, a drag over the section's handle moved its cut to {offset}",
    );
}

/// Where the block of `a_part_holding_a_block` is raised in its history.
const THE_BLOCK_S_STEP: usize = 2;

/// A view with the block, the part's one volume, drawn `mode`, reached one
/// click of its visibility button at a time.
fn with_the_body(mode: VolumeState) -> ViewportState {
    let document = a_part_holding_a_block();
    let mut state = ViewportState::default();
    for _ in 0..3 {
        if state.volume_mode(&document, THE_BLOCK_S_STEP) == mode {
            break;
        }
        state.next_volume_mode(&document, THE_BLOCK_S_STEP);
    }
    assert_eq!(
        state.volume_mode(&document, THE_BLOCK_S_STEP),
        mode,
        "three clicks never reach {mode:?}",
    );
    state
}

/// The cursor brought over a place, then a click there, choosing the plane of
/// a new sketch on the block, its body drawn `mode`.
fn a_sketch_started_on_a_body(mode: VolumeState, origin: DVec3, direction: DVec3) -> Started {
    a_sketch_started_seen(
        a_part_holding_a_block(),
        with_the_body(mode),
        origin,
        direction,
    )
}

/// The cursor brought over a place, then a click there, choosing the plane of
/// a new sketch on `document`, seen the way `state` says.
fn a_sketch_started_seen(
    mut document: PartDocument,
    mut state: ViewportState,
    origin: DVec3,
    direction: DVec3,
) -> Started {
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    editor.start_choosing_plane(&lang);

    let (offered, started) = {
        let mut context = SketchContext {
            document: &mut document,
            editor: &mut editor,
            extrusion: &mut extrusion,
            lang: &lang,
        };
        choose_a_plane(&mut state, &mut context, false, origin, direction);
        let offered = context.editor.hovered_plane;
        let started = choose_a_plane(&mut state, &mut context, true, origin, direction);
        (offered, started)
    };
    Started {
        state,
        document,
        editor,
        offered,
        started,
    }
}

#[test]
fn a_face_of_a_see_through_body_is_offered_to_draw_on() {
    let started = a_sketch_started_on_a_body(
        VolumeState::SeeThrough,
        DVec3::new(5.0, 5.0, 50.0),
        DVec3::NEG_Z,
    );

    assert!(
        matches!(started.offered, Some(PlaneChoice::Face { .. })),
        "the top of a see-through block is offered as {:?}, not as a face",
        started.offered,
    );
    assert!(
        started.started,
        "a click on the top of a see-through block starts no sketch"
    );
    let face = anchored_face(&started.document);
    assert!(
        lies_flat_at(&started.document, face, 4.0, (0.0, 10.0)),
        "the sketch is anchored to face {face}, not the top of the block",
    );
}

#[test]
fn an_invisible_body_offers_nothing_of_itself_to_draw_on() {
    for (origin, direction) in [
        (DVec3::new(5.0, 5.0, 50.0), DVec3::NEG_Z),
        (DVec3::new(50.0, 5.0, 1.0), DVec3::NEG_X),
    ] {
        let started = a_sketch_started_on_a_body(VolumeState::Invisible, origin, direction);

        assert!(
            !matches!(
                started.offered,
                Some(PlaneChoice::Face { .. } | PlaneChoice::Curved(_) | PlaneChoice::Section(_))
            ),
            "an invisible block offers {:?} under a ray from {origin} along {direction}",
            started.offered,
        );
        assert!(
            !matches!(
                started.document.history.operations().last(),
                Some(Operation::CreateSketch { on: Some(_), .. })
            ),
            "a click on an invisible block anchors a sketch to it",
        );
    }
}

#[test]
fn an_invisible_body_offers_no_face_to_cut_along() {
    let on_the_top = DVec3::new(5.0, 5.0, 4.0);
    let clicked_with = |clicks: usize| {
        let document = a_part_holding_a_block();
        let mut section = Section::new(WorkPlane::XY);
        section.offset = 10.0;
        section.wait_for_a_face();
        let mut state = ViewportState::default();
        state.section = Some(section);
        for _ in 0..clicks {
            state.next_volume_mode(&document, THE_BLOCK_S_STEP);
        }
        a_click_from_above_while_the_section_waits(document, state, on_the_top)
    };
    let waiting = |state: &ViewportState| {
        state
            .section
            .as_ref()
            .expect("the section stays on")
            .is_waiting_for_a_face()
    };
    assert!(
        !waiting(&clicked_with(0)),
        "the fixture wants a click on the top of an opaque block to take it as the cutting face",
    );

    assert!(
        waiting(&clicked_with(2)),
        "a click on the top of an invisible block takes it as the cutting face",
    );
}

/// Where the post stands in the history of `a_block_and_a_post_beside_it`.
const THE_POST: usize = 4;

/// The block of `a_part_holding_a_block`, 10 by 10 and 4 high, and a post 4 by
/// 4 and 6 high beside it over x from 20 to 24: two volumes.
fn a_block_and_a_post_beside_it() -> PartDocument {
    let mut document = a_part_holding_a_block();
    raise(
        &mut document,
        0,
        &[(DVec2::new(20.0, 0.0), DVec2::new(24.0, 4.0))],
        6.0,
    );
    assert!(
        matches!(
            document.history.applied_operations().get(THE_POST),
            Some(Operation::Extrude { .. })
        ),
        "the fixture wants the post raised at step {THE_POST}",
    );
    document
}

/// The view with the post stepped `clicks` times round its states, the block
/// left opaque.
fn the_post_clicked(clicks: usize) -> ViewportState {
    let document = a_block_and_a_post_beside_it();
    let mut state = ViewportState::default();
    for _ in 0..clicks {
        state.next_volume_mode(&document, THE_POST);
    }
    state
}

/// Whether `face` of the document's own body stands square to X at `x`.
fn stands_square_to_x_at(document: &PartDocument, face: usize, x: f64) -> bool {
    let corners: Vec<DVec3> = document.body().triangles_of(face).flatten().collect();
    !corners.is_empty() && corners.iter().all(|at| (at.x - x).abs() < 1e-9)
}

#[test]
fn a_face_of_a_see_through_volume_is_offered_to_draw_on_beside_an_opaque_one() {
    let see_through = the_post_clicked(1);
    assert_eq!(
        see_through.volume_mode(&a_block_and_a_post_beside_it(), THE_POST),
        VolumeState::SeeThrough,
    );

    for (what, origin, height, (from, to)) in [
        (
            "the see-through post",
            DVec3::new(22.0, 2.0, 50.0),
            6.0,
            (20.0, 24.0),
        ),
        (
            "the opaque block",
            DVec3::new(5.0, 5.0, 50.0),
            4.0,
            (0.0, 10.0),
        ),
    ] {
        let started = a_sketch_started_seen(
            a_block_and_a_post_beside_it(),
            the_post_clicked(1),
            origin,
            DVec3::NEG_Z,
        );

        assert!(
            matches!(started.offered, Some(PlaneChoice::Face { .. })),
            "the top of {what} is offered as {:?}, not as a face",
            started.offered,
        );
        assert!(
            started.started,
            "a click on the top of {what} starts no sketch"
        );
        let face = anchored_face(&started.document);
        assert!(
            lies_flat_at(&started.document, face, height, (from, to)),
            "the sketch is anchored to face {face}, not the top of {what}",
        );
    }
}

#[test]
fn a_hidden_volume_offers_none_of_its_faces_and_the_face_behind_it_is_offered() {
    let started = a_sketch_started_seen(
        a_block_and_a_post_beside_it(),
        the_post_clicked(2),
        DVec3::new(22.0, 2.0, 50.0),
        DVec3::NEG_Z,
    );
    assert!(
        !matches!(
            started.offered,
            Some(PlaneChoice::Face { .. } | PlaneChoice::Curved(_) | PlaneChoice::Section(_))
        ),
        "the top of the invisible post is offered as {:?}",
        started.offered,
    );
    assert!(
        !matches!(
            started.document.history.operations().last(),
            Some(Operation::CreateSketch { on: Some(_), .. })
        ),
        "a click on the invisible post anchors a sketch to it",
    );

    let through = a_sketch_started_seen(
        a_block_and_a_post_beside_it(),
        the_post_clicked(2),
        DVec3::new(50.0, 2.0, 1.0),
        DVec3::NEG_X,
    );
    assert!(
        through.started,
        "a ray through the invisible post starts no sketch on the block behind it"
    );
    let face = anchored_face(&through.document);
    assert!(
        stands_square_to_x_at(&through.document, face, 10.0),
        "a ray through the invisible post is anchored to face {face}, not the block's side at x = 10",
    );
}

/// Where the U stands in the history of `a_block_and_a_u_beside_it`.
const THE_U: usize = 5;

/// A block 10 by 10 and 4 high at the origin, then a U 6 high over x from 20
/// to 30, its two arms 2 wide standing up along Y from a bar 2 deep: two
/// volumes. The U is the outer rectangle less the notch drawn over it.
fn a_block_and_a_u_beside_it() -> PartDocument {
    use cao_part::{ExtrusionMode, PointRef};

    let mut document = a_part_holding_a_block();
    for (corner, opposite) in [
        (DVec2::new(20.0, 0.0), DVec2::new(30.0, 10.0)),
        (DVec2::new(22.0, 2.0), DVec2::new(28.0, 12.0)),
    ] {
        document.apply(Operation::AddRectangle {
            sketch: 0,
            corner: PointRef::New(corner),
            opposite: PointRef::New(opposite),
            construction: false,
        });
    }
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::new(21.0, 1.0)]),
        distance: 6.0.into(),
        mode: ExtrusionMode::Add,
    });
    assert!(
        matches!(
            document.history.applied_operations().get(THE_U),
            Some(Operation::Extrude { .. })
        ),
        "the fixture wants the U raised at step {THE_U}",
    );
    document
}

/// The section along XZ through the U's two arms, the side they stand on
/// drawn: it parts the U's top and its floor, each in two, and numbers a piece
/// of each anew — the top's over the left arm, the floor's under the right.
fn across_the_u_s_arms() -> Section {
    let mut section = Section::new(WorkPlane::XZ);
    section.offset = -5.0;
    section
}

/// The view with `section` on and the U hidden, the block left opaque.
fn the_u_hidden_under(document: &PartDocument, section: Section) -> ViewportState {
    let mut state = ViewportState::default();
    state.section = Some(section);
    state.next_volume_mode(document, THE_U);
    state.next_volume_mode(document, THE_U);
    assert_eq!(state.volume_mode(document, THE_U), VolumeState::Invisible);
    let drawn = shown(&state, &SketchEditor::default(), document.body());
    let over_the_u = |piece: usize| {
        let corners: Vec<DVec3> = drawn.opaque.triangles_of(piece).flatten().collect();
        !corners.is_empty() && corners.iter().all(|corner| corner.x > 19.999)
    };
    assert!(
        drawn.opaque.pieces().into_iter().any(over_the_u),
        "the fixture wants the section to part a face of the U and number a piece anew",
    );
    state
}

#[test]
fn a_hidden_volume_the_section_parts_offers_no_piece_of_itself_to_draw_on() {
    for (what, origin, direction) in [
        (
            "the top of the left arm",
            DVec3::new(21.0, 7.0, 50.0),
            DVec3::NEG_Z,
        ),
        (
            "the floor of the right arm",
            DVec3::new(29.0, 7.0, -50.0),
            DVec3::Z,
        ),
    ] {
        let document = a_block_and_a_u_beside_it();
        let state = the_u_hidden_under(&document, across_the_u_s_arms());

        let started = a_sketch_started_seen(document, state, origin, direction);

        assert!(
            !matches!(
                started.offered,
                Some(PlaneChoice::Face { .. } | PlaneChoice::Curved(_))
            ),
            "{what} of the invisible U is offered as {:?}",
            started.offered,
        );
        assert!(
            !matches!(
                started.document.history.operations().last(),
                Some(Operation::CreateSketch { on: Some(_), .. })
            ),
            "a click on {what} of the invisible U anchors a sketch to it",
        );
    }
}

/// One click at `at` on a canvas seen from straight above, while the section
/// of `state` waits for a face: the view after it.
fn a_click_from_above_while_the_section_waits(
    mut document: PartDocument,
    mut state: ViewportState,
    at: DVec3,
) -> ViewportState {
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    let (yaw, pitch) = view_angles_towards(glam::Vec3::Z);
    state.camera.set_view_angles(yaw, pitch);
    let on_screen = on_the_screen(&state, at);
    let context = SketchContext {
        document: &mut document,
        editor: &mut editor,
        extrusion: &mut extrusion,
        lang: &lang,
    };

    let egui = egui::Context::default();
    let none = egui::Modifiers::NONE;
    for events in [
        vec![],
        vec![egui::Event::PointerMoved(on_screen)],
        pressed(on_screen, egui::PointerButton::Primary, none, true),
        pressed(on_screen, egui::PointerButton::Primary, none, false),
    ] {
        egui.begin_pass(egui::RawInput {
            screen_rect: Some(SCREEN),
            events,
            ..Default::default()
        });
        egui::Area::new(egui::Id::new("the canvas"))
            .fixed_pos(SCREEN.min)
            .show(&egui, |ui| {
                ui.set_min_size(SCREEN.size());
                let response = ui.interact(
                    SCREEN,
                    egui::Id::new("the canvas's surface"),
                    egui::Sense::click_and_drag(),
                );
                let scale = ViewScale::of(&state.camera, SCREEN, 1.0, &state.config, 1.0);
                point_the_section(ui, &mut state, &response, (SCREEN, scale), &context);
            });
        egui.end_pass().textures_delta.clear();
    }
    state
}

#[test]
fn a_hidden_volume_the_section_parts_offers_no_piece_of_itself_to_cut_along() {
    let document = a_block_and_a_u_beside_it();
    let mut waiting = across_the_u_s_arms();
    waiting.wait_for_a_face();
    let state = the_u_hidden_under(&document, waiting);

    let after =
        a_click_from_above_while_the_section_waits(document, state, DVec3::new(21.0, 7.0, 6.0));

    let section = after.section.as_ref().expect("the section stays on");
    let plane = section.tilted();
    let on_the_u_s_top = plane.normal().z.abs() > 1.0 - 1e-9
        && (plane.origin.z + plane.normal().z * section.offset - 6.0).abs() < 1e-6;
    assert!(
        section.is_waiting_for_a_face() || !on_the_u_s_top,
        "a click on the top of the invisible U's left arm takes it as the cutting face",
    );
}

/// Whether the part's own face numbered `face` holds `point`: a triangle it
/// is drawn with lies through the point and holds it within its outline.
fn the_part_s_face_holds(document: &PartDocument, face: usize, point: DVec3) -> bool {
    document.body().triangles_of(face).any(|[a, b, c]| {
        let normal = (b - a).cross(c - a);
        let on_the_plane = normal.dot(point - a).abs() <= 1e-9 * normal.length();
        let inside = [(a, b), (b, c), (c, a)]
            .iter()
            .all(|(from, to)| (*to - *from).cross(point - *from).dot(normal) >= -1e-9);
        on_the_plane && inside
    })
}

/// Rectangles drawn on the first sketch of `document`.
fn rectangles(document: &mut PartDocument, rectangles: &[(DVec2, DVec2)]) {
    use cao_part::PointRef;

    for &(corner, opposite) in rectangles {
        document.apply(Operation::AddRectangle {
            sketch: 0,
            corner: PointRef::New(corner),
            opposite: PointRef::New(opposite),
            construction: false,
        });
    }
}

/// The areas of the first sketch at these places raised or cut that far.
fn extruded(document: &mut PartDocument, inside: &[DVec2], distance: f64, cut: bool) {
    use cao_part::ExtrusionMode;

    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, inside),
        distance: distance.into(),
        mode: if cut {
            ExtrusionMode::Cut
        } else {
            ExtrusionMode::Add
        },
    });
}

/// Where the first block of `two_blocks_slotted_across` is raised.
const THE_LOW_BLOCK: usize = 4;

/// A block 10 by 10 and 4 high at the origin, a second beside it over x from
/// 10 to 20 and 6 high, then a slot 2 wide cut along X through both at
/// y from 4 to 6, all the way down: each top left in two pieces.
fn two_blocks_slotted_across() -> PartDocument {
    let mut document = PartDocument::new("part", chrono::Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    rectangles(
        &mut document,
        &[
            (DVec2::ZERO, DVec2::splat(10.0)),
            (DVec2::new(10.0, 0.0), DVec2::new(20.0, 10.0)),
            (DVec2::new(-1.0, 4.0), DVec2::new(21.0, 6.0)),
        ],
    );
    let across = |x: f64| [2.0, 5.0, 8.0].map(|y| DVec2::new(x, y));
    extruded(&mut document, &across(5.0), 4.0, false);
    extruded(&mut document, &across(15.0), 6.0, false);
    let slot = [-0.5, 5.0, 15.0, 20.5].map(|x| DVec2::new(x, 5.0));
    extruded(&mut document, &slot, 10.0, true);
    assert!(
        matches!(
            document.history.applied_operations().get(THE_LOW_BLOCK),
            Some(Operation::Extrude { .. })
        ),
        "the fixture wants the low block raised at step {THE_LOW_BLOCK}",
    );
    let (low, high) = document.body().bounds().expect("the blocks stand");
    assert!(
        low.distance(DVec3::ZERO) < 1e-9 && high.distance(DVec3::new(20.0, 10.0, 6.0)) < 1e-9,
        "the blocks span {low} to {high}",
    );
    document
}

#[test]
fn with_one_block_hidden_each_piece_of_the_other_s_slotted_top_anchors_on_it() {
    for piece in [DVec3::new(15.0, 2.0, 6.0), DVec3::new(15.0, 8.0, 6.0)] {
        let document = two_blocks_slotted_across();
        let mut state = ViewportState::default();
        state.next_volume_mode(&document, THE_LOW_BLOCK);
        state.next_volume_mode(&document, THE_LOW_BLOCK);
        assert_eq!(
            state.volume_mode(&document, THE_LOW_BLOCK),
            VolumeState::Invisible
        );

        let started = a_sketch_started_seen(document, state, piece + DVec3::Z * 50.0, DVec3::NEG_Z);

        let Some(PlaneChoice::Face { plane, .. }) = started.offered else {
            panic!(
                "the piece of the high block's top at {piece} is offered as {:?}",
                started.offered
            );
        };
        assert!(
            (plane.origin.z - 6.0).abs() < 1e-9 && plane.normal().z > 1.0 - 1e-9,
            "the piece at {piece} offers the plane {plane:?}, not the high top's",
        );
        let face = anchored_face(&started.document);
        assert!(
            the_part_s_face_holds(&started.document, face, piece),
            "the sketch started on the piece at {piece} is anchored to face {face}, which \
             does not hold it in the part",
        );
    }
}

/// Where the cut of `a_block_notched_past_its_top` is.
const THE_NOTCH: usize = 4;

/// A block 10 by 10 and 4 high at the origin, notched at its corner by a
/// square 10 by 10 from (5, 5) cut 10 up from XY: a cut that reaches past the
/// block above it and beside it. Both squares are drawn before either step,
/// so that the block is raised from both its areas.
fn a_block_notched_past_its_top() -> PartDocument {
    let mut document = PartDocument::new("part", chrono::Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    rectangles(
        &mut document,
        &[
            (DVec2::ZERO, DVec2::splat(10.0)),
            (DVec2::splat(5.0), DVec2::splat(15.0)),
        ],
    );
    extruded(
        &mut document,
        &[DVec2::splat(2.0), DVec2::splat(7.0)],
        4.0,
        false,
    );
    extruded(
        &mut document,
        &[DVec2::splat(7.0), DVec2::splat(12.0)],
        10.0,
        true,
    );
    assert!(
        matches!(
            document.history.applied_operations().get(THE_NOTCH),
            Some(Operation::Extrude { .. })
        ),
        "the fixture wants the notch cut at step {THE_NOTCH}",
    );
    assert!(
        (document.body().volume() - 300.0).abs() < 1e-6,
        "the notched block holds {}, not three quarters of 400",
        document.body().volume(),
    );
    document
}

/// The notched block with its cut clicked `clicks` times round its states.
fn the_notch_clicked(document: &PartDocument, clicks: usize) -> ViewportState {
    let mut state = ViewportState::default();
    for _ in 0..clicks {
        state.next_volume_mode(document, THE_NOTCH);
    }
    state
}

/// Whether what was offered is anything of the part: a face, flat or curved.
fn offers_the_part(started: &Started) -> bool {
    matches!(
        started.offered,
        Some(PlaneChoice::Face { .. } | PlaneChoice::Curved(_))
    ) || matches!(
        started.document.history.operations().last(),
        Some(Operation::CreateSketch { on: Some(_), .. })
    )
}

#[test]
fn a_see_through_cut_s_ghost_lets_the_ray_go_on_where_the_part_has_no_face() {
    let document = a_block_notched_past_its_top();
    let state = the_notch_clicked(&document, 1);
    assert_eq!(
        state.volume_mode(&document, THE_NOTCH),
        VolumeState::SeeThrough
    );

    let through = a_sketch_started_seen(
        a_block_notched_past_its_top(),
        state,
        DVec3::new(50.0, 7.0, 2.0),
        DVec3::NEG_X,
    );
    let face = anchored_face(&through.document);
    assert!(
        the_part_s_face_holds(&through.document, face, DVec3::new(5.0, 7.0, 2.0)),
        "a ray through the ghost's side, where the part has no face, is anchored to face \
         {face} rather than to the notch's wall behind it, at x = 5",
    );

    let above = a_sketch_started_seen(
        a_block_notched_past_its_top(),
        the_notch_clicked(&a_block_notched_past_its_top(), 1),
        DVec3::new(12.0, 12.0, 50.0),
        DVec3::NEG_Z,
    );
    assert!(
        !offers_the_part(&above),
        "the ghost's top, beside the block, where the part has no face, is offered as {:?}",
        above.offered,
    );
}

#[test]
fn the_top_an_invisible_cut_fills_in_offers_nothing_where_the_part_has_none() {
    let document = a_block_notched_past_its_top();
    let state = the_notch_clicked(&document, 2);
    assert_eq!(
        state.volume_mode(&document, THE_NOTCH),
        VolumeState::Invisible
    );

    let started = a_sketch_started_seen(document, state, DVec3::new(7.0, 7.0, 50.0), DVec3::NEG_Z);

    assert!(
        !offers_the_part(&started),
        "the top filled in over the notch, which the part does not have there, is offered as \
         {:?}",
        started.offered,
    );
}

/// Where the long block of `a_cylinder_beside_a_long_block` is raised.
const THE_LONG_BLOCK: usize = 4;

/// A cylinder of radius 10 and 20 high on the origin, and beside it a block
/// from x = 15 to 100, 10 wide and as high: two volumes, a flat face standing
/// behind the cylinder's wall for a ray along X.
fn a_cylinder_beside_a_long_block() -> PartDocument {
    use cao_part::PointRef;

    let mut document = PartDocument::new("part", chrono::Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddCircle {
        sketch: 0,
        center: PointRef::New(DVec2::ZERO),
        radius: 10.0,
        rim: Vec::new(),
        construction: false,
    });
    rectangles(
        &mut document,
        &[(DVec2::new(15.0, -5.0), DVec2::new(100.0, 5.0))],
    );
    extruded(&mut document, &[DVec2::ZERO], 20.0, false);
    extruded(&mut document, &[DVec2::new(50.0, 0.0)], 20.0, false);
    assert!(
        matches!(
            document.history.applied_operations().get(THE_LONG_BLOCK),
            Some(Operation::Extrude { .. })
        ),
        "the fixture wants the long block raised at step {THE_LONG_BLOCK}",
    );
    document
}

/// A ray along X onto the cylinder's wall, the long block behind it.
const ONTO_THE_WALL: (DVec3, DVec3) = (DVec3::new(-50.0, 4.3, 5.0), DVec3::X);

fn assert_the_wall_refuses(started: &Started, what: &str) {
    assert!(
        matches!(started.offered, Some(PlaneChoice::Curved(_))),
        "{what}, the cylinder's wall is offered as {:?}, not refused as a curved face",
        started.offered,
    );
    assert!(
        !started.started,
        "{what}, a click on the cylinder's wall starts a sketch"
    );
}

#[test]
fn a_curved_wall_under_a_section_still_refuses_rather_than_letting_the_ray_through() {
    let mut section = Section::new(WorkPlane::XY);
    section.offset = 10.0;

    let (origin, direction) = ONTO_THE_WALL;
    let started = a_sketch_started(a_cylinder_beside_a_long_block(), section, origin, direction);

    assert_the_wall_refuses(&started, "under a section at mid-height");
}

#[test]
fn a_curved_wall_of_a_part_shown_still_refuses_rather_than_letting_the_ray_through() {
    let document = a_cylinder_beside_a_long_block();
    let mut state = ViewportState::default();
    state.next_volume_mode(&document, THE_LONG_BLOCK);
    state.next_volume_mode(&document, THE_LONG_BLOCK);
    assert_eq!(
        state.volume_mode(&document, THE_LONG_BLOCK),
        VolumeState::Invisible
    );

    let (origin, direction) = ONTO_THE_WALL;
    let started = a_sketch_started_seen(document, state, origin, direction);

    assert_the_wall_refuses(&started, "the long block invisible");
}

/// Where the pocket of `a_block_pocketed_from_halfway_up` is cut.
const THE_POCKET: usize = 5;

/// The block of `a_part_holding_a_block`, 10 by 10 and 4 high, and a sketch on
/// XY lifted to z = 2 from which a square 10 by 10 from (5, 5) is cut 10 up:
/// a pocket in the block's corner whose floor, at z = 2, is a face of the
/// part, and a cut reaching far past the block above it and beside it.
fn a_block_pocketed_from_halfway_up() -> PartDocument {
    use cao_part::{ExtrusionMode, PointRef};

    let mut document = a_part_holding_a_block();
    document.apply(Operation::CreateSketch {
        plane: WorkPlane {
            origin: DVec3::Z * 2.0,
            ..WorkPlane::XY
        },
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 1,
        corner: PointRef::New(DVec2::splat(5.0)),
        opposite: PointRef::New(DVec2::splat(15.0)),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch: 1,
        areas: document.areas_at(1, &[DVec2::splat(10.0)]),
        distance: 10.0.into(),
        mode: ExtrusionMode::Cut,
    });
    assert!(
        matches!(
            document.history.applied_operations().get(THE_POCKET),
            Some(Operation::Extrude { .. })
        ),
        "the fixture wants the pocket cut at step {THE_POCKET}",
    );
    assert!(
        (document.body().volume() - 350.0).abs() < 1e-6,
        "the pocketed block holds {}, not 400 less a pocket 5 by 5 by 2",
        document.body().volume(),
    );
    document
}

#[test]
fn a_section_laid_on_the_cut_of_a_ghost_follows_no_face_of_the_part() {
    let document = a_block_pocketed_from_halfway_up();
    let mut waiting = Section::new(WorkPlane::XY);
    waiting.offset = 7.0;
    waiting.wait_for_a_face();
    let mut state = ViewportState::default();
    state.next_volume_mode(&document, THE_POCKET);
    state.section = Some(waiting);
    assert_eq!(
        state.volume_mode(&document, THE_POCKET),
        VolumeState::SeeThrough
    );

    let after =
        a_click_from_above_while_the_section_waits(document, state, DVec3::new(12.0, 12.0, 7.0));

    let section = after.section.as_ref().expect("the section stays on");
    assert!(
        !section.is_waiting_for_a_face(),
        "the fixture wants the cut of the ghost, flat on the section's plane, taken as the \
         cutting face",
    );
    assert_eq!(
        section.on.map(|anchor| anchor.face),
        None,
        "a section laid on the cut of the ghost, which the part has nothing of, follows a face \
         of the part",
    );
}
