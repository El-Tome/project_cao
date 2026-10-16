use super::{Random, in_steps};

#[test]
fn the_same_seed_draws_the_same_numbers() {
    let (mut first, mut second) = (Random::seeded(448), Random::seeded(448));
    for _ in 0..100 {
        assert_eq!(first.number(), second.number());
    }
}

#[test]
fn two_seeds_draw_different_numbers() {
    let (mut first, mut second) = (Random::seeded(1), Random::seeded(2));
    assert_ne!(first.number(), second.number());
}

#[test]
fn a_number_on_the_lattice_stays_between_its_bounds() {
    let mut random = Random::seeded(7);
    for _ in 0..1000 {
        let drawn = random.on_lattice(-2.0, 10.0, 0.5);
        assert!((-2.0..=10.0).contains(&drawn), "{drawn}");
        assert_eq!(
            (drawn * 2.0).fract(),
            0.0,
            "{drawn} is on the half-unit lattice"
        );
    }
}

#[test]
fn a_unit_number_stays_below_one() {
    let mut random = Random::seeded(9);
    assert!((0..1000).all(|_| (0.0..1.0).contains(&random.unit())));
}

#[test]
fn a_bound_a_last_bit_off_a_step_counts_that_step_either_side() {
    let reach = 6.0_f64;
    for bound in [
        reach - reach * f64::EPSILON,
        reach,
        reach + reach * f64::EPSILON,
    ] {
        assert_eq!(in_steps(bound, 0.5), 12.0, "{bound:e}");
        let mut random = Random::seeded(3);
        let drawn: Vec<f64> = (0..200)
            .map(|_| random.on_lattice(5.0, bound, 0.5))
            .collect();
        assert!(
            drawn.contains(&6.0) && drawn.iter().all(|value| *value <= 6.0),
            "the step at {reach} is drawn whichever side of it the bound fell, got {drawn:?}"
        );
    }
}

#[test]
fn a_hair_laid_on_purpose_is_not_a_last_bit() {
    assert!(in_steps(6.0 - 1e-8, 0.5) < 12.0);
    assert!(in_steps(6.0 + 1e-8, 0.5) > 12.0);
}
