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

use glam::{DVec2, DVec3};

use super::*;
use crate::lang::Catalogue;
use crate::screens::extrusion::ExtrusionState;
use crate::screens::sketch::SketchEditor;
use crate::screens::viewport::section::Section;
use cao_part::PartDocument;
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
    use cao_part::{ExtrusionMode, PointRef};

    let mut document = PartDocument::new("part", chrono::Utc::now());
    document.apply(Operation::CreateSketch {
        plane: WorkPlane::XY,
        on: None,
    });
    document.apply(Operation::AddRectangle {
        sketch: 0,
        corner: PointRef::New(DVec2::ZERO),
        opposite: PointRef::New(DVec2::new(10.0, 10.0)),
        construction: false,
    });
    document.apply(Operation::Extrude {
        sketch: 0,
        areas: document.areas_at(0, &[DVec2::new(5.0, 5.0)]),
        distance: 4.0.into(),
        mode: ExtrusionMode::Add,
    });
    document
}

/// What happened once a plane was chosen for a new sketch, the block cut by a
/// section through XY at mid-height, its upper half hidden.
struct Started {
    state: ViewportState,
    document: PartDocument,
    editor: SketchEditor,
    offered: Option<PlaneChoice>,
    started: bool,
}

/// The cursor brought over a place, then a click there.
fn a_sketch_started_through_a_section(origin: DVec3, direction: DVec3) -> Started {
    let mut document = a_part_holding_a_block();
    let mut editor = SketchEditor::default();
    let mut extrusion = ExtrusionState::default();
    let lang = Catalogue::french();
    editor.start_choosing_plane(&lang);
    let mut section = Section::new(WorkPlane::XY);
    section.offset = 2.0;
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
    let Some(Operation::CreateSketch { plane, on }) = document.history.operations().last() else {
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
