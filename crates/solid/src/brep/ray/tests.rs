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

fn lying(center: [f64; 2], radius: f64, from: f64, to: f64) -> Body {
    let center = DVec2::from(center);
    let outline = Contour {
        corners: vec![center + DVec2::X * radius],
        runs: vec![Run::Round {
            center,
            turn: std::f64::consts::TAU,
        }],
    };
    let frame = Frame {
        origin: DVec3::Y * from,
        u: DVec3::X,
        v: DVec3::Z,
    };
    Body::raised(&outline, &[], frame, DVec3::Y * (to - from)).expect("a cylinder lies")
}

/// The bodies of the sixteen cases of `docs/exact-kernel-journal.md`.
fn sixteen() -> Vec<Body> {
    let stock = || standing([0.0, 0.0], 20.0, 0.0, 10.0);
    let slab = || block([-20.0, -20.0, 0.0], [20.0, 20.0, 10.0]);
    let hole = |x: f64, from: f64, to: f64| standing([x, 0.0], 5.0, from, to);
    let cut = |body: Body, tool: Body| body.cut_by(&tool).expect("a case the kernel answers");
    let add = |body: Body, tool: Body| body.joined(&tool).expect("a case the kernel answers");
    vec![
        stock(),
        cut(stock(), hole(8.0, 0.0, 10.0)),
        cut(stock(), hole(8.0, -1.0, 11.0)),
        cut(stock(), hole(15.0, 0.0, 10.0)),
        cut(stock(), hole(15.0, -1.0, 11.0)),
        cut(stock(), hole(0.0, 0.0, 10.0)),
        cut(stock(), hole(0.0, -1.0, 11.0)),
        cut(stock(), hole(8.0, 1e-7, 10.0)),
        add(stock(), hole(8.0, 10.0, 15.0)),
        add(stock(), hole(8.0, 9.0, 15.0)),
        add(stock(), hole(18.0, 10.0, 15.0)),
        add(slab(), block([15.0, -5.0, 10.0], [25.0, 5.0, 15.0])),
        add(slab(), block([20.0, -20.0, 0.0], [60.0, 20.0, 10.0])),
        add(slab(), block([20.0, -10.0, 0.0], [60.0, 30.0, 10.0])),
        cut(slab(), hole(15.0, 0.0, 10.0)),
        add(slab(), lying([0.0, 15.0], 5.0, -15.0, 15.0)),
        cut(cut(stock(), hole(-5.0, 0.0, 10.0)), hole(5.0, 0.0, 10.0)),
        cut(slab(), block([10.0, -5.0, 5.0], [20.0, 5.0, 10.0])),
    ]
}

/// Bodies whose walls meet across each other along the curve of `meet.rs`:
/// the stock bored across, a bar through it, a post and a bar of one radius.
fn meeting() -> Vec<Body> {
    let stock = || standing([0.0, 0.0], 20.0, 0.0, 10.0);
    vec![
        stock().cut_by(&lying([3.0, 5.0], 3.0, -25.0, 25.0)),
        stock().joined(&lying([-4.0, 6.0], 5.0, -30.0, 30.0)),
        standing([0.0, 0.0], 5.0, 0.0, 20.0).joined(&lying([0.0, 10.0], 5.0, -12.0, 12.0)),
    ]
    .into_iter()
    .map(|body| body.expect("a case the kernel answers"))
    .collect()
}

#[test]
fn the_crossings_along_a_line_are_those_every_face_gives_on_the_sixteen_cases_and_walls_meeting() {
    let mut crossed = 0;
    for body in sixteen().into_iter().chain(meeting()) {
        let reach = DVec3::splat(body.reach() + 1.0);
        let lines = crate::soundness::Lines::across(-reach, reach, 48);
        let eps = body.scale().eps();
        for index in 0..lines.count() {
            let (origin, direction) = lines.line(index);
            let boxed = body.crossings_along(origin, direction, eps);
            let every = body.crossings_through(origin, direction, eps, None);
            assert_eq!(boxed, every, "line {index} across {body:?}");
            crossed += usize::from(matches!(&boxed, Ok(found) if !found.is_empty()));
        }
    }
    assert!(crossed > 10_000, "{crossed} lines crossed a body");
}

