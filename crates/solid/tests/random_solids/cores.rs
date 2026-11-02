//! Many cases weighed at once, one thread to a core, so that a test going
//! through a hundred seeds does not hold the gate on one core while the others
//! wait.

use std::iter;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

/// What `each` makes of every item, in the order of the items, each item
/// handed to the first core free: one slow item keeps its own core alone,
/// where items dealt out ahead in hands would have waited behind it (#630).
pub fn on_every_core<T: Sync, R: Send>(items: &[T], each: impl Fn(&T) -> R + Sync) -> Vec<R> {
    let cores = thread::available_parallelism().map_or(1, |cores| cores.get());
    let next = AtomicUsize::new(0);
    let (each, next) = (&each, &next);
    let taken: Vec<Vec<(usize, R)>> = thread::scope(|scope| {
        let hands: Vec<_> = (0..cores)
            .map(|_| {
                scope.spawn(move || {
                    iter::from_fn(|| {
                        let at = next.fetch_add(1, Ordering::Relaxed);
                        items.get(at).map(|item| (at, each(item)))
                    })
                    .collect()
                })
            })
            .collect();
        hands
            .into_iter()
            .map(|hand| {
                hand.join()
                    .unwrap_or_else(|panic| std::panic::resume_unwind(panic))
            })
            .collect()
    });
    let mut weighed: Vec<Option<R>> = items.iter().map(|_| None).collect();
    for (at, made) in taken.into_iter().flatten() {
        weighed[at] = Some(made);
    }
    weighed
        .into_iter()
        .map(|made| made.expect("every item was weighed"))
        .collect()
}
