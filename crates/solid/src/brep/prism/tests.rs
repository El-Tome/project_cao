use glam::{DVec2, DVec3};

use super::*;
use crate::brep::Declined;
use crate::brep::surface::Surface;
use crate::brep::topology::Coedge;
use crate::profile::{Contour, Frame};

fn ground() -> Frame {
    Frame {
        origin: DVec3::ZERO,
        u: DVec3::X,
        v: DVec3::Y,
    }
}

fn counts(body: &Body) -> [usize; 3] {
    [body.faces.len(), body.edges.len(), body.vertices.len()]
}

#[test]
fn a_circle_raised_is_three_faces_two_rings_and_no_corner() {
    let disc = Contour::circle(DVec2::ZERO, 20.0);
    let body = Body::raised(&disc, &[], ground(), DVec3::Z * 10.0).expect("a disc raises");
    assert_eq!(counts(&body), [3, 2, 0]);
    assert!(body.edges.iter().all(|edge| edge.ends.is_none()));
}

/// Every edge on the surfaces of the faces using it, its ends on its vertices,
/// used once along its way and once against it, and every loop closed.
fn assert_sound(body: &Body) {
    let eps = body.scale().eps();
    for edge in body.edge_ids() {
        let uses = body.uses(edge);
        let ways: Vec<bool> = uses.iter().map(|(_, forward)| *forward).collect();
        assert!(
            ways.len() == 2 && ways[0] != ways[1],
            "edge {edge:?} is used {uses:?}"
        );
        let stretch = body.edge(edge);
        for step in 0..=8 {
            let t = stretch.from + (stretch.to - stretch.from) * f64::from(step) / 8.0;
            let point = body.point_on(edge, t);
            for (face, _) in &uses {
                let surface = body.surface(body.face(*face).surface);
                assert!(
                    surface.distance(point).abs() <= eps,
                    "edge {edge:?} leaves the surface of face {face:?} at {point}"
                );
            }
        }
        if let Some([start, end]) = stretch.ends {
            assert!(
                body.point_on(edge, stretch.from)
                    .distance(body.vertex(start).point)
                    <= eps
            );
            assert!(
                body.point_on(edge, stretch.to)
                    .distance(body.vertex(end).point)
                    <= eps
            );
        }
    }
    for face in body.face_ids() {
        for lap in &body.face(face).loops {
            for (index, coedge) in lap.iter().enumerate() {
                let next = lap[(index + 1) % lap.len()];
                let end = |coedge: Coedge, last: bool| {
                    body.edge(coedge.edge)
                        .ends
                        .map(|[from, to]| if coedge.forward == last { to } else { from })
                };
                assert_eq!(end(*coedge, true), end(next, false), "face {face:?} opens");
            }
        }
    }
    for vertex in body.vertex_ids() {
        let corner = body.vertex(vertex);
        assert!(corner.on.windows(2).all(|pair| pair[0] < pair[1]));
        for surface in &corner.on {
            assert!(body.surface(*surface).distance(corner.point).abs() <= eps);
        }
    }
}

#[test]
fn a_rectangle_raised_is_six_faces_twelve_edges_and_eight_corners() {
    let block = Contour::rectangle(DVec2::new(-20.0, -20.0), DVec2::new(20.0, 20.0));
    let body = Body::raised(&block, &[], ground(), DVec3::Z * 10.0).expect("a block raises");
    assert_eq!(counts(&body), [6, 12, 8]);
    assert_sound(&body);
    assert!(body.vertices.iter().all(|corner| corner.on.len() == 3));
}

fn slot() -> Contour {
    use crate::profile::Run;
    use std::f64::consts::PI;
    Contour {
        corners: vec![
            DVec2::new(-10.0, -5.0),
            DVec2::new(10.0, -5.0),
            DVec2::new(10.0, 5.0),
            DVec2::new(-10.0, 5.0),
        ],
        runs: vec![
            Run::Straight,
            Run::Round {
                center: DVec2::new(10.0, 0.0),
                turn: PI,
            },
            Run::Straight,
            Run::Round {
                center: DVec2::new(-10.0, 0.0),
                turn: PI,
            },
        ],
    }
}