/// A point of radius 5 on the plane `y = 0`, its apex at `(0, 10, 0)`,
/// turned whole about Y: a face no edge reaches the tip of.
fn whole_point() -> Body {
    point(std::f64::consts::TAU)
}

/// The same point turned by `angle` radians.
fn point(angle: f64) -> Body {
    use crate::turning::{Axis, Corner, Straight, Turn};
    let corners = [[0.0, 0.0], [10.0, 0.0], [0.0, 5.0]];
    let straight = Straight {
        side: -1.0,
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
            origin: DVec2::ZERO,
            direction: DVec2::Y,
        },
        angle,
        resolution: 0.0,
        on_the_axis: 0.0,
    };
    Body::turned(&straight, ground(0.0), &turn).expect("the point turns")
}

#[test]
fn a_whole_point_s_face_box_holds_its_apex() {
    let body = whole_point();
    let boxes = Boxes::of(&body, body.scale().eps());
    let cone = body
        .face_ids()
        .find(|&face| body.apex_held(face).is_some())
        .expect("the cone's face holds its apex");
    for place in [DVec3::new(0.0, 9.9, 0.0), DVec3::new(0.0, 5.0, 2.0)] {
        assert!(!boxes.misses(cone, place), "{place}");
    }
    assert!(boxes.misses(cone, DVec3::new(0.0, 10.5, 0.0)));
}

#[test]
fn a_ray_through_a_cone_crosses_it_twice_at_most() {
    let whole = whole_point();
    assert_windings(
        &whole,
        &[
            ([1.0, 3.0, 1.0], 1),
            ([3.4, 3.0, 0.0], 1),
            ([0.0, 9.9, 0.0], 1),
            ([3.6, 3.0, 0.0], 0),
            ([0.0, 10.5, 0.0], 0),
            ([0.0, -1.0, 0.0], 0),
            ([0.0, 1.0, 6.0], 0),
        ],
    );
    let crossed = whole
        .crossings_along(DVec3::new(-10.0, 3.0, 0.5), DVec3::X, EPS)
        .expect("a line square to the axis crosses the point");
    let half = (3.5f64 * 3.5 - 0.25).sqrt();
    assert_eq!(crossed.len(), 2, "{crossed:?}");
    assert_eq!([crossed[0].1, crossed[1].1], [1, -1]);
    for ((at, _), expected) in crossed.iter().zip([10.0 - half, 10.0 + half]) {
        assert!(
            (at - expected).abs() < 1e-12 * 40.0,
            "{at} against {expected}"
        );
    }
}

#[test]
fn a_point_turned_a_quarter_wraps_one_quarter_only() {
    let quarter = point(std::f64::consts::FRAC_PI_2);
    let windings = [[1.0, 1.0], [-1.0, 1.0], [-1.0, -1.0], [1.0, -1.0]].map(|[x, z]| {
        quarter
            .winding(DVec3::new(x, 3.0, z), EPS)
            .expect("a point off the quarter's ends is wound")
    });
    assert_eq!(
        windings.iter().filter(|&&winding| winding == 1).count(),
        1,
        "{windings:?}"
    );
    assert!(windings.iter().all(|&winding| winding == 0 || winding == 1));
}

#[test]
fn a_ray_through_the_apex_is_doubtful() {
    let direction = DVec3::new(0.3, -1.0, 0.2).normalize();
    let apex = DVec3::new(0.0, 10.0, 0.0);
    for eps in [EPS, EPS * 1e-6] {
        assert_eq!(
            whole_point().crossings_along(apex - direction * 20.0, direction, eps),
            Err(Declined::Tie),
            "{eps}"
        );
    }
}

#[test]
fn a_ray_along_a_ruling_is_doubtful() {
    let direction = DVec3::new(-5.0, 10.0, 0.0).normalize();
    for eps in [EPS, EPS * 1e-6] {
        assert_eq!(
            whole_point().crossings_along(
                DVec3::new(5.0, 0.0, 0.0) - direction * 3.0,
                direction,
                eps
            ),
            Err(Declined::Tie),
            "{eps}"
        );
    }
}
