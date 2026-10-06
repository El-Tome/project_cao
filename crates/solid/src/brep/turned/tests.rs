use std::f64::consts::{FRAC_PI_2, PI, TAU};

use glam::{DVec2, DVec3};

use super::*;
use crate::brep::curve::Curve;
use crate::brep::surface::Surface;
use crate::brep::{EdgeId, FaceId, VertexId};
use crate::soundness::{closed, listed, uncrossed};
use crate::turning::{Axis, Corner};

/// The sketch's plane, XY, its profile read away from the axis along X.
fn ground() -> Frame {
    Frame {
        origin: DVec3::ZERO,
        u: DVec3::X,
        v: DVec3::Y,
    }
}

/// A turn about Y by `degrees`, read with no tolerance of its own.
fn about_y(degrees: f64) -> Turn {
    Turn {
        axis: Axis {
            origin: DVec2::ZERO,
            direction: DVec2::Y,
        },
        angle: degrees.to_radians(),
        resolution: 0.0,
        on_the_axis: 0.0,
    }
}

/// A profile laid square to Y on the side of X, its corners `(h, r)`, the
/// outline first: run `k` leaves the `k`th corner, counted on through the
/// holes.
fn laid(contours: &[&[[f64; 2]]]) -> Straight {
    let mut run = 0;
    let mut last_off_the_axis = None;
    let contours = contours
        .iter()
        .map(|corners| {
            let count = corners.len();
            (0..count)
                .map(|index| {
                    let [at, next] = [corners[index], corners[(index + 1) % count]];
                    if at[1] != 0.0 || next[1] != 0.0 {
                        last_off_the_axis = Some(run);
                    }
                    run += 1;
                    Corner {
                        at: DVec2::from(at),
                        run: run - 1,
                    }
                })
                .collect()
        })
        .collect();
    Straight {
        side: -1.0,
        contours,
        runs: run,
        last_off_the_axis,
    }
}

fn square_on_its_side() -> Straight {
    laid(&[&[[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]]])
}

fn ring() -> Straight {
    laid(&[&[[0.0, 5.0], [10.0, 5.0], [10.0, 10.0], [0.0, 10.0]]])
}

fn stepped_shaft() -> Straight {
    laid(&[&[
        [0.0, 0.0],
        [20.0, 0.0],
        [20.0, 4.0],
        [12.0, 4.0],
        [12.0, 7.0],
        [5.0, 7.0],
        [5.0, 10.0],
        [0.0, 10.0],
    ]])
}

fn holed() -> Straight {
    laid(&[
        &[[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]],
        &[[3.0, 4.0], [3.0, 7.0], [6.0, 7.0], [6.0, 4.0]],
    ])
}

fn notched() -> Straight {
    laid(&[&[
        [0.0, 0.0],
        [3.0, 0.0],
        [3.0, 2.0],
        [6.0, 2.0],
        [6.0, 0.0],
        [9.0, 0.0],
        [9.0, 4.0],
        [0.0, 4.0],
    ]])
}

/// What the profile turned by `angle` radians holds, by Pappus: the angle
/// times the integral of the distance from the axis over the area, the holes
/// taken out.
fn pappus(straight: &Straight, angle: f64) -> f64 {
    let integral = |corners: &[Corner]| -> f64 {
        let count = corners.len();
        (0..count)
            .map(|index| {
                let [one, next] = [corners[index].at, corners[(index + 1) % count].at];
                one.perp_dot(next) * (one.y + next.y)
            })
            .sum::<f64>()
            .abs()
            / 6.0
    };
    let (outline, holes) = straight.contours.split_first().expect("an outline");
    let area = integral(outline) - holes.iter().map(|hole| integral(hole)).sum::<f64>();
    angle.abs() * area
}

fn turned(straight: &Straight, degrees: f64) -> Body {
    Body::turned(straight, ground(), &about_y(degrees)).expect("the profile turns")
}

