//! A search for cases that break a rule, for as long as whoever runs it says.

use std::cell::Cell;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Arc;
use std::sync::mpsc;
use std::time::Duration;

use super::{Flaw, Rule, Silence, shrink};

/// What a check is: a case in, a rule it broke out.
pub type Check<C> = Arc<dyn Fn(&C) -> Result<(), Flaw> + Send + Sync>;

/// A case that broke a rule, as it was drawn and as small as it would go.
#[derive(Clone, Debug)]
pub struct Finding<C> {
    pub seed: u64,
    pub drawn: C,
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

/// How many shrunk cases are kept for one rule. The rest are counted: past a
/// few, the same failure found again says nothing a human needs to read.
const KEPT_PER_RULE: usize = 3;

/// The answer a check gives a case, run apart so that a kernel that panics or
/// never comes back is a finding rather than the end of the search.
///
/// A check that has not answered after `patience` is left running: a thread
/// cannot be stopped from outside, and the campaign stops soon after.
pub fn answer<C: Send + 'static>(
    case: C,
    check: &Check<C>,
    patience: Duration,
) -> Result<(), Flaw> {
    let (sender, receiver) = mpsc::channel();
    let check = Arc::clone(check);
    std::thread::spawn(move || {
        let outcome = catch_unwind(AssertUnwindSafe(|| check(&case)));
        let _ = sender.send(outcome);
    });
    match receiver.recv_timeout(patience) {
        Ok(Ok(verdict)) => verdict,
        Ok(Err(payload)) => Err(Flaw::NoAnswer(Silence::Panicked(
            payload
                .downcast_ref::<&str>()
                .map(|text| text.to_string())
                .or_else(|| payload.downcast_ref::<String>().cloned()),
        ))),
        Err(_) => Err(Flaw::NoAnswer(Silence::Late(patience))),
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
                    drawn,
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

#[cfg(test)]
mod tests;
