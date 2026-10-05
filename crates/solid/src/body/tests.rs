//! What a body answers about the matter it holds, whichever kernel computed
//! it.

use std::f64::consts::{PI, TAU};

use glam::{DVec2, DVec3};

use super::*;
use crate::profile::{Contour, Frame, Run};

const FLAT: Frame = Frame {
    origin: DVec3::ZERO,
    u: DVec3::X,
    v: DVec3::Y,
};

const TOP: usize = 1;

fn exact(outline: Contour, holes: Vec<Contour>) -> Profile<'static> {
    Profile {
        exact: Some((outline, holes)),
        sampled: Loop::straight(&[]),
        sampled_holes: Vec::new(),
        triangles: &[],
    }
}

fn disc(center: DVec2, radius: f64) -> Contour {
    Contour {
        corners: vec![center + DVec2::X * radius],
        runs: vec![Run::Round { center, turn: TAU }],
    }
}

fn raised(part: &Body, profile: &Profile, height: f64) -> Body {
    part.tool_raised(profile, FLAT, DVec3::Z * height)
        .expect("the kernel raises it")
}

#[test]
fn a_circle_raised_encloses_the_arithmetics_volume() {
    let body = raised(
        &Body::default(),
        &exact(disc(DVec2::ZERO, 20.0), vec![]),
        10.0,
    );

    let arithmetic = PI * 20.0 * 20.0 * 10.0;
    assert!(
        (body.volume() - arithmetic).abs() < 1e-9 * arithmetic,
        "{} against {arithmetic}",
        body.volume(),
    );
}

fn block(low: DVec2, high: DVec2, height: f64) -> Body {
    raised(
        &Body::default(),
        &exact(Contour::rectangle(low, high), vec![]),
        height,
    )
}

fn bored_block() -> Body {
    let block = block(DVec2::ZERO, DVec2::new(30.0, 20.0), 10.0);
    let bore = raised(
        &block,
        &exact(disc(DVec2::new(15.0, 10.0), 4.0), vec![]),
        10.0,
    );
    block.difference(&bore).expect("the kernel bores it")
}

#[test]
fn every_triangle_drawn_belongs_to_a_face_of_the_exact_body() {
    let body = bored_block();

    let drawn = body.triangles();
    let by_face: Vec<[DVec3; 3]> = (0..body.faces_end())
        .flat_map(|face| body.triangles_of(face).collect::<Vec<_>>())
        .collect();

    assert!(!drawn.is_empty());
    assert_eq!(by_face.len(), drawn.len(), "each triangle on one face");
    assert!(drawn.iter().all(|triangle| by_face.contains(triangle)));
    assert!(
        (0..body.faces_end())
            .all(|face| !body.has_face(face) || body.triangles_of(face).count() > 0),
        "every face the body has is drawn",
    );
    assert_eq!(body.triangles_of(body.faces_end()).count(), 0);
}

#[test]
fn a_ray_meets_the_face_the_exact_body_has_there_with_its_exact_normal() {
    let post = raised(
        &Body::default(),
        &exact(disc(DVec2::ZERO, 10.0), vec![]),
        10.0,
    );
    let wall = 2;

    let hit = post
        .ray_hit(DVec3::new(20.0, 0.3, 5.0), DVec3::NEG_X)
        .expect("the post's wall");

    let on_the_circle = DVec3::new((100.0f64 - 0.09).sqrt(), 0.3, 0.0);
    assert_eq!(hit.face, wall);
    assert!(
        (hit.distance - (20.0 - on_the_circle.x)).abs() < 1e-3 * 10.0,
        "{}",
        hit.distance,
    );
    assert!(
        hit.normal.dot(on_the_circle / 10.0) > (1e-4f64).cos(),
        "the circle's own normal, not a facet's: {}",
        hit.normal,
    );

    let top = post
        .ray_hit(DVec3::new(1.0, 2.0, 30.0), DVec3::NEG_Z)
        .expect("the post's top");
    assert_eq!(top.face, TOP);
    assert_eq!(top.normal, DVec3::Z);
}