#[test]
fn a_slot_raised_gets_a_vertical_edge_where_each_straight_runs_into_its_arc() {
    let body = Body::raised(&slot(), &[], ground(), DVec3::Z * 10.0).expect("a slot raises");
    assert_eq!(counts(&body), [6, 12, 8]);
    assert_sound(&body);
    let rounds = body
        .faces
        .iter()
        .filter(|face| matches!(body.surface(face.surface), Surface::Cylinder(_)) && !face.flipped);
    assert_eq!(rounds.count(), 2);
    assert_eq!(body.scale().reach(), 15.0);
}

#[test]
fn a_round_hole_in_a_rectangle_is_a_wall_with_its_matter_outside() {
    let block = Contour::rectangle(DVec2::new(-20.0, -20.0), DVec2::new(20.0, 20.0));
    let hole = Contour::circle(DVec2::new(8.0, 0.0), 5.0);
    let body = Body::raised(&block, &[hole], ground(), DVec3::Z * 10.0).expect("a plate raises");
    assert_eq!(counts(&body), [7, 14, 8]);
    assert_sound(&body);
    let wall = body
        .faces
        .iter()
        .find(|face| matches!(body.surface(face.surface), Surface::Cylinder(_)))
        .expect("the hole has a wall");
    assert!(wall.flipped);
    assert!(body.faces.iter().take(2).all(|cap| cap.loops.len() == 2));
}

#[test]
fn a_profile_raised_backwards_stands_below_its_plane_with_its_caps_swapped() {
    let block = Contour::rectangle(DVec2::new(0.0, 0.0), DVec2::new(4.0, 3.0));
    let body = Body::raised(&block, &[], ground(), DVec3::Z * -10.0).expect("a block raises");
    assert_eq!(counts(&body), [6, 12, 8]);
    assert_sound(&body);
    let lowest = body
        .vertices
        .iter()
        .map(|corner| corner.point.z)
        .fold(0.0, f64::min);
    assert_eq!(lowest, -10.0);
    for cap in &body.faces[..2] {
        let Surface::Plane(plane) = body.surface(cap.surface) else {
            panic!("a cap is flat");
        };
        let outward = if cap.flipped {
            -plane.normal
        } else {
            plane.normal
        };
        let above = plane.offset() * plane.normal.z > -5.0;
        assert_eq!(
            outward.z > 0.0,
            above,
            "the cap at {} faces {outward}",
            plane.offset()
        );
    }
}

#[test]
fn a_travel_leaning_off_the_normal_is_declined() {
    let block = Contour::rectangle(DVec2::ZERO, DVec2::ONE);
    let raise = |travel| Body::raised(&block, &[], ground(), travel).map(|_| ());
    assert_eq!(raise(DVec3::new(0.0, 1.0, 10.0)), Err(Declined::Travel));
    assert_eq!(raise(DVec3::new(1e-10, 0.0, 10.0)), Err(Declined::Travel));
    assert_eq!(raise(DVec3::ZERO), Err(Declined::Travel));
    assert_eq!(raise(DVec3::new(1e-12, 0.0, 10.0)), Ok(()));
}

#[test]
fn a_profile_that_describes_no_solid_is_declined() {
    use crate::profile::Run;
    let square = |corners: Vec<DVec2>| Contour::straight(corners);
    let raise = |contour: &Contour| Body::raised(contour, &[], ground(), DVec3::Z).map(|_| ());
    let empty = square(Vec::new());
    let doubled = square(vec![
        DVec2::ZERO,
        DVec2::X,
        DVec2::X + DVec2::new(1e-12, 0.0),
        DVec2::ONE,
    ]);
    let folded = square(vec![DVec2::ZERO, DVec2::X * 2.0, DVec2::X]);
    let mut astray = Contour::circle(DVec2::ZERO, 1.0);
    astray.runs[0] = Run::Round {
        center: DVec2::ZERO,
        turn: 3.0,
    };
    for contour in [&empty, &doubled, &folded, &astray] {
        assert_eq!(raise(contour), Err(Declined::Profile), "{contour:?}");
    }
}

