//! An ellipse of a profile raised (#571): its wall an elliptic cylinder, its
//! rims the sections its caps cut, whole or in arcs, its walls numbered as
//! the flats number them.

use std::f64::consts::{FRAC_PI_2, PI, TAU};

use glam::{DVec2, DVec3};

use super::{assert_sound, counts, ground};
use crate::brep::Declined;
use crate::brep::conic::Arm;
use crate::brep::curve::Curve;
use crate::brep::surface::Surface;
use crate::brep::topology::{Body, FaceId};
use crate::profile::{Contour, Frame, Run};
use crate::soundness::{closed, enclosed, listed, uncrossed};

const CENTER: DVec2 = DVec2::new(3.0, -1.0);

/// A whole ellipse of half-axes 7 and 3 about `CENTER`, its first half-axis
/// turned by `turned`, walked from its end anticlockwise once round.
fn ellipse(turned: f64, first: f64, second: f64) -> Contour {
    let axis = DVec2::from_angle(turned) * first;
    Contour {
        corners: vec![CENTER + axis],
        runs: vec![Run::Oval {
            center: CENTER,
            first: axis,
            second,
            turn: TAU,
        }],
    }
}

/// Where the ellipse of `oval` stands at the eccentric angle `angle`.
fn on(first: DVec2, second: f64, angle: f64) -> DVec2 {
    CENTER + first * angle.cos() + first.perp().normalize() * second * angle.sin()
}

fn wall_faces(body: &Body) -> Vec<FaceId> {
    body.face_ids()
        .filter(|&face| matches!(body.surface(body.face(face).surface), Surface::Elliptic(_)))
        .collect()
}

fn held(body: &Body, volume: f64) {
    assert_eq!(listed(&body.listing(), body.scale().reach()), Ok(()));
    assert_sound(body);
    assert!(
        (body.volume() - volume).abs() <= 1e-12 * volume,
        "the body holds {}, the arithmetic {volume}",
        body.volume()
    );
}

#[test]
fn an_ellipse_raised_is_three_faces_two_whole_rims_and_no_corner() {
    let body = Body::raised(&ellipse(0.4, 7.0, 3.0), &[], ground(), DVec3::Z * 10.0)
        .expect("an ellipse raises");
    assert_eq!(counts(&body), [3, 2, 0]);
    assert!(body.edges.iter().all(|edge| edge.ends.is_none()));
    assert!(body.curves.iter().all(|curve| matches!(
        curve,
        Curve::Conic(conic) if conic.arm == Arm::Whole && (conic.k - 1.0 / 9.0).abs() < 1e-15
    )));
    let walls = wall_faces(&body);
    assert_eq!(walls.len(), 1);
    assert_eq!(body.face(walls[0]).numbers, vec![2]);
    held(&body, PI * 7.0 * 3.0 * 10.0);
}

#[test]
fn an_ellipse_drawn_from_its_minor_half_axis_or_backwards_raises_the_same_solid() {
    let volume = PI * 7.0 * 3.0 * 10.0;
    let minor_first = ellipse(0.4 + FRAC_PI_2, 3.0, 7.0);
    let backwards = Contour {
        runs: vec![Run::Oval {
            center: CENTER,
            first: DVec2::from_angle(0.4) * 7.0,
            second: 3.0,
            turn: -TAU,
        }],
        ..ellipse(0.4, 7.0, 3.0)
    };
    for contour in [minor_first, backwards] {
        let body = Body::raised(&contour, &[], ground(), DVec3::Z * 10.0).expect("it raises");
        held(&body, volume);
    }
    let down = Body::raised(&ellipse(0.4, 7.0, 3.0), &[], ground(), -DVec3::Z * 10.0)
        .expect("it raises below its plane");
    held(&down, volume);
}

#[test]
fn an_ellipse_whose_half_axes_are_one_within_the_tolerance_is_raised_as_a_circular_cylinder() {
    let body = Body::raised(
        &ellipse(0.4, 5.0, 5.0 + 1e-12),
        &[],
        ground(),
        DVec3::Z * 10.0,
    )
    .expect("a circle drawn as an ellipse raises");
    assert!(wall_faces(&body).is_empty());
    assert!(body.surfaces.iter().any(|surface| matches!(
        surface,
        Surface::Cylinder(cylinder) if (cylinder.radius - 5.0).abs() < 1e-11
    )));
    held(&body, PI * 25.0 * 10.0);
}

#[test]
fn an_elliptic_hole_in_a_block_is_a_wall_with_its_matter_outside() {
    let block = Contour::rectangle(DVec2::new(-10.0, -10.0), DVec2::new(15.0, 10.0));
    let body = Body::raised(&block, &[ellipse(0.4, 7.0, 3.0)], ground(), DVec3::Z * 4.0)
        .expect("a block with an elliptic hole raises");
    let walls = wall_faces(&body);
    assert_eq!(walls.len(), 1);
    assert!(body.face(walls[0]).flipped);
    assert_eq!(body.face(walls[0]).numbers, vec![6]);
    held(&body, (25.0 * 20.0 - PI * 21.0) * 4.0);
}

