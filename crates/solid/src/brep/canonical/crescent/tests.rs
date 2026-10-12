use std::f64::consts::PI;

use glam::{DVec2, DVec3};

use super::*;
use crate::brep::surface::{Cone, Cylinder};
use crate::profile::{Contour, Frame, Run};
use crate::turning::{Axis, Corner, Straight, Turn};

fn scale() -> Scale {
    Scale::of(60.0)
}

fn raised(outline: &Contour) -> Body {
    let ground = Frame {
        origin: DVec3::ZERO,
        u: DVec3::X,
        v: DVec3::Y,
    };
    Body::raised(outline, &[], ground, DVec3::Z * 10.0).expect("a profile raises")
}

/// A slot along X from `from` to `to`, of radius `radius`.
fn slot(from: DVec2, to: DVec2, radius: f64) -> Body {
    let across = DVec2::Y * radius * (to.x - from.x).signum();
    raised(&Contour {
        corners: vec![from - across, to - across, to + across, from + across],
        runs: vec![
            Run::Straight,
            Run::Round {
                center: to,
                turn: PI,
            },
            Run::Straight,
            Run::Round {
                center: from,
                turn: PI,
            },
        ],
    })
}

fn post(center: DVec2, radius: f64) -> Body {
    raised(&Contour::circle(center, radius))
}

fn cylinders(body: &Body) -> Vec<Cylinder> {
    body.surfaces
        .iter()
        .filter_map(|surface| match surface {
            Surface::Cylinder(cylinder) => Some(*cylinder),
            Surface::Plane(_) | Surface::Cone(_) => None,
        })
        .collect()
}

fn on_its_surfaces(body: &Body) {
    for vertex in &body.vertices {
        for on in &vertex.on {
            assert!(body.surface(*on).distance(vertex.point).abs() < 1e-12);
        }
    }
    for edge in body.edge_ids() {
        let stretch = body.edge(edge);
        let ends = stretch.ends.into_iter().flatten();
        for (end, at) in ends.zip([stretch.from, stretch.to]) {
            assert!(body.point_on(edge, at).distance(body.vertex(end).point) < 1e-12);
        }
    }
}

#[test]
fn a_slot_whose_cap_stands_a_hair_across_from_a_post_of_its_radius_is_moved_whole_onto_it() {
    let hair = 15.0 * scale().eps();
    let one = post(DVec2::ZERO, 2.0);
    let other = slot(DVec2::new(0.0, hair), DVec2::new(-6.0, hair), 2.0);
    let moved = closed(&one, &other, scale()).expect("the slot moves");
    let [wall] = cylinders(&one)[..] else {
        panic!("one wall");
    };
    let caps = cylinders(&moved);
    assert!(
        caps.iter()
            .any(|cap| cap.origin.distance(wall.origin) < 1e-12 && cap.radius == wall.radius),
        "{caps:?}"
    );
    for (before, after) in cylinders(&other).iter().zip(&caps) {
        assert!((after.origin - before.origin + DVec3::Y * hair).length() < 1e-12);
    }
    on_its_surfaces(&moved);
}

#[test]
fn a_slot_moved_onto_a_post_keeps_its_faces_their_ranks_and_their_numbers() {
    let hair = 15.0 * scale().eps();
    let one = post(DVec2::ZERO, 2.0);
    let other = slot(DVec2::new(0.0, hair), DVec2::new(-6.0, hair), 2.0).renumbered(9);
    let moved = closed(&one, &other, scale()).expect("the slot moves");
    assert_ne!(moved.surfaces, other.surfaces);
    assert_eq!(moved.faces, other.faces);
}

