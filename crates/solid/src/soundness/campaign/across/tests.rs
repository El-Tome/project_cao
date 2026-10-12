//! What a campaign does on several threads: its seeds drawn, and the seeds it
//! named run again and shrunk.
//!
//! Closes #566.
//! - the same seeds shrunk on one thread and on several give the same shrunk
//!   cases, handed back in seed order —
//!   `seeds_shrunk_across_threads_give_what_one_thread_gives_in_seed_order`
//! - a seed whose shrink hangs past the patience is named as such, and does
//!   not stop the others —
//!   `a_seed_that_hangs_is_named_so_and_the_others_are_still_shrunk`
//! - `docs/soundness.md` says so beside `CAO_FUZZ_THREADS` — no test: it is
//!   prose, which nothing asserts

use std::panic::AssertUnwindSafe;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use super::*;
use crate::soundness::{Flaw, Rule, Silence, campaign};

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

/// Every seed handed back by [`shrunk_across`], in the order it handed them.
fn rerun<C: Clone + PartialEq + Send + 'static>(
    threads: usize,
    seeds: impl IntoIterator<Item = u64>,
    draw: impl Fn(u64) -> C + Sync,
    check: Check<C>,
    smaller: impl Fn(&C) -> Vec<C> + Sync,
    patience: Duration,
) -> Vec<Rerun<C>> {
    let mut handed = Vec::new();
    shrunk_across(threads, seeds, draw, check, smaller, patience, |rerun| {
        handed.push(rerun)
    });
    handed
}

/// A case breaks the volume rule when its numbers add up to more than twelve,
/// after a pause that grows with its first number, so that threads finish
/// out of order.
fn over_twelve() -> Check<Vec<u64>> {
    Arc::new(|case: &Vec<u64>| {
        std::thread::sleep(Duration::from_micros(
            100 * case.first().copied().unwrap_or(0),
        ));
        if case.iter().sum::<u64>() > 12 {
            Err(volume_flaw())
        } else {
            Ok(())
        }
    })
}

/// What a rerun says of its seed, without the case as drawn.
fn told(rerun: &Rerun<Vec<u64>>) -> (u64, Option<Vec<u64>>, bool) {
    match rerun {
        Rerun::Holds(seed) => (*seed, None, false),
        Rerun::Breaks { finding, hung } => (finding.seed, Some(finding.shrunk.clone()), *hung),
    }
}

#[test]
fn seeds_shrunk_across_threads_give_what_one_thread_gives_in_seed_order() {
    let seeds: Vec<u64> = (0..120).rev().collect();
    let alone = rerun(
        1,
        seeds.clone(),
        draw,
        over_twelve(),
        |case: &Vec<u64>| shorter(case),
        PATIENCE,
    );
    let across = rerun(
        8,
        seeds,
        draw,
        over_twelve(),
        |case: &Vec<u64>| shorter(case),
        PATIENCE,
    );

    let alone: Vec<_> = alone.iter().map(told).collect();
    let across: Vec<_> = across.iter().map(told).collect();
    assert_eq!(across, alone);
    assert_eq!(
        across.iter().map(|(seed, ..)| *seed).collect::<Vec<_>>(),
        (0..120).collect::<Vec<_>>()
    );
    assert!(across.iter().any(|(_, shrunk, _)| shrunk.is_some()));
    assert!(across.iter().any(|(_, shrunk, _)| shrunk.is_none()));
}

#[test]
fn seeds_are_shrunk_on_every_thread() {
    let drawn_on: Arc<std::sync::Mutex<std::collections::HashSet<std::thread::ThreadId>>> =
        Arc::default();
    let seen = Arc::clone(&drawn_on);
    rerun(
        4,
        0..64,
        move |seed| {
            seen.lock()
                .expect("not poisoned")
                .insert(std::thread::current().id());
            vec![50, seed % 10]
        },
        over_twelve(),
        |case: &Vec<u64>| shorter(case),
        PATIENCE,
    );

    assert_eq!(drawn_on.lock().expect("not poisoned").len(), 4);
}

/// Ten never answers in time, and every even number breaks the volume rule.
fn hangs_at_ten() -> Check<u64> {
    Arc::new(|case: &u64| {
        if *case == 10 {
            std::thread::sleep(Duration::from_secs(2));
        }
        if case.is_multiple_of(2) {
            Err(volume_flaw())
        } else {
            Ok(())
        }
    })
}

#[test]
fn a_seed_that_hangs_is_named_so_and_the_others_are_still_shrunk() {
    let handed = rerun(
        2,
        [40, 21, 12, 10, 4],
        |seed| seed,
        hangs_at_ten(),
        |case: &u64| case.checked_sub(2).into_iter().collect(),
        Duration::from_millis(200),
    );

    let told: Vec<_> = handed
        .iter()
        .map(|rerun| match rerun {
            Rerun::Holds(seed) => (*seed, None, false),
            Rerun::Breaks { finding, hung } => (
                finding.seed,
                Some((finding.shrunk, finding.flaw.rule())),
                *hung,
            ),
        })
        .collect();
    assert_eq!(
        told,
        vec![
            (4, Some((0, Rule::Volume)), false),
            (10, Some((10, Rule::Answers)), true),
            (12, Some((12, Rule::Volume)), true),
            (21, None, false),
            (40, Some((12, Rule::Volume)), true),
        ]
    );
    let Rerun::Breaks { finding, .. } = &handed[1] else {
        unreachable!("ten breaks a rule")
    };
    assert!(matches!(finding.flaw, Flaw::NoAnswer(Silence::Late(_))));
}

#[test]
fn a_seed_that_cannot_be_drawn_stops_every_thread_from_taking_another() {
    let drawn = AtomicUsize::new(0);
    let outcome = std::panic::catch_unwind(AssertUnwindSafe(|| {
        rerun(
            2,
            0..10_000,
            |seed| {
                drawn.fetch_add(1, Ordering::SeqCst);
                assert_ne!(seed, 3, "seed three cannot be drawn");
                vec![50, seed % 10]
            },
            over_twelve(),
            |case: &Vec<u64>| shorter(case),
            PATIENCE,
        )
    }));

    assert!(outcome.is_err());
    assert!(drawn.load(Ordering::SeqCst) < 100, "{drawn:?}");
}
