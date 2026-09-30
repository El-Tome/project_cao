use glam::{DVec2, DVec3};

use super::*;
use crate::brep::scale::Scale;
use crate::profile::{Contour, Frame, Run};

const EPS: f64 = 1e-9 * 40.0;

fn ground(height: f64) -> Frame {
    Frame {
        origin: DVec3::Z * height,
        u: DVec3::X,
        v: DVec3::Y,
    }
}

fn block(low: [f64; 3], high: [f64; 3]) -> Body {
    let outline = Contour::rectangle(DVec2::new(low[0], low[1]), DVec2::new(high[0], high[1]));
    Body::raised(&outline, &[], ground(low[2]), DVec3::Z * (high[2] - low[2]))
        .expect("a block raises")
}

fn standing(center: [f64; 2], radius: f64, from: f64, to: f64) -> Body {
    let center = DVec2::from(center);
    let outline = Contour {
        corners: vec![center + DVec2::X * radius],
        runs: vec![Run::Round {
            center,
            turn: std::f64::consts::TAU,
        }],
    };
    Body::raised(&outline, &[], ground(from), DVec3::Z * (to - from)).expect("a cylinder raises")
}

fn assert_windings(body: &Body, cases: &[([f64; 3], i32)]) {
    for (point, expected) in cases {
        let found = body.winding(DVec3::from(*point), EPS);
        assert_eq!(found, Ok(*expected), "the winding at {point:?}");
    }
}

#[test]
fn a_ray_from_inside_a_block_leaves_it_once_and_from_outside_never_or_twice() {
    let body = block([-20.0, -20.0, 0.0], [20.0, 20.0, 10.0]);
    let near = 1e-6;
    assert_windings(
        &body,
        &[
            ([0.0, 0.0, 5.0], 1),
            ([19.0, -19.0, 9.0], 1),
            ([20.0 - near, 0.0, 5.0], 1),
            ([0.0, 0.0, near], 1),
            ([20.0 + near, 0.0, 5.0], 0),
            ([0.0, 0.0, -near], 0),
            ([30.0, 30.0, 5.0], 0),
            ([-100.0, 3.0, 5.0], 0),
            ([0.0, 0.0, 50.0], 0),
        ],
    );
}

#[test]
fn a_ray_from_inside_a_cylinder_leaves_it_once_and_from_beside_it_never() {
    let body = standing([8.0, 0.0], 5.0, 0.0, 10.0);
    let near = 1e-6;
    assert_windings(
        &body,
        &[
            ([8.0, 0.0, 5.0], 1),
            ([12.9, 0.0, 9.9], 1),
            ([13.0 - near, 0.0, 5.0], 1),
            ([8.0, 5.0 - near, 5.0], 1),
            ([13.0 + near, 0.0, 5.0], 0),
            ([8.0, -5.0 - near, 5.0], 0),
            ([12.0, 4.0, 5.0], 0),
            ([0.0, 0.0, 5.0], 0),
            ([8.0, 0.0, 10.0 + near], 0),
        ],
    );
}

#[test]
fn a_point_on_the_plane_of_a_face_but_off_the_face_is_outside() {
    let body = standing([8.0, 0.0], 5.0, 10.0, 15.0);
    assert_windings(
        &body,
        &[
            ([0.0, 0.0, 10.0], 0),
            ([30.0, 0.0, 15.0], 0),
            ([8.0, 0.0, 12.0], 1),
        ],
    );
}

#[test]
fn a_block_with_a_hole_winds_nothing_in_the_hole_and_once_in_its_matter() {
    let outline = Contour::rectangle(DVec2::new(-20.0, -20.0), DVec2::new(20.0, 20.0));
    let hole = Contour::rectangle(DVec2::new(-5.0, -5.0), DVec2::new(5.0, 5.0));
    let body = Body::raised(&outline, &[hole], ground(0.0), DVec3::Z * 10.0).expect("a frame");
    assert_windings(
        &body,
        &[
            ([0.0, 0.0, 5.0], 0),
            ([10.0, 0.0, 5.0], 1),
            ([-5.5, 0.0, 5.0], 1),
        ],
    );
}

#[test]
fn a_line_through_a_block_crosses_it_in_on_one_side_and_out_on_the_other() {
    let body = block([-20.0, -20.0, 0.0], [20.0, 20.0, 10.0]);
    let found = body
        .crossings_along(DVec3::new(0.0, 3.0, 4.0), DVec3::X, EPS)
        .expect("a line square to two walls");
    assert_eq!(found.len(), 2);
    assert!(
        (found[0].0 + 20.0).abs() < 1e-12 && found[0].1 == 1,
        "{found:?}"
    );
    assert!(
        (found[1].0 - 20.0).abs() < 1e-12 && found[1].1 == -1,
        "{found:?}"
    );
}

#[test]
fn a_line_through_a_cylinder_crosses_its_wall_where_the_circle_says() {
    let body = standing([8.0, 0.0], 5.0, 0.0, 10.0);
    let found = body
        .crossings_along(DVec3::new(0.0, 3.0, 4.0), DVec3::X, EPS)
        .expect("a line across the wall");
    assert_eq!(found.len(), 2);
    assert!(
        (found[0].0 - 4.0).abs() < 1e-12 && found[0].1 == 1,
        "{found:?}"
    );
    assert!(
        (found[1].0 - 12.0).abs() < 1e-12 && found[1].1 == -1,
        "{found:?}"
    );
    let scale = Scale::of(40.0);
    let below = body.crossings_along(DVec3::new(0.0, 3.0, -4.0), DVec3::X, scale.eps());
    assert_eq!(below, Ok(Vec::new()));
}

#[test]
fn a_line_through_an_edge_of_a_block_is_declined_rather_than_counted_twice() {
    let body = block([-20.0, -20.0, 0.0], [20.0, 20.0, 10.0]);
    let found = body.crossings_along(DVec3::new(20.0, 0.0, 10.0), DVec3::new(1.0, 0.3, 1.0), EPS);
    assert_eq!(found, Err(Declined::Tie));
}