#[test]
fn a_slot_whose_cap_stands_a_hair_along_from_a_post_moves_its_near_cap_alone() {
    let hair = 15.0 * scale().eps();
    let one = post(DVec2::ZERO, 2.0);
    let other = slot(DVec2::new(hair, 0.0), DVec2::new(-6.0, 0.0), 2.0);
    let moved = closed(&one, &other, scale()).expect("the cap moves");
    let changed: Vec<usize> = (0..other.surfaces.len())
        .filter(|&rank| moved.surfaces[rank] != other.surfaces[rank])
        .collect();
    let [rank] = changed[..] else {
        panic!("{changed:?} moved");
    };
    let Surface::Cylinder(cap) = moved.surfaces[rank] else {
        panic!("the cap moves");
    };
    assert!(cap.origin.length() < 1e-12);
    on_its_surfaces(&moved);
}

#[test]
fn a_wall_taken_for_another_within_the_tolerance_is_moved_onto_it() {
    let eps = scale().eps();
    let one = post(DVec2::ZERO, 2.0);
    let other = post(DVec2::X * eps / 2.0, 2.0);
    let moved = closed(&one, &other, scale()).expect("the wall moves onto the post's");
    let [wall] = cylinders(&moved)[..] else {
        panic!("one wall");
    };
    assert!(wall.origin.distance(cylinders(&one)[0].origin) < 1e-12);
    on_its_surfaces(&moved);
}

#[test]
fn a_wall_standing_on_another_but_for_rounding_is_not_moved() {
    let eps = scale().eps();
    let one = post(DVec2::ZERO, 2.0);
    let other = post(DVec2::X * 1e-9 * eps, 2.0);
    assert!(closed(&one, &other, scale()).is_none());
}

#[test]
fn a_wall_of_another_radius_or_further_than_a_hair_is_not_moved() {
    let eps = scale().eps();
    let one = post(DVec2::ZERO, 2.0);
    for other in [
        post(DVec2::X * 10.0 * eps, 2.0 + 2.0 * eps),
        post(DVec2::X * (Scale::HAIR + 1.0) * eps, 2.0),
    ] {
        assert!(closed(&one, &other, scale()).is_none());
    }
}

#[test]
fn a_slot_whose_side_is_one_with_the_body_s_is_not_moved_onto_a_wall_a_hair_across() {
    let hair = 15.0 * scale().eps();
    let one = raised(&Contour {
        corners: vec![
            DVec2::new(0.0, -2.0),
            DVec2::new(6.0, -2.0),
            DVec2::new(6.0, 2.0 + hair),
            DVec2::new(0.0, 2.0 + hair),
            DVec2::new(0.0, 2.0),
        ],
        runs: vec![
            Run::Straight,
            Run::Straight,
            Run::Straight,
            Run::Straight,
            Run::Round {
                center: DVec2::ZERO,
                turn: PI,
            },
        ],
    });
    let other = slot(DVec2::new(0.0, hair), DVec2::new(-6.0, hair), 2.0);
    assert!(closed(&one, &other, scale()).is_none());
}

/// A point of radius 2 standing on the top of a post, from `z = 10` to its
/// apex at `z = 15`, turned whole about the line along Z through `(across,
/// 0, 0)`.
fn point_on_top(across: f64) -> Body {
    let corners = [[10.0, 0.0], [15.0, 0.0], [10.0, 2.0]];
    let straight = Straight {
        side: 1.0,
        contours: vec![
            (0..3)
                .map(|run| Corner {
                    at: DVec2::from(corners[run]),
                    run: run as u32,
                })
                .collect(),
        ],
        runs: 3,
        last_off_the_axis: Some(2),
    };
    let turn = Turn {
        axis: Axis {
            origin: DVec2::new(across, 0.0),
            direction: DVec2::Y,
        },
        angle: 2.0 * PI,
        resolution: 0.0,
        on_the_axis: 0.0,
    };
    let upright = Frame {
        origin: DVec3::ZERO,
        u: DVec3::X,
        v: DVec3::Z,
    };
    Body::turned(&straight, upright, &turn).expect("the point turns")
}

