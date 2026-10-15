//! Profiles holding arcs, turned into spheres and tori (#576): what they
//! are made of, and that each is verified and listed sound.

use std::f64::consts::{FRAC_PI_2, PI, TAU};

use glam::{DVec2, DVec3};

use super::{about_y, counts, faces_numbered, ground};
use crate::brep::surface::Surface;
use crate::brep::topology::Body;
use crate::brep::{Curve, Declined};
use crate::soundness::listed;
use crate::turning::{Bend, Corner, Straight};

/// A corner laid in `(h, r)`, and the arc leaving it, if one does: its
/// centre and its turn.
type Laid = ([f64; 2], Option<([f64; 2], f64)>);

/// A profile laid in `(h, r)`, run `k` leaving the `k`th corner.
fn bent(contours: &[&[Laid]]) -> Straight {
    let mut run = 0;
    let contours: Vec<Vec<Corner>> = contours
        .iter()
        .map(|corners| {
            corners
                .iter()
                .map(|&(at, bend)| {
                    run += 1;
                    Corner {
                        at: DVec2::from(at),
                        run: run - 1,
                        bend: bend.map(|(center, turn)| Bend {
                            center: DVec2::from(center),
                            turn,
                        }),
                    }
                })
                .collect()
        })
        .collect();
    Straight {
        side: -1.0,
        contours,
        runs: run,
        last_off_the_axis: Some(run - 1),
    }
}

/// A ball of radius 5 about the origin: a semicircle on the axis, turned.
fn ball() -> Straight {
    bent(&[&[([-5.0, 0.0], Some(([0.0, 0.0], -PI))), ([5.0, 0.0], None)]])
}

/// A ring of tube radius 2 about a circle of radius 6 at `h = 3`.
fn ring() -> Straight {
    bent(&[&[
        ([3.0, 8.0], Some(([3.0, 6.0], -PI))),
        ([3.0, 4.0], Some(([3.0, 6.0], -PI))),
    ]])
}

/// A zone of the ball of radius 5 between `h = -3` and `h = 3`.
fn zone() -> Straight {
    bent(&[&[
        ([-3.0, 0.0], None),
        ([-3.0, 4.0], Some(([0.0, 0.0], -1.2870022175865685))),
        ([3.0, 4.0], None),
        ([3.0, 0.0], None),
    ]])
}

/// A cap of the ball of radius 5 above `h = -3`, to the pole.
fn cap() -> Straight {
    bent(&[&[
        ([-3.0, 0.0], None),
        (
            [-3.0, 4.0],
            Some(([0.0, 0.0], -(FRAC_PI_2 + 0.6435011087932844))),
        ),
        ([5.0, 0.0], None),
    ]])
}

/// A shaft of radius 4 from `h = 0` to `h = 10`, ending in a ball.
fn rounded_shaft() -> Straight {
    bent(&[&[
        ([0.0, 0.0], None),
        ([0.0, 4.0], None),
        ([10.0, 4.0], Some(([10.0, 0.0], -FRAC_PI_2))),
        ([14.0, 0.0], None),
    ]])
}

/// An apple: the arc of the circle of radius 2 about `(0, 1)` on the far
/// side of the axis, from the axis back to it.
fn apple() -> Straight {
    let reach = 3f64.sqrt();
    bent(&[&[
        ([-reach, 0.0], Some(([0.0, 1.0], -2.0 * (2.0 * PI / 3.0)))),
        ([reach, 0.0], None),
    ]])
}

fn turned(straight: &Straight, degrees: f64) -> Body {
    Body::turned(straight, ground(), &about_y(degrees)).expect("the profile turns")
}

fn rounds(body: &Body) -> Vec<usize> {
    body.face_ids()
        .filter(|&face| {
            matches!(
                body.surface(body.face(face).surface),
                Surface::Sphere(_) | Surface::Torus(_)
            )
        })
        .map(|face| face.0 as usize)
        .collect()
}

