//! The laid profile's own questions on a slant, asked of corners a rounding
//! off it: the grid of `crate::orientation`'s tests, near `(0.5, 0.5)` and
//! read against the diagonal through `(12, 12)` and `(24, 24)`, where plain
//! floating point reads most of the corners off it on it, and some of them
//! on the wrong side.

use glam::DVec2;

use super::{back, cross, overlap};

const FAR: DVec2 = DVec2::splat(24.0);
const NEAR: DVec2 = DVec2::splat(12.0);

/// Corners a rounding apart, each with how many steps it stands across and
/// up: on the diagonal where the two are equal.
fn corners() -> impl Iterator<Item = (usize, usize, DVec2)> {
    let step = f64::EPSILON / 2.0;
    (0..16).flat_map(move |across| {
        (0..16).map(move |up| {
            let corner = DVec2::new(0.5 + across as f64 * step, 0.5 + up as f64 * step);
            (across, up, corner)
        })
    })
}

#[test]
fn a_run_comes_back_along_the_one_before_only_when_its_corner_is_on_their_line() {
    for (across, up, corner) in corners() {
        assert_eq!(back(FAR, corner, NEAR), across == up, "{across} {up}");
    }
}

#[test]
fn two_slants_overlap_only_where_they_lie_on_one_line() {
    for (across, up, corner) in corners() {
        let laid = [FAR, corner, NEAR, DVec2::splat(18.0)];
        assert_eq!(
            overlap(&laid, (0, 1), (2, 3)),
            across == up,
            "{across} {up}"
        );
    }
}

#[test]
fn a_run_from_a_corner_a_rounding_beside_a_slant_crosses_it_towards_the_other_side() {
    for (across, up, corner) in corners() {
        let end = if up > across {
            DVec2::new(12.0, 24.0)
        } else {
            DVec2::new(12.0, 0.0)
        };
        let laid = [FAR, corner, NEAR, end];
        assert_eq!(cross(&laid, (0, 1), (2, 3)), across != up, "{across} {up}");
    }
}
