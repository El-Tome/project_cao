use std::sync::Arc;
use std::time::Duration;

use super::*;
use crate::soundness::{Flaw, Rule, campaign};

fn volume_flaw() -> Flaw {
    Flaw::Volume {
        promised: 1.0,
        enclosed: 0.0,
        worst: None,
    }
}

/// A case is a list of numbers, and breaks the volume rule when it holds a
/// seven.
fn sevens() -> Check<Vec<u64>> {
    Arc::new(|case: &Vec<u64>| {
        if case.contains(&7) {
            Err(volume_flaw())
        } else {
            Ok(())
        }
    })
}

fn draw(seed: u64) -> Vec<u64> {
    vec![seed % 10, seed % 7 + 5, seed % 3]
}

fn shorter(case: &[u64]) -> Vec<Vec<u64>> {
    (0..case.len())
        .map(|index| {
            let mut fewer = case.to_vec();
            fewer.remove(index);
            fewer
        })
        .collect()
}

const PATIENCE: Duration = Duration::from_secs(5);

#[test]
fn seeds_run_across_threads_name_the_same_failures_in_seed_order_as_one_thread() {
    let alone = campaign(
        0..400,
        draw,
        sevens(),
        |case: &Vec<u64>| shorter(case),
        PATIENCE,
        || true,
    );
    let across = campaign_across(
        8,
        0..400,
        draw,
        sevens(),
        |case: &Vec<u64>| shorter(case),
        PATIENCE,
        || true,
    );

    assert_eq!(across.tried, alone.tried);
    assert_eq!(across.failed, alone.failed);
    assert_eq!(across.broken, alone.broken);
}

#[test]
fn every_thread_draws_seeds() {
    let drawn_on: Arc<std::sync::Mutex<std::collections::HashSet<std::thread::ThreadId>>> =
        Arc::default();
    let seen = Arc::clone(&drawn_on);
    let slow: Check<Vec<u64>> = Arc::new(|_| {
        std::thread::sleep(Duration::from_millis(5));
        Ok(())
    });
    campaign_across(
        4,
        0..64,
        move |seed| {
            seen.lock()
                .expect("not poisoned")
                .insert(std::thread::current().id());
            draw(seed)
        },
        slow,
        |case: &Vec<u64>| shorter(case),
        PATIENCE,
        || true,
    );

    assert_eq!(drawn_on.lock().expect("not poisoned").len(), 4);
}

#[test]
fn the_same_failure_found_by_two_threads_is_kept_once() {
    let report = campaign_across(
        8,
        0..400,
        draw,
        sevens(),
        |case: &Vec<u64>| shorter(case),
        PATIENCE,
        || true,
    );

    assert_eq!(report.findings.len(), 1, "{:?}", report.findings);
    assert_eq!(report.findings[0].shrunk, vec![7]);
}

#[test]
fn a_campaign_across_threads_told_to_stop_draws_nothing_more() {
    let mut budget = 3;
    let report = campaign_across(
        8,
        0..400,
        draw,
        sevens(),
        |case: &Vec<u64>| shorter(case),
        PATIENCE,
        || {
            budget -= 1;
            budget >= 0
        },
    );

    assert!(report.tried <= 3, "{}", report.tried);
}

#[test]
fn a_case_that_hangs_stops_the_drawing_on_every_thread() {
    let hangs_at_ten: Check<u64> = Arc::new(|seed: &u64| {
        if *seed == 10 {
            std::thread::sleep(Duration::from_secs(2));
        } else {
            std::thread::sleep(Duration::from_millis(5));
        }
        Ok(())
    });
    let report = campaign_across(
        4,
        0..100_000,
        |seed| seed,
        hangs_at_ten,
        |_: &u64| Vec::new(),
        Duration::from_millis(200),
        || true,
    );

    assert!(report.tried < 1_000, "{}", report.tried);
    assert_eq!(report.failed, vec![(10, Rule::Answers)], "{report:?}");
}
