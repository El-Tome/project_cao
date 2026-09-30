use std::sync::Arc;
use std::time::Duration;

use super::*;

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
fn a_check_that_panics_answers_with_the_panic() {
    let check: Check<u64> = Arc::new(|_| panic!("the partition never ends"));
    assert_eq!(
        answer(1, &check, PATIENCE),
        Err(Flaw::NoAnswer(Silence::Panicked(Some(
            "the partition never ends".to_string()
        ))))
    );
}

#[test]
fn a_check_that_never_comes_back_is_no_answer() {
    let check: Check<u64> = Arc::new(|_| {
        std::thread::sleep(Duration::from_secs(2));
        Ok(())
    });
    let verdict = answer(1, &check, Duration::from_millis(20));
    assert!(
        matches!(verdict, Err(Flaw::NoAnswer(Silence::Late(_)))),
        "{verdict:?}"
    );
}

#[test]
fn a_campaign_shrinks_what_it_finds_and_counts_the_rest() {
    let report = campaign(
        0..40,
        draw,
        sevens(),
        |case: &Vec<u64>| shorter(case),
        PATIENCE,
        || true,
    );

    assert_eq!(report.tried, 40);
    let broken: usize = report.broken.iter().map(|(_, count)| count).sum();
    assert_eq!(
        broken,
        (0..40).filter(|seed| draw(*seed).contains(&7)).count()
    );
    assert!(!report.findings.is_empty());
    for finding in &report.findings {
        assert_eq!(finding.shrunk, vec![7], "{finding:?}");
        assert_eq!(finding.shrunk_flaw.rule(), Rule::Volume);
    }
}

#[test]
fn the_same_failure_found_twice_is_kept_once() {
    let report = campaign(
        0..40,
        draw,
        sevens(),
        |case: &Vec<u64>| shorter(case),
        PATIENCE,
        || true,
    );
    assert_eq!(report.findings.len(), 1, "{:?}", report.findings);
}

#[test]
fn a_campaign_told_to_stop_tries_nothing_more() {
    let mut budget = 3;
    let report = campaign(
        0..40,
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

/// Sevens panic, and a case left with nothing but its sevens hangs.
fn sevens_panic_until_bare() -> Check<Vec<u64>> {
    Arc::new(|case: &Vec<u64>| {
        if case.is_empty() || !case.contains(&7) {
            return Ok(());
        }
        if case.iter().all(|number| *number == 7) {
            std::thread::sleep(Duration::from_secs(2));
            return Ok(());
        }
        panic!("seven")
    })
}

#[test]
fn a_panic_is_never_shrunk_into_a_case_that_hangs() {
    let report = campaign(
        [17],
        |_| vec![3, 7, 1],
        sevens_panic_until_bare(),
        |case: &Vec<u64>| shorter(case),
        Duration::from_millis(200),
        || true,
    );
    let finding = &report.findings[0];
    assert!(
        finding.shrunk.contains(&7) && finding.shrunk.len() == 2,
        "{finding:?}"
    );
    assert!(
        matches!(finding.shrunk_flaw, Flaw::NoAnswer(Silence::Panicked(_))),
        "{finding:?}"
    );
}

#[test]
fn a_case_that_hangs_while_shrinking_ends_the_campaign() {
    let report = campaign(
        0..40,
        |_| vec![3, 7, 1],
        sevens_panic_until_bare(),
        |case: &Vec<u64>| shorter(case),
        Duration::from_millis(200),
        || true,
    );
    assert_eq!(report.tried, 1, "{report:?}");
}
