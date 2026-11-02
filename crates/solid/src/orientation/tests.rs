//! Closes #585.
//! - the list of the kernel's sign decisions is written in
//!   `docs/exact-kernel.md`, each marked exact, tolerance, or curve — no test:
//!   it is prose, the section "Which side a point stands on"
//! - every decision of the first kind goes through an adaptive predicate,
//!   tested on inputs whose plain `f64` answer is wrong —
//!   `a_point_a_rounding_away_from_a_long_line_is_seen_on_its_own_side`,
//!   `a_loop_a_rounding_from_flat_turns_the_way_its_last_corner_stands`,
//!   `a_point_a_rounding_beside_an_edge_in_space_is_seen_on_its_own_side`;
//!   the laid profile asks the first of them in
//!   `turning/straight/contacts/tests.rs`, on inputs of the same grid
//! - the campaigns are run before and after, the families that moved are
//!   named, the time the gate and the campaigns take is measured, and no seed
//!   that held before fails — no test: a campaign is run by hand
//!   (`docs/soundness.md`), and what this one found is written in
//!   `docs/exact-kernel-failures.md`, "#585"
//!
//! Each predicate is held on a grid of points a rounding apart, where plain
//! floating point reads some of them on the wrong side. In space, the edge
//! runs along the line where `x`, `y` and `z` are equal, the point stands on
//! the plane where `z` is `x`, and the one it is held against on the side
//! where `y` is greater: the two stand on one side exactly where the point
//! stands further up than across.

use std::cmp::Ordering;

use glam::{DVec2, DVec3};

use super::expansion::Expansion;
use super::{beside, turn, winding};

/// Sixteen by sixteen points a rounding apart from `base`, each with how
/// many steps it stands across and up: on the diagonal where the two are
/// equal, and to its left where it stands further up.
fn grid(base: f64, step: f64) -> impl Iterator<Item = (usize, usize, DVec2)> {
    (0..16).flat_map(move |across| {
        (0..16).map(move |up| {
            let point = DVec2::new(base + across as f64 * step, base + up as f64 * step);
            (across, up, point)
        })
    })
}

fn sign(value: f64) -> Ordering {
    value.partial_cmp(&0.0).unwrap_or(Ordering::Equal)
}

/// The side of a line asked in plain floating point, as the laid profile
/// asked it before #585.
fn plain_turn(a: DVec2, b: DVec2, c: DVec2) -> Ordering {
    sign((b - a).perp_dot(c - a))
}

/// The shoelace in plain floating point, as the laid contours were told
/// apart before #585.
fn plain_winding(points: &[DVec2]) -> Ordering {
    sign(
        points
            .iter()
            .zip(points.iter().cycle().skip(1))
            .map(|(from, to)| from.perp_dot(*to))
            .sum(),
    )
}

/// The side of an edge in space asked in plain floating point, as the
/// triangles folded onto each other asked it before #585.
fn plain_beside(a: DVec3, b: DVec3, c: DVec3, d: DVec3) -> Ordering {
    let normal = (b - a).cross(d - a).normalize();
    sign((b - a).cross(c - a).dot(normal))
}

#[test]
fn a_point_left_of_a_line_turns_counterclockwise_and_one_on_it_does_not_turn() {
    assert_eq!(turn(DVec2::ZERO, DVec2::X, DVec2::Y), Ordering::Greater);
    assert_eq!(turn(DVec2::ZERO, DVec2::X, -DVec2::Y), Ordering::Less);
    assert_eq!(
        turn(DVec2::ZERO, DVec2::X, DVec2::new(3.0, 0.0)),
        Ordering::Equal
    );
}

#[test]
fn a_point_a_rounding_away_from_a_long_line_is_seen_on_its_own_side() {
    let (from, to) = (DVec2::splat(12.0), DVec2::splat(24.0));
    let mut misread = 0;
    for (across, up, point) in grid(0.5, f64::EPSILON / 2.0) {
        assert_eq!(turn(from, to, point), up.cmp(&across), "{across} {up}");
        misread += usize::from(plain_turn(from, to, point) != up.cmp(&across));
    }
    assert!(
        misread > 0,
        "plain floating point read the whole grid right"
    );
}

#[test]
fn a_loop_turns_anticlockwise_or_clockwise_and_a_flat_one_not_at_all() {
    let square = [DVec2::ZERO, DVec2::X, DVec2::ONE, DVec2::Y];
    assert_eq!(winding(&square), Ordering::Greater);
    let backwards: Vec<DVec2> = square.iter().rev().copied().collect();
    assert_eq!(winding(&backwards), Ordering::Less);
    let flat = [DVec2::ZERO, DVec2::X, DVec2::new(2.0, 0.0)];
    assert_eq!(winding(&flat), Ordering::Equal);
    assert_eq!(winding(&[]), Ordering::Equal);
}

#[test]
fn a_loop_a_rounding_from_flat_turns_the_way_its_last_corner_stands() {
    let (first, second) = (DVec2::splat(0.7), DVec2::splat(9.1));
    let mut misread = 0;
    for (across, up, third) in grid(3.9, 2.0 * f64::EPSILON) {
        let corners = [first, second, third];
        assert_eq!(winding(&corners), up.cmp(&across), "{across} {up}");
        misread += usize::from(plain_winding(&corners) != up.cmp(&across));
    }
    assert!(
        misread > 0,
        "plain floating point read the whole grid right"
    );
}

#[test]
fn two_points_on_one_side_of_an_edge_stand_beside_each_other_and_one_on_its_line_nowhere() {
    let (from, to) = (DVec3::ZERO, DVec3::X);
    let (left, right) = (DVec3::new(2.0, 3.0, 0.0), DVec3::new(2.0, -3.0, 0.0));
    assert_eq!(beside(from, to, DVec3::Y, left), Ordering::Greater);
    assert_eq!(beside(from, to, DVec3::Y, right), Ordering::Less);
    assert_eq!(
        beside(from, to, DVec3::new(0.0, 1.0, 1.0), DVec3::Y),
        Ordering::Greater
    );
    assert_eq!(
        beside(from, to, DVec3::new(5.0, 0.0, 0.0), DVec3::Y),
        Ordering::Equal
    );
}

#[test]
fn a_point_a_rounding_beside_an_edge_in_space_is_seen_on_its_own_side() {
    let (from, to, theirs) = (DVec3::splat(12.0), DVec3::splat(24.0), DVec3::Y);
    let mut misread = 0;
    for (across, up, place) in grid(0.5, f64::EPSILON / 2.0) {
        let mine = DVec3::new(place.x, place.y, place.x);
        assert_eq!(
            beside(from, to, mine, theirs),
            up.cmp(&across),
            "{across} {up}"
        );
        misread += usize::from(plain_beside(from, to, mine, theirs) != up.cmp(&across));
    }
    assert!(
        misread > 0,
        "plain floating point read the whole grid right"
    );
}

#[test]
fn an_exact_sum_keeps_what_rounding_drops() {
    let mut sum = Expansion::default();
    for part in [1e16, 1.0, -1e16] {
        sum.add(part);
    }
    assert_eq!(sum.sign(), Ordering::Greater);
    assert_eq!(1e16 + 1.0 - 1e16, 0.0);

    let hair = f64::EPSILON;
    let product = Expansion::difference(1.0, -hair).times(&Expansion::difference(1.0, hair));
    assert_eq!(
        product.minus(&Expansion::difference(1.0, 0.0)).sign(),
        Ordering::Less
    );
    assert_eq!((1.0 + hair) * (1.0 - hair) - 1.0, 0.0);
}