fn assert_sound(body: &Body, what: &str) {
    let reach = body.scale().reach();
    let listing = body.listing();
    let summary: Vec<String> = listing
        .faces
        .iter()
        .map(|face| {
            format!(
                "{:?}/{:?}/{:?}/{:?}",
                face.surface, face.outward, face.loops, face.poles
            )
        })
        .collect();
    let edges: Vec<String> = listing
        .edges
        .iter()
        .map(|edge| {
            format!(
                "{:?} {}..{} {:?}",
                edge.curve, edge.from, edge.to, edge.ends
            )
        })
        .collect();
    assert_eq!(
        listed(&listing, reach),
        Ok(()),
        "{what}\nFACES {summary:#?}\nEDGES {edges:#?}\nVERTICES {:?}",
        listing.vertices
    );
}

#[test]
fn a_semicircle_on_the_axis_turned_whole_is_one_face_with_two_poles_and_no_loop() {
    let body = turned(&ball(), 360.0);
    assert_eq!(counts(&body), [1, 0, 2]);
    let face = body.face(crate::brep::FaceId(0));
    assert!(face.loops.is_empty());
    assert_eq!(face.poles.len(), 2);
    assert!(!face.flipped);
    let Surface::Sphere(sphere) = body.surface(face.surface) else {
        panic!("a ball lies on a sphere");
    };
    assert_eq!((sphere.middle(), sphere.radius), (DVec3::ZERO, 5.0));
    for pole in &body.vertices {
        assert!(pole.point.distance(DVec3::ZERO) - 5.0 < 1e-12);
        assert!(pole.point.x == 0.0 && pole.point.z == 0.0, "{pole:?}");
    }
    assert_eq!(body.reach(), 5.0);
    assert_sound(&body, "a ball");
}

#[test]
fn a_whole_circle_off_the_axis_turned_whole_is_one_face_with_no_loop_and_no_vertex() {
    let body = turned(&ring(), 360.0);
    assert_eq!(counts(&body), [1, 0, 0]);
    let face = body.face(crate::brep::FaceId(0));
    assert!(face.loops.is_empty() && face.poles.is_empty());
    let Surface::Torus(torus) = body.surface(face.surface) else {
        panic!("a ring lies on a torus");
    };
    assert_eq!((torus.center, torus.radius), (DVec2::new(3.0, 6.0), 2.0));
    assert_eq!(body.numbers(crate::brep::FaceId(0)), [0, 1]);
    assert_eq!(body.reach(), 8.0);
    assert_sound(&body, "a ring");
}

#[test]
fn a_ball_turned_part_way_is_a_lune_between_two_meridians_and_two_half_discs() {
    let body = turned(&ball(), 90.0);
    assert_eq!(counts(&body), [3, 3, 2]);
    let [lune] = rounds(&body)[..] else {
        panic!("one face on the sphere");
    };
    let lune = body.face(crate::brep::FaceId(lune as u32));
    assert_eq!(lune.loops.len(), 1);
    assert_eq!(lune.loops[0].len(), 2);
    assert!(lune.poles.is_empty());
    let meridians = body
        .edges
        .iter()
        .filter(|edge| matches!(body.curve(edge.curve), Curve::Circle(circle) if circle.center == DVec3::ZERO))
        .count();
    assert_eq!(meridians, 2);
    assert_sound(&body, "a lune");
    for degrees in [45.0, 180.0, 270.0, -120.0] {
        assert_sound(&turned(&ball(), degrees), &format!("a lune at {degrees}"));
    }
}

#[test]
fn a_ring_turned_part_way_is_bounded_by_two_meridian_circles_with_no_vertex() {
    for degrees in [90.0, 180.0, 300.0] {
        let body = turned(&ring(), degrees);
        assert_eq!(counts(&body), [3, 2, 0], "{degrees}°");
        assert!(body.edges.iter().all(|edge| edge.ends.is_none()));
        assert_sound(&body, "a ring turned part way");
    }
}

