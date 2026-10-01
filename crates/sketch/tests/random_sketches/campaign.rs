//! A search for drawings that break a rule, for as long as whoever runs it
//! says, and the shrinking of each one found.

use std::cell::Cell;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Arc;
use std::sync::mpsc;
use std::time::Duration;

use super::rules::{Flaw, Rule, Silence};

/// What a check is: a case in, a rule it broke out.
pub type Check<C> = Arc<dyn Fn(&C) -> Result<(), Flaw> + Send + Sync>;

/// A case that broke a rule, as it was drawn and as small as it would go.
#[derive(Clone, Debug)]
pub struct Finding<C> {
    pub seed: u64,
    pub flaw: Flaw,
    pub shrunk: C,
    pub shrunk_flaw: Flaw,
}

/// Everything a campaign turned up.
#[derive(Clone, Debug)]
pub struct Report<C> {
    pub tried: usize,
    /// How many of the cases tried broke each rule.
    pub broken: Vec<(Rule, usize)>,
    /// A few cases per rule, shrunk, the first ones found.
    pub findings: Vec<Finding<C>>,
}

/// The stack a check runs on: deep, so that a walk recursing without end is
/// the last thing to go rather than the program, and free until it is used.
const STACK: usize = 256 << 20;

/// How many shrunk cases are kept for one rule. The rest are counted: past a
/// few, the same failure found again says nothing a human needs to read.
const KEPT_PER_RULE: usize = 3;

/// Runs `work` on a thread of its own, so that a walk that panics or never
/// comes back is something to report rather than the end of the search.
///
/// Work that has not finished after `patience` is left running: a thread
/// cannot be stopped from outside, and the campaign stops soon after.
pub fn apart<T: Send + 'static>(
    work: impl FnOnce() -> T + Send + 'static,
    patience: Duration,
) -> Result<T, Silence> {
    let (sender, receiver) = mpsc::channel();
    std::thread::Builder::new()
        .stack_size(STACK)
        .spawn(move || {
            let _ = sender.send(catch_unwind(AssertUnwindSafe(work)));
        })
        .expect("a thread to run the check on");
    match receiver.recv_timeout(patience) {
        Ok(Ok(done)) => Ok(done),
        Ok(Err(payload)) => Err(Silence::Panicked(
            payload
                .downcast_ref::<&str>()
                .map(|text| text.to_string())
                .or_else(|| payload.downcast_ref::<String>().cloned()),
        )),
        Err(_) => Err(Silence::Late(patience)),
    }
}

/// The answer a check gives a case, run apart.
pub fn answer<C: Send + 'static>(
    case: C,
    check: &Check<C>,
    patience: Duration,
) -> Result<(), Flaw> {
    let check = Arc::clone(check);
    match apart(move || check(&case), patience) {
        Ok(verdict) => verdict,
        Err(silence) => Err(Flaw::NoAnswer(silence)),
    }
}

/// Draws a case per seed and checks it, until `keep_going` says to stop.
///
/// A case that fails is shrunk to the smallest one still failing the same
/// way, while the search is allowed to go on. A check that never came back —
/// drawn so, or met while shrinking — ends the campaign: its thread is still
/// running, and every case after it would be timed against a machine it is
/// slowing down.
pub fn campaign<C: Clone + PartialEq + Send + 'static>(
    seeds: impl IntoIterator<Item = u64>,
    draw: impl Fn(u64) -> C,
    check: Check<C>,
    smaller: impl Fn(&C) -> Vec<C>,
    patience: Duration,
    mut keep_going: impl FnMut() -> bool,
) -> Report<C> {
    let mut report = Report {
        tried: 0,
        broken: Vec::new(),
        findings: Vec::new(),
    };
    for seed in seeds {
        if !keep_going() {
            break;
        }
        let drawn = draw(seed);
        report.tried += 1;
        let Err(flaw) = answer(drawn.clone(), &check, patience) else {
            continue;
        };
        let rule = flaw.rule();
        match report.broken.iter_mut().find(|(broken, _)| *broken == rule) {
            Some((_, count)) => *count += 1,
            None => report.broken.push((rule, 1)),
        }
        let hung = Cell::new(matches!(flaw, Flaw::NoAnswer(Silence::Late(_))));
        let kept = report
            .findings
            .iter()
            .filter(|finding| finding.flaw.rule() == rule)
            .count();
        if kept < KEPT_PER_RULE {
            let mut last = flaw.clone();
            let shrunk = if hung.get() {
                drawn.clone()
            } else {
                shrink(
                    drawn.clone(),
                    &smaller,
                    |candidate| match answer(candidate.clone(), &check, patience) {
                        Err(broken) if broken.is_like(&flaw) => {
                            last = broken;
                            true
                        }
                        Err(Flaw::NoAnswer(Silence::Late(_))) => {
                            hung.set(true);
                            false
                        }
                        _ => false,
                    },
                    || !hung.get() && keep_going(),
                )
            };
            let shrunk_flaw = if shrunk == drawn { flaw.clone() } else { last };
            if !report
                .findings
                .iter()
                .any(|finding| finding.shrunk == shrunk)
            {
                report.findings.push(Finding {
                    seed,
                    flaw,
                    shrunk,
                    shrunk_flaw,
                });
            }
        }
        if hung.get() {
            break;
        }
    }
    report
}

/// Shrinks `case` for as long as one of the cases `smaller` offers still
/// fails, and hands back the last one that did.
///
/// Greedy: the first smaller case that still fails is taken, and the search
/// starts again from it. That does not find the smallest case there is, but it
/// finds one nothing offered can shrink further, which is what a human needs
/// to read — two rectangles rather than eleven gestures.
///
/// `still_fails` answers whether a case breaks the rule the original broke,
/// not merely whether it breaks something: a case shrunk into a different
/// failure is a different bug, and the report would describe neither.
pub fn shrink<C: Clone>(
    case: C,
    smaller: impl Fn(&C) -> Vec<C>,
    mut still_fails: impl FnMut(&C) -> bool,
    mut keep_going: impl FnMut() -> bool,
) -> C {
    let mut smallest = case;
    'search: while keep_going() {
        for candidate in smaller(&smallest) {
            if !keep_going() {
                break 'search;
            }
            if still_fails(&candidate) {
                smallest = candidate;
                continue 'search;
            }
        }
        break;
    }
    smallest
}