#[test]
fn a_profile_or_a_frame_holding_a_number_that_is_not_one_is_declined() {
    use crate::profile::Run;
    let mut lost_center = Contour::circle(DVec2::ZERO, 1.0);
    lost_center.runs[1] = Run::Round {
        center: DVec2::new(f64::NAN, 0.0),
        turn: std::f64::consts::PI,
    };
    let lost_corner = Contour::rectangle(DVec2::ZERO, DVec2::new(f64::NAN, 1.0));
    let endless_corner = Contour::rectangle(DVec2::ZERO, DVec2::new(f64::INFINITY, 1.0));
    let askew = Frame {
        origin: DVec3::ZERO,
        u: DVec3::ONE.normalize(),
        v: DVec3::new(1.0, -2.0, 1.0).normalize(),
    };
    for contour in [&lost_center, &lost_corner, &endless_corner] {
        for frame in [ground(), askew] {
            let raised = Body::raised(contour, &[], frame, frame.normal());
            assert_eq!(raised.map(|_| ()), Err(Declined::Profile), "{contour:?}");
        }
    }
    let block = Contour::rectangle(DVec2::ZERO, DVec2::ONE);
    let adrift = Frame {
        origin: DVec3::new(f64::NAN, 0.0, 0.0),
        ..ground()
    };
    let unturned = Frame {
        u: DVec3::new(f64::NAN, 0.0, 0.0),
        ..ground()
    };
    for frame in [adrift, unturned] {
        assert!(
            Body::raised(&block, &[], frame, DVec3::Z).is_err(),
            "{frame:?}"
        );
    }
    let endless = DVec3::new(0.0, 0.0, f64::INFINITY);
    assert_eq!(
        Body::raised(&block, &[], ground(), endless).map(|_| ()),
        Err(Declined::Travel)
    );
}

#[test]
fn two_runs_on_one_line_make_one_wall_and_two_walls_on_one_line_one_plane() {
    let notched = Contour::straight(vec![
        DVec2::new(0.0, 0.0),
        DVec2::new(15.0, 0.0),
        DVec2::new(30.0, 0.0),
        DVec2::new(30.0, 10.0),
        DVec2::new(20.0, 10.0),
        DVec2::new(20.0, 5.0),
        DVec2::new(10.0, 5.0),
        DVec2::new(10.0, 10.0),
        DVec2::new(0.0, 10.0),
    ]);
    let body = Body::raised(&notched, &[], ground(), DVec3::Z).expect("a notched block raises");
    assert_eq!(counts(&body), [10, 24, 16]);
    assert_eq!(body.surfaces.len(), 9);
    assert_sound(&body);
}

#[test]
fn the_listing_of_a_raised_profile_agrees_with_its_body() {
    let block = Contour::rectangle(DVec2::new(-20.0, -20.0), DVec2::new(20.0, 20.0));
    let hole = Contour::circle(DVec2::new(8.0, 0.0), 5.0);
    let body = Body::raised(&block, &[hole], ground(), DVec3::Z * 10.0).expect("a plate raises");
    let listing = body.listing();
    assert_eq!(
        [
            listing.faces.len(),
            listing.edges.len(),
            listing.vertices.len()
        ],
        counts(&body)
    );
    for (rank, face) in listing.faces.iter().enumerate() {
        for (edge, forward) in face.loops.iter().flatten() {
            assert!(listing.edges[*edge].sides.contains(&(rank, *forward)));
        }
    }
    for edge in &listing.edges {
        assert_eq!(edge.sides.len(), 2);
        if let Some([from, to]) = edge.ends {
            let eps = body.scale().eps();
            assert!(edge.curve.point(edge.from).distance(listing.vertices[from]) <= eps);
            assert!(edge.curve.point(edge.to).distance(listing.vertices[to]) <= eps);
        }
    }
}

#[test]
fn a_disc_raised_keeps_its_rings_on_its_wall_and_its_caps() {
    let disc = Contour::circle(DVec2::new(8.0, -3.0), 5.0);
    let body = Body::raised(&disc, &[], ground(), DVec3::Z * 10.0).expect("a disc raises");
    assert_sound(&body);
    assert_eq!(body.faces[2].loops.len(), 2);
}

/// A block of 10 by 1 whose top bulges along an arc of a circle a hundred
/// times larger than the block, landing `astray` off its end.
fn lens(astray: f64) -> Contour {
    use crate::profile::Run;
    let center = DVec2::new(5.0, -99.0);
    let radius = DVec2::new(10.0, 1.0).distance(center);
    let turn = 2.0 * (5.0f64 / 100.0).atan() + astray / radius;
    Contour {
        corners: vec![
            DVec2::new(0.0, 0.0),
            DVec2::new(10.0, 0.0),
            DVec2::new(10.0, 1.0),
            DVec2::new(0.0, 1.0),
        ],
        runs: vec![
            Run::Straight,
            Run::Straight,
            Run::Round { center, turn },
            Run::Straight,
        ],
    }
}

