//! What sketch · solver/stall.rs is held to.

use super::*;

#[test]
fn a_sweep_still_gaining_is_let_go_on() {
    let mut stall = Stall::new();
    let mut worst = 1.0;
    for iteration in 1..=200 {
        worst *= 0.99;
        assert!(!stall.stalled(iteration, worst), "at {iteration}");
    }
}

#[test]
fn a_sweep_whose_error_stands_still_is_stopped_at_the_second_look() {
    let mut stall = Stall::new();
    let stopped = (1..=200).find(|iteration| stall.stalled(*iteration, 2.5e-3));

    assert_eq!(stopped, Some(2 * WINDOW));
}

#[test]
fn a_solve_clearing_a_tenth_a_round_from_far_off_begins_no_round_that_cannot_get_there() {
    let mut rounds = Rounds::new(12);
    assert!(
        rounds.worth_another(1e-2),
        "the first round says nothing of the pace"
    );

    assert!(
        !rounds.worth_another(8e-3),
        "ten more rounds clearing a fifth each would still leave near a thousandth"
    );
}

#[test]
fn a_solve_clearing_fast_enough_to_get_there_goes_on() {
    let mut rounds = Rounds::new(12);
    let mut error = 1e-2;
    for round in 1..12 {
        assert!(
            rounds.worth_another(error),
            "at round {round}, {error:e} left"
        );
        error *= 0.3;
    }
}

#[test]
fn a_solve_that_cleared_less_than_a_tenth_is_stopped_as_it_always_was() {
    let mut rounds = Rounds::new(12);
    rounds.worth_another(1e-6);

    assert!(!rounds.worth_another(0.95e-6));
}