/// A slot whose two ends are halves of two ellipses, one of half-axes 4
/// along the slot and 3 across it, the other 2 along and 3 across.
fn slot_with_elliptic_ends() -> Contour {
    let [left, right] = [DVec2::new(-5.0, 0.0), DVec2::new(5.0, 0.0)];
    Contour {
        corners: vec![
            DVec2::new(-5.0, -3.0),
            DVec2::new(5.0, -3.0),
            DVec2::new(5.0, 3.0),
            DVec2::new(-5.0, 3.0),
        ],
        runs: vec![
            Run::Straight,
            Run::Oval {
                center: right,
                first: DVec2::new(0.0, -3.0),
                second: 4.0,
                turn: PI,
            },
            Run::Straight,
            Run::Oval {
                center: left,
                first: DVec2::new(0.0, 3.0),
                second: 2.0,
                turn: PI,
            },
        ],
    }
}

/// The same slot, its left end a half circle of radius 3.
fn slot_with_a_round_and_an_elliptic_end() -> Contour {
    let mut slot = slot_with_elliptic_ends();
    slot.runs[3] = Run::Round {
        center: DVec2::new(-5.0, 0.0),
        turn: PI,
    };
    slot
}

#[test]
fn a_slot_with_elliptic_ends_numbers_a_wall_for_each_run() {
    let body = Body::raised(&slot_with_elliptic_ends(), &[], ground(), DVec3::Z * 2.0)
        .expect("a slot with elliptic ends raises");
    let mut numbers: Vec<Vec<u32>> = body
        .face_ids()
        .map(|face| body.face(face).numbers.clone())
        .collect();
    numbers.sort();
    assert_eq!(numbers, [[0], [1], [2], [3], [4], [5]]);
    assert_eq!(wall_faces(&body).len(), 2);
    let area = 10.0 * 6.0 + PI * 3.0 * 4.0 / 2.0 + PI * 3.0 * 2.0 / 2.0;
    held(&body, area * 2.0);
}

#[test]
fn two_arcs_of_one_ellipse_make_one_wall_answering_to_both_runs() {
    let first = DVec2::from_angle(0.4) * 7.0;
    let halves = Contour {
        corners: vec![on(first, 3.0, 0.0), on(first, 3.0, PI)],
        runs: vec![
            Run::Oval {
                center: CENTER,
                first,
                second: 3.0,
                turn: PI,
            },
            Run::Oval {
                center: CENTER,
                first,
                second: 3.0,
                turn: PI,
            },
        ],
    };
    let body = Body::raised(&halves, &[], ground(), DVec3::Z * 10.0).expect("two halves raise");
    assert_eq!(counts(&body), [3, 2, 0]);
    let walls = wall_faces(&body);
    assert_eq!(body.face(walls[0]).numbers, vec![2, 3]);
    held(&body, PI * 21.0 * 10.0);

    let cut_by_a_trait = Contour {
        corners: vec![
            on(first, 3.0, -0.5),
            on(first, 3.0, 1.0),
            on(first, 3.0, 2.5),
        ],
        runs: vec![
            Run::Oval {
                center: CENTER,
                first,
                second: 3.0,
                turn: 1.5,
            },
            Run::Oval {
                center: CENTER,
                first,
                second: 3.0,
                turn: 1.5,
            },
            Run::Straight,
        ],
    };
    let body = Body::raised(&cut_by_a_trait, &[], ground(), DVec3::Z * 10.0).expect("a cap raises");
    let walls = wall_faces(&body);
    assert_eq!(walls.len(), 1);
    assert_eq!(body.face(walls[0]).numbers, vec![2, 3]);
    let sweep = 3.0;
    let cap = 21.0 * (sweep - f64::sin(sweep)) / 2.0;
    held(&body, cap * 10.0);
}

#[test]
fn a_profile_raised_in_a_leaning_frame_with_an_ellipse_is_sound() {
    let normal = DVec3::new(0.3, -0.4, 1.0).normalize();
    let u = normal.cross(DVec3::X).normalize();
    let frame = Frame {
        origin: DVec3::new(40.0, -25.0, 12.0),
        u,
        v: normal.cross(u),
    };
    let body = Body::raised(
        &slot_with_elliptic_ends(),
        &[ellipse(1.0, 1.5, 0.5)],
        frame,
        normal * 3.0,
    )
    .expect("a slot with an elliptic hole raises in a leaning frame");
    let area = 10.0 * 6.0 + PI * 3.0 * 4.0 / 2.0 + PI * 3.0 * 2.0 / 2.0 - PI * 1.5 * 0.5;
    held(&body, area * 3.0);
}