#[test]
fn an_arc_is_read_at_the_tolerance_of_the_body_it_makes_not_of_the_circle_it_runs_on() {
    let body = Body::raised(&lens(0.0), &[], ground(), DVec3::Z).expect("a lens raises");
    assert_sound(&body);
    let eps = body.scale().eps();
    assert_eq!(
        Body::raised(&lens(5.0 * eps), &[], ground(), DVec3::Z).map(|_| ()),
        Err(Declined::Profile)
    );
}

#[test]
fn a_travel_shorter_than_the_tolerance_is_declined() {
    let block = Contour::rectangle(DVec2::ZERO, DVec2::ONE);
    let raise = |travel| Body::raised(&block, &[], ground(), travel).map(|_| ());
    let eps = crate::brep::scale::Scale::of(1.0).eps();
    assert_eq!(raise(DVec3::Z * 1e-12), Err(Declined::Travel));
    assert_eq!(raise(DVec3::Z * -0.5 * eps), Err(Declined::Travel));
    assert_eq!(raise(DVec3::Z * 3.0 * eps), Ok(()));
}

/// A block 200 wide with a notch in its top, the top left of the notch
/// raised by `near` at the notch and `far` at the block's end.
fn notched_with_a_hair(near: f64, far: f64) -> Contour {
    Contour::straight(vec![
        DVec2::new(-100.0, 0.0),
        DVec2::new(100.0, 0.0),
        DVec2::new(100.0, 50.0),
        DVec2::new(20.0, 50.0),
        DVec2::new(20.0, 30.0),
        DVec2::new(-20.0, 30.0),
        DVec2::new(-20.0, 50.0 + near),
        DVec2::new(-100.0, 50.0 + far),
    ])
}

/// A disc of radius 5 with a notch on either side, its lower arc drawn about
/// a centre `shift` away from the upper arc's, along the lower arc's start.
fn disc_notched_twice(shift: f64) -> Contour {
    use crate::profile::Run;
    use std::f64::consts::PI;
    let at = |angle: f64| DVec2::from_angle(angle);
    let lower = at(PI + 0.3) * shift;
    Contour {
        corners: vec![
            at(0.3) * 5.0,
            at(PI - 0.3) * 5.0,
            DVec2::new(-2.0, 0.0),
            lower + at(PI + 0.3) * (5.0 + shift),
            lower + at(-0.3) * (5.0 + shift),
            DVec2::new(2.0, 0.0),
        ],
        runs: vec![
            Run::Round {
                center: DVec2::ZERO,
                turn: PI - 0.6,
            },
            Run::Straight,
            Run::Straight,
            Run::Round {
                center: lower,
                turn: PI - 0.6,
            },
            Run::Straight,
            Run::Straight,
        ],
    }
}

#[test]
fn two_walls_nearly_on_one_surface_share_it_only_when_each_lies_on_it_within_the_tolerance() {
    let eps = crate::brep::scale::Scale::of(100.0).eps();
    let raise = |outline: &Contour| {
        Body::raised(outline, &[], ground(), DVec3::Z * 10.0).expect("a notched block raises")
    };
    let shared = raise(&notched_with_a_hair(0.3 * eps, 0.3 * eps));
    assert_sound(&shared);
    assert_eq!(shared.surfaces.len(), 9);
    let apart = raise(&notched_with_a_hair(1.1375 * eps, 1.8875 * eps));
    assert_sound(&apart);
    assert_eq!(apart.surfaces.len(), 10);
    let eps = crate::brep::scale::Scale::of(10.0).eps();
    let one_circle = raise(&disc_notched_twice(0.0));
    assert_sound(&one_circle);
    assert_eq!(one_circle.surfaces.len(), 7);
    let two_circles = raise(&disc_notched_twice(0.9 * eps));
    assert_sound(&two_circles);
    assert_eq!(two_circles.surfaces.len(), 8);
}

