//! What sketch · construct.rs is held to.

use super::*;

#[test]
fn a_circle_through_two_points_is_centred_where_it_can_be() {
    let (a, b) = (DVec2::new(0.0, 0.0), DVec2::new(40.0, 0.0));
    let centre = centre_through(a, b, DVec2::new(35.0, 30.0)).unwrap();

    assert!((centre.x - 20.0).abs() < 1e-9, "centre = {centre:?}");
    assert!((centre.distance(a) - centre.distance(b)).abs() < 1e-9);
}

#[test]
fn a_circle_touching_two_lines_sits_on_their_bisector() {
    let first = (DVec2::new(0.0, 0.0), DVec2::new(100.0, 0.0));
    let second = (DVec2::new(0.0, 0.0), DVec2::new(0.0, 100.0));
    let found = centre_touching_two(first, second, DVec2::new(30.0, 20.0)).unwrap();

    let (centre, radius) = (found.centre, found.radius);
    assert!((centre.x - centre.y).abs() < 1e-9, "centre = {centre:?}");
    assert!((radius - centre.x).abs() < 1e-9);

    // Told to be 10 across, it slides down the same bisector.
    let bigger = resize_touching(found, 10.0);
    assert!((bigger.radius - 10.0).abs() < 1e-9);
    assert!(
        (bigger.centre.x - 10.0).abs() < 1e-9,
        "centre = {:?}",
        bigger.centre
    );
}

#[test]
fn two_lines_that_never_meet_take_the_middle() {
    let first = (DVec2::new(0.0, 0.0), DVec2::new(100.0, 0.0));
    let second = (DVec2::new(0.0, 40.0), DVec2::new(100.0, 40.0));
    let found = centre_touching_two(first, second, DVec2::new(60.0, 5.0)).unwrap();

    assert!(
        (found.centre.y - 20.0).abs() < 1e-9,
        "centre = {:?}",
        found.centre
    );
    assert!((found.centre.x - 60.0).abs() < 1e-9);
    assert!((found.radius - 20.0).abs() < 1e-9);
    assert!(found.anchor.is_none(), "two parallels meet at no corner");
}

#[test]
fn a_circle_of_a_given_size_through_two_points() {
    let (a, b) = (DVec2::new(0.0, 0.0), DVec2::new(6.0, 0.0));
    let centre = centre_through_at(a, b, DVec2::new(3.0, 10.0), 5.0).unwrap();

    assert!(
        centre.distance(DVec2::new(3.0, 4.0)) < 1e-9,
        "centre = {centre:?}"
    );
    assert!(centre_through_at(a, b, DVec2::new(3.0, 10.0), 2.0).is_none());
}

#[test]
fn a_circumcentre_is_equally_far_from_all_three_points() {
    let a = DVec2::new(0.0, 0.0);
    let b = DVec2::new(40.0, 0.0);
    let c = DVec2::new(35.0, 30.0);
    let centre = circumcentre(a, b, c).unwrap();

    let (to_a, to_b, to_c) = (centre.distance(a), centre.distance(b), centre.distance(c));
    assert!((to_a - to_b).abs() < 1e-9, "a = {to_a}, b = {to_b}");
    assert!((to_a - to_c).abs() < 1e-9, "a = {to_a}, c = {to_c}");
}

#[test]
fn three_points_on_a_line_circumscribe_no_circle() {
    let a = DVec2::new(0.0, 0.0);
    let b = DVec2::new(10.0, 0.0);
    let c = DVec2::new(20.0, 0.0);

    assert_eq!(circumcentre(a, b, c), None);
}

#[test]
fn a_circle_touching_three_lines_is_the_one_inside_them() {
    // A 3-4-5 triangle, whose inscribed circle has a radius of exactly 1.
    let a = DVec2::new(0.0, 0.0);
    let b = DVec2::new(4.0, 0.0);
    let c = DVec2::new(0.0, 3.0);
    let (centre, radius) = circle_touching_three((a, b), (b, c), (c, a)).unwrap();

    assert!((radius - 1.0).abs() < 1e-9, "radius = {radius}");
    assert!(
        centre.distance(DVec2::new(1.0, 1.0)) < 1e-9,
        "centre = {centre:?}"
    );
}