#[test]
fn an_ellipse_touching_another_loop_of_its_profile_is_declined() {
    let block = Contour::rectangle(DVec2::new(-10.0, -10.0), DVec2::new(15.0, 10.0));
    let touching_the_side = Contour {
        corners: vec![DVec2::new(15.0, 0.0)],
        runs: vec![Run::Oval {
            center: DVec2::new(8.0, 0.0),
            first: DVec2::new(7.0, 0.0),
            second: 3.0,
            turn: TAU,
        }],
    };
    let touching_the_other = Contour {
        corners: vec![DVec2::new(13.0, -1.0)],
        runs: vec![Run::Oval {
            center: DVec2::new(11.5, -1.0),
            first: DVec2::new(1.5, 0.0),
            second: 1.0,
            turn: TAU,
        }],
    };
    for holes in [
        vec![touching_the_side],
        vec![ellipse(0.0, 7.0, 3.0), touching_the_other],
    ] {
        assert_eq!(
            Body::raised(&block, &holes, ground(), DVec3::Z * 4.0).map(|_| ()),
            Err(Declined::Profile)
        );
    }
}

#[test]
fn a_stretch_of_an_ellipse_run_both_ways_is_declined() {
    let mut outline = slot_with_elliptic_ends();
    outline.runs[3] = Run::Straight;
    let back_along_its_end = Contour {
        corners: vec![DVec2::new(5.0, 3.0), DVec2::new(5.0, -3.0)],
        runs: vec![
            Run::Oval {
                center: DVec2::new(5.0, 0.0),
                first: DVec2::new(0.0, 3.0),
                second: 4.0,
                turn: -PI,
            },
            Run::Straight,
        ],
    };
    assert_eq!(
        Body::raised(&outline, &[back_along_its_end], ground(), DVec3::Z * 2.0).map(|_| ()),
        Err(Declined::Profile)
    );
}

#[test]
fn an_oval_run_whose_ends_miss_its_ellipse_is_declined() {
    let astray = Contour {
        corners: vec![
            CENTER + DVec2::new(7.0, 1e-3),
            CENTER - DVec2::new(7.0, 0.0),
        ],
        runs: vec![
            Run::Oval {
                center: CENTER,
                first: DVec2::new(7.0, 0.0),
                second: 3.0,
                turn: PI,
            },
            Run::Straight,
        ],
    };
    assert_eq!(
        Body::raised(&astray, &[], ground(), DVec3::Z * 4.0).map(|_| ()),
        Err(Declined::Profile)
    );
}

#[test]
fn the_triangles_of_an_elliptic_prism_close_and_enclose_its_volume() {
    let block = Contour::rectangle(DVec2::new(-10.0, -10.0), DVec2::new(15.0, 10.0));
    for (outline, holes, volume) in [
        (ellipse(0.4, 7.0, 3.0), vec![], PI * 21.0 * 4.0),
        (
            block,
            vec![ellipse(0.4, 7.0, 3.0)],
            (500.0 - PI * 21.0) * 4.0,
        ),
        (slot_with_elliptic_ends(), vec![], (60.0 + 9.0 * PI) * 4.0),
        (
            slot_with_a_round_and_an_elliptic_end(),
            vec![ellipse(1.0, 1.5, 0.5)],
            (60.0 + 6.0 * PI + 4.5 * PI - 0.75 * PI) * 4.0,
        ),
    ] {
        let body = Body::raised(&outline, &holes, ground(), DVec3::Z * 4.0).expect("it raises");
        for tolerance in [1e-3, 0.02, 0.5] {
            let triangles = body.triangles(tolerance);
            closed(&triangles).expect("the triangles close");
            uncrossed(&triangles).expect("no triangle crosses another");
            let drawn = enclosed(&triangles);
            let wall = 2.0 * PI * 7.0 * 4.0;
            assert!(
                (drawn - volume).abs() <= tolerance * wall,
                "{drawn} drawn for {volume} at {tolerance}"
            );
        }
    }
}

#[test]
fn a_ray_winds_once_inside_an_elliptic_prism_and_never_outside() {
    let body = Body::raised(&ellipse(0.4, 7.0, 3.0), &[], ground(), DVec3::Z * 4.0)
        .expect("an ellipse raises");
    let eps = body.scale().eps();
    let first = DVec2::from_angle(0.4) * 7.0;
    for (angle, inside) in [(0.3, 0.99), (2.0, 0.5), (4.0, 0.9), (0.3, 1.01), (5.5, 1.3)] {
        let place = CENTER + (on(first, 3.0, angle) - CENTER) * inside;
        let winding = body
            .winding(place.extend(2.0), eps)
            .expect("a ray settles it");
        assert_eq!(winding, i32::from(inside < 1.0), "at {angle} by {inside}");
    }
}