/// Half a disc of radius 5 whose arc is drawn in two quarters, the second
/// about a centre `shift` below the first's and ending `astray` further out.
fn half_disc_in_two_quarters(shift: f64, astray: f64) -> Contour {
    use crate::profile::Run;
    use std::f64::consts::FRAC_PI_2;
    let lower = DVec2::new(0.0, -shift);
    let top = DVec2::new(0.0, 5.0);
    let landed = lower + DVec2::from_angle(FRAC_PI_2).rotate(top - lower);
    Contour {
        corners: vec![
            DVec2::new(5.0, 0.0),
            top,
            landed - DVec2::ONE.normalize() * astray,
        ],
        runs: vec![
            Run::Round {
                center: DVec2::ZERO,
                turn: FRAC_PI_2,
            },
            Run::Round {
                center: lower,
                turn: FRAC_PI_2,
            },
            Run::Straight,
        ],
    }
}

#[test]
fn two_arcs_on_one_circle_make_one_wall_only_when_the_whole_arc_lands_on_their_end() {
    let raise = |outline: &Contour| Body::raised(outline, &[], ground(), DVec3::Z * 10.0);
    let whole = raise(&half_disc_in_two_quarters(0.0, 0.0)).expect("half a disc raises");
    assert_sound(&whole);
    assert_eq!(counts(&whole), [4, 6, 4]);
    let eps = whole.scale().eps();
    assert_eq!(
        raise(&half_disc_in_two_quarters(0.4 * eps, 0.8 * eps)).map(|_| ()),
        Err(Declined::Profile)
    );
}

#[test]
fn a_run_of_nearly_straight_corners_keeps_every_corner_on_its_walls() {
    let steps = 1000;
    let bow = |step: usize| 1e-12 * (step * (steps - step)) as f64;
    let mut corners: Vec<DVec2> = (0..=steps)
        .map(|step| DVec2::new(step as f64 / 100.0, bow(step)))
        .collect();
    corners.extend([DVec2::new(10.0, -5.0), DVec2::new(0.0, -5.0)]);
    let bowed = Contour::straight(corners.clone());
    let body = Body::raised(&bowed, &[], ground(), DVec3::Z).expect("a bowed block raises");
    assert_sound(&body);
    let eps = body.scale().eps();
    for corner in corners {
        let at = DVec3::new(corner.x, corner.y, 0.0);
        let nearest = body.faces[2..]
            .iter()
            .map(|wall| body.surface(wall.surface).distance(at).abs())
            .fold(f64::INFINITY, f64::min);
        assert!(nearest <= eps, "{corner} stands {nearest} off every wall");
    }
}

#[test]
fn a_profile_passing_twice_through_one_corner_is_declined() {
    use crate::profile::Run;
    use std::f64::consts::PI;
    let raise = |outline: &Contour, holes: &[Contour]| {
        Body::raised(outline, holes, ground(), DVec3::Z * 10.0).map(|_| ())
    };
    let circle_and_triangle = Contour {
        corners: vec![
            DVec2::new(5.0, 0.0),
            DVec2::new(-5.0, 0.0),
            DVec2::new(5.0, 0.0),
            DVec2::new(10.0, -5.0),
            DVec2::new(10.0, 5.0),
        ],
        runs: vec![
            Run::Round {
                center: DVec2::ZERO,
                turn: PI,
            },
            Run::Round {
                center: DVec2::ZERO,
                turn: PI,
            },
            Run::Straight,
            Run::Straight,
            Run::Straight,
        ],
    };
    assert_eq!(raise(&circle_and_triangle, &[]), Err(Declined::Profile));
    let bow_tie = Contour::straight(vec![
        DVec2::new(0.0, 0.0),
        DVec2::new(2.0, 0.0),
        DVec2::new(1.0, 1.0),
        DVec2::new(2.0, 2.0),
        DVec2::new(0.0, 2.0),
        DVec2::new(1.0, 1.0 + 1e-12),
    ]);
    assert_eq!(raise(&bow_tie, &[]), Err(Declined::Profile));
    let block = Contour::rectangle(DVec2::ZERO, DVec2::new(10.0, 10.0));
    let in_the_corner = Contour::straight(vec![
        DVec2::new(10.0, 10.0),
        DVec2::new(5.0, 8.0),
        DVec2::new(8.0, 5.0),
    ]);
    assert_eq!(
        raise(&block, std::slice::from_ref(&in_the_corner)),
        Err(Declined::Profile)
    );
    let hole = Contour::rectangle(DVec2::new(2.0, 2.0), DVec2::new(5.0, 5.0));
    assert_eq!(raise(&block, &[hole]), Ok(()));
}
