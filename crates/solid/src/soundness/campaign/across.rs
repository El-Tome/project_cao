//! A campaign whose seeds are handed to several threads at once: the same
//! search, on every core of the machine rather than one.

use std::cell::Cell;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Duration;

use super::{Check, Finding, KEPT_PER_RULE, Report, answer};
use crate::soundness::{Flaw, Rule, Silence, shrink};

/// What the threads share while they search: the seeds still to draw, what
/// was found, and how many shrinks of each rule are under way, so that no
/// more than a few are kept per rule however many threads find it at once.
struct Search<C> {
    failed: Vec<(u64, Rule)>,
    findings: Vec<Finding<C>>,
    shrinking: Vec<(Rule, usize)>,
}

/// Draws a case per seed and checks it, as [`campaign`](super::campaign)
/// does, on `threads` threads.
///
/// It names the same failing seeds, in seed order, and counts them the same.
/// Which cases are kept shrunk may differ when more than a few break one
/// rule, as the threads find them in another order; each is kept once. A
/// check that never came back stops every thread from drawing another seed:
/// the seeds already drawn are finished.
pub fn campaign_across<C: Clone + PartialEq + Send + 'static>(
    threads: usize,
    seeds: impl IntoIterator<Item = u64, IntoIter: Send>,
    draw: impl Fn(u64) -> C + Sync,
    check: Check<C>,
    smaller: impl Fn(&C) -> Vec<C> + Sync,
    patience: Duration,
    keep_going: impl FnMut() -> bool + Send,
) -> Report<C> {
    let seeds = Mutex::new(seeds.into_iter());
    let keep_going = Mutex::new(keep_going);
    let going = || (keep_going.lock().expect("a thread to have let go"))();
    let hung = AtomicBool::new(false);
    let tried = AtomicUsize::new(0);
    let search = Mutex::new(Search {
        failed: Vec::new(),
        findings: Vec::new(),
        shrinking: Vec::new(),
    });

    std::thread::scope(|scope| {
        for _ in 0..threads.max(1) {
            scope.spawn(|| {
                while !hung.load(Ordering::SeqCst) && going() {
                    let Some(seed) = seeds.lock().expect("a thread to have let go").next() else {
                        break;
                    };
                    let drawn = draw(seed);
                    tried.fetch_add(1, Ordering::SeqCst);
                    let Err(flaw) = answer(drawn.clone(), &check, patience) else {
                        continue;
                    };
                    let rule = flaw.rule();
                    let late = matches!(flaw, Flaw::NoAnswer(Silence::Late(_)));
                    if late {
                        hung.store(true, Ordering::SeqCst);
                    }
                    if !reserve(&search, seed, rule) {
                        continue;
                    }
                    let finding = shrunk(seed, drawn, flaw, &check, &smaller, patience, || {
                        !hung.load(Ordering::SeqCst) && going()
                    });
                    if finding.1 {
                        hung.store(true, Ordering::SeqCst);
                    }
                    keep(&search, finding.0);
                }
            });
        }
    });

    let Search {
        mut failed,
        mut findings,
        ..
    } = search.into_inner().expect("every thread to have let go");
    failed.sort_by_key(|(seed, _)| *seed);
    findings.sort_by_key(|finding| finding.seed);
    let mut broken: Vec<(Rule, usize)> = Vec::new();
    for (_, rule) in &failed {
        match broken.iter_mut().find(|(seen, _)| seen == rule) {
            Some((_, count)) => *count += 1,
            None => broken.push((*rule, 1)),
        }
    }
    Report {
        tried: tried.into_inner(),
        broken,
        findings,
        failed,
    }
}

/// Names the seed as failed, and says whether it is to be shrunk: no more
/// than a few cases per rule are, counting those other threads are shrinking.
fn reserve<C>(search: &Mutex<Search<C>>, seed: u64, rule: Rule) -> bool {
    let mut search = search.lock().expect("a thread to have let go");
    search.failed.push((seed, rule));
    let kept = search
        .findings
        .iter()
        .filter(|finding| finding.flaw.rule() == rule)
        .count();
    let under_way = search
        .shrinking
        .iter()
        .find(|(shrinking, _)| *shrinking == rule)
        .map_or(0, |(_, count)| *count);
    if kept + under_way >= KEPT_PER_RULE {
        return false;
    }
    match search
        .shrinking
        .iter_mut()
        .find(|(shrinking, _)| *shrinking == rule)
    {
        Some((_, count)) => *count += 1,
        None => search.shrinking.push((rule, 1)),
    }
    true
}

/// Keeps a finding unless another thread kept the same shrunk case, and ends
/// its shrink's reservation.
fn keep<C: PartialEq>(search: &Mutex<Search<C>>, finding: Finding<C>) {
    let mut search = search.lock().expect("a thread to have let go");
    let rule = finding.flaw.rule();
    if let Some((_, count)) = search
        .shrinking
        .iter_mut()
        .find(|(shrinking, _)| *shrinking == rule)
    {
        *count -= 1;
    }
    if !search
        .findings
        .iter()
        .any(|kept| kept.shrunk == finding.shrunk)
    {
        search.findings.push(finding);
    }
}

/// The case shrunk while it still breaks the rule the same way, as one
/// thread's campaign shrinks it, and whether a check met on the way never
/// came back.
fn shrunk<C: Clone + PartialEq + Send + 'static>(
    seed: u64,
    drawn: C,
    flaw: Flaw,
    check: &Check<C>,
    smaller: impl Fn(&C) -> Vec<C>,
    patience: Duration,
    going: impl Fn() -> bool,
) -> (Finding<C>, bool) {
    let hung = Cell::new(matches!(flaw, Flaw::NoAnswer(Silence::Late(_))));
    let mut last = flaw.clone();
    let shrunk = if hung.get() {
        drawn.clone()
    } else {
        shrink(
            drawn.clone(),
            &smaller,
            |candidate| match answer(candidate.clone(), check, patience) {
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
            || !hung.get() && going(),
        )
    };
    let shrunk_flaw = if shrunk == drawn { flaw.clone() } else { last };
    let finding = Finding {
        seed,
        drawn,
        flaw,
        shrunk,
        shrunk_flaw,
    };
    (finding, hung.get())
}

#[cfg(test)]
mod tests;
