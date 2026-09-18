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

use glam::{DVec2, DVec3};

use super::*;
use cao_render::OrbitCamera;

/// Which way is up on screen once the view has swung onto a face — the same
/// answer `once_facing` gives, without a viewport to ask.
fn facing(normal: DVec3) -> DVec3 {
    let mut camera = OrbitCamera::default();
    let (yaw, pitch) = view_angles_towards(normal.as_vec3());
    camera.set_view_angles(yaw, pitch);
    camera.up().as_dvec3()
}

/// A disc pushed along its own normal: flat ends, and a wall of many flats
/// that is one curved face.
fn cylinder(radius: f64, height: f64) -> Mesh {
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
    cao_solid::prism(
        cao_solid::Loop {
            points: &points,
            curves: &curves,
        },
        &[],
        &triangles,
        |point| DVec3::new(point.x, point.y, 0.0),
        DVec3::Z * height,
    )
}

/// A block, whose top is one flat face stored as two triangles.
fn block(width: f64, depth: f64, height: f64) -> Mesh {
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
    cao_solid::prism(
        cao_solid::Loop::straight(&points),
        &[],
        &triangles,
        |point| DVec3::new(point.x, point.y, 0.0),
        DVec3::Z * height,
    )
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

    let corners: Vec<DVec3> = body
        .pieces_of(face)
        .flat_map(|piece| piece.corners.iter().copied())
        .collect();
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
