use std::cmp::Ordering;

use glam::DVec2;

use super::turn;

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
    let step = f64::EPSILON / 2.0;
    for across in 0..16 {
        for up in 0..16 {
            let point = DVec2::new(0.5 + across as f64 * step, 0.5 + up as f64 * step);
            assert_eq!(turn(from, to, point), up.cmp(&across), "{across} {up}");
        }
    }
}
