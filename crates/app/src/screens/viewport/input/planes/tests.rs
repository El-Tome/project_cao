//! What the part offers to draw on, face by face.
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

#[test]
fn a_curved_wall_offers_no_plane_to_draw_on() {
    let body = cylinder(10.0, 20.0);

    let offered = what_the_part_offers(&body, DVec3::new(40.0, 0.0, 10.0), DVec3::NEG_X)
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

    let offered = what_the_part_offers(&body, DVec3::new(0.0, 0.0, 50.0), DVec3::NEG_Z)
        .expect("the top is there to be hit");

    let plane = offered.plane().expect("a flat face offers its plane");
    assert!(
        plane.normal().dot(DVec3::Z).abs() > 0.99,
        "the top of a cylinder faces up, not {:?}",
        plane.normal(),
    );
}