fn counts(body: &Body) -> [usize; 3] {
    [body.faces.len(), body.edges.len(), body.vertices.len()]
}

fn assert_volume(body: &Body, expected: f64) {
    let volume = body.volume();
    assert!(
        (volume - expected).abs() <= 1e-9 * expected,
        "{volume} against {expected}"
    );
}

fn faces_numbered(body: &Body, number: u32) -> Vec<FaceId> {
    body.face_ids()
        .filter(|&face| body.numbers(face).contains(&number))
        .collect()
}

#[test]
fn a_square_turned_about_its_side_is_two_discs_and_a_cylinder_between_two_circles_with_no_vertex() {
    let body = turned(&square_on_its_side(), 360.0);
    assert_eq!(counts(&body), [3, 2, 0]);
    assert!(body.edges.iter().all(|edge| edge.ends.is_none()));
    let planes = body
        .faces
        .iter()
        .filter(|face| matches!(body.surface(face.surface), Surface::Plane(_)))
        .count();
    assert_eq!(planes, 2);
    assert_volume(&body, 1000.0 * PI);
}

#[test]
fn a_rectangle_off_the_axis_turned_whole_is_the_ring_raised_along_the_axis() {
    let body = turned(&ring(), 360.0);
    let frame = Frame {
        origin: DVec3::ZERO,
        u: DVec3::Z,
        v: DVec3::X,
    };
    let raised = Body::raised(
        &Contour::circle(DVec2::ZERO, 10.0),
        &[Contour::circle(DVec2::ZERO, 5.0)],
        frame,
        DVec3::Y * 10.0,
    )
    .expect("the ring raises");
    assert_eq!(body.surfaces.len(), raised.surfaces.len());
    for surface in &body.surfaces {
        assert!(
            raised.surfaces.contains(surface),
            "{surface:?} is not one of {:?}",
            raised.surfaces
        );
    }
    assert_volume(&body, raised.volume());
    assert_volume(&body, 750.0 * PI);
}

#[test]
fn a_quarter_turn_ends_on_two_planes_of_the_origin_holding_the_axis() {
    let body = turned(&square_on_its_side(), 90.0);
    for end in [4, 5] {
        let [face] = faces_numbered(&body, end)[..] else {
            panic!("one face answers to {end}");
        };
        let Surface::Plane(plane) = body.surface(body.face(face).surface) else {
            panic!("an end is flat");
        };
        assert_eq!(plane.origin, DVec3::ZERO);
        assert!(
            plane.normal == DVec3::X || plane.normal == DVec3::Z,
            "{plane:?}"
        );
    }
    assert_volume(&body, 250.0 * PI);
}

#[test]
fn a_partial_turn_of_a_square_on_its_side_has_six_corners_nine_edges_and_five_faces() {
    let body = turned(&square_on_its_side(), 90.0);
    assert_eq!(counts(&body), [5, 9, 6]);
    let on_the_axis = body
        .vertices
        .iter()
        .filter(|vertex| vertex.point.x == 0.0 && vertex.point.z == 0.0)
        .count();
    assert_eq!(on_the_axis, 2);
}

#[test]
fn a_half_turn_on_the_axis_has_one_flat_answering_to_both_ends_and_no_corner_on_the_axis() {
    let body = turned(&square_on_its_side(), 180.0);
    let ends = faces_numbered(&body, 4);
    assert_eq!(ends, faces_numbered(&body, 5));
    let [end] = ends[..] else {
        panic!("one face answers to the two ends: {ends:?}");
    };
    assert_eq!(body.numbers(end), [4, 5]);
    assert!(
        body.vertices
            .iter()
            .all(|vertex| vertex.point.x.hypot(vertex.point.z) > 1.0),
        "{:?}",
        body.vertices
    );
    assert_eq!(counts(&body), [4, 6, 4]);
    assert_volume(&body, 500.0 * PI);
}