#[test]
fn a_ray_inside_a_bore_meets_its_wall_facing_back_at_it() {
    let body = bored_block();
    let center = DVec3::new(15.0, 10.0, 5.0);

    let hit = body.ray_hit(center, DVec3::X).expect("the bore's wall");

    assert!(body.has_face(hit.face) && !body.is_flat(hit.face));
    assert!(
        hit.normal.dot(DVec3::NEG_X) > (1e-4f64).cos(),
        "{}",
        hit.normal
    );
}

#[test]
fn the_top_of_a_raised_profile_offers_its_exact_plane_and_its_corners() {
    let post = raised(
        &Body::default(),
        &exact(disc(DVec2::ZERO, 20.0), vec![]),
        10.0,
    );

    let plane = post.plane_of(TOP).expect("the post has a top");

    assert!(post.is_flat(TOP));
    assert_eq!(plane.normal, DVec3::Z);
    for rim in [
        DVec3::new(20.0, 0.0, 10.0),
        DVec3::new(0.0, 20.0, 10.0),
        DVec3::new(-20.0, 0.0, 10.0),
        DVec3::new(0.0, -20.0, 10.0),
    ] {
        assert!(
            plane
                .corners
                .iter()
                .any(|corner| corner.distance(rim) < 1e-12),
            "{rim} among {:?}",
            plane.corners,
        );
    }
    assert!(plane.corners.iter().all(|corner| corner.z == 10.0));
    assert_eq!(post.plane_of(post.faces_end()), None);
}

#[test]
fn a_wall_raised_from_an_arc_is_not_flat() {
    let post = raised(
        &Body::default(),
        &exact(disc(DVec2::ZERO, 20.0), vec![]),
        10.0,
    );
    let wall = 2;

    assert!(post.has_face(wall));
    assert!(!post.is_flat(wall));
    assert_eq!(post.plane_of(wall), None);
}

fn polygon_area(points: &[DVec2]) -> f64 {
    (0..points.len())
        .map(|index| points[index].perp_dot(points[(index + 1) % points.len()]))
        .sum::<f64>()
        / 2.0
}

fn sampled_disc(center: DVec2, radius: f64) -> (Vec<DVec2>, Vec<Option<usize>>, Vec<[DVec2; 3]>) {
    let points: Vec<DVec2> = (0..48)
        .map(|step| center + DVec2::from_angle(TAU * step as f64 / 48.0) * radius)
        .collect();
    let curves = vec![Some(0); points.len()];
    let triangles = (1..points.len() - 1)
        .map(|index| [points[0], points[index], points[index + 1]])
        .collect();
    (points, curves, triangles)
}

fn ring_turned_about_y() -> Body {
    let outline = [
        DVec2::new(30.0, 0.0),
        DVec2::new(34.0, 0.0),
        DVec2::new(34.0, 4.0),
        DVec2::new(30.0, 4.0),
    ];
    let triangles = [
        [outline[0], outline[1], outline[2]],
        [outline[0], outline[2], outline[3]],
    ];
    Body::revolution(
        Loop::straight(&outline),
        &[],
        &triangles,
        |point| DVec3::new(point.x, point.y, 0.0),
        DVec2::ZERO,
        DVec2::Y,
        TAU,
    )
    .expect("a ring on one side of its axis")
}

#[test]
fn a_revolution_joined_to_an_exact_body_is_flats_from_then_on() {
    let block = block(DVec2::ZERO, DVec2::splat(10.0), 10.0);
    let ring = ring_turned_about_y();
    let part = block.union(&ring).expect("the flats never decline");

    assert!(
        (part.volume() - (1000.0 + ring.volume())).abs() < 1e-6,
        "{} against {}",
        part.volume(),
        1000.0 + ring.volume(),
    );

    let (points, curves, triangles) = sampled_disc(DVec2::splat(5.0), 3.0);
    let profile = Profile {
        exact: Some((disc(DVec2::splat(5.0), 3.0), vec![])),
        sampled: Loop {
            points: &points,
            curves: &curves,
        },
        sampled_holes: Vec::new(),
        triangles: &triangles,
    };
    let boss = part
        .tool_raised(&profile, FLAT, DVec3::Z * 20.0)
        .expect("the flats never decline");
    let flats = polygon_area(&points) * 20.0;
    assert!(
        (boss.volume() - flats).abs() < 1e-9 * flats,
        "raised by the flats: {} against {flats}",
        boss.volume(),
    );
}

