//! What sketch · ellipsing.rs is held to.

use super::*;

fn long_one() -> EllipseDraft {
    EllipseDraft {
        centre: DVec2::new(10.0, 5.0),
        first: DVec2::new(30.0, 0.0),
        second: 10.0,
    }
}

#[test]
fn a_place_off_the_first_axis_counts_only_for_how_far_it_stands_square_to_it() {
    let drawn = EllipseDraft::through(DVec2::ZERO, DVec2::new(20.0, 0.0), DVec2::new(7.0, -4.0))
        .expect("an ellipse");

    assert_eq!(drawn.second, 4.0);
    assert_eq!(drawn.first, DVec2::new(20.0, 0.0));
}

#[test]
fn a_second_click_on_the_first_axis_makes_no_ellipse() {
    assert_eq!(
        EllipseDraft::through(DVec2::ZERO, DVec2::new(20.0, 0.0), DVec2::new(-5.0, 0.0)),
        None,
    );
}

#[test]
fn a_quarter_turn_lands_on_the_end_of_the_second_axis() {
    let drawn = long_one();

    assert!(drawn.at(0.0).distance(DVec2::new(40.0, 5.0)) < 1e-12);
    assert!(
        drawn
            .at(std::f64::consts::FRAC_PI_2)
            .distance(DVec2::new(10.0, 15.0))
            < 1e-12
    );
}

#[test]
fn the_nearest_place_is_found_on_a_long_ellipse_from_either_side() {
    let drawn = long_one();

    for turn in [0.1, 0.8, 1.6, 2.5, 3.3, 4.4, 5.9] {
        let on = drawn.at(turn);
        let outward = (on - drawn.centre).normalize();
        for off in [-2.0, 3.0] {
            let found = drawn.nearest(on + outward * off);
            assert!(
                drawn.distance(on + outward * off) <= off.abs() + 1e-9,
                "the curve is never further than a place it was pushed from: turn {turn}, off {off}, found {found}",
            );
        }
        assert!(drawn.distance(on) < 1e-9, "a place on the curve is on it");
    }
}

#[test]
fn a_turned_ellipse_fits_the_box_its_extremes_touch() {
    let drawn = EllipseDraft {
        centre: DVec2::ZERO,
        first: DVec2::new(3.0, 4.0) * 2.0,
        second: 5.0,
    };
    let (low, high) = drawn.bounds();

    let places = drawn.places();
    let (seen_low, seen_high) = places
        .iter()
        .fold((places[0], places[0]), |(low, high), place| {
            (low.min(*place), high.max(*place))
        });
    assert!(low.cmple(seen_low + 1e-9).all() && high.cmpge(seen_high - 1e-9).all());
    assert!(seen_low.distance(low) < 0.1 && seen_high.distance(high) < 0.1);
}