#[test]
fn a_zone_and_a_cap_are_bounded_by_their_latitude_circles_and_a_cap_holds_its_pole() {
    let band = turned(&zone(), 360.0);
    assert_eq!(counts(&band), [3, 2, 0]);
    assert_eq!(rounds(&band).len(), 1);
    assert_sound(&band, "a zone");

    let dome = turned(&cap(), 360.0);
    assert_eq!(counts(&dome), [2, 1, 1]);
    let [round] = rounds(&dome)[..] else {
        panic!("one face on the sphere");
    };
    let face = body_face(&dome, round);
    assert_eq!(face.poles.len(), 1);
    assert!(dome.vertices[0].point.distance(DVec3::new(0.0, 5.0, 0.0)) < 1e-12);
    assert_sound(&dome, "a cap");
    for degrees in [90.0, 180.0, 250.0] {
        assert_sound(&turned(&zone(), degrees), "a zone turned part way");
        assert_sound(&turned(&cap(), degrees), "a cap turned part way");
    }
}

fn body_face(body: &Body, face: usize) -> &crate::brep::Face {
    body.face(crate::brep::FaceId(face as u32))
}

#[test]
fn a_rounded_shaft_end_is_a_sphere_tangent_to_its_shaft_sharing_the_shaft_s_circle() {
    let body = turned(&rounded_shaft(), 360.0);
    assert_eq!(counts(&body), [3, 2, 1]);
    let [round] = rounds(&body)[..] else {
        panic!("one face on the sphere");
    };
    assert_eq!(
        faces_numbered(&body, 2),
        [crate::brep::FaceId(round as u32)]
    );
    assert_eq!(body_face(&body, round).poles.len(), 1);
    let shared = body.edges.iter().filter(|edge| {
        matches!(body.curve(edge.curve), Curve::Circle(circle) if (circle.center.y - 10.0).abs() < 1e-12)
    });
    assert_eq!(
        shared.count(),
        1,
        "one circle where the ball meets the shaft"
    );
    assert_sound(&body, "a rounded shaft end");
    for degrees in [90.0, 180.0, 300.0, -90.0] {
        let body = turned(&rounded_shaft(), degrees);
        assert_sound(&body, "a rounded shaft end turned part way");
    }
}

#[test]
fn an_apple_turned_whole_is_one_face_with_two_poles_on_the_axis() {
    let body = turned(&apple(), 360.0);
    assert_eq!(counts(&body), [1, 0, 2]);
    let face = body_face(&body, 0);
    assert!(face.loops.is_empty() && face.poles.len() == 2);
    let Surface::Torus(torus) = body.surface(face.surface) else {
        panic!("an apple lies on a torus");
    };
    assert_eq!(torus.center, DVec2::new(0.0, 1.0));
    assert!((torus.radius - 2.0).abs() < 1e-12);
    assert_eq!(torus.poles().len(), 2);
    assert_sound(&body, "an apple");
    assert_sound(&turned(&apple(), 90.0), "an apple turned part way");
}

#[test]
fn a_turn_backwards_of_a_rounded_profile_mirrors_the_turn_forwards() {
    for degrees in [60.0, 180.0] {
        let forwards = turned(&rounded_shaft(), degrees);
        let backwards = turned(&rounded_shaft(), -degrees);
        assert_eq!(counts(&forwards), counts(&backwards));
        let eps = forwards.scale().eps();
        for vertex in &backwards.vertices {
            let mirrored = vertex.point * DVec3::new(1.0, 1.0, -1.0);
            assert!(
                forwards
                    .vertices
                    .iter()
                    .any(|other| other.point.distance(mirrored) <= eps)
            );
        }
    }
}

#[test]
fn an_arc_tangent_to_the_axis_at_its_end_is_declined_as_a_profile() {
    let horn = bent(&[&[
        ([0.0, 0.0], None),
        ([0.0, 4.0], None),
        ([10.0, 4.0], Some(([10.0, 2.0], -PI))),
        ([10.0, 0.0], None),
    ]]);
    assert_eq!(
        Body::turned(&horn, ground(), &about_y(360.0)),
        Err(Declined::Profile)
    );
}

#[test]
fn a_whole_turn_s_arc_bulge_is_read_into_the_reach_and_the_slit_a_short_turn_leaves() {
    let body = turned(&apple(), 360.0);
    assert!((body.reach() - 3.0).abs() < 1e-12, "{}", body.reach());
    assert_eq!(
        Body::turned(&apple(), ground(), &about_y(TAU.to_degrees() * 1e-13)),
        Err(Declined::Travel)
    );
}
