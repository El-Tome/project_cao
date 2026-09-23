//! What sketch · ellipse_placing.rs is held to.

use super::*;

const NOTHING_TYPED: LockedInput = LockedInput {
    first: None,
    second: None,
};

#[test]
fn over_the_centre_alone_the_cursor_is_where_the_first_axis_ends() {
    let aimed = ellipse_aimed(
        EllipseMode::ByCentre,
        &[DVec2::ZERO],
        DVec2::new(7.0, 3.0),
        NOTHING_TYPED,
        1.0,
    );

    assert_eq!(aimed, DVec2::new(7.0, 3.0));
}

#[test]
fn a_width_typed_for_the_first_axis_reaches_half_of_it_each_side_of_the_centre() {
    let locked = LockedInput {
        first: Some(40.0),
        second: None,
    };

    let aimed = ellipse_aimed(
        EllipseMode::ByCentre,
        &[DVec2::new(10.0, 10.0)],
        DVec2::new(10.0, 13.0),
        locked,
        1.0,
    );

    assert!(aimed.distance(DVec2::new(10.0, 30.0)) < 1e-12, "{aimed}");
}

#[test]
fn an_angle_typed_turns_the_first_axis_and_leaves_its_reach_to_the_cursor() {
    let locked = LockedInput {
        first: None,
        second: Some(90.0),
    };

    let aimed = ellipse_aimed(
        EllipseMode::ByCentre,
        &[DVec2::ZERO],
        DVec2::new(5.0, 0.0),
        locked,
        1.0,
    );

    assert!(aimed.distance(DVec2::new(0.0, 5.0)) < 1e-12, "{aimed}");
}

#[test]
fn a_width_typed_for_the_second_axis_keeps_to_the_side_the_cursor_is_on() {
    let places = [DVec2::ZERO, DVec2::new(30.0, 0.0)];
    let locked = LockedInput {
        first: Some(20.0),
        second: None,
    };

    let aimed = ellipse_aimed(
        EllipseMode::ByCentre,
        &places,
        DVec2::new(12.0, -3.0),
        locked,
        1.0,
    );

    assert!(aimed.distance(DVec2::new(0.0, -10.0)) < 1e-12, "{aimed}");
    let drawn = ellipse_from(EllipseMode::ByCentre, &places, aimed).expect("an ellipse");
    assert!((drawn.second - 10.0).abs() < 1e-12);
}

#[test]
fn the_centre_and_one_axis_make_no_ellipse_until_the_second_reaches_somewhere() {
    assert_eq!(
        ellipse_from(EllipseMode::ByCentre, &[DVec2::ZERO], DVec2::new(3.0, 4.0)),
        None
    );
    assert_eq!(
        ellipse_from(
            EllipseMode::ByCentre,
            &[DVec2::ZERO, DVec2::new(10.0, 0.0)],
            DVec2::new(4.0, 0.0)
        ),
        None,
    );
}