#[test]
fn a_turn_past_half_a_turn_holds_its_volume() {
    for degrees in [200.0, 270.0, 315.0, 359.0] {
        for straight in [ring(), stepped_shaft(), square_on_its_side()] {
            let body = turned(&straight, degrees);
            assert_volume(&body, pappus(&straight, f64::to_radians(degrees)));
        }
    }
}

#[test]
fn a_turn_backwards_is_the_mirror_of_the_turn_forwards() {
    for degrees in [45.0, 90.0, 180.0, 250.0] {
        let forwards = turned(&stepped_shaft(), degrees);
        let backwards = turned(&stepped_shaft(), -degrees);
        assert_eq!(counts(&backwards), counts(&forwards));
        let eps = forwards.scale().eps();
        for vertex in &backwards.vertices {
            let mirrored = vertex.point * DVec3::new(1.0, 1.0, -1.0);
            assert!(
                forwards
                    .vertices
                    .iter()
                    .any(|other| other.point.distance(mirrored) <= eps),
                "{mirrored} is no corner of the turn forwards by {degrees}°"
            );
        }
        assert_volume(&backwards, forwards.volume());
    }
}

#[test]
fn a_profile_hole_turned_whole_leaves_a_ring_shaped_hollow() {
    let body = turned(&holed(), 360.0);
    assert_volume(&body, PI * (1000.0 - 33.0 * 3.0));
    let eps = body.scale().eps();
    assert_eq!(body.winding(DVec3::new(5.5, 4.5, 0.0), eps), Ok(0));
    assert_eq!(body.winding(DVec3::new(0.0, 5.5, 5.5), eps), Ok(0));
    assert_eq!(body.winding(DVec3::new(8.0, 4.5, 0.0), eps), Ok(1));
}

#[test]
fn a_notch_on_the_axis_turned_whole_leaves_a_closed_hollow() {
    let body = turned(&notched(), 360.0);
    assert_volume(&body, PI * (16.0 * 9.0 - 4.0 * 3.0));
    let eps = body.scale().eps();
    assert_eq!(body.winding(DVec3::new(1.0, 4.5, 0.0), eps), Ok(0));
    assert_eq!(body.winding(DVec3::new(0.0, 4.5, 3.0), eps), Ok(1));
    assert_eq!(body.winding(DVec3::new(0.0, 1.5, 1.0), eps), Ok(1));
}

#[test]
fn every_turned_listing_holds_to_its_geometry() {
    for straight in [
        square_on_its_side(),
        ring(),
        stepped_shaft(),
        holed(),
        notched(),
    ] {
        for degrees in [45.0, 90.0, 135.0, 180.0, 270.0, 300.0, 360.0, -90.0, -200.0] {
            let body = turned(&straight, degrees);
            let reach = body.scale().reach();
            assert_eq!(listed(&body.listing(), reach), Ok(()), "{degrees}°");
            let triangles = body.triangles(1e-3 * reach);
            assert_eq!(closed(&triangles), Ok(()), "{degrees}°");
            assert_eq!(uncrossed(&triangles), Ok(()), "{degrees}°");
            assert_volume(&body, pappus(&straight, f64::to_radians(degrees)));
        }
    }
}

#[test]
fn faces_are_numbered_one_per_run_then_the_opening_and_the_closing_end() {
    let body = turned(&ring(), 90.0);
    for number in 0..6 {
        let [face] = faces_numbered(&body, number)[..] else {
            panic!("one face answers to {number}");
        };
        assert_eq!(body.numbers(face), [number]);
    }
    let opening = faces_numbered(&body, 4)[0];
    let Surface::Plane(plane) = body.surface(body.face(opening).surface) else {
        panic!("the opening end is flat");
    };
    assert_eq!(
        plane.normal,
        DVec3::Z,
        "the opening end is the sketch's plane"
    );
}