#[test]
fn a_profile_the_exact_kernel_cannot_raise_is_raised_by_the_flats() {
    let part = block(DVec2::ZERO, DVec2::splat(10.0), 10.0);
    let (points, curves, triangles) = sampled_disc(DVec2::splat(5.0), 3.0);
    let oval = Profile {
        exact: None,
        sampled: Loop {
            points: &points,
            curves: &curves,
        },
        sampled_holes: Vec::new(),
        triangles: &triangles,
    };

    let boss = part
        .tool_raised(&oval, FLAT, DVec3::Z * 20.0)
        .expect("the flats never decline");
    let joined = part.union(&boss).expect("the flats never decline");

    let flats = polygon_area(&points) * 10.0;
    assert!(
        (joined.volume() - (1000.0 + flats)).abs() < 1e-6,
        "{} against {}",
        joined.volume(),
        1000.0 + flats,
    );
}

#[test]
fn an_exact_body_behind_a_plane_keeps_only_what_is_behind_it() {
    let body = bored_block();

    let behind = body.behind(DVec3::Z, 4.0);

    let (low, high) = behind.bounds().expect("something is left");
    assert!(high.z <= 4.0 + 1e-9 && low.z >= -1e-9, "{low} {high}");
    assert!((high.x - 30.0).abs() < 1e-9 && (high.y - 20.0).abs() < 1e-9);
    let bore = behind
        .ray_hit(DVec3::new(15.0, 10.0, 2.0), DVec3::X)
        .expect("the bore's wall, below the plane");
    assert!(
        (bore.distance - 4.0).abs() < 1e-3 * 30.0,
        "{}",
        bore.distance
    );
    assert_eq!(behind.ray_hit(DVec3::new(15.0, 10.0, 6.0), DVec3::X), None);
}

#[test]
fn a_disc_s_box_reaches_its_rim_exactly() {
    let post = raised(
        &Body::default(),
        &exact(disc(DVec2::ZERO, 20.0), vec![]),
        10.0,
    );

    let (low, high) = post.bounds().expect("the post is there");

    assert!(low.distance(DVec3::new(-20.0, -20.0, 0.0)) < 1e-12, "{low}");
    assert!(
        high.distance(DVec3::new(20.0, 20.0, 10.0)) < 1e-12,
        "{high}"
    );
}

#[test]
fn a_slant_across_a_bore_is_handed_back_as_a_decline() {
    let part = block(DVec2::ZERO, DVec2::splat(20.0), 10.0);
    let (sin, cos) = (30.0f64).to_radians().sin_cos();
    let slanted = Frame {
        origin: DVec3::new(10.0, 10.0, -5.0),
        u: DVec3::X,
        v: DVec3::new(0.0, cos, sin),
    };
    let bore = part
        .tool_raised(
            &exact(disc(DVec2::ZERO, 3.0), vec![]),
            slanted,
            slanted.normal() * 20.0,
        )
        .expect("a bore stands on its own");

    assert_eq!(part.difference(&bore), Err(Declined::Unsupported));
}

#[test]
fn a_kernel_that_stops_on_a_bug_hands_back_a_decline() {
    let stopped: Result<(), Declined> = exact::caught(|| panic!("a shape nobody tried"));

    assert_eq!(stopped, Err(Declined::Panicked));
}

#[test]
fn a_tools_faces_are_numbered_above_every_number_the_part_gave() {
    let part = block(DVec2::ZERO, DVec2::splat(10.0), 10.0);
    let boss = block(DVec2::splat(2.0), DVec2::splat(5.0), 20.0);

    let joined = part.union(&boss).expect("the kernel joins them");

    assert_eq!(part.faces_end(), 6, "a floor, a top and four walls");
    assert_eq!(joined.faces_end(), 12);
    assert!(joined.has_face(TOP) && joined.has_face(6 + TOP));
    assert!(!joined.has_face(12));
}

