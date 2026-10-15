//! Rays through spheres and tori (#576): a ball crossed twice, a ring four
//! times, a tangent ray and a ray through a pole doubtful.

use glam::DVec3;

use super::{EPS, assert_windings};
use crate::brep::Declined;
use crate::brep::turned::tests::round::{ball, ring, turned};

#[test]
fn a_ball_is_wound_once_inside_and_nought_outside_and_a_line_crosses_it_twice() {
    let body = turned(&ball(), 360.0);
    assert_windings(
        &body,
        &[
            ([0.0, 0.0, 0.0], 1),
            ([3.0, 3.0, 0.0], 1),
            ([0.0, 4.9, 0.0], 1),
            ([5.1, 0.0, 0.0], 0),
            ([0.0, 0.0, -6.0], 0),
            ([3.0, 3.0, 3.0], 0),
        ],
    );
    let crossed = body
        .crossings_along(DVec3::new(-10.0, 1.0, 0.5), DVec3::X, EPS)
        .expect("a line through the ball crosses it");
    let half = (25.0f64 - 1.25).sqrt();
    assert_eq!(crossed.len(), 2, "{crossed:?}");
    assert_eq!([crossed[0].1, crossed[1].1], [1, -1]);
    for ((at, _), expected) in crossed.iter().zip([10.0 - half, 10.0 + half]) {
        assert!((at - expected).abs() < 1e-12 * 40.0, "{at} against {expected}");
    }
}

#[test]
fn a_ring_is_crossed_four_times_and_wound_once_in_its_tube_and_nought_in_its_hole() {
    let body = turned(&ring(), 360.0);
    assert_windings(
        &body,
        &[
            ([6.0, 3.0, 0.0], 1),
            ([0.0, 3.0, 6.0], 1),
            ([7.5, 3.0, 0.0], 1),
            ([0.0, 3.0, 0.0], 0),
            ([6.0, 3.0, 2.5], 0),
            ([6.0, 6.0, 0.0], 0),
        ],
    );
    let crossed = body
        .crossings_along(DVec3::new(-10.0, 3.0, 0.0), DVec3::X, EPS)
        .expect("a line through the ring crosses it");
    assert_eq!(crossed.len(), 4, "{crossed:?}");
    assert_eq!(
        crossed.iter().map(|(_, step)| *step).collect::<Vec<i32>>(),
        [1, -1, 1, -1]
    );
    for ((at, _), expected) in crossed.iter().zip([2.0, 6.0, 14.0, 18.0]) {
        assert!((at - expected).abs() < 1e-9, "{at} against {expected}");
    }
}

#[test]
fn a_ray_touching_a_ball_or_passing_through_its_pole_is_doubtful() {
    let body = turned(&ball(), 360.0);
    assert_eq!(
        body.crossings_along(DVec3::new(-10.0, 0.0, 5.0), DVec3::X, EPS),
        Err(Declined::Tie),
        "a tangent ray"
    );
    assert_eq!(
        body.crossings_along(DVec3::new(0.0, -10.0, 0.0), DVec3::Y, EPS),
        Err(Declined::Tie),
        "a ray along the axis through both poles"
    );
    assert_eq!(body.winding(DVec3::new(0.0, 4.999, 0.0), EPS), Ok(1));
}