#[test]
fn two_runs_on_one_line_turn_into_one_face_answering_to_both() {
    let straight = laid(&[&[
        [0.0, 5.0],
        [4.0, 5.0],
        [10.0, 5.0],
        [10.0, 10.0],
        [0.0, 10.0],
    ]]);
    for degrees in [90.0, 360.0] {
        let body = turned(&straight, degrees);
        let [face] = faces_numbered(&body, 0)[..] else {
            panic!("one face answers to the first run");
        };
        assert_eq!(body.numbers(face), [0, 1]);
    }
}

#[test]
fn a_run_on_the_axis_names_a_number_no_face_bears() {
    for degrees in [90.0, 360.0] {
        let body = turned(&square_on_its_side(), degrees);
        assert!(faces_numbered(&body, 0).is_empty());
        for run in 1..4 {
            assert_eq!(faces_numbered(&body, run).len(), 1);
        }
    }
}

#[test]
fn every_face_of_a_turned_body_answers_to_a_run_or_an_end() {
    for straight in [stepped_shaft(), holed(), notched()] {
        for degrees in [90.0, 180.0, 360.0] {
            let body = turned(&straight, degrees);
            for face in body.face_ids() {
                let numbers = body.numbers(face);
                assert!(!numbers.is_empty());
                assert!(numbers.iter().all(|&number| number < straight.runs + 2));
            }
        }
    }
}

#[test]
fn a_turn_a_hair_from_half_a_turn_is_half_a_turn() {
    let straight = square_on_its_side();
    let hair = Turn {
        angle: PI + 1e-12,
        ..about_y(0.0)
    };
    let body = Body::turned(&straight, ground(), &hair).expect("the profile turns");
    assert_eq!(counts(&body), [4, 6, 4]);
    assert_eq!(faces_numbered(&body, 4), faces_numbered(&body, 5));
    let quarter = Turn {
        angle: FRAC_PI_2 - 1e-12,
        ..about_y(0.0)
    };
    let body = Body::turned(&straight, ground(), &quarter).expect("the profile turns");
    let closing = faces_numbered(&body, 5)[0];
    let Surface::Plane(plane) = body.surface(body.face(closing).surface) else {
        panic!("the closing end is flat");
    };
    assert_eq!(plane.normal, DVec3::X);
}

#[test]
fn a_turn_too_short_for_the_tolerance_is_declined_as_travel() {
    let short = Turn {
        angle: 1e-11,
        ..about_y(0.0)
    };
    assert_eq!(
        Body::turned(&ring(), ground(), &short),
        Err(Declined::Travel)
    );
    assert!(Body::turned(&ring(), ground(), &about_y(TAU.to_degrees() / 2.0)).is_ok());
}

/// A shaft of radius 10 and length 30 along Y, its far end chamfered by 2.
fn chamfered_shaft() -> Straight {
    laid(&[&[
        [0.0, 0.0],
        [30.0, 0.0],
        [30.0, 8.0],
        [28.0, 10.0],
        [0.0, 10.0],
    ]])
}

/// A point of radius 5 on the sketch's plane, its apex 10 along Y.
fn point() -> Straight {
    laid(&[&[[0.0, 0.0], [10.0, 0.0], [0.0, 5.0]]])
}

fn cones(body: &Body) -> Vec<FaceId> {
    body.face_ids()
        .filter(|&face| matches!(body.surface(body.face(face).surface), Surface::Cone(_)))
        .collect()
}

fn cone_of(body: &Body, face: FaceId) -> Cone {
    let Surface::Cone(cone) = *body.surface(body.face(face).surface) else {
        panic!("the face lies on a cone");
    };
    cone
}

#[test]
fn a_chamfered_shaft_turns_into_a_cylinder_a_cone_and_two_discs() {
    let body = turned(&chamfered_shaft(), 360.0);
    assert_eq!(counts(&body), [4, 3, 0]);
    let [chamfer] = faces_numbered(&body, 2)[..] else {
        panic!("one face answers to the chamfer");
    };
    assert_eq!(cones(&body), [chamfer]);
    let cone = cone_of(&body, chamfer);
    assert_eq!(cone.axis, DVec3::Y);
    for corner in [DVec3::new(8.0, 30.0, 0.0), DVec3::new(0.0, 28.0, -10.0)] {
        assert!(cone.distance(corner).abs() < 1e-12, "{corner}");
    }
    let kinds = |kind: fn(&Surface) -> bool| {
        body.faces
            .iter()
            .filter(|face| kind(body.surface(face.surface)))
            .count()
    };
    assert_eq!(kinds(|surface| matches!(surface, Surface::Plane(_))), 2);
    assert_eq!(kinds(|surface| matches!(surface, Surface::Cylinder(_))), 1);
}