fn cones(body: &Body) -> Vec<Cone> {
    body.surfaces
        .iter()
        .filter_map(|surface| match surface {
            Surface::Cone(cone) => Some(*cone),
            Surface::Plane(_) | Surface::Cylinder(_) => None,
        })
        .collect()
}

/// A cone is decided against a surface of revolution only about one axis:
/// one whose axis stands a hair off a wall's, whatever its radius, is moved
/// onto it, as a wall of one radius is.
#[test]
fn a_point_turned_a_hair_off_a_post_s_axis_is_moved_onto_it() {
    let hair = 5.0 * scale().eps();
    let one = post(DVec2::ZERO, 1.0);
    let other = point_on_top(hair);
    let moved = closed(&one, &other, scale()).expect("the point moves onto the post's axis");
    let [cone] = cones(&moved)[..] else {
        panic!("one cone");
    };
    assert!(cone.origin.length() < 1e-12, "{cone:?}");
    on_its_surfaces(&moved);
}

#[test]
fn a_post_a_hair_off_a_point_s_axis_is_moved_onto_it() {
    let hair = 5.0 * scale().eps();
    let one = point_on_top(0.0);
    let other = post(DVec2::X * hair, 1.0);
    let moved = closed(&one, &other, scale()).expect("the post moves onto the point's axis");
    let [wall] = cylinders(&moved)[..] else {
        panic!("one wall");
    };
    assert!(wall.origin.length() < 1e-12, "{wall:?}");
    on_its_surfaces(&moved);
}

/// A post of radius 3 from `z = 0` to `z = 10`, a pocket of radius 1 cut
/// into its top about an axis a hair off its own; and a ring about the
/// post's axis, its bore of radius 1 widening along a cone onto the post's
/// wall at `z = 2`. Moved onto the pocket, the bore would take the cone a
/// hair off the wall it meets about one axis (5361127255).
#[test]
fn a_bore_of_a_pocket_s_radius_is_not_moved_onto_it_when_its_cone_would_leave_a_wall_s_axis() {
    let hair = 5.0 * scale().eps();
    let at = |height: f64| Frame {
        origin: DVec3::Z * height,
        u: DVec3::X,
        v: DVec3::Y,
    };
    let post = Body::raised(
        &Contour::circle(DVec2::ZERO, 3.0),
        &[],
        at(0.0),
        DVec3::Z * 10.0,
    )
    .expect("the post raises");
    let pocket = Body::raised(
        &Contour::circle(DVec2::X * hair, 1.0),
        &[],
        at(8.0),
        DVec3::Z * 4.0,
    )
    .expect("the pocket raises");
    let one = post.cut_by(&pocket).expect("the pocket is cut");
    let corners = [
        [-5.0, 1.0],
        [-5.0, 4.0],
        [2.0, 4.0],
        [2.0, 3.0],
        [-3.0, 1.0],
    ];
    let straight = Straight {
        side: 1.0,
        contours: vec![
            (0..5)
                .map(|run| Corner {
                    at: DVec2::from(corners[run]),
                    run: run as u32,
                })
                .collect(),
        ],
        runs: 5,
        last_off_the_axis: Some(4),
    };
    let turn = Turn {
        axis: Axis {
            origin: DVec2::ZERO,
            direction: DVec2::Y,
        },
        angle: 2.0 * PI,
        resolution: 0.0,
        on_the_axis: 0.0,
    };
    let upright = Frame {
        origin: DVec3::ZERO,
        u: DVec3::X,
        v: DVec3::Z,
    };
    let ring = Body::turned(&straight, upright, &turn).expect("the ring turns");
    assert_eq!(cones(&ring).len(), 1);
    assert!(closed(&one, &ring, scale()).is_none());
}

#[test]
fn a_point_further_than_a_hair_off_a_post_s_axis_is_not_moved() {
    let one = post(DVec2::ZERO, 1.0);
    let other = point_on_top((Scale::HAIR + 1.0) * scale().eps());
    assert!(closed(&one, &other, scale()).is_none());
}