#[test]
fn a_top_a_trench_parts_in_two_gives_its_second_piece_a_fresh_number() {
    let part = block(DVec2::ZERO, DVec2::new(30.0, 10.0), 10.0);
    let trench = part
        .tool_raised(
            &exact(
                Contour::rectangle(DVec2::new(10.0, -1.0), DVec2::new(20.0, 11.0)),
                vec![],
            ),
            Frame {
                origin: DVec3::Z * 5.0,
                ..FLAT
            },
            DVec3::Z * 10.0,
        )
        .expect("a trench stands on its own");

    let cut = part.difference(&trench).expect("the kernel cuts it");

    let fresh = 12;
    assert_eq!(cut.faces_end(), fresh + 1);
    let sides: Vec<f64> = [TOP, fresh]
        .iter()
        .map(|&face| {
            let xs: Vec<f64> = cut
                .triangles_of(face)
                .flatten()
                .map(|corner| corner.x)
                .collect();
            assert!(!xs.is_empty(), "face {face} is drawn");
            xs.iter().sum::<f64>() / xs.len() as f64
        })
        .collect();
    assert!(
        (sides[0] < 10.0) != (sides[1] < 10.0),
        "one piece on each side of the trench: {sides:?}",
    );
}

#[test]
fn a_step_the_kernel_declined_still_moves_the_count_past_its_numbers() {
    let mut part = block(DVec2::ZERO, DVec2::splat(10.0), 10.0);
    let declined = exact(disc(DVec2::splat(5.0), 3.0), vec![]);

    part.count_past(&declined);

    assert_eq!(part.faces_end(), 6 + 3, "a floor, a top and one wall");
    let after = block(DVec2::splat(2.0), DVec2::splat(5.0), 20.0);
    let joined = part.union(&after).expect("the kernel joins them");
    assert!(joined.has_face(9 + TOP));
}

/// A block the flats raise, its top one flat face stored as two triangles.
fn flats_block(width: f64, depth: f64, height: f64) -> Body {
    let corners = [
        DVec2::ZERO,
        DVec2::new(width, 0.0),
        DVec2::new(width, depth),
        DVec2::new(0.0, depth),
    ];
    let triangles = [
        [corners[0], corners[1], corners[2]],
        [corners[0], corners[2], corners[3]],
    ];
    let profile = Profile {
        exact: None,
        sampled: Loop::straight(&corners),
        sampled_holes: Vec::new(),
        triangles: &triangles,
    };
    Body::default()
        .tool_raised(&profile, FLAT, DVec3::Z * height)
        .expect("the flats never decline")
}

#[test]
fn a_face_of_the_flats_is_drawn_with_the_triangles_it_was_raised_from() {
    let body = flats_block(10.0, 6.0, 4.0);

    let triangles: Vec<[DVec3; 3]> = body.triangles_of(TOP).collect();

    assert_eq!(triangles.len(), 2, "{triangles:?}");
    assert!(
        triangles.iter().flatten().all(|corner| corner.z == 4.0),
        "every triangle of the top lies on it: {triangles:?}",
    );
    assert_eq!(body.triangles_of(body.faces_end()).count(), 0);
}

#[test]
fn a_flat_face_of_the_flats_offers_its_plane_with_every_corner_of_every_piece() {
    let body = flats_block(10.0, 6.0, 4.0);

    let plane = body.plane_of(TOP).expect("the block has a top");

    assert!(body.is_flat(TOP));
    assert!(plane.normal.dot(DVec3::Z) > 0.999, "{}", plane.normal);
    assert_eq!(plane.corners.len(), 6, "two triangles: {:?}", plane.corners);
    assert_eq!(body.plane_of(body.faces_end()), None);
    let hit = body
        .ray_hit(DVec3::new(5.0, 2.0, 20.0), DVec3::NEG_Z)
        .expect("the top of the block");
    assert_eq!(hit.face, TOP);
    assert!((hit.distance - 16.0).abs() < 1e-9, "{}", hit.distance);
}

#[test]
fn an_exact_body_is_drawn_closed_uncrossed_and_round_enough() {
    let body = bored_block();

    let triangles = body.triangles();

    assert_eq!(crate::soundness::closed(&triangles), Ok(()));
    assert_eq!(crate::soundness::uncrossed(&triangles), Ok(()));
    let flats = 30.0 * 20.0 * 10.0 - polygon_area(&sampled_disc(DVec2::ZERO, 4.0).0) * 10.0;
    let drawn = crate::soundness::enclosed(&triangles);
    assert!(
        (drawn - body.volume()).abs() <= (flats - body.volume()).abs(),
        "drawn no coarser than the flats: {drawn} against {flats}, exactly {}",
        body.volume(),
    );
}