#[test]
fn a_chamfer_s_matter_lies_on_the_shaft_s_side() {
    for degrees in [90.0, 360.0, -200.0] {
        let body = turned(&chamfered_shaft(), degrees);
        let [chamfer] = faces_numbered(&body, 2)[..] else {
            panic!("one face answers to the chamfer at {degrees}°");
        };
        let cone = cone_of(&body, chamfer);
        let middle = DVec3::new(9.0, 29.0, 0.0);
        let at = cone.parameters(middle);
        let normal = cone.normal(at.x);
        let outward = if body.face(chamfer).flipped {
            -normal
        } else {
            normal
        };
        let away = (DVec3::Y + DVec3::X).normalize();
        assert!(
            outward.dot(away) > 0.99,
            "{outward} at {degrees}° points into the shaft"
        );
    }
}

#[test]
fn a_point_turned_whole_has_one_corner_at_its_tip_held_by_its_cone() {
    let body = turned(&point(), 360.0);
    assert_eq!(counts(&body), [2, 1, 1]);
    assert!(faces_numbered(&body, 0).is_empty());
    let [cone] = cones(&body)[..] else {
        panic!("one face lies on the cone");
    };
    assert_eq!(body.face(cone).apex, Some(VertexId(0)));
    assert!(body.vertices[0].point.distance(DVec3::new(0.0, 10.0, 0.0)) <= 1e-12);
    assert_eq!(body.vertices[0].on, [body.face(cone).surface]);
    assert_eq!(faces_numbered(&body, 1), [cone]);
    let [disc] = faces_numbered(&body, 2)[..] else {
        panic!("one face answers to the base");
    };
    assert_eq!(body.face(disc).apex, None);
}

#[test]
fn a_whole_point_s_listing_names_its_tip_as_a_vertex_of_its_cone() {
    let body = turned(&point(), 360.0);
    let listing = body.listing();
    let [cone] = cones(&body)[..] else {
        panic!("one face lies on the cone");
    };
    assert_eq!(listing.faces[cone.0 as usize].apex, Some(0));
    assert_eq!(listing.vertices, [body.vertices[0].point]);
}

#[test]
fn a_point_turned_part_way_has_its_apex_as_a_corner_of_both_ends() {
    let body = turned(&point(), 90.0);
    let apex = DVec3::new(0.0, 10.0, 0.0);
    let corners: Vec<usize> = (0..body.vertices.len())
        .filter(|&rank| body.vertices[rank].point.distance(apex) <= body.scale().eps())
        .collect();
    let [corner] = corners[..] else {
        panic!("one corner stands at the apex: {:?}", body.vertices);
    };
    for end in [3, 4] {
        let [face] = faces_numbered(&body, end)[..] else {
            panic!("one face answers to {end}");
        };
        let reaches = body.face(face).loops.iter().flatten().any(|coedge| {
            body.edge(coedge.edge)
                .ends
                .is_some_and(|ends| ends.contains(&VertexId(corner as u32)))
        });
        assert!(reaches, "the end {end} does not reach the apex");
    }
    let [cone] = cones(&body)[..] else {
        panic!("one face lies on the cone");
    };
    assert_eq!(body.face(cone).loops.len(), 1);
    assert_eq!(body.face(cone).loops[0].len(), 3);
    assert_eq!(body.face(cone).apex, None);
}

