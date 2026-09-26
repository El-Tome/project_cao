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
