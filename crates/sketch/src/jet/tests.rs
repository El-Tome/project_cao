use glam::DVec2;

use super::{Jet, JetPlace};

/// Every operation at once, over two places: what a cut's formula leans on.
fn mixed(a: JetPlace<4>, b: JetPlace<4>) -> Jet<4> {
    let unit = (b - a).normalized();
    let turn = a.perp_dot(b).atan2(a.dot(b));
    let grown = (unit.x * 3.0 + 1.0) / (b.length() + unit.y.abs());
    grown * turn.sin() - (a - b).length().sqrt()
}

fn value_at(coordinates: [f64; 4]) -> f64 {
    let a = DVec2::new(coordinates[0], coordinates[1]);
    let b = DVec2::new(coordinates[2], coordinates[3]);
    mixed(JetPlace::fixed(a), JetPlace::fixed(b)).value
}

#[test]
fn a_jet_carries_the_slope_a_difference_measures() {
    let coordinates = [1.3, -0.4, -2.1, 3.7];
    let a = DVec2::new(coordinates[0], coordinates[1]);
    let b = DVec2::new(coordinates[2], coordinates[3]);
    let carried = mixed(JetPlace::free(a, 0), JetPlace::free(b, 2));

    assert_eq!(carried.value, value_at(coordinates));
    let step = 1e-6;
    for rank in 0..4 {
        let (mut ahead, mut behind) = (coordinates, coordinates);
        ahead[rank] += step;
        behind[rank] -= step;
        let measured = (value_at(ahead) - value_at(behind)) / (2.0 * step);
        assert!(
            (carried.slope[rank] - measured).abs() < 1e-7,
            "coordinate {rank}: the jet carries {} where a difference measures {measured}",
            carried.slope[rank],
        );
    }
}

#[test]
fn a_constant_moves_with_nothing() {
    let fixed = JetPlace::<4>::fixed(DVec2::new(2.0, 5.0));
    let free = JetPlace::<4>::free(DVec2::new(-1.0, 1.0), 0);

    let length = (fixed - free).length();

    assert_eq!(length.slope[2], 0.0);
    assert_eq!(length.slope[3], 0.0);
    assert!(length.slope[0] != 0.0 && length.slope[1] != 0.0);
}