#[test]
fn a_half_turn_of_a_point_keeps_its_two_rulings_apart() {
    let body = turned(&point(), 180.0);
    let [cone] = cones(&body)[..] else {
        panic!("one face lies on the cone");
    };
    let edges: Vec<EdgeId> = body.face(cone).loops[0]
        .iter()
        .map(|coedge| coedge.edge)
        .collect();
    assert_eq!(edges.len(), 3);
    assert!(edges[0] != edges[2] && edges[0] != edges[1] && edges[1] != edges[2]);
    let lines = edges
        .iter()
        .filter(|edge| matches!(body.curve(body.edge(**edge).curve), Curve::Line(_)))
        .count();
    assert_eq!(lines, 2);
}

#[test]
fn a_slanted_run_turned_backwards_gives_the_same_cone() {
    for degrees in [90.0, 360.0] {
        let forwards = turned(&chamfered_shaft(), degrees);
        let backwards = turned(&chamfered_shaft(), -degrees);
        let [one, other] = [&forwards, &backwards].map(|body| {
            let [face] = faces_numbered(body, 2)[..] else {
                panic!("one face answers to the chamfer");
            };
            cone_of(body, face)
        });
        assert_eq!(one, other, "at {degrees}°");
    }
}

#[test]
fn a_countersunk_ring_names_one_face_per_run() {
    let ring = laid(&[&[
        [0.0, 2.0],
        [6.0, 2.0],
        [11.0, 7.0],
        [11.0, 12.0],
        [0.0, 12.0],
    ]]);
    for degrees in [90.0, 360.0] {
        let body = turned(&ring, degrees);
        for run in 0..5 {
            let [face] = faces_numbered(&body, run)[..] else {
                panic!("one face answers to run {run} at {degrees}°");
            };
            assert_eq!(body.numbers(face), [run]);
        }
        assert_eq!(cones(&body), faces_numbered(&body, 1));
    }
}

#[test]
fn two_slanted_runs_on_one_line_turn_into_one_cone_answering_to_both() {
    let straight = laid(&[&[
        [0.0, 0.0],
        [10.0, 0.0],
        [10.0, 5.0],
        [7.0, 8.0],
        [4.0, 11.0],
        [0.0, 11.0],
    ]]);
    for degrees in [90.0, 360.0] {
        let body = turned(&straight, degrees);
        let [face] = cones(&body)[..] else {
            panic!("one face lies on a cone at {degrees}°");
        };
        assert_eq!(body.numbers(face), [2, 3]);
    }
}

#[test]
fn a_corner_on_the_axis_between_two_pieces_off_it_is_declined_as_a_profile() {
    let diamond = laid(&[&[[5.0, 0.0], [10.0, 5.0], [5.0, 10.0], [0.0, 5.0]]]);
    for degrees in [90.0, 360.0] {
        assert_eq!(
            Body::turned(&diamond, ground(), &about_y(degrees)),
            Err(Declined::Profile)
        );
    }
}

#[test]
fn a_whole_point_s_reach_and_bounds_hold_its_apex() {
    let body = turned(&point(), 360.0);
    assert_eq!(body.reach(), 10.0);
    assert_eq!(body.scale().reach(), 10.0);
    let (low, high) = body.bounds().expect("the point has a box");
    assert_eq!(high.y, 10.0);
    assert_eq!(low.y, 0.0);
    let [cone] = cones(&body)[..] else {
        panic!("one face lies on the cone");
    };
    assert_eq!(body.apex_held(cone), Some(DVec3::new(0.0, 10.0, 0.0)));
}

#[test]
fn a_point_turned_part_way_holds_no_apex_beside_its_corner() {
    for degrees in [90.0, 180.0, 270.0] {
        let body = turned(&point(), degrees);
        assert!(
            body.face_ids().all(|face| body.apex_held(face).is_none()),
            "at {degrees}°"
        );
    }
    let frustum = turned(&chamfered_shaft(), 360.0);
    assert!(
        frustum
            .face_ids()
            .all(|face| frustum.apex_held(face).is_none())
    );
}
