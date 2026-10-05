//! Many cases weighed at once, one thread to a core, so that a test going
//! through a hundred seeds does not hold the gate on one core while the others
//! wait.

use std::thread;

/// What `each` makes of every item, in the order of the items, the items dealt
/// out to every core in turn.
pub fn on_every_core<T: Sync, R: Send>(items: &[T], each: impl Fn(&T) -> R + Sync) -> Vec<R> {
    let cores = thread::available_parallelism().map_or(1, |cores| cores.get());
    let each = &each;
    let dealt: Vec<Vec<R>> = thread::scope(|scope| {
        let hands: Vec<_> = (0..cores)
            .map(|core| {
                scope.spawn(move || items.iter().skip(core).step_by(cores).map(each).collect())
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
    let mut dealt: Vec<_> = dealt.into_iter().map(Vec::into_iter).collect();
    (0..items.len())
        .map(|at| dealt[at % cores].next().expect("every item was weighed"))
        .collect()
}
