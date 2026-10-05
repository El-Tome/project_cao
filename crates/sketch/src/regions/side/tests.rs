//! Closes #504.
//! - the side a corner lies on is read exactly: a corner a hair off a diagonal
//!   at 45° is on the side it is on —
//!   `a_corner_a_hair_off_a_diagonal_at_45_degrees_is_on_its_own_side`; a
//!   corner exactly on a line is on neither —
//!   `a_corner_exactly_on_a_line_whose_products_do_not_come_out_exact_is_on_it`
//! - the drawing of the issue is tinted as it encloses — no test: the
//!   drawing is held, its ignore taken off, in what_random_sketches_found.rs
//! - `regions/tests.rs` and the corridors cut out to holes still hold — no
//!   test: the suite beside `regions.rs` is that, unchanged and run in the gate

use glam::DVec2;

use super::{Side, side};

#[test]
fn a_corner_a_hair_off_a_diagonal_at_45_degrees_is_on_its_own_side() {
    let from = DVec2::new(4.383883476483184, -2.6161165235168156);
    let to = DVec2::new(0.5303300858899107, 1.237436867076458);
    let corner = DVec2::new(0.8838834764831843, 0.8838834764831843);

    assert!(
        (to - from).perp_dot(corner - from) < 0.0,
        "the rounded product reads the corner on the right, which is the case"
    );
    assert_eq!(side(corner, from, to), Side::Left);
    assert_eq!(side(corner, to, from), Side::Right);
}

#[test]
fn a_corner_exactly_on_a_line_whose_products_do_not_come_out_exact_is_on_it() {
    let from = DVec2::new(0.1, 0.3);
    let to = DVec2::ZERO;
    let corner = DVec2::new(0.4, 1.2);

    assert!(
        (to - from).perp_dot(corner - from) != 0.0,
        "the rounded product puts the corner off the line, which is the case"
    );
    assert_eq!(side(corner, from, to), Side::On);
}

#[test]
fn a_corner_well_off_a_line_is_on_the_side_it_looks() {
    assert_eq!(side(DVec2::Y, DVec2::ZERO, DVec2::X), Side::Left);
    assert_eq!(side(-DVec2::Y, DVec2::ZERO, DVec2::X), Side::Right);
}
