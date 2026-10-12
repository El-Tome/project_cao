use std::f64::consts::PI;

use glam::{DVec2, DVec3};

use super::*;
use crate::profile::{Contour, Frame};

fn ground() -> Frame {
    Frame {
        origin: DVec3::ZERO,
        u: DVec3::X,
        v: DVec3::Y,
    }
}

fn assert_relative(found: f64, expected: f64) {
    assert!(
        (found - expected).abs() <= 1e-12 * expected.abs(),
        "{found} against {expected}, off by {}",
        (found - expected) / expected
    );
}

#[test]
fn a_block_holds_its_base_times_its_height() {
    let block = Contour::rectangle(DVec2::new(-20.0, -20.0), DVec2::new(20.0, 20.0));
    let body = Body::raised(&block, &[], ground(), DVec3::Z * 10.0).expect("a block raises");
    assert_relative(body.volume(), 40.0 * 40.0 * 10.0);
}

/// A frame standing off the origin, its plane leaning on all three axes.
fn leaning() -> Frame {
    Frame {
        origin: DVec3::new(3.0, -2.0, 5.0),
        u: DVec3::new(1.0, 1.0, 0.0) / 2f64.sqrt(),
        v: DVec3::new(-1.0, 1.0, 2f64.sqrt()) / 2.0,
    }
}

