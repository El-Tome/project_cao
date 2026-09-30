use super::Random;

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