fn slot() -> Contour {
    use crate::profile::Run;
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
fn a_raised_profile_holds_its_area_times_its_height_in_any_frame() {
    let block = Contour::rectangle(DVec2::new(-20.0, -20.0), DVec2::new(20.0, 20.0));
    let disc = Contour::circle(DVec2::new(8.0, -3.0), 5.0);
    let hole = Contour::circle(DVec2::new(8.0, 0.0), 5.0);
    let square_hole = Contour::rectangle(DVec2::new(-15.0, -15.0), DVec2::new(-5.0, -5.0));
    let cases: [(&Contour, &[Contour], f64); 5] = [
        (&block, &[], 1600.0),
        (&disc, &[], PI * 25.0),
        (&slot(), &[], 200.0 + PI * 25.0),
        (&block, std::slice::from_ref(&hole), 1600.0 - PI * 25.0),
        (&block, &[hole.clone(), square_hole], 1500.0 - PI * 25.0),
    ];
    for frame in [ground(), leaning()] {
        for height in [10.0, -7.0] {
            for (outline, holes, area) in &cases {
                let travel = frame.normal() * height;
                let body = Body::raised(outline, holes, frame, travel).expect("a profile raises");
                assert_relative(body.volume(), area * height.abs());
            }
        }
    }
}

#[test]
fn a_body_with_every_face_turned_inside_out_holds_the_negative() {
    let block = Contour::rectangle(DVec2::new(-20.0, -20.0), DVec2::new(20.0, 20.0));
    let hole = Contour::circle(DVec2::new(8.0, 0.0), 5.0);
    let body = Body::raised(&block, &[hole], leaning(), leaning().normal() * 10.0)
        .expect("a plate raises");
    let mut inside_out = body.clone();
    for face in &mut inside_out.faces {
        face.flipped = !face.flipped;
        for lap in &mut face.loops {
            lap.reverse();
            for coedge in lap.iter_mut() {
                coedge.forward = !coedge.forward;
            }
        }
    }
    assert_relative(inside_out.volume(), -body.volume());
}

#[test]
fn the_quadrature_along_a_circle_agrees_with_its_closed_form() {
    use crate::brep::curve::Circle;
    use crate::brep::surface::Cylinder;
    let frame = leaning();
    let cylinder = Cylinder::about(frame.at(DVec2::new(8.0, 1.0)), frame.normal(), 5.0);
    let circle = Circle::on(&cylinder, 4.0);
    let (plane, _) = Plane::through(circle.center, frame.normal());
    let [from, to] = [-2.5, 1.75];
    let on_cylinder = Surface::Cylinder(cylinder);
    let on_plane = Surface::Plane(plane);
    let trace = traced(&Curve::Circle(circle), &on_plane, from, to, 1e-9).expect("a round");
    let exact = [
        wall(&cylinder, from, to, 4.0, 4.0).expect("a level circle"),
        plane.offset() * swept(&trace).expect("a round has an area"),
    ];
    let numeric = [on_cylinder, on_plane]
        .map(|surface| along(&Curve::Circle(circle), from, to, &surface, 0.0));
    for (exact, numeric) in exact.into_iter().zip(numeric) {
        assert_relative(numeric, exact);
    }
}

#[test]
fn the_quadrature_along_a_cone_s_circle_agrees_with_its_closed_form() {
    use crate::brep::surface::Cone;
    let leaning = DVec3::new(1.0, 2.0, 2.0) / 3.0;
    let through = DVec3::new(3.0, -2.0, 5.0);
    for corners in [[[0.0, 5.0], [10.0, 2.0]], [[0.0, 2.0], [10.0, 5.0]]] {
        let cone = Cone::through(through, leaning, corners.map(DVec2::from));
        let height = (through + leaning * 4.0).dot(cone.axis);
        let circle = cone.circle(height, cone.section(height));
        let l = cone.parameters(circle.point(0.0)).y;
        let [from, to] = [-2.5, 1.75];
        for anchor in [0.0, cone.apex_at()] {
            let exact = cone::straight(&cone, anchor, DVec2::new(from, l), DVec2::new(to, l))
                .expect("a level circle");
            let numeric = along(
                &Curve::Circle(circle),
                from,
                to,
                &Surface::Cone(cone),
                anchor,
            );
            assert_relative(numeric, exact);
        }
    }
}

#[test]
fn an_outline_drawn_clockwise_still_holds_a_positive_volume() {
    let mut block = Contour::rectangle(DVec2::ZERO, DVec2::new(4.0, 3.0));
    block.corners.reverse();
    let body = Body::raised(&block, &[], ground(), DVec3::Z * 2.0).expect("a block raises");
    assert_relative(body.volume(), 24.0);
}

/// The stock bored across, its bore a hair either side of touching the
/// stock's wall from inside: the curve they meet along turns within the
/// square root of that hair of its parameter, at the waist between its two
/// windows or at the neck of its one.
#[test]
fn a_bore_a_hair_from_the_stock_s_wall_holds_what_arithmetic_promises_however_thin_the_waist() {
    use crate::brep::tessellation::tests::{across, fixtures};
    let radius = across::TOUCHING;
    let stock = fixtures::disc_volume(fixtures::STOCK_RADIUS, fixtures::HEIGHT);
    for hair in [1e-7, 1e-5, 1e-3, 1e-1, 1.0] {
        let inside = fixtures::STOCK_RADIUS - radius - hair;
        let through = fixtures::STOCK_RADIUS - radius + hair;
        for (offset, body) in [
            (
                inside,
                across::stock_bored_across_at(radius, DVec3::X, inside),
            ),
            (through, across::stock_bored_through_its_wall(through)),
        ] {
            let promised = stock - across::common_across(radius, offset);
            assert!(
                (body.volume() - promised).abs() <= 1e-10 * promised,
                "{hair}: {} against {promised}",
                body.volume()
            );
        }
    }
}

/// A profile laid square to Y on the side of X, its corners `(h, r)`, turned
/// about Y by `degrees`.
fn turned(corners: &[[f64; 2]], degrees: f64) -> Body {
    use crate::turning::{Axis, Corner, Straight, Turn};
    let count = corners.len();
    let off_the_axis =
        (0..count).filter(|&run| corners[run][1] != 0.0 || corners[(run + 1) % count][1] != 0.0);
    let straight = Straight {
        side: -1.0,
        contours: vec![
            (0..count)
                .map(|run| Corner {
                    at: DVec2::from(corners[run]),
                    run: run as u32,
                })
                .collect(),
        ],
        runs: count as u32,
        last_off_the_axis: off_the_axis.max().map(|run| run as u32),
    };
    let turn = Turn {
        axis: Axis {
            origin: DVec2::ZERO,
            direction: DVec2::Y,
        },
        angle: degrees.to_radians(),
        resolution: 0.0,
        on_the_axis: 0.0,
    };
    Body::turned(&straight, ground(), &turn).expect("the profile turns")
}

/// What the profile of corners `(h, r)` turned by `degrees` holds, by
/// Pappus: the angle times the integral of the distance from the axis over
/// the area.
fn pappus(corners: &[[f64; 2]], degrees: f64) -> f64 {
    let count = corners.len();
    let moment: f64 = (0..count)
        .map(|index| {
            let [one, next] = [corners[index], corners[(index + 1) % count]].map(DVec2::from);
            one.perp_dot(next) * (one.y + next.y)
        })
        .sum();
    degrees.to_radians().abs() * moment.abs() / 6.0
}

/// A shaft of radius 10 and length 30, its far end chamfered by 2: the cone
/// narrows along the axis.
const CHAMFERED: [[f64; 2]; 5] = [
    [0.0, 0.0],
    [30.0, 0.0],
    [30.0, 8.0],
    [28.0, 10.0],
    [0.0, 10.0],
];
/// A funnel widening along the axis from radius 2 to radius 5.
const FUNNEL: [[f64; 2]; 4] = [[0.0, 0.0], [10.0, 0.0], [10.0, 5.0], [0.0, 2.0]];
/// A ring countersunk on its inside: the cone faces the axis.
const COUNTERSUNK: [[f64; 2]; 4] = [[0.0, 3.0], [0.0, 8.0], [5.0, 8.0], [5.0, 2.0]];
/// A point of radius 5, its apex 10 along the axis.
const POINT: [[f64; 2]; 3] = [[0.0, 0.0], [10.0, 0.0], [0.0, 5.0]];

const ANGLES: [f64; 4] = [360.0, 90.0, -200.0, 180.0];

#[test]
fn a_frustum_holds_its_formula_s_volume() {
    assert_relative(turned(&CHAMFERED, 360.0).volume(), 8888.0 * PI / 3.0);
    assert_relative(turned(&FUNNEL, 360.0).volume(), 130.0 * PI);
    for corners in [&CHAMFERED[..], &FUNNEL, &COUNTERSUNK] {
        for degrees in ANGLES {
            assert_relative(turned(corners, degrees).volume(), pappus(corners, degrees));
        }
    }
}

#[test]
fn a_point_turned_whole_holds_a_third_of_its_cylinder() {
    assert_relative(turned(&POINT, 360.0).volume(), 250.0 * PI / 3.0);
    assert_relative(turned(&POINT, -360.0).volume(), 250.0 * PI / 3.0);
}

#[test]
fn a_point_turned_part_way_holds_its_share() {
    for degrees in [90.0, -90.0, 180.0, 270.0, -200.0] {
        assert_relative(
            turned(&POINT, degrees).volume(),
            250.0 * PI / 3.0 * degrees.abs() / 360.0,
        );
    }
}

#[test]
fn a_near_cylinder_frustum_holds_its_volume() {
    let corners = [[0.0, 0.0], [10.0, 0.0], [10.0, 5.0 + 1e-5], [0.0, 5.0]];
    for degrees in ANGLES {
        assert_relative(
            turned(&corners, degrees).volume(),
            pappus(&corners, degrees),
        );
    }
}

#[test]
fn a_near_disc_frustum_holds_its_volume() {
    let corners = [
        [0.0, 0.0],
        [10.0, 0.0],
        [10.0, 2.0],
        [10.0 - 3e-6, 5.0],
        [0.0, 5.0],
    ];
    for degrees in ANGLES {
        assert_relative(
            turned(&corners, degrees).volume(),
            pappus(&corners, degrees),
        );
    }
}

/// `½∮x × dx` round a face's loops, by quadrature along each edge: the
/// face's vector area, whatever surface it lies on. Every ruling of a cone
/// runs through its apex, so the position less the apex lies along the cone
/// everywhere on it, and the flux through a face of a cone is the apex
/// against this: an answer that owes nothing to the face's parameters nor
/// to where its flux is anchored.
fn vector_area(body: &Body, face: crate::brep::FaceId) -> DVec3 {
    let mut total = DVec3::ZERO;
    for coedge in body.face(face).loops.iter().flatten() {
        let edge = body.edge(coedge.edge);
        let curve = body.curve(edge.curve);
        let mut area = DVec3::ZERO;
        for [start, end] in panels(curve, edge.from, edge.to) {
            let (middle, width) = ((start + end) / 2.0, end - start);
            for (node, weight) in GAUSS_LEGENDRE {
                for side in [-1.0, 1.0] {
                    let t = middle + side * node * width / 2.0;
                    area += curve.point(t).cross(curve.derivative(t)) * (weight * width / 4.0);
                }
            }
        }
        total += if coedge.forward { area } else { -area };
    }
    total
}

#[test]
fn a_cone_face_s_flux_is_its_apex_against_its_vector_area() {
    for corners in [&CHAMFERED[..], &FUNNEL, &COUNTERSUNK, &POINT] {
        for degrees in ANGLES {
            let body = turned(corners, degrees);
            for face in body.face_ids() {
                let Surface::Cone(cone) = body.surface(body.face(face).surface) else {
                    continue;
                };
                let flux: f64 = body.fluxes(face).sum();
                let area = vector_area(&body, face);
                let expected = cone.apex().dot(area);
                assert!(
                    (flux - expected).abs() <= 1e-12 * cone.apex().length() * area.length(),
                    "{corners:?} by {degrees}°: {flux} against {expected}"
                );
            }
        }
    }
}
